use crate::cfg::loader::app_cfg;

pub fn name() -> String {
    app_cfg().app.clone().and_then(|el| el.name).unwrap()
}

pub fn machine_id() -> u64 {
    app_cfg().app.clone().unwrap().machine_id
}
