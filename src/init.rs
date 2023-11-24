use glium::{glutin::{self, context},Display};
use winit::{event_loop::{EventLoop, EventLoopBuilder}, window::WindowBuilder};
//pub fn init()->(){
//    let mut evl = EventLoopBuilder::new().build(); 
//    let wb = glutin::window::WindowBuilder::new()
//            .with_inner_size(PhysicalSize::new(320,240))
//            .with_resizable(false)
//            .with_title("bob");
//    (display,evl)
//}
pub fn init()->(){
    let mut evl:EventLoop<()> = EventLoopBuilder::with_user_event().build();
    let mut wnd = WindowBuilder::new().with_title("argl").with_resizable(false).build(&evl);
}
