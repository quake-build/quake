// TODO E gen 'ctx, Cow or Yoke?
// TODO separate scheduler?

use std::borrow::Cow;
use std::marker::PhantomData;

use quake_core::engine::Engine;

// TODO think this over
// pub mod scheduler;

pub struct RuntimeBuilder<E: Engine> {
    options: RuntimeOptions,
    phantom: PhantomData<E>,
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

pub struct Runtime<E> {
    // context: Cow,
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
