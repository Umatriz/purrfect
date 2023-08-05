use purrfect::setup;

#[test]
fn setup_test() {
    setup();

    log::error!("info")
}
