use winit::event_loop::EventLoop;

pub fn get_available_monitors() -> Vec<String> {
    let event_loop = EventLoop::new();

    event_loop
        .available_monitors()
        .map(|monitor| monitor.name().unwrap())
        .collect::<Vec<_>>()
}
