#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#[macro_use]
extern crate glium;
pub mod init;
pub mod algeo;
use std::sync::mpsc::{Receiver, RecvError};
use std::{fs,sync::mpsc,time::Duration};
use glium::ProgramCreationError;
use hotwatch::{Hotwatch,Event as hwevnt,EventKind};
use crate::init::*;
use crate::algeo::*;
use glium::glutin::event::ElementState;
use glium::{glutin,Program,Surface,index};
use glium::Display;
use glium::glutin::{event_loop::EventLoop,
                    event,event::Event,event::StartCause};
use glium::VertexBuffer;
pub struct stuff{
    pub id:usize,
    pub vecprg:Vec<Program>,
    pub vecvrt:Vec<VertexBuffer<vert>>
}
pub fn rldshd(d:&Display,s:&mut stuff){
    s.vecprg.pop();
    let vrtsh = fs::read_to_string("shdrs/verts.glsl").expect("couldnt read vertex shader");
    let frgsh = fs::read_to_string("shdrs/frags.glsl").expect("couldnt read fragment shader");
    let prog = Program::from_source(d,&vrtsh.to_string(),&frgsh.to_string(),None);
    let errprog=Program::from_source(d,include_str!("shdrs/verts.glsl"),include_str!("shdrs/errshd.glsl"),None).unwrap();
    match prog {
        Ok(Program)=>s.vecprg.push(Program),
        Err(ProgramCreationError)=>{s.vecprg.push(errprog);println!("{}",ProgramCreationError)},
    }
    println!("rldshd");
}
pub struct shapedata{
    x:f32,
    y:f32,
    r:f32,
}
impl shapedata{
    pub fn new()->Self{
        shapedata{x:0.0,y:0.0,r:0.1}
    }
}
fn main() {
    let (dsp,evl) = init();
    let mut utime:f32 = 0.0;
    let mut shp1:shapedata=shapedata::new();
    let mut shp2:shapedata=shapedata::new();
    let mut sdfo:i8=1;
    let (snd,rcv) = mpsc::channel();
    let mut world = stuff{vecprg:Vec::new(),vecvrt:Vec::new(),id:0};
    let mut hotwatch = Hotwatch::new_with_custom_delay(Duration::from_millis(250)).expect("hotwatch failed");
    hotwatch.watch("shdrs/frags.glsl",move |event:hwevnt| {
       if let EventKind::Modify(_)=event.kind{
           snd.send(0).unwrap();
           println!("send");
       }
    }).expect("failed to watch file");
    let vert1 = vert{pos:[-1.0, 1.0]};
    let vert2 = vert{pos:[ 1.0, 1.0]};
    let vert3 = vert{pos:[-1.0,-1.0]};
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
    evl.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match event {
            Event::WindowEvent { event, .. } => match event {
                event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                event::WindowEvent::KeyboardInput {input,..}=> {
                    if input.state == ElementState::Pressed {
                       if let Some(key)=input.virtual_keycode{
                           match key{
                               event::VirtualKeyCode::R =>{
                                   rldshd(&dsp,&mut world);
                               }
                               event::VirtualKeyCode::Escape=>{

                               }
                               event::VirtualKeyCode::Left=>{
                                   shp1.x+=0.10;
                               }
                               event::VirtualKeyCode::Right=>{
                                   shp1.x-=0.10;
                               }
                               event::VirtualKeyCode::Up=>{
                                   shp1.y-=0.10;
                               }
                               event::VirtualKeyCode::Down=>{
                                   shp1.y+=0.10;
                               }
                               event::VirtualKeyCode::H=>{
                                   shp2.x+=0.10;
                               }
                               event::VirtualKeyCode::L=>{
                                   shp2.x-=0.10;
                               }
                               event::VirtualKeyCode::K=>{
                                   shp2.y-=0.10;
                               }
                               event::VirtualKeyCode::J=>{
                                   shp2.y+=0.10;
                               }
                               event::VirtualKeyCode::D=>{
                                   shp1.r+=0.01;
                               }
                               event::VirtualKeyCode::S=>{
                                   shp1.r-=0.01;
                               }
                               event::VirtualKeyCode::Z=>{
                                   shp2.r+=0.01;
                               }
                               event::VirtualKeyCode::E=>{
                                   shp2.r-=0.01;
                               }
                               event::VirtualKeyCode::Key1=>{
                                   sdfo = 1;
                               }
                               event::VirtualKeyCode::Key2=>{
                                   sdfo = 2;
                               }
                               event::VirtualKeyCode::Key3=>{
                                   sdfo = 3;
                               }

                               _=>{}
                           }
                       }
                    }
                },

                _ => return,
            },
            Event::NewEvents(cause) => match cause {
                StartCause::ResumeTimeReached { .. } => (),
                StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }
        // if rcv.try_recv().is_err()==true{
        //     rldshd(&dsp,&mut world);
        //
        match rcv.try_recv(){
           Ok(_)=> rldshd(&dsp,&mut world),
           Err(_)=>{},
        }
        utime += 0.01;
        let mut trg = dsp.draw();
        trg.clear_color(0.0, 0.0, 0.0, 1.0);
        for i in 0..world.id {
            trg.draw(&world.vecvrt[i], &indices,&world.vecprg[i],
                &uniform!{utime:utime,shp1x:shp1.x,shp1y:shp1.y,shp1r:shp1.r,
                            shp2x:shp2.x,shp2y:shp2.y,shp2r:shp2.r,sdfo:sdfo},&Default::default()).unwrap();
        }
        trg.finish().unwrap();
    });
}
