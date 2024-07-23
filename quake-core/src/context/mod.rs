use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::{FairMutex, Mutex, RwLock, RwLockReadGuard};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::engine::Engine;
use crate::events::{Event, Subscriber};
use crate::tasks::TaskGraph;

mod metadata;
use self::metadata::Metadata;

// pub type Ctx<'ctx, E> = &'ctx Context<E>;

// #[derive(Clone)]
pub struct Context<'ctx, E: Engine<'ctx>> {
    // TODO Arc? why nest? size?
    // can we use lock_api/arc_lock to map inner
    inner: Box<ContextInner<'ctx, E>>,
}

impl<'ctx, E: Engine<'ctx>> Context<'ctx, E> {
    #[inline]
    pub fn engine(&self) -> &E {
        &self.inner.engine
    }

    #[inline]
    pub fn engine_mut(&mut self) -> &mut E {
        &mut self.inner.engine
    }

    #[inline]
    pub fn metadata(&self) -> impl 'ctx + Deref<Target = Metadata<E::Task>> {
        self.inner.metadata.read_arc()
    }

    #[inline]
    pub fn try_metadata(&self) -> Option<impl 'ctx + Deref<Target = Metadata<E::Task>>> {
        self.inner.metadata.try_read_arc()
    }

    #[inline]
    pub fn metadata_mut(&self) -> impl 'ctx + DerefMut<Target = Metadata<E::Task>> {
        self.inner.metadata.write_arc()
    }

    #[inline]
    pub fn try_metadata_mut(&self) -> Option<impl 'ctx + DerefMut<Target = Metadata<E::Task>>> {
        self.inner.metadata.try_write_arc()
    }

    async fn init(&self) -> Result<(), E::Error> {
        let task_graph = self.inner.engine.init(&self).await?; // FIXME engine-aware error
        self.inner.metadata.write().task_graph = task_graph;
        Ok(())
    }

    async fn send_event(&self, event: &Event<'ctx, E>) -> crate::Result<()> {
        for sub in &*self.inner.subscribers {
            sub.recv_event(&self, event).await?;
        }
        Ok(())
    }
}

struct ContextInner<'ctx, E>
where
    Self: 'ctx,
    E: Engine<'ctx>,
{
    engine: E,
    metadata: Arc<RwLock<Metadata<E::Task>>>,
    subscribers: Vec<Box<dyn Subscriber<'ctx, E>>>,
}

// pub struct CtxHandle<'ctx, E> where: E: Engine<'ctx> {
//     inner: &'ctx Context<'ctx, E>
// }

pub struct ContextBuilder<'ctx, E: Engine<'ctx>> {
    subscribers: Vec<Box<dyn Subscriber<'ctx, E>>>,
}

impl<'ctx, E: Engine<'ctx>> ContextBuilder<'ctx, E> {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn add_subscriber(&mut self, subscriber: Box<dyn Subscriber<'ctx, E>>) {
        self.subscribers.push(subscriber);
    }

    #[inline]
    pub fn add_subscribers<I>(&mut self, subscribers: I)
    where
        I: IntoIterator<Item = Box<dyn Subscriber<'ctx, E>>>,
    {
        self.subscribers.extend(subscribers);
    }

    #[inline]
    #[must_use]
    pub fn with_subscriber(mut self, subscriber: Box<dyn Subscriber<'ctx, E>>) -> Self {
        self.add_subscriber(subscriber);
        self
    }

    #[inline]
    #[must_use]
    pub fn with_subscribers<I>(mut self, subscribers: I) -> Self
    where
        I: IntoIterator<Item = Box<dyn Subscriber<'ctx, E>>>,
    {
        self.add_subscribers(subscribers);
        self
    }

    #[inline]
    #[must_use]
    pub async fn init(mut self, engine: E) -> Result<Context<'ctx, E>, E::Error> {
        // TODO engine install should (?) take priority over external builder
        // ...unless we introduce mechanisms which permit interception of events
        engine.install(&mut self);

        let context = Context {
            inner: Box::new(ContextInner {
                engine,
                metadata: Default::default(),
                subscribers: self.subscribers,
            }),
        };
        context.init().await?;

        Ok(context)
    }
}

impl<'ctx, E: Engine<'ctx>> Default for ContextBuilder<'ctx, E> {
    fn default() -> Self {
        Self {
            subscribers: Default::default(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct TaskContext<'ctx, E: Engine<'ctx> + ?Sized> {
    phantom: PhantomData<&'ctx E>,
}

// impl<'ctx, E: Engine + ?Sized> TaskContext<'ctx, E> {}
