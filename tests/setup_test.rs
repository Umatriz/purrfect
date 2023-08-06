#[test]
fn setup_test() {
    purrfect::PurrfectBuilder::new()
        .file("Purrfect.toml")
        .build();

    log::error!("Error");
    log::info!("info");
    log::debug!("debug");
    log::warn!("warn");
    log::trace!("trace")
}
