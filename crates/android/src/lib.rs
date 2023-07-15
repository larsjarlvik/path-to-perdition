#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

#[no_mangle]
#[cfg(target_os = "android")]
fn android_main(app: AndroidApp) {
    use ptp_game::events;
    use winit::event_loop::EventLoopBuilder;
    use winit::platform::android::EventLoopBuilderExtAndroid;

    android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Warn));

    let event_loop = EventLoopBuilder::new().with_android_app(app).build();
    pollster::block_on(events::run(event_loop));
}
