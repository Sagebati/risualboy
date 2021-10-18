use crate::cpu::{Cpu, Flags};
use crate::mmu::MMU;

#[repr(u16)]
#[derive(Clone, Copy, Debug)]
pub enum Bytes {
    None,
    One(u8),
    Pair([u8;2])
}

impl Bytes {
    pub fn get_one(self) -> u8 {
        match self {
            None => panic!("Cannot unwrap one byte on a none value"),
            Bytes::One(value) => value,
            Bytes::Pair(_) => panic!("Cannot unwrap one byte on a pair"),
        }
    }

    #[inline(always)]
    pub fn get_pair(self) -> [u8;2] {
        match self {
            None => panic!("Cannot unwrap a pair on a none value"),
            Bytes::One(_) => panic!("Cannot unwrap pair on a one"),
            Bytes::Pair(y) => y
        }
    }

    #[inline(always)]
    pub fn get_pair_u16(self) -> u16 {
        u16::from_le_bytes(self.get_pair())
    }
}

#[derive(Clone)]
pub struct OpCode {
    pub name: &'static str,
    pub operand_size: u8,
    pub time: u16,
    pub f: fn(&mut Cpu, &mut MMU, operands: Bytes) -> (),
}

pub const OP_CODES: &[OpCode] = &[
    OpCode { name: "Nop", operand_size: 0, time: 4, f: |_, _, _| () }, // 0x0
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "LD nn,SP", operand_size: 2, time: 10, f: |x, _, y| x.sp = y.unwrap() }, // 0x08
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "LDH (n), A", operand_size: 1, time: 6, f: |c, m, y| m.wb(0xFF00 + y.map(|x|x.get_one() as u16).unwrap(), c.a) }, // 0xe0
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "ADD HL,DE", operand_size: 0, time: 4, f: |x, _, _| x.add_hl_de() }, // 0x19
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "JR cc,n", operand_size: 1, time: 4, f: |x, _, y| x.jr(!Flags::ZERO, y as u8) }, // 0x20
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    // FIXME 22
    OpCode { name: "HLI, A", operand_size: 0, time: 2, f: |x, _, _| x.w_hl(x.a as u16) }, // 0x22
    OpCode { name: "INC HL", operand_size: 2, time: 4, f: |x, _, y| x.w_hl(x.r_hl() + y) }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "LD SP NN", operand_size: 2, time: 12, f: |c, _, v| c.ld_sp_nn(v) }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "Dec A", operand_size: 0, time: 2, f: |x, _, _| Cpu::dec(&mut x.a)}, // 0x3d
    // FIXME not sure
    OpCode { name: "LD A,#", operand_size: 1, time: 4, f: |x, _, y| x.a = y as u8 }, // 0x3e
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "SBC A,C", operand_size: 0, time: 4, f: |x, _, _|  x.sbc_a(x.c)  }, // 0x99
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "xor_n", operand_size: 1, time: 8, f: |c, _, v| c.xor_n(v) }, // 0xee
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    // FIXME LD nn, A
    OpCode { name: "LD (nn),A", operand_size: 2, time: 8, f: |x, mem, _| mem.ww(x.pc, x.a as u16) }, // 0xea
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "LD SP,HL", operand_size: 0, time: 4, f: |x, _, _| { x.sp = x.r_hl() } }, // 0xf9
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "not implemented", operand_size: 2, time: 2, f: |x, m, y| { unimplemented!() } }, // 0x31
    OpCode { name: "CPn", operand_size: 1, time: 8, f: |c, _, v| c.cp_n(v) }, // 0xfe
    OpCode { name: "RST 38H", operand_size: 0, time: 16, f: |c, m, _| c.rst(0x38, m) }, // 0xff
];