use fileflow_core::{Action, Context, Progress, Result};

pub struct EchoAction;

impl Action for EchoAction {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn execute(&self, ctx: &mut Context) -> Result<()> {
        ctx.info("EchoAction: inicio");
        ctx.set_progress(Progress::new(1, 3).with_message("Preparando..."));

        ctx.info("EchoAction: trabajando...");
        ctx.set_progress(Progress::new(2, 3).with_message("A mitad..."));

        ctx.info("EchoAction: final");
        ctx.set_progress(Progress::new(3, 3).with_message("Terminado"));

        Ok(())
    }
}
