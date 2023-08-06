#[derive(Debug, Clone)]
pub struct Wrapper<T>(pub T);

pub(crate) type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;
