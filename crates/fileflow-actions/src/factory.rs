use fileflow_core::{Action, Result};

pub trait ActionFactory {
    fn name(&self) -> &'static str;

    /// Texto corto para ayudar al usuario (actions list)
    fn help(&self) -> &'static str;

    /// Construye la acción a partir de args genéricos tipo ["--src","a","--dst","b"]
    fn build(&self, args: &[String]) -> Result<Box<dyn Action>>;
}
