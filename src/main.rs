use std::sync::Arc;

#[allow(non_snake_case)]
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod renderer;
use renderer::vulkan::Vulkan;

fn main() {
    let app_name:String = "this is coldera baby".to_string();

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let renderer = Vulkan::new(app_name, Arc::new(window));
    println!("Renderer has been loaded");

    _ = event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                elwt.exit();
            }
            Event::AboutToWait => {
                // main loop
            }
            _ => (),
        }
    });
}
