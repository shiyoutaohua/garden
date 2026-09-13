use garden_core::cfg::appcfg;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(self) struct AppCfg {
    pub(self) app: Option<App>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct App {
    pub(super) name: Option<String>,
    pub(super) version: Option<String>,
    pub(super) machine_id: u64,
    pub(super) port: u16,
}

pub fn name() -> String {
    appcfg::<AppCfg>().app.clone().and_then(|el| el.name).unwrap()
}

pub fn machine_id() -> u64 {
    appcfg::<AppCfg>().app.clone().unwrap().machine_id
}

pub fn port() -> u16 {
    appcfg::<AppCfg>().app.clone().unwrap().port
}
