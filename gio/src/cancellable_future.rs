// Take a look at the license at the top of the repository in the LICENSE file.

use crate::cancellable::CancellableExtManual;
use crate::cancellable::CancelledHandlerId;
use crate::prelude::CancellableExt;
use crate::Cancellable;
use crate::IOErrorEnum;
use pin_project_lite::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

pin_project! {
    // rustdoc-stripper-ignore-next
    /// A future which can be cancelled via [`Cancellable`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use futures::FutureExt;
    /// # use gio::traits::CancellableExt;
    /// # use gio::CancellableFuture;
    /// let l = glib::MainLoop::new(None, false);
    /// let c = gio::Cancellable::new();
    ///
    /// l.context().spawn_local(CancellableFuture::new(async { 42 }, c.clone()).map(|_| ()));
    /// c.cancel();
    ///
    /// ```
    pub struct CancellableFuture<F> {
        #[pin]
        future: F,

        #[pin]
        waker_signal_id: Option<CancelledHandlerId>,

        cancellable: Cancellable,
    }
}

impl<F> CancellableFuture<F> {
    // rustdoc-stripper-ignore-next
    /// Creates a new `CancellableFuture` using a [`Cancellable`].
    ///
    /// When [`cancel`](CancellableExt::cancel) is called, the future will complete
    /// immediately without making any further progress. In such a case, an error
    /// will be returned by this future (i.e., [`IOErrorEnum::Cancelled`]).
    pub fn new(future: F, cancellable: Cancellable) -> Self {
        Self {
            future,
            waker_signal_id: None,
            cancellable,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Checks whether the future has been cancelled.
    ///
    /// Note that all this method indicates is whether [`cancel`](CancellableExt::cancel)
    /// was called. This means that it will return true even if:
    ///   * `cancel` was called after the future had completed.
    ///   * `cancel` was called while the future was being polled.
    #[inline]
    pub fn is_cancelled(&self) -> bool {
        self.cancellable.is_cancelled()
    }
}

impl<F> Future for CancellableFuture<F>
where
    F: Future,
{
    type Output = Result<<F as Future>::Output, glib::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.is_cancelled() {
            return Poll::Ready(Err(glib::Error::new(
                IOErrorEnum::Cancelled,
                "Task cancelled",
            )));
        }

        let mut this = self.as_mut().project();

        match this.future.poll(cx) {
            Poll::Ready(out) => Poll::Ready(Ok(out)),

            Poll::Pending => {
                if let Some(prev_signal_id) = this.waker_signal_id.take() {
                    this.cancellable.disconnect_cancelled(prev_signal_id);
                }

                let canceller_handler_id = this.cancellable.connect_cancelled({
                    let w = cx.waker().clone();
                    move |_| w.wake()
                });

                match canceller_handler_id {
                    Some(canceller_handler_id) => {
                        *this.waker_signal_id = Some(canceller_handler_id);
                        Poll::Pending
                    }

                    None => Poll::Ready(Err(glib::Error::new(
                        IOErrorEnum::Cancelled,
                        "Task cancelled",
                    ))),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cancellable;
    use super::CancellableExt;
    use super::CancellableFuture;
    use super::IOErrorEnum;
    use futures_channel::oneshot;

    #[test]
    fn cancellable_future_ok() {
        let ctx = glib::MainContext::new();
        let c = Cancellable::new();
        let (tx, rx) = oneshot::channel();

        {
            ctx.spawn_local(async {
                let cancellable_future = CancellableFuture::new(async { 42 }, c);
                assert!(!cancellable_future.is_cancelled());

                let result = cancellable_future.await;
                assert!(matches!(result, Ok(42)));

                tx.send(()).unwrap();
            });
        }

        ctx.block_on(rx).unwrap()
    }

    #[test]
    fn cancellable_future_cancel() {
        let ctx = glib::MainContext::new();
        let c = Cancellable::new();
        let (tx, rx) = oneshot::channel();

        {
            let c = c.clone();
            ctx.spawn_local(async move {
                let error = CancellableFuture::new(std::future::pending::<()>(), c)
                    .await
                    .unwrap_err();

                assert!(error.matches(IOErrorEnum::Cancelled));
                tx.send(()).unwrap();
            });
        }

        std::thread::spawn(move || c.cancel()).join().unwrap();

        ctx.block_on(rx).unwrap();
    }
}
