use std::path::Path;

pub struct PurrfectBuilder<P: AsRef<Path>> {
    config_path: P,
}
