pub mod cpu;
pub mod opcodes;
pub mod status_register;
pub mod bus;

use std::env;
use std::fs;
use std::io;

use crate::cpu::CPU;

#[cfg(test)]
mod test;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} program.bin", &args[0]);
        return;
    }

    match fs::read(&args[1]) {
        Ok(bytes) => {
            let mut cpu = CPU::new();
            cpu.load_and_run(bytes);
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
