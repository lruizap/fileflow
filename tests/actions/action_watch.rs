use std::fs;

use fileflow_actions::build_action;
use fileflow_core::{Engine, JobStatus};

#[test]
fn watch_action_once_runs_pipeline_config() {
    let dir = tempfile::tempdir().unwrap();

    let watch_dir = dir.path().join("watch");
    let src = dir.path().join("src");
    let dst = dir.path().join("dst");
    let config = dir.path().join("pipeline.json");

    fs::create_dir_all(&watch_dir).unwrap();
    fs::create_dir_all(&src).unwrap();

    fs::write(src.join("a.txt"), "watch sync").unwrap();

    let json = format!(
        r#"{{
  "name": "watch_once_demo",
  "steps": [
    {{
      "action": "sync",
      "args": ["--src", "{}", "--dst", "{}", "--recursive"]
    }}
  ]
}}"#,
        src.to_string_lossy(),
        dst.to_string_lossy()
    );

    fs::write(&config, json).unwrap();

    let args = vec![
        "--path".to_string(),
        watch_dir.to_string_lossy().to_string(),
        "--config".to_string(),
        config.to_string_lossy().to_string(),
        "--once".to_string(),
    ];

    let action = build_action("watch", &args).expect("watch should build");
    let engine = Engine::new();
    let out = engine.run_action(action.as_ref());

    assert!(matches!(out.job.status, JobStatus::Success));
    assert_eq!(fs::read_to_string(dst.join("a.txt")).unwrap(), "watch sync");
}