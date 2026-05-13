use fileflow_core::{Action, Context, Engine, FileFlowError, JobStatus, Progress, Result};

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

struct OkAction;

impl Action for OkAction {
    fn name(&self) -> &'static str {
        "ok_action"
    }

    fn execute(&self, ctx: &mut Context) -> Result<()> {
        ctx.info("ok: start");
        ctx.set_progress(Progress::new(1, 2).with_message("half"));
        ctx.info("ok: end");
        ctx.set_progress(Progress::new(2, 2).with_message("done"));
        Ok(())
    }
}

struct FailAction;

impl Action for FailAction {
    fn name(&self) -> &'static str {
        "fail_action"
    }

    fn execute(&self, ctx: &mut Context) -> Result<()> {
        ctx.error("fail: boom");
        Err(FileFlowError::Message("boom".to_string()))
    }
}

struct CancelAction;

impl Action for CancelAction {
    fn name(&self) -> &'static str {
        "cancel_action"
    }

    fn execute(&self, ctx: &mut Context) -> Result<()> {
        ctx.info("cancel: requesting cancel");
        ctx.cancel();
        ctx.ensure_not_cancelled()?;
        Ok(())
    }
}

#[test]
fn engine_runs_action_and_returns_success() {
    let engine = Engine::new();
    let action = EchoAction;

    let out = engine.run_action(&action);

    assert_eq!(out.job.action_name, "echo");
    assert!(matches!(out.job.status, JobStatus::Success));
    assert!(!out.logs.is_empty());
    assert!(out.job.progress.is_some());
}

#[test]
fn engine_runs_ok_action_success() {
    let engine = Engine::new();
    let out = engine.run_action(&OkAction);

    assert_eq!(out.job.action_name, "ok_action");
    assert!(matches!(out.job.status, JobStatus::Success));
    assert!(out.job.started_at.is_some());
    assert!(out.job.finished_at.is_some());

    let progress = out.job.progress.expect("expected progress");
    assert_eq!(progress.current, 2);
    assert_eq!(progress.total, 2);
    assert_eq!(progress.message.as_deref(), Some("done"));

    assert!(out.logs.iter().any(|l| l.message.contains("ok: start")));
    assert!(out.logs.iter().any(|l| l.message.contains("ok: end")));
}

#[test]
fn engine_runs_fail_action_failed_status_contains_message() {
    let engine = Engine::new();
    let out = engine.run_action(&FailAction);

    match out.job.status {
        JobStatus::Failed(message) => assert!(message.contains("boom")),
        other => panic!("expected Failed, got: {:?}", other),
    }

    assert!(out.logs.iter().any(|l| l.message.contains("fail: boom")));
}

#[test]
fn engine_runs_cancel_action_cancelled_status() {
    let engine = Engine::new();
    let out = engine.run_action(&CancelAction);

    assert!(matches!(out.job.status, JobStatus::Cancelled));
    assert!(out
        .logs
        .iter()
        .any(|l| l.message.contains("requesting cancel")));
}
