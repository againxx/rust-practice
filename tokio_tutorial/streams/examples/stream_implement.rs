use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, Instant};
use tokio_stream::Stream;

struct Delay {
    when: Instant,
    waker: Option<Arc<Mutex<Waker>>>,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            let waker = cx.waker().clone();
            let when = self.when;

            thread::spawn(move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }
                waker.wake();
            });

            Poll::Pending
        }
    }
}

struct Interval {
    rem: usize,
    delay: Delay,
}

impl Stream for Interval {
    type Item = ();

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.rem == 0 {
            return Poll::Ready(None);
        }

        match Pin::new(&mut self.delay).poll(cx) {
            Poll::Ready(_) => {
                let when = self.delay.when + Duration::from_millis(10);
                self.delay = Delay { when };
                self.rem -= 1;
                Poll::Ready(Some(()))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
