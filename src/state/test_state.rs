use crate::state::*;
use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Builder, Debug, Default)]
#[builder(setter(into), default)]
#[serde(default)]
pub struct TestState {
    pub mnemonics: Vec<String>,
    pub directory: String,
    pub add: Add,
    pub edit: Edit,
    pub list: List,
    pub rm: Rm,
    pub show: Show,
    pub filesystem: FileSystem,
}

impl TestStateBuilder {
    pub fn new() -> Self {
        Self::default()
    }
}
impl FileSystemBuilder {
    pub fn new() -> Self {
        Self::default()
    }
}
impl AddBuilder {
    pub fn new() -> Self {
        Self::default()
    }
}
impl EditBuilder {
    pub fn new() -> Self {
        Self::default()
    }
}
impl ShowBuilder {
    pub fn new() -> Self {
        Self::default()
    }
}
impl RmBuilder {
    pub fn new() -> Self {
        Self::default()
    }
}
