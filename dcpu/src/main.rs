#![feature(attr_literals)]
#[macro_use]
extern crate dcpu_macros;
extern crate dcpu;
use dcpu::instruction::InstructionInfo;
use dcpu::registers::Register;
#[derive(InstructionInfo)]
#[opcode = 0x0f]
#[cycles = 1]
struct Baz;

use dcpu::CPU;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut cpu = CPU::new();
    cpu.push(5);
    let mut file = File::open("/tmp/dcpu/a.out").unwrap();
    let mut buf = [0u8; 0x2000];
    file.read(&mut buf).unwrap();
    let mem: &mut [u16; 0x1000] = unsafe { std::mem::transmute(&mut buf) };
    let buf: Vec<u16> = mem.iter_mut().map(|val| u16::from_be(*val)).collect();
    unsafe {
        std::ptr::copy(buf.as_ptr(), &mut cpu.memory as *mut _ as *mut u16, buf.len());
    }
    cpu.tick();
    println!("{:?}", cpu.registers);
    cpu.tick();
    println!("{:?}", cpu.registers);

}
