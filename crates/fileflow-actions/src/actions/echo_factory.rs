use fileflow_core::{Action, Result};

use crate::factory::ActionFactory;
use crate::actions::echo::EchoAction;

pub struct EchoFactory;

impl ActionFactory for EchoFactory {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn help(&self) -> &'static str {
        "echo  -> demo action (no args)"
    }

    fn build(&self, _args: &[String]) -> Result<Box<dyn Action>> {
        Ok(Box::new(EchoAction))
    }
}
