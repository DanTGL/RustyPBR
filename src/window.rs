use winit::{event_loop::EventLoop, window::{Window, WindowBuilder}, dpi::LogicalSize};


pub fn create_window(width: u32, height: u32) -> (EventLoop<()>, Window) {
    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new(width as f64, height as f64);
        
        WindowBuilder::new()
            .with_title("Raytracer")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    (event_loop, window)
}