#[test]
fn setup_test() {
    purrfect::PurrfectBuilder::new()
        .config("Purrfect.toml")
        .build()
        .unwrap();

    log::error!("Error");
    log::info!("info");
    log::debug!("debug");
    log::warn!("warn");
    log::trace!("trace")
}
