//! Executors
//!
//! This module contains tools for managing the raw execution of futures,
//! which is needed when building *executors* (places where futures can run).
//!
//! More information about executors can be [found online at tokio.rs][online].
//!
//! [online]: https://tokio.rs/docs/going-deeper-futures/tasks/

#[cfg(feature = "use_std")]
pub mod current_thread;

#[cfg(feature = "use_std")]
mod enter;

#[allow(deprecated)]
#[cfg(feature = "use_std")]
pub use task_impl::{Unpark, Executor, Run};

pub use task_impl::{Spawn, spawn, Notify, with_notify};

pub use task_impl::{UnsafeNotify, NotifyHandle};

#[cfg(feature = "use_std")]
pub use self::current_thread::CurrentThread;

#[cfg(feature = "use_std")]
pub use self::enter::{enter, Enter};
