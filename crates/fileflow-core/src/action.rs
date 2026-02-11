use crate::{context::Context, error::Result};

pub trait Action {
    fn name(&self) -> &'static str;
    fn execute(&self, ctx: &mut Context) -> Result<()>;
}
