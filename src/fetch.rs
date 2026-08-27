use crate::error::{Error, Result};

#[derive(Debug, Clone, Default)]
pub enum Fetch<T> {
    #[default]
    Loading,
    Loaded(T),
    Failed(Error),
}

impl<T> Fetch<T> {
    pub fn loaded(&self) -> Option<&T> {
        match self {
            Fetch::Loaded(value) => Some(value),
            _ => None,
        }
    }
}

impl<T> From<Result<T>> for Fetch<T> {
    fn from(result: Result<T>) -> Self {
        match result {
            Ok(value) => Fetch::Loaded(value),
            Err(error) => Fetch::Failed(error),
        }
    }
}
