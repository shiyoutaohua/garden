use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::OnceLock};
use tracing::info;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct ApplicationConfiguration {
    pub(super) app: Option<Application>,
    pub(super) consul: Consul,
    pub(super) zk: ZooKeeper,
    pub(super) redis: Redis,
    pub(super) ds: HashMap<String, Datasource>,
    pub(super) opensearch: OpenSearch,
    pub(super) nats: Nats,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct Application {
    pub(super) name: Option<String>,
    pub(super) version: Option<String>,
    pub(super) machine_id: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct Consul {
    pub(super) address: String,
    pub(super) token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct ZooKeeper {
    pub(super) url: String,
    pub(super) username: Option<String>,
    pub(super) password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct Redis {
    pub(super) url: String,
    pub(super) max_size: Option<u32>,
    pub(super) min_idle: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct Datasource {
    pub(super) url: String,
    pub(super) max_connections: Option<u32>,
    pub(super) min_connections: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct OpenSearch {
    pub(super) url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct Nats {
    pub(super) address: String,
}

static APP_CFG: OnceLock<ApplicationConfiguration> = OnceLock::new();
pub(super) fn app_cfg() -> ApplicationConfiguration {
    APP_CFG
        .get_or_init(|| {
            let filename = cfg_filename();
            info!("use cfg file {}", filename);
            let path = std::env::current_dir().unwrap().join("res").join(filename);
            let cfg_text = std::fs::read_to_string(path.as_path()).expect("fail to read cfg file");
            let file = config::File::from_str(&cfg_text, config::FileFormat::Toml);
            let cfg = config::Config::builder()
                .add_source(file)
                // 读取环境覆盖[diy_app_name=AppName0 cargo r]
                .add_source(config::Environment::with_prefix("DIY").separator("_"))
                .build()
                .unwrap()
                .try_deserialize::<ApplicationConfiguration>()
                .expect("can't refresh application configuration");
            cfg
        })
        .clone()
}

fn cfg_filename() -> String {
    let mut mode = std::env::var("RUNENV").unwrap_or(String::new());
    mode.make_ascii_lowercase();
    if mode.is_empty() {
        format!("application.toml")
    } else {
        format!("application-{mode}.toml")
    }
}
