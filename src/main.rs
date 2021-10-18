use log::*;

use cpu::Cpu;
use mmu::MMU;

use crate::op_codes::OP_CODES;

mod mmu;
mod op_codes;
pub mod cpu;

fn main() {
    let tetris_rom = include_bytes!("../tetris.gb");
    env_logger::builder().filter_level(LevelFilter::Debug).init();

    let mut cpu = Cpu::default();
    let mut mmu = MMU::default();
    mmu.load_rom(tetris_rom);
    cpu.run(&mut mmu);
}
