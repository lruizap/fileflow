#[cfg(test)]
mod tests {
    use crate::{Action, Context, Engine, Progress, Result};

    struct EchoAction;

    impl Action for EchoAction {
        fn name(&self) -> &'static str {
            "echo"
        }

        fn execute(&self, ctx: &mut Context) -> Result<()> {
            ctx.info("Hola desde EchoAction");
            ctx.set_progress(Progress::new(1, 1).with_message("Done"));
            Ok(())
        }
    }

    #[test]
    fn engine_runs_action_and_returns_success() {
        let engine = Engine::new();
        let action = EchoAction;

        let out = engine.run_action(&action);

        assert_eq!(out.job.action_name, "echo");
        assert!(matches!(out.job.status, crate::JobStatus::Success));
        assert!(!out.logs.is_empty());
        assert!(out.job.progress.is_some());
    }
}
