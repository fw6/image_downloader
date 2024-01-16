use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

uniffi::setup_scaffolding!();

#[derive(Serialize, Deserialize, Default, Debug, JsonSchema)]
#[serde(default, rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct Example1 {
    pub content: String,
}

#[derive(uniffi::Object)]
pub struct Example2(RwLock<Example1>);

#[uniffi::export]
impl Example2 {
    /// Initialize a new instance
    #[uniffi::constructor]
    pub fn new(settings: &Option<String>) -> Arc<Self> {
        Arc::new(Self(RwLock::new(Example1 {
            content: settings.clone().unwrap_or_default(),
        })))
    }

    /// Test method, echoes back the input
    pub fn echo(&self, msg: String) -> String {
        msg
    }
}
