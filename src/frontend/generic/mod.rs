use serde::{Deserialize, Serialize};

pub mod loading;
pub mod message;

#[derive(Clone, Deserialize, Serialize)]
pub enum Res<T: Clone> {
    Loading,
    Loaded(T),
}
