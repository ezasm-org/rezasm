use std::path::PathBuf;

pub fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().expect("Parent for rezasm-core not found. Was the project restructured?")
        .parent().expect("Parent for rezasm-source not found. Was the project restructured?")
        .into()
}
