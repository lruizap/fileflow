use std::sync::{
    atomic::AtomicBool,
    Arc,
};

#[derive(Clone)]
pub struct CancelState {
    pub flag: Arc<AtomicBool>,
}

impl Default for CancelState {
    fn default() -> Self {
        Self {
            flag: Arc::new(AtomicBool::new(false)),
        }
    }
}