// #![allow(unused)]

use crate::{instruction::{funct3, funct10, imm_b, imm_i, imm_j, imm_s, imm_u, opcode, rd, rs1, rs2}, mem::{BasicRam, Bus, Memory}};

mod mem;
mod instruction;

struct Hart {
    memory: Box<dyn Memory>,
    regs: [i32;32],
    pc: u32,
    inst: u32,
}

impl Memory for Hart {
    fn read_8(&mut self, addr: u32) -> u8 {
        self.memory.read_8(addr)
    }

    fn write_8(&mut self, addr: u32, data: u8) {
        self.memory.write_8(addr, data);
    }
}

impl Hart {
    fn new(memory: Box<dyn Memory>) -> Hart {
        let mut regs: [i32;32] = [0;32];
        regs[1] = 5;
        regs[2] = 1;
        regs[3] = 42;
        regs[4] = 69;

        regs[2] = 0x880000;

        let pc = 0x80000000;

        Hart { memory, regs, pc, inst: 0 }
    }

    fn fetch(&mut self) {
        // TODO: instruction-address-misaligned exception 
        self.inst = self.read_32(self.pc);
        // 0x003170b3
    }

    fn execute(&mut self) {
        self.fetch();
        self.regs[0] = 0;
        self.pc += 4;

        match opcode(self.inst) {
            opcode::OP => {
                match funct10(self.inst) {
                    opcode::op::ADD => {
                        *self.rd() = self.rs1() + self.rs2();
                    }
                    opcode::op::SUB => {
                            *self.rd() = self.rs1() - self.rs2();
                    }
                    opcode::op::SLL => {
                        *self.rd() = self.rs1() << (self.rs2() & 0b11111);
                    }
                    opcode::op::SLT => {
                        *self.rd() =
                            if self.rs1() < self.rs2() {1}
                            else {0};
                    }
                    opcode::op::SLTU => {
                        *self.rd() =
                            if (self.rs1() as u64) < (self.rs2() as u64) {1}
                            else {0};
                    }
                    opcode::op::XOR => {
                        *self.rd() = self.rs1() ^ self.rs2();
                    }
                    opcode::op::SRL => {
                        *self.rd() = ((self.rs1() as u32) >> (self.rs2() & 0b11111)) as i32;
                    }
                    opcode::op::SRA => {
                        *self.rd() = self.rs1() >> (self.rs2() & 0b11111);
                    }
                    opcode::op::OR => {
                        *self.rd() = self.rs1() | self.rs2();
                    }
                    opcode::op::AND => {
                        *self.rd() = self.rs1() & self.rs2();
                    }

                    opcode::op::MUL => {
                        *self.rd() = self.rs1() * self.rs2();
                    }
                    opcode::op::MULH => {
                        *self.rd() = (((self.rs1() as i64) * (self.rs2() as i64)) >> 32) as i32;
                    }
                    opcode::op::MULHSU => {
                        // *self.rd() = (((self.rs1() as i64) * (self.rs2() as u64)) >> 32) as i32;
                        todo!("MULHSU")
                    }
                    opcode::op::MULHU => {
                        *self.rd() = (((self.rs1() as u64) * (self.rs2() as u64)) >> 32) as i32;
                    }
                    opcode::op::DIV => {
                        *self.rd() = self.rs1() / self.rs2();
                    }
                    opcode::op::DIVU => {
                        *self.rd() = ((self.rs1() as u32) / (self.rs2() as u32)) as i32;
                    }
                    opcode::op::REM => {
                        *self.rd() = self.rs1() % self.rs2();
                    }
                    opcode::op::REMU => {
                        *self.rd() = ((self.rs1() as u32) % (self.rs2() as u32)) as i32;
                    }
                    _ => todo!("OP {:?}", funct3(self.inst))
                }
            }
            opcode::OP_IMM => {
                match funct10(self.inst) {
                    opcode::op_imm::ADDI => {
                        *self.rd() = self.rs1() + imm_i(self.inst);
                    }
                    opcode::op_imm::SLLI => {
                        *self.rd() = self.rs1() << (imm_i(self.inst) & 0b11111);
                    }
                    opcode::op_imm::SLTI => {
                        *self.rd() =
                            if self.rs1() < imm_i(self.inst) {1}
                            else {0};
                    }
                    opcode::op_imm::SLTIU => {
                        *self.rd() =
                            if (self.rs1() as u64) < (imm_i(self.inst) as u64) {1}
                            else {0};
                    }
                    opcode::op_imm::XORI => {
                        *self.rd() = self.rs1() ^ imm_i(self.inst);
                    }
                    opcode::op_imm::SRLI => {
                        *self.rd() = ((self.rs1() as u32) >> (imm_i(self.inst) & 0b11111)) as i32;
                    }
                    opcode::op_imm::SRAI => {
                        *self.rd() = self.rs1() >> (imm_i(self.inst) & 0b11111);
                    }
                    opcode::op_imm::ORI => {
                        *self.rd() = self.rs1() | imm_i(self.inst);
                    }
                    opcode::op_imm::ANDI => {
                        *self.rd() = self.rs1() & imm_i(self.inst);
                    }
                    _ => todo!("OP-IMM {:?}", funct3(self.inst))
                }
            }
            opcode::LUI => {
                *self.rd() = imm_u(self.inst);
            }
            opcode::AUIPC => {
                *self.rd() = imm_u(self.inst) + self.pc as i32;
            }
            opcode::JAL => {
                *self.rd() = self.pc as i32 + 4;
                self.pc += (imm_j(self.inst) - 4) as u32;
            }
            opcode::JALR => {
                *self.rd() = self.pc as i32 + 4;
                self.pc += (imm_i(self.inst) + self.rs1() - 4) as u32 &!1;
            }
            opcode::BRANCH => {
                match funct3(self.inst) {
                    opcode::branch::BEQ => {
                        if self.rs1() == self.rs2() {
                            self.pc += (imm_b(self.inst) - 4) as u32;
                        }
                    }
                    opcode::branch::BNE => {
                        if self.rs1() != self.rs2() {
                            self.pc += (imm_b(self.inst) - 4) as u32;
                        }
                    }
                    opcode::branch::BLT => {
                        if self.rs1() < self.rs2() {
                            self.pc += (imm_b(self.inst) - 4) as u32;
                        }
                    }
                    opcode::branch::BGE => {
                        if self.rs1() >= self.rs2() {
                            self.pc += (imm_b(self.inst) - 4) as u32;
                        }
                    }
                    opcode::branch::BLTU => {
                        if (self.rs1() as u32) < (self.rs2() as u32) {
                            self.pc += (imm_b(self.inst) - 4) as u32;
                        }
                    }
                    opcode::branch::BGEU => {
                        if (self.rs1() as u32) >= (self.rs2() as u32) {
                            self.pc += (imm_b(self.inst) - 4) as u32;
                        }
                    }
                    _ => todo!("BRANCH {:?}", funct3(self.inst))
                }
            }
            opcode::LOAD => {
                match funct3(self.inst) {
                    opcode::load::LB => {
                        *self.rd() = self.read_8((imm_i(self.inst) + self.rs1()) as u32) as i8 as i32;
                    }
                    opcode::load::LH => {
                        *self.rd() = self.read_16((imm_i(self.inst) + self.rs1()) as u32) as i16 as i32;
                    }
                    opcode::load::LW => {
                        *self.rd() = self.read_32((imm_i(self.inst) + self.rs1()) as u32) as i32;
                    }
                    opcode::load::LBU => {
                        *self.rd() = self.read_8((imm_i(self.inst) + self.rs1()) as u32) as i32;
                    }
                    opcode::load::LHU => {
                        *self.rd() = self.read_16((imm_i(self.inst) + self.rs1()) as u32) as i32;
                    }
                    _ => todo!("LOAD {:?}", funct3(self.inst))
                }
            }
            opcode::STORE => {
                match funct3(self.inst) {
                    opcode::store::SB => {
                        self.write_8((imm_s(self.inst) + self.rs1()) as u32, self.rs2() as u8);
                    }
                    opcode::store::SH => {
                        self.write_16((imm_s(self.inst) + self.rs1()) as u32, self.rs2() as u16);
                    }
                    opcode::store::SW => {
                        self.write_32((imm_s(self.inst) + self.rs1()) as u32, self.rs2() as u32);
                    }
                    _ => todo!("STORE {:?}", funct3(self.inst))
                }
            }
            opcode::MISC_MEM => {
                match funct3(self.inst) {
                    opcode::misc_mem::FENCE => (), // because memory is synced, we have no use for FENCE
                    _ => todo!("MISC-MEM {:?}", funct3(self.inst))
                }
            }
            opcode::SYSTEM => {
                match funct3(self.inst) {
                    opcode::system::PRIV => {todo!("PRIV")}
                    _ => todo!("SYSTEM {:?}", funct3(self.inst))
                }
            }

            _ => todo!("opcode {:?}", self.inst)
        }

    }

    fn rd(&mut self) -> &mut i32 {
        &mut self.regs[rd(self.inst)]
    }

    fn rs1(&self) -> i32 {
        self.regs[rs1(self.inst)]
    }

    fn rs2(&self) -> i32 {
        self.regs[rs2(self.inst)]
    }

    fn debug(&self) {
        println!("PC -> {:b}", self.pc);
        println!("x0 -> {:b}", self.regs[0]);
        println!("x1 -> {:b}", self.regs[1]);
        println!("x2 -> {:b}", self.regs[2]);
        println!("x3 -> {:b}", self.regs[3]);
        println!("x4 -> {:b}\n", self.regs[4]);
    }
}


fn main() {
    let memory = BasicRam::new();
    let bus = Bus::new(Box::new(memory));

    let mut cpu = Hart::new(Box::new(bus));

    loop {
        cpu.debug();
        cpu.execute();
        // debug_inst(cpu.inst);
    }
}
