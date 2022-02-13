pub mod cpu;
pub mod opcodes;
pub mod status_register;
pub mod bus;

use std::{env, fs};
use std::error::Error;

use match_all::match_all;

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::{Keycode, Mod};

use crate::cpu::{CPU, MemLayout, RAMAccess};

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

fn char_to_shift_mod(charcode: u8) -> u8 {
    match charcode {
        0x60 => 0x7E,
        0x30 => 0x29,
        0x31 => 0x21,
        0x32 => 0x40,
        0x33 => 0x23,
        0x34 => 0x24,
        0x35 => 0x25,
        0x36 => 0x5E,
        0x37 => 0x26,
        0x38 => 0x2A,
        0x39 => 0x28,
        0x2D => 0x5F,
        0x3D => 0x2B,
        0x27 => 0x22,
        0x3B => 0x3A,
        0x2F => 0x3F,
        0x2E => 0x3E,
        0x2C => 0x3C,
        0x5B => 0x7B,
        0x5D => 0x7D,
        0x5C => 0x7C,
        0x61..=0x7A => charcode ^ 0x20,
        _ => charcode
    }
}

fn update_screen_state(cpu: &CPU, frame: &mut [u8; 3072]) -> bool {
    let mut frame_idx = 0;
    let mut update = false;
    for i in MemLayout::VGA_BUF_START..=MemLayout::VGA_BUF_END {
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
        match_all!{ event,
            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                if !*cpu.p().I() {
                    cpu.mem_write_word(MemLayout::IRQ_VECTOR, 0);
                    cpu.mem_write_byte(*cpu.pc(), 0);
                }
            },
            Event::KeyDown { keycode: Some(keycode), keymod, .. } => {
                match keycode as i32 {
                    0x00..=0x7f => {
                        match keymod {
                            Mod::LSHIFTMOD | Mod::RSHIFTMOD => {
                                cpu.mem_write_byte(MemLayout::KEYCODE_ADDR, char_to_shift_mod(keycode as u8));
                            },
                            _ => {
                                cpu.mem_write_byte(MemLayout::KEYCODE_ADDR, keycode as u8);
                            }
                        }
                    },
                    _ => ()
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Usage: {} program.bin", &args[0]);
        return Ok(());
    }

    let bytes = fs::read(&args[1])?;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Yanes", 320u32, 320u32)
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

    let mut screen_state = [0u8; 3072];

    cpu.run_with_callback(move |cpu| {
        handle_user_input(cpu, &mut event_pump);
        if update_screen_state(cpu, &mut screen_state) {
            texture.update(None, &screen_state, 96).unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }
    });

    println!("{:?}", cpu);
    Ok(())
}
