use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot;


pub struct SubsBroadcast<T>(Vec<oneshot::Sender<T>>)
where
    T: Clone;


impl<T> Default for SubsBroadcast<T>
where
    T: Clone,
{
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T> SubsBroadcast<T>
where
    T: Clone,
{
    pub fn subscription(&mut self) -> oneshot::Receiver<T> {
        let (tx, rx) = oneshot::channel();
        self.0.push(tx);
        rx
    }

    #[must_use]
    pub fn broadcast(self, data: T) -> bool {
        let mut is_ok = true;

        for waiter in self.0 {
            is_ok = waiter.send(data.clone()).is_ok() && is_ok;
        }
        is_ok
    }
}


pub struct SubsSinge<T>(oneshot::Sender<T>, Option<oneshot::Receiver<T>>);

impl<T> SubsSinge<T> {
    #[inline]
    pub fn subscription(&mut self) -> oneshot::Receiver<T> {
        self.1.take().unwrap()
    }

    pub fn broadcast(self, data: T) -> Result<(), T> {
        self.0.send(data)
    }
}


impl<T> Default for SubsSinge<T> {
    fn default() -> Self {
        let (tx, rx) = oneshot::channel();
        Self(tx, Some(rx))
    }
}


pub struct SubsChannel<T> {
    pub sender: UnboundedSender<T>,
    pub receiver: UnboundedReceiver<T>,
}


impl<T> Default for SubsChannel<T> {
    fn default() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel::<T>();
        Self {
            sender,
            receiver,
        }
    }
}
