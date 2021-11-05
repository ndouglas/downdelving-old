use rltk::prelude::*;
mod builder;
pub use builder::*;
mod logstore;
use logstore::*;
pub use logstore::{clear_log, clone_log, print_log, restore_log};
use serde::{Deserialize, Serialize};
mod events;
pub use events::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogFragment {
    pub color: RGB,
    pub text: String,
}
