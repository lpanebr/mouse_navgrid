use enigo::*;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    // Configuração da janela
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Mouse Grid")
        .with_inner_size(LogicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(window_size.width, window_size.height, surface_texture).unwrap();

    // Configuração da grade
    let cols = 8;
    let rows = 3;
    let cell_width = window_size.width / cols as u32;
    let cell_height = window_size.height / rows as u32;

    // Mapeamento das teclas
    let key_map: Vec<VirtualKeyCode> = vec![
        VirtualKeyCode::Q, VirtualKeyCode::W, VirtualKeyCode::E, VirtualKeyCode::R,
        VirtualKeyCode::U, VirtualKeyCode::I, VirtualKeyCode::O, VirtualKeyCode::P,
        VirtualKeyCode::A, VirtualKeyCode::S, VirtualKeyCode::D, VirtualKeyCode::F,
        VirtualKeyCode::J, VirtualKeyCode::K, VirtualKeyCode::L, VirtualKeyCode::Grave,
        VirtualKeyCode::Z, VirtualKeyCode::X, VirtualKeyCode::C, VirtualKeyCode::V,
        VirtualKeyCode::M, VirtualKeyCode::Comma, VirtualKeyCode::Period, VirtualKeyCode::Semicolon,
    ];

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(key),
                            ..
                        },
                    ..
                } => {
                    if let Some(pos) = key_map.iter().position(|&k| k == key) {
                        let col = pos % cols;
                        let row = pos / cols;

                        // Calcula as coordenadas do centro da célula
                        let center_x = (col as u32 * cell_width + cell_width / 2) as i32;
                        let center_y = (row as u32 * cell_height + cell_height / 2) as i32;

                        // Move o cursor e sai do app
                        let mut enigo = Enigo::new();
                        enigo.mouse_move_to(center_x, center_y);
                        *control_flow = ControlFlow::Exit;
                    }
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                let frame = pixels.frame_mut();
                for y in 0..window_size.height {
                    for x in 0..window_size.width {
                        let idx = ((y * window_size.width + x) * 4) as usize;

                        // Define cor do fundo
                        frame[idx] = 0;     // Red
                        frame[idx + 1] = 0; // Green
                        frame[idx + 2] = 0; // Blue
                        frame[idx + 3] = 255; // Alpha

                        // Desenha linhas azuis da grade
                        if (x % cell_width == 0) || (y % cell_height == 0) {
                            frame[idx] = 0;
                            frame[idx + 1] = 0;
                            frame[idx + 2] = 255;
                        }
                    }
                }
                pixels.render().unwrap();
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
