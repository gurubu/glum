#![allow(non_camel_case_types,unused_imports)]
use std::{fs, env};
use std::sync::mpsc::{Receiver,RecvError, self};
use std::time::Duration;
use egui::epaint::Shadow;
use egui::{Style, Visuals, Rounding, Color32};
use egui_glium::{EguiGlium, egui_winit::egui::Widget};
use glium::glutin::surface::WindowSurface;
use glium::vertex::{EmptyVertexAttributes, EmptyInstanceAttributes};
use glium::{implement_uniform_block, uniform};
use glium::uniforms::UniformBuffer;
use glium::{Program,VertexBuffer,backend::{glutin::{SimpleWindowBuilder, self}, Context}, Surface, Display, implement_vertex, uniforms::EmptyUniforms};
use hotwatch::{Hotwatch, EventKind,Event as hwevnt};
use winit::{event::WindowEvent, event_loop::ControlFlow, dpi::{PhysicalSize, LogicalSize}, window::WindowBuilder};
pub mod hot;
pub mod init;
#[derive(Copy,Clone)]
pub struct vert{
    pub pos: [f32;2]
}
pub struct stuff{
    pub id:usize,
    pub vecprg:Vec<Program>,
    pub vecvrt:Vec<VertexBuffer<vert>>
}
pub fn rldshd(d:&Display<WindowSurface>,s:&mut stuff){
    s.vecprg.pop();
    let vrtsh = fs::read_to_string("shdrs/verts.glsl").expect("couldnt read vertex shader");
    let frgsh = fs::read_to_string("shdrs/frags.glsl").expect("couldnt read fragment shader");
    let prog = Program::from_source(d,&vrtsh.to_string(),&frgsh.to_string(),None);
    let errprog=Program::from_source(d,include_str!("shdrs/verts.glsl"),include_str!("shdrs/errshd.glsl"),None).unwrap();
    match prog {
        Ok(Program)=>s.vecprg.push(Program),
        Err(ProgramCreationError)=>{s.vecprg.push(errprog);println!("{}",ProgramCreationError)},
    }
}

fn main(){
    let evl:winit::event_loop::EventLoop<()> = winit::event_loop::EventLoopBuilder::with_user_event().build(); 
    let (wnd,dsp)= SimpleWindowBuilder::new()
                    .set_window_builder(WindowBuilder::new().with_resizable(false).with_active(true))
                    .with_inner_size(1040,480)
                    .with_title("glum")
                    .build(&evl);
    let (snd,rcv) = mpsc::channel();
    let mut hotwatch = Hotwatch::new_with_custom_delay(Duration::from_millis(250)).expect("hotwatch failed");
    let path = env::current_dir().unwrap();
    hotwatch.watch("shdrs/frags.glsl",move|event:hwevnt|{
        if let EventKind::Modify(_)=event.kind{
            snd.send(0).unwrap();
        }
    }).expect("failed to watch file");
    let mut egui = EguiGlium::new(&dsp,&wnd,&evl);
    let mut world:stuff=stuff{id:0,vecprg:Vec::new(),vecvrt:Vec::new()};
    let vert1 = vert{pos:[-0.5, 1.0]};
    let vert2 = vert{pos:[ 1.0, 1.0]};
    let vert3 = vert{pos:[-0.5,-1.0]};
    let vert4 = vert{pos:[ 1.0,-1.0]};
    implement_vertex!(vert,pos);
    let shape = vec![vert1, vert2, vert3,vert4];
    world.vecvrt.push(VertexBuffer::new(&dsp,&shape).unwrap());
    let vrtsh = fs::read_to_string("shdrs/verts.glsl").expect("couldnt read vertex shader");
    let frgsh = fs::read_to_string("shdrs/frags.glsl").expect("couldnt read fragment shader");
    world.vecprg.push(Program::from_source(&dsp,&vrtsh.to_string(),
                                           &frgsh.to_string(),None).unwrap());
    world.id = 1;
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
    egui.egui_ctx.style_mut(|style|{
        style.visuals.window_shadow = Shadow::NONE;
        style.visuals.window_rounding = Rounding::ZERO;
        style.visuals.dark_mode = false;
        style.visuals.window_fill = Color32::GRAY;
        style.visuals.override_text_color = Some(Color32::WHITE);
    });
    //#[derive(Copy,Clone)]
    //struct shape{
    //    posx:[f32;2],
    //    posy:[f32;2],
    //    radius:[f32;2],
    //}
    let mut shp1posx:f32 = 0.0;
    let mut shp2posx:f32 = 0.0;
    let mut shp1posy:f32 = 0.0;
    let mut shp2posy:f32 = 0.0;
    let mut shp1radi:f32 = 0.0;
    let mut shp2radi:f32 = 0.0;
    let mut shp1bool:u8  = 0;
    #[derive(PartialEq,Copy,Clone)]
    pub enum boolop{
        union,inters,negative
    }
    impl boolop{
        pub fn intousize(self)->i8{
            match self{
                boolop::union=>0,
                boolop::inters=>1,
                boolop::negative=>2,
            }
        }
    }
    let mut shpboolop = boolop::union;
    //let mut shp:shape=shape{posx:[0.0,0.0],posy:[0.0,0.0], radius:[0.4,0.4]};
    //implement_uniform_block!(shape,posx,posy,radius);
    //let mut shp1=UniformBuffer::new(&dsp,shp).unwrap();

    evl.run(move|event,_,control_flow|{
        let mut redraw=||{
            let mut quit = false;
            let repaintafter=egui.run(&wnd,|eguictx|{
               egui::Window::new("test")
                    .movable(true)
                    .constrain(false)
                    .resizable(false)
                    .show(eguictx,|ui|{
                        ui.add(egui::Slider::new(&mut shp1posx,-1.0..=1.0).text("1x"));
                        ui.add(egui::Slider::new(&mut shp1posy,-1.0..=1.0).text("1y"));
                        ui.add(egui::Slider::new(&mut shp1radi, 0.0..=1.0).text("1r"));
                        ui.add(egui::Slider::new(&mut shp2posx,-1.0..=1.0).text("2x"));
                        ui.add(egui::Slider::new(&mut shp2posy,-1.0..=1.0).text("2y"));
                        ui.add(egui::Slider::new(&mut shp2radi, 0.0..=1.0).text("2r"));
                        ui.selectable_value(&mut shpboolop,boolop::union,"union");
                        ui.selectable_value(&mut shpboolop,boolop::inters,"intersection");
                        ui.selectable_value(&mut shpboolop,boolop::negative,"negative");
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
                match rcv.try_recv(){
                    Ok(_)=>rldshd(&dsp,&mut world),
                    Err(_)=>{},
                }
                let mut frame = dsp.draw();
                frame.clear_color(0.0,0.0,0.0,1.0);
                for i in 0..world.id {
                    frame.draw(&world.vecvrt[i], &indices,&world.vecprg[i],
                    &uniform!{shp1posx:shp1posx,
                              shp2posx:shp2posx,
                              shp1posy:shp1posy,
                              shp2posy:shp2posy,
                              shp1radi:shp1radi,
                              shp2radi:shp2radi,
                              shpbool:shpboolop.intousize()},&Default::default()).unwrap();
                }
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
