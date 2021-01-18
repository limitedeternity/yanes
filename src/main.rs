pub mod cpu;
pub mod opcodes;
pub mod status_register;
pub mod bus;

use std::env;
use std::fs;
use std::io;

use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;

use crate::cpu::{CPU, Mem};

#[cfg(test)]
mod test;

fn color(byte: u8) -> Color {
    match byte {
        0 => Color::BLACK,
        1 => Color::WHITE,
        2 | 9 => Color::GREY,
        3 | 10 => Color::RED,
        4 | 11 => Color::GREEN,
        5 | 12 => Color::BLUE,
        6 | 13 => Color::MAGENTA,
        7 | 14 => Color::YELLOW,
        _ => Color::CYAN
    }
}

fn update_screen_state(cpu: &CPU, frame: &mut [u8; 32 * 3 * 32]) -> bool {
    let mut frame_idx = 0;
    let mut update = false;
    for i in 0x0200..0x0600 {
        let color_idx = cpu.mem_read_byte(i as u16);
        let (b1, b2, b3) = color(color_idx).rgb();
        if frame[frame_idx] != b1 || frame[frame_idx + 1] != b2 || frame[frame_idx + 2] != b3 {
            frame[frame_idx] = b1;
            frame[frame_idx + 1] = b2;
            frame[frame_idx + 2] = b3;
            update = true;
        }
        frame_idx += 3;
    }
    update
}

fn handle_user_input(cpu: &mut CPU, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                cpu.mem_write_byte(*cpu.pc(), 0x0);
            },
            Event::KeyDown { keycode: Some(key), .. } => {
                match key as i32 {
                    0x0..=0x7f => {
                        cpu.mem_write_byte(0xff, key as u8);
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} program.bin", &args[0]);
        return;
    }

    match fs::read(&args[1]) {
        Ok(bytes) => {
            let sdl_context = sdl2::init().unwrap();
            let video_subsystem = sdl_context.video().unwrap();
            let window = video_subsystem
                .window("Yanes", (32.0 * 10.0) as u32, (32.0 * 10.0) as u32)
                .position_centered()
                .build().unwrap();

            let mut canvas = window.into_canvas().present_vsync().build().unwrap();
            let mut event_pump = sdl_context.event_pump().unwrap();
            canvas.set_scale(10.0, 10.0).unwrap();

            let creator = canvas.texture_creator();
            let mut texture = creator
                .create_texture_target(PixelFormatEnum::RGB24, 32, 32).unwrap();

            let mut cpu = CPU::new();
            cpu.load(bytes);
            cpu.reset();

            let mut screen_state = [0 as u8; 32 * 3 * 32];

            cpu.run_with_callback(move |cpu| {
                handle_user_input(cpu, &mut event_pump);
                if update_screen_state(cpu, &mut screen_state) {
                    texture.update(None, &screen_state, 32 * 3).unwrap();
                    canvas.copy(&texture, None, None).unwrap();
                    canvas.present();
                }

                ::std::thread::sleep(std::time::Duration::from_millis(70));
            });

            println!("{:?}", cpu);
        },
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    eprintln!("file not found");
                },
                io::ErrorKind::PermissionDenied => {
                    eprintln!("invalid permissions");
                },
                _ => panic!("{}", e)
            }
        }
    }
}
