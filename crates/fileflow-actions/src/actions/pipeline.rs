use fileflow_core::{Action, Context, Progress, Result};

pub struct PipelineAction {
    steps: Vec<Box<dyn Action>>,
}

impl PipelineAction {
    pub fn new(steps: Vec<Box<dyn Action>>) -> Self {
        Self { steps }
    }
}

impl Action for PipelineAction {
    fn name(&self) -> &'static str {
        "pipeline"
    }

    fn execute(&self, ctx: &mut Context) -> Result<()> {
        let total_steps = self.steps.len() as u64;

        if total_steps == 0 {
            ctx.warn("PipelineAction: no steps configured");
            ctx.set_progress(Progress::new(0, 0).with_message("Pipeline vacío"));
            return Ok(());
        }

        ctx.info(format!("PipelineAction: iniciando {} step(s)", total_steps));
        ctx.set_progress(Progress::new(0, total_steps).with_message("Iniciando pipeline"));

        for (index, step) in self.steps.iter().enumerate() {
            ctx.ensure_not_cancelled()?;

            let current_step = index as u64 + 1;
            let step_name = step.name();

            ctx.info(format!(
                "PipelineAction: step {}/{} -> {}",
                current_step, total_steps, step_name
            ));

            ctx.set_progress(
                Progress::new(index as u64, total_steps)
                    .with_message(format!("Ejecutando step: {}", step_name)),
            );

            step.execute(ctx)?;

            ctx.info(format!(
                "PipelineAction: step completado {}/{} -> {}",
                current_step, total_steps, step_name
            ));

            ctx.set_progress(
                Progress::new(current_step, total_steps)
                    .with_message(format!("Completado: {}", step_name)),
            );
        }

        ctx.info("PipelineAction: finalizado correctamente");
        Ok(())
    }
}