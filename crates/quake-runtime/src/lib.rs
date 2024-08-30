use std::borrow::Cow;
use std::marker::PhantomData;

use quake_core::context::Context;
use quake_core::engine::Engine;

pub mod scheduler;

pub struct Runtime<E: Engine> {
    context: Context<E>,
}

#[derive(Clone)]
struct RuntimeOptions {
    #[cfg(feature = "multi-thread")]
    num_threads: Option<usize>,
    #[cfg(feature = "multi-thread")]
    max_concurrent: Option<usize>,
}

impl Default for RuntimeOptions {
    fn default() -> Self {
        Self {
            num_threads: None,
            max_concurrent: None,
        }
    }
}

pub struct RuntimeBuilder<E: Engine> {
    options: RuntimeOptions,
    _marker: PhantomData<E>,
}

impl<E: Engine> RuntimeBuilder<E> {
    #[inline]
    pub const fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "multi-thread")]
    #[inline]
    pub const fn with_num_threads(mut self, num_threads: usize) -> Self {
        self.options.num_threads = Some(num_threads);
        self
    }

    #[cfg(feature = "multi-thread")]
    #[inline]
    pub const fn with_max_concurrent(mut self, max_concurrent: usize) -> Self {
        self.options.max_concurrent = Some(max_concurrent);
        self
    }
}

impl<E> Default for RuntimeBuilder<E> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

pub trait Scheduler: Subscriber {
    // TODO: make subcontext?
    async fn run<'rt>(&mut self, rt: RuntimeHandle<'rt>, ctx: &'a Context<E>) -> Result<()>;
}
