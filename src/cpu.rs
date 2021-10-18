use crate::mmu::MMU;
use crate::op_codes::OP_CODES;
use log::*;
use bitflags::bitflags;

pub type Register<T = u8> = T;
pub type Addr = u16;

bitflags! {
    pub struct Flags:u8 {
        const ZERO = 0x80;
        const NEGATIVE= 0x40;
        const HALFCARRY = 0x20;
        const CARRY = 0x10;
    }
}

impl Default for Flags {
    fn default() -> Self {
       Flags::empty()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Cpu {
    pub a: Register,

    pub b: Register,
    pub c: Register,

    pub d: Register,
    pub e: Register,

    pub h: Register,
    pub l: Register,

    pub f: Flags,
    pub pc: Register<u16>,
    pub sp: Register<u16>,

    pub int_clk: u16,
    pub clock: u16,

    pub boot_off : bool
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            f: Flags::default(),
            pc: 0,
            sp: 0,
            int_clk: 0,
            clock: 0,
            boot_off: false
        }
    }
}

impl Cpu {
    #[inline]
    pub fn set_flag_or_reset(&mut self, flag : Flags , cond: bool) {
        if cond {
            self.f |= flag;
        }else {
            self.f.remove(flag)
        }

    }

    #[inline]
    pub fn r_hl(&self) -> u16 {
        u16::from_le_bytes([self.h, self.l])
    }

    #[inline]
    pub fn w_hl(&mut self, val: u16) {
        let [x1, x2] = val.to_le_bytes();
        self.h = x1;
        self.l = x2;
    }

    #[inline]
    pub fn r_de(&mut self) -> u16 {
        u16::from_le_bytes([self.d, self.e])
    }

    #[inline]
    pub fn w_de(&mut self, val: u16) {
        let [d, e] = val.to_le_bytes();
        self.d = d;
        self.e = e;
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn run(&mut self, memory: &mut MMU) {
        loop {
            let op = memory.rb(self.pc);
            debug!("Dump: sp:{:#x} pc:{:#x}", self.sp, self.pc);
            let op_code = &OP_CODES[op as usize];
            self.pc += 1; // increment the program counter to the next instruction
            let operands = match op_code.operand_size {
                1 => memory.rb(self.pc) as u16,
                2 => memory.rw(self.pc),
                _ => 0
            };
            self.pc += op_code.operand_size as u16;
            self.int_clk = 0;
            debug!("OpCode:{:#x}, name {}, Operands: {:#x}", op, op_code.name, operands);
            (OP_CODES[op as usize].f)(self, memory, operands); // Running the opcode behaviour
            if self.int_clk == 0 {
                self.int_clk = op_code.time
            }
            self.clock += self.int_clk;
        }
    }

    pub fn pop_stack<const BYTES:usize>(&mut self, mem: &mut MMU) -> [u8; BYTES]  {
        let pc = self.pc;
        self.pc+=BYTES as u16;
        mem.read::<BYTES>(pc)
    }

    /// Push a u16 value onto the stack, the stack pointer is at the end of the momory so the pointer
    /// is mooving backwards
    pub fn push_stack<const BYTES: usize>(&mut self, mem: &mut MMU,val : [u8;BYTES]) {
        mem.write(self.sp,val);
        self.sp -= BYTES as u16;
    }

    /// Push the actual address into the stack the increment the stack ptr;
    pub fn push_pc_stack(&mut self, mem: &mut MMU) {
        self.push_stack(mem,self.pc.to_le_bytes());
    }

    /// jump to the new address
    pub fn jmp(&mut self, addr: u16) {
        self.pc = addr;
    }

    /// Description:
    ///   Add n to HL.
    ///  Use with:
    ///   n = BC,DE,HL,SP
    ///  Flags affected:
    ///   Z - Not affected.
    ///   N - Reset.
    ///   H - Set if carry from bit 11.
    ///   C - Set if carry from bit 15.
    pub fn add_u16(&mut self, to: &mut u16, from: u16) {
        let result = from as u32 + *to as u32;
        self.set_flag_or_reset(Flags::CARRY, result & 0xffff0000 != 0 );
        *to = result as u16 & 0xffff;
        self.set_flag_or_reset(Flags::HALFCARRY, ((*to & 0x0f) + (from & 0x0f)) > 0x0f);
        self.f.remove(Flags::NEGATIVE)
    }

    pub fn dec(register: &mut u8) {
        // TODO
        unimplemented!()
    }


    pub fn sbc_a(&mut self, operand: u8) {
        let result = self.a as i32 - operand as i32;
        self.f |= Flags::NEGATIVE;
        self.set_flag_or_reset(Flags::ZERO, result == 0);
        self.set_flag_or_reset(Flags::CARRY, operand > self.a);
        self.set_flag_or_reset(Flags::HALFCARRY, (operand & 0x0f) > (self.a & 0x0f));
        self.a.wrapping_sub(operand);
    }

    /// 0x19
    pub fn add_hl_de(&mut self) {
        let mut hl = self.r_hl();
        let de = self.r_de();

        self.add_u16(&mut hl, de);
        self.w_hl(hl);
    }

    /// 0x20
    /// All the jr are handled bu this function, id the control and the cc are the same
    /// then jump to the location bu the addr in jmp.
    pub fn jr(&mut self, ctrl: Flags, offset_jmp: u8) {
        if ctrl == self.f {
            self.jmp(self.pc + offset_jmp as u16);
        }
    }

    /// 0x31 opcode
    /// Load 16 bites to sp register
    pub fn ld_sp_nn(&mut self, operands: u16) {
        debug!("Loaded {} into sp", operands);
        self.sp = operands;
    }

    /// 0xAF
    /// XOR N
    /// Logical exclusive OR n with register A, result in A.
    pub fn xor_n(&mut self, operands: u16) {
        self.f &= Flags::empty();
        self.a ^= operands as u8;
        if self.a == 0 {
            self.f = Flags::ZERO;
        }
    }
    /// 0xFE
    /// Compare A with n. This is basically an A - n
    ///   subtraction instruction but the results are thrown
    ///   away
    #[inline]
    pub fn cp_n(&mut self, operand: u16) {
        let operand = operand as u8;
        self.f &= Flags::empty();

        if self.a == operand {
            self.f |= Flags::ZERO;
        } else if self.a < operand {
            self.f |= Flags::CARRY;
        } else if (operand & 0x0f) > self.a & 0x0f {
            self.f |= Flags::HALFCARRY;
        }
        self.f |= Flags::NEGATIVE;
    }

    /// Generic reset
    /// Push the actual address to the stack then jumps to the address inp parameter.
    pub fn rst(&mut self, n: u16, mem: &mut MMU) {
        self.push_pc_stack(mem);
        self.jmp(n);
    }
}
