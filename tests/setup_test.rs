use purrfect::setup;

#[test]
fn setup_test() {
    setup();

    log::error!("Error");
    log::error!("Error2");
    log::error!("Error3");
}
