use std::future::Future;
use bevy::prelude::Component;
use bevy::tasks::AsyncComputeTaskPool;
use futures_channel::oneshot;

///
/// Component that executes a future on bevy's Task pool.
///

#[ derive( Component, Debug ) ]
pub struct AsyncTask< O >
{
  receiver : oneshot::Receiver< O >,
}

impl< O : Send + 'static > AsyncTask< O >
{
  ///
  /// Spawns 'future' on the bevy's AsyncComputeTaskPool.
  ///

  pub fn spawn( future : impl Future< Output = O > + Send + 'static ) -> Self
  {
    let ( sender, receiver ) = oneshot::channel();
    AsyncComputeTaskPool::get().spawn
    (
      async
      {
        if sender.send( future.await ).is_err()
        {
          // TODO: logging
        }
      }
    )
    .detach();

    AsyncTask{ receiver }
  }

  ///
  /// Returns the future's output if it's ready, otherwise None.
  ///
  /// # Panics
  ///
  /// Panics if called after returning the future's output.

  pub fn result( &mut self ) -> Option< O >
  {
    self.receiver.try_recv().unwrap()
  }
}
