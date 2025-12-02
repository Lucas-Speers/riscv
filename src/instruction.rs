
// pub fn debug_inst(inst: u32) {
//     dbg!(opcode(inst));
//     dbg!(rd(inst));
//     dbg!(funct3(inst));
//     dbg!(rs1(inst));
//     dbg!(rs2(inst));
//     dbg!(funct7(inst));
//     // dbg!(imm_i(inst));
//     // dbg!(imm_s(inst));
//     // dbg!(imm_b(inst));
//     // dbg!(imm_u(inst));
//     // dbg!(imm_j(inst));
// }

pub fn opcode(inst: u32) -> u32 {
    (inst >> 2) & 0b11111
}

pub fn rd(inst: u32) -> usize {
    ((inst >> 7) & 0b11111) as usize
}

pub fn funct3(inst: u32) -> u32 {
    (inst >> 12) & 0b111
}

pub fn rs1(inst: u32) -> usize {
    ((inst >> 15) & 0b11111) as usize
}

pub fn rs2(inst: u32) -> usize {
    ((inst >> 20) & 0b11111) as usize
}

pub fn funct7(inst: u32) -> u32 {
    inst >> 25
}

pub fn imm_i(inst: u32) -> i32 {
    (inst as i32) >> 20
}

pub fn imm_s(inst: u32) -> i32 {
    ((inst as i32 >> 7) & 0b11111) |
    ((inst as i32) >> 20)
}

pub fn imm_b(inst: u32) -> i32 {
    ((inst as i32 >> 7) & 0b11110) |
    ((inst as i32 >> 20) & 0b11111100000) |
    (((inst as i32) << 4) & 0b100000000000) |
    ((inst as i32 >> 19) & 0b11111111111111111111000000000000u32 as i32)
}

pub fn imm_u(inst: u32) -> i32 {
    inst as i32 & 0b11111111111111111111000000000000u32 as i32
}

pub fn imm_j(inst: u32) -> i32 {
    ((inst as i32 >> 20) & 0b11111111110) |
    ((inst as i32 >> 9) & 0b100000000000) |
    (inst as i32 & 0b11111111000000000000) |
    ((inst as i32 >> 11) & 0b11111111111100000000000000000000u32 as i32)
}

pub fn funct10(inst: u32) -> u32 {
    (funct7(inst) << 3) | funct3(inst)
}

pub mod opcode {
    pub const OP: u32 = 0b01100;
    pub mod op {
        pub const ADD: u32 = 0b000;
        pub const SUB: u32 = 0b0100000000;
        pub const SLL: u32 = 0b001;
        pub const SLT: u32 = 0b010;
        pub const SLTU: u32 = 0b011;
        pub const XOR: u32 = 0b100;
        pub const SRL: u32 = 0b101;
        pub const SRA: u32 = 0b0100000101;
        pub const OR: u32 = 0b110;
        pub const AND: u32 = 0b111;

        pub const MUL: u32 = 0b0000001000;
        pub const MULH: u32 = 0b0000001001;
        pub const MULHSU: u32 = 0b0000001010;
        pub const MULHU: u32 = 0b0000001011;
        pub const DIV: u32 = 0b0000001100;
        pub const DIVU: u32 = 0b0000001101;
        pub const REM: u32 = 0b0000001110;
        pub const REMU: u32 = 0b0000001111;
    }
    pub const OP_IMM: u32 = 0b00100;
    pub mod op_imm {
        pub const ADDI: u32 = 0b000;
        pub const SLLI: u32 = 0b001;
        pub const SLTI: u32 = 0b010;
        pub const SLTIU: u32 = 0b011;
        pub const XORI: u32 = 0b100;
        pub const SRLI: u32 = 0b101;
        pub const SRAI: u32 = 0b0100000101;
        pub const ORI: u32 = 0b110;
        pub const ANDI: u32 = 0b111;
    }
    pub const LUI: u32 = 0b01101;
    pub const AUIPC: u32 = 0b00101;
    pub const JAL: u32 = 0b11011;
    pub const JALR: u32 = 0b11001;
    pub const BRANCH: u32 = 0b11000;
    pub mod branch {
        pub const BEQ: u32 = 0b000;
        pub const BNE: u32 = 0b001;
        pub const BLT: u32 = 0b100;
        pub const BGE: u32 = 0b101;
        pub const BLTU: u32 = 0b110;
        pub const BGEU: u32 = 0b111;
    }
    pub const LOAD: u32 = 0b00000;
    pub mod load {
        pub const LB: u32 = 0b000;
        pub const LH: u32 = 0b001;
        pub const LW: u32 = 0b010;
        pub const LBU: u32 = 0b100;
        pub const LHU: u32 = 0b101;
    }
    pub const STORE: u32 = 0b01000;
    pub mod store {
        pub const SB: u32 = 0b000;
        pub const SH: u32 = 0b001;
        pub const SW: u32 = 0b010;
    }
    pub const MISC_MEM: u32 = 0b00011;
    pub const SYSTEM: u32 = 0b11100;
    pub mod system {
        pub const PRIV: u32 = 0;
    }
}