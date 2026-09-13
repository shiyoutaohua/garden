use serde::de::DeserializeOwned;
use std::{any::Any, path::PathBuf, sync::OnceLock};
use tracing::info;

static APPCFG: OnceLock<Box<dyn Any + Send + Sync>> = OnceLock::new();
pub fn appcfg<T>() -> T
where
    T: DeserializeOwned + Clone + Send + Sync + 'static,
{
    let cfg = APPCFG.get_or_init(|| {
        let cfg_text = std::fs::read_to_string(cfg_filepath()).expect("failed to read cfg file");

        let cfg = config::Config::builder()
            .add_source(config::File::from_str(&cfg_text, config::FileFormat::Toml))
            .add_source(config::Environment::with_prefix("DIY").separator("_"))
            .build()
            .expect("failed to build configuration")
            .try_deserialize::<T>()
            .expect("failed to deserialize configuration");

        Box::new(cfg)
    });

    cfg.downcast_ref::<T>()
        .expect("can't reinit with a different configuration type")
        .clone()
}

static CFGPREFIX: OnceLock<PathBuf> = OnceLock::new();

pub fn cfg_prefix(prefix: impl Into<PathBuf>) -> Result<(), PathBuf> {
    CFGPREFIX.set(prefix.into())
}

fn cfg_filepath() -> PathBuf {
    let mut path = std::env::current_dir().expect("failed to get current directory");

    if let Some(prefix) = CFGPREFIX.get() {
        path.push(prefix);
    }
    path.push("res");
    path.push(cfg_filename());

    info!("use cfg file {}", path.display());
    path
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
