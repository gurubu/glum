use glium::glutin::event_loop::EventLoop;
use glium::{glutin,Display};
use glium::glutin::{dpi::*,event_loop};
pub fn init()->(Display,EventLoop<()>){
    let mut evl = EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(320,240))
            .with_resizable(false)
            .with_title("bob");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &evl).unwrap();
    (display,evl)
}
