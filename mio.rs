use futures::task::noop_waker_ref;
use std::future::Future;
use std::io::{prelude::*, ErrorKind::WouldBlock};
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::mpsc::{channel, Sender};
use std::sync::{LazyLock, Mutex, OnceLock};
use std::task::{Context, Poll, Waker};

static WAKERS: Mutex<Vec<Waker>> = Mutex::new(Vec::new());
static TASK_SENDER: OnceLock<Sender<BoxedFuture>> = OnceLock::new();
static MIO_POLL: LazyLock<Mutex<mio::Poll>> =
    LazyLock::new(|| Mutex::new(mio::Poll::new().unwrap()));

type BoxedFuture = Pin<Box<dyn Future<Output = ()> + Send>>;

fn spawn_task<F: Future<Output = ()> + Send + 'static>(future: F) {
    TASK_SENDER.get().unwrap().send(Box::pin(future)).unwrap();
}

fn bind(addr: SocketAddr) -> anyhow::Result<mio::net::TcpListener> {
    let mut listener = mio::net::TcpListener::bind(addr)?;
    MIO_POLL.lock().unwrap().registry().register(
        &mut listener,
        mio::Token(0), // We ignore tokens and wake all tasks on any event.
        mio::Interest::READABLE,
    )?;
    Ok(listener)
}

async fn accept(
    listener: &mut mio::net::TcpListener,
) -> anyhow::Result<(mio::net::TcpStream, SocketAddr)> {
    let future = futures::future::poll_fn(|context| match listener.accept() {
        Ok((mut stream, addr)) => {
            MIO_POLL.lock().unwrap().registry().register(
                &mut stream,
                mio::Token(0), // We ignore tokens and wake all tasks on any event.
                mio::Interest::READABLE,
            )?;
            Poll::Ready(Ok((stream, addr)))
        }
        Err(e) if e.kind() == WouldBlock => {
            WAKERS.lock().unwrap().push(context.waker().clone());
            Poll::Pending
        }
        Err(e) => Poll::Ready(Err(e.into())),
    });
    future.await
}

async fn read(stream: &mut mio::net::TcpStream, buf: &mut [u8]) -> anyhow::Result<usize> {
    let future = futures::future::poll_fn(|context| match stream.read(buf) {
        Ok(n) => Poll::Ready(Ok(n)),
        Err(e) if e.kind() == WouldBlock => {
            WAKERS.lock().unwrap().push(context.waker().clone());
            Poll::Pending
        }
        Err(e) => Poll::Ready(Err(e.into())),
    });
    future.await
}

async fn echo_stream(mut stream: mio::net::TcpStream, addr: SocketAddr) -> anyhow::Result<()> {
    loop {
        let mut buf = [0; 1024];
        let n = read(&mut stream, &mut buf).await?;
        if n == 0 {
            return Ok(());
        }
        println!("read({}): \"{}\"", addr.port(), buf[..n].escape_ascii());
        // Quick and dirty: assume this write won't block.
        writeln!(&mut stream, "echo: \"{}\"", buf[..n].escape_ascii())?;
    }
}

async fn echo_listener() -> anyhow::Result<()> {
    let addr = "127.0.0.1:8000".parse()?;
    let mut listener = bind(addr)?;
    loop {
        let (stream, addr) = accept(&mut listener).await?;
        println!("connection opened: {addr}");
        spawn_task(async move {
            echo_stream(stream, addr).await.expect("stream error");
        });
    }
}

fn main() -> anyhow::Result<()> {
    let (task_sender, task_receiver) = channel();
    TASK_SENDER.set(task_sender).unwrap();
    let mut context = Context::from_waker(noop_waker_ref());
    let main_task = async {
        echo_listener().await.expect("listener error");
    };
    let mut tasks: Vec<BoxedFuture> = vec![Box::pin(main_task)];
    let mut events = mio::Events::with_capacity(1);
    loop {
        // Poll all existing tasks, removing any that are finished.
        let is_pending = |task: &mut BoxedFuture| task.as_mut().poll(&mut context).is_pending();
        tasks.retain_mut(is_pending);
        // Any of the tasks we just polled might've called spawn_task() internally. Drain the
        // TASK_SENDER channel into our local tasks Vec.
        while let Ok(mut task) = task_receiver.try_recv() {
            // Poll each new tasks once, and keep the ones that are pending.
            if task.as_mut().poll(&mut context).is_pending() {
                tasks.push(task);
            }
        }
        // If there are no tasks left, we're done!
        if tasks.is_empty() {
            break;
        }
        // Block on mio::Poll until something is readable.
        MIO_POLL.lock().unwrap().poll(
            &mut events,
            None, // We don't sleep in this example, so there's no timeout.
        )?;
        // Invoke all the wakers. A real async runtime would track the relationships between IO
        // events, wakers, and tasks, but it doesn't matter in this example, because our main loop
        // always polls every task. In fact, this example would work even if we ignored wakers
        // entirely, because we're not using JoinAll/FuturesOrdered/etc. But we're going to be good
        // async citizens and uphold the Poll::Pending contract anyway.
        WAKERS.lock().unwrap().drain(..).for_each(Waker::wake);
    }
    Ok(())
}
