pub trait Engine<'ctx>: 'ctx {
    fn load(&mut self, ctx: &'ctx Context) -> Result<()>;

    async fn run_task(
        &self,
        ctx: &'ctx Context,
        task: TaskRef<'ctx>,
    ) -> Result<bool>;

    fn abort_task(
        &self,
        ctx: &'ctx Context,
        task: TaskRef<'ctx>,
    ) -> Result<()>;

    fn abort_all_tasks(&self, ctx: &'ctx Context) -> Result<()>;
}
