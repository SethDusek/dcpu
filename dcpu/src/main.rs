#![feature(attr_literals)]
#[macro_use]
extern crate dcpu_macros;
extern crate dcpu;
use dcpu::instructions::InstructionInfo;
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
    let mut buf: [u8; 2] = [0; 2];
    file.read(&mut buf);
    let buf: u16 = u16::from_be(unsafe { std::mem::transmute(buf) });
    
    let instr = dcpu::Instr::new(buf);
    print!("{:x} {:?}", instr.opcode(), instr.arg_a(&mut cpu));
    println!("{:?}", instr.arg_b(&mut cpu));
}
