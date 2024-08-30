use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use parking_lot::{ArcRwLockWriteGuard, RawRwLock, RwLock};
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

use crate::engine::Engine;
use crate::event::{Event, EventKind};
use crate::metadata::Metadata;
use crate::task::TaskStatus;

mod engine;
pub use engine::*;

type EventSender<E> = mpsc::UnboundedSender<Event<E>>;
type EventReceiver<E> = mpsc::UnboundedReceiver<Event<E>>;

pub struct Context<E: Engine> {
    metadata: RwLock<Metadata<E>>,
    event_tx: EventSender<E>,
}

impl<E: Engine> Context<E> {
    fn new(event_tx: EventSender<E>) -> Self {
        Self {
            metadata: Default::default(),
            event_tx,
        }
    }

    fn send_event(&self, event: Event<E>) {
        self.event_tx.send(event).expect("event channel closed");
    }
}

impl<E: Engine> Context<E> {
    #[inline]
    pub fn metadata(&self) -> impl Deref<Target = Metadata<E>> {
        self.metadata.read()
    }
}

pub struct RuntimeCtx<E: Engine> {
    inner: Context<E>,
    engine: E,
    event_rx: EventReceiver<E>,
}

impl<E: Engine> RuntimeCtx<E> {
    fn new(engine: E) -> Self {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let ctx = Context::new(event_tx);
        Self {
            inner: ctx,
            engine,
            event_rx,
        }
    }

    async fn init(&self) -> Result<(), E::Error> {
        // let task_graph = self.engine.init(self.subcontext()).await;
        // self.metadata.write().task_graph = self.engine.init(self).await?;
        Ok(())
    }
}

impl<E: Engine> RuntimeCtx<E> {
    pub async fn run_task(&self, task: ()) {
        todo!()
    }

    pub async fn abort_task(&self, task: ()) {
        todo!()
    }

    pub async fn abort_all_tasks(&self) {
        todo!()
    }
}

impl<E: Engine> Deref for RuntimeCtx<E> {
    type Target = Context<E>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct EngineCtx<'ctx, E: Engine> {
    inner: &'ctx Context<E>,
    task: Option<()>,
}

impl<'ctx, E: Engine> EngineCtx<'ctx, E> {
    const fn new(ctx: &'ctx Context<E>, task: Option<()>) -> Self {
        Self { inner: ctx, task }
    }

    pub fn log(&self, message: impl ToString) {
        self.send_event(Event::new(
            EventKind::Log(message.to_string()),
            self.task.clone(),
        ));
    }

    pub fn report_error(&self, error: E::Error) {
        self.send_event(Event::new(EventKind::Error(error), self.task.clone()));
    }

    pub fn emit_event(&self, event: E::Event) {
        self.send_event(Event::new(EventKind::Other(event), self.task.clone()));
    }

    #[inline]
    pub fn capture<T>(&self, f: impl FnOnce() -> Result<T, E::Error>) -> Option<T> {
        match f() {
            Ok(val) => Some(val),
            Err(err) => {
                self.report_error(err);
                None
            }
        }
    }
}

impl<'ctx, E: Engine> Deref for EngineCtx<'ctx, E> {
    type Target = Context<E>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
