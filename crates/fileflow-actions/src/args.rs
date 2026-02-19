use std::collections::HashMap;

use fileflow_core::{FileFlowError, Result};

#[derive(Debug, Clone)]
pub struct ParsedArgs {
    pub flags: HashMap<String, Option<String>>,
}

impl ParsedArgs {
    pub fn from_vec(args: &[String]) -> Result<Self> {
        let mut flags: HashMap<String, Option<String>> = HashMap::new();
        let mut i = 0;

        while i < args.len() {
            let tok = &args[i];
            if !tok.starts_with("--") {
                return Err(FileFlowError::Message(format!(
                    "Invalid argument '{tok}'. Expected '--key value' or '--flag'."
                )));
            }

            let key = tok.trim_start_matches("--").to_string();

            // boolean flag if next is missing or another flag
            let val = if i + 1 >= args.len() || args[i + 1].starts_with("--") {
                None
            } else {
                i += 1;
                Some(args[i].clone())
            };

            flags.insert(key, val);
            i += 1;
        }

        Ok(Self { flags })
    }

    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.flags.get(key).and_then(|v| v.as_deref())
    }

    pub fn has_flag(&self, key: &str) -> bool {
        self.flags.contains_key(key) && self.flags.get(key).unwrap().is_none()
    }

    pub fn require_str(&self, key: &str) -> Result<String> {
        self.get_str(key)
            .map(|s| s.to_string())
            .ok_or_else(|| FileFlowError::Message(format!("Missing required flag: --{key} <value>")))
    }
}
