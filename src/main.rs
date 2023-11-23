use egui_glium::{EguiGlium, egui_winit::egui::Widget};
use glium::{backend::glutin::{SimpleWindowBuilder, self}, Surface};
use winit::{event::WindowEvent, event_loop::ControlFlow, dpi::{PhysicalSize, LogicalSize}, window::WindowBuilder};
fn main(){
    let evl:winit::event_loop::EventLoop<()> = winit::event_loop::EventLoopBuilder::with_user_event().build(); 
    //let (wnd,dsp) = SimpleWindowBuilder::new()
    //                .with_resizable(false)
    //                .with_inner_size(640,480)
    //                .with_title("glum")
    //                .build(&evl);
    //
    let wnd= WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(640,480))
        .with_resizable(false)
        .with_title("adventures in crappy software creation by dumbass corp.")
        .build(&evl)
        .unwrap();
    let dsp = DisplayBuilder::new().with_window_builder(Some(wnd));
    let mut egui = EguiGlium::new(&dsp,&wnd,&evl);
    evl.run(move|event,_,control_flow|{
        let mut redraw=||{
            let mut quit = false;
            let repaintafter=egui.run(&wnd,|egui_ctx|{
                egui::SidePanel::left("test").show(egui_ctx,|ui|{
                    ui.heading("bob");
                    if ui.button("gtfo").clicked(){
                        quit = true;
                    }
                });
            });
            *control_flow=if quit{
                ControlFlow::Exit
            }
            else if repaintafter.is_zero(){
                wnd.request_redraw();
                ControlFlow::Poll
            }
            else if let Some(repaint_after_instant)=
                std::time::Instant::now().checked_add(repaintafter)
            {
                ControlFlow::WaitUntil(repaint_after_instant)
            }else{
                ControlFlow::Wait
            };
            {
                let mut frame = dsp.draw();
                frame.clear_color(0.0,1.0,0.0,1.0);
                egui.paint(&dsp,&mut frame);
                frame.finish().unwrap();
            }
        };
        match event{
            winit::event::Event::RedrawEventsCleared=>redraw(),
            winit::event::Event::WindowEvent{event,..}=>{
                match event{
                    WindowEvent::CloseRequested=>*control_flow = ControlFlow::Exit,
                    _=>(),
                }
                let evresp = egui.on_event(&event);
                if evresp.repaint{
                    wnd.request_redraw();
                }
            }
            _=>(),
        }
    });
}
