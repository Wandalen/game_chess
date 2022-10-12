use std::future::Future;
use bevy::prelude::Component;
use bevy::tasks::
{
  AsyncComputeTaskPool,
  Task
};
use futures_lite::future;

///
/// Component that executes a future on bevy's Task pool.
///

#[ derive( Component, Debug ) ]
pub struct AsyncTask< O >
{
  task : Task< O >,
}

impl< O : Send + 'static > AsyncTask< O >
{
  ///
  /// Spawns 'future' on the bevy's AsyncComputeTaskPool.
  ///

  pub fn spawn( future : impl Future< Output = O > + Send + 'static ) -> Self
  {
    let task = AsyncComputeTaskPool::get().spawn( future );

    AsyncTask{ task }
  }

  ///
  /// Returns the future's output if it's ready, otherwise None.
  ///
  /// # Panics
  ///
  /// Panics if called after returning the future's output.

  pub fn result( &mut self ) -> Option< O >
  {
    future::block_on( future::poll_once( &mut self.task ) )
  }
}
