#[allow(dead_code)]
#[cfg(not(target_os = "android"))]
fn main() {
    use ptp_game::events;
    use winit::event_loop::EventLoopBuilder;

    env_logger::builder()
        .filter_level(log::LevelFilter::Warn)
        .parse_default_env()
        .init();

    let event_loop = EventLoopBuilder::new().build();
    pollster::block_on(events::run(event_loop));
}
