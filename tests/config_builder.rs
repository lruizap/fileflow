use std::fs;

use fileflow_actions::{build_pipeline_from_config, build_pipeline_from_config_file, PipelineConfig, StepConfig};
use fileflow_core::{Engine, JobStatus};

#[test]
fn build_pipeline_from_config_runs_successfully() {
    let config = PipelineConfig {
        name: "demo".to_string(),
        steps: vec![
            StepConfig {
                action: "echo".to_string(),
                args: vec![],
            },
            StepConfig {
                action: "echo".to_string(),
                args: vec![],
            },
        ],
    };

    let action = build_pipeline_from_config(&config).expect("config should build");
    let engine = Engine::new();
    let out = engine.run_action(action.as_ref());

    assert!(matches!(out.job.status, JobStatus::Success));
    assert!(out.logs.iter().any(|l| l.message.contains("PipelineAction")));
}

#[test]
fn build_pipeline_from_config_file_runs_move_successfully() {
    let dir = tempfile::tempdir().unwrap();

    let src = dir.path().join("src.txt");
    let dst = dir.path().join("dst.txt");
    let json_path = dir.path().join("pipeline.json");

    fs::write(&src, "hola desde json").unwrap();

    let json = format!(
        r#"{{
  "name": "move_demo",
  "steps": [
    {{
      "action": "move",
      "args": ["--src", "{}", "--dst", "{}", "--overwrite"]
    }}
  ]
}}"#,
        src.to_string_lossy(),
        dst.to_string_lossy()
    );

    fs::write(&json_path, json).unwrap();

    let action = build_pipeline_from_config_file(&json_path).expect("json config should build");
    let engine = Engine::new();
    let out = engine.run_action(action.as_ref());

    assert!(matches!(out.job.status, JobStatus::Success));
    assert!(!src.exists());
    assert_eq!(fs::read_to_string(&dst).unwrap(), "hola desde json");
}

#[test]
fn build_pipeline_from_config_file_fails_with_unknown_action() {
    let dir = tempfile::tempdir().unwrap();
    let json_path = dir.path().join("bad_pipeline.json");

    let json = r#"
{
  "name": "bad_demo",
  "steps": [
    {
      "action": "unknown_action",
      "args": []
    }
  ]
}
"#;

    fs::write(&json_path, json).unwrap();

    let err = build_pipeline_from_config_file(&json_path).unwrap_err();
    assert!(err.to_string().contains("Action not found"));
}