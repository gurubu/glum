//use glium::glutin::window::Window;
use glium::{glutin,Display};
use winit::event_loop::{EventLoop, EventLoopBuilder};
pub fn init()->(){
    let mut evl = EventLoopBuilder::new().build(); 
    let wb = glutin::window::WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(320,240))
            .with_resizable(false)
            .with_title("bob");
    (display,evl)
}
