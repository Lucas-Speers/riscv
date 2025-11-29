
/// Little-endian by default
pub trait Memory {
    fn read_8(&mut self, addr: u32) -> u8;
    fn write_8(&mut self, addr: u32, data: u8);

    fn read_16(&mut self, addr: u32) -> u16 {
        ((self.read_8(addr) as u16) << 8) | (self.read_8(addr+1) as u16)
    }

    fn read_32(&mut self, addr: u32) -> u32 {
        ((self.read_16(addr) as u32) << 16) | (self.read_16(addr+2) as u32)
    }

    fn write_16(&mut self, addr: u32, data: u16) {
        self.write_8(addr, (data >> 8) as u8);
        self.write_8(addr+1, data as u8);
    }

    fn write_32(&mut self, addr: u32, data: u32) {
        self.write_16(addr, (data >> 16) as u16);
        self.write_16(addr+2, data as u16);
    }

}


pub struct BasicRam {
    mem: Vec<u8>
}

impl BasicRam {

    pub fn new() -> BasicRam {
        let program = include_bytes!("../program");

        let mem = program.to_vec();

        BasicRam { mem }
    }
}

impl Memory for BasicRam {
    fn read_8(&mut self, addr: u32) -> u8 {
        self.mem[addr as usize]
    }

    fn write_8(&mut self, addr: u32, data: u8) {
        self.mem[addr as usize] = data;
    }
}


pub struct Bus {
    ram: Box<dyn Memory>
}

impl Bus {
    pub fn new(ram: Box<dyn Memory>) -> Bus {
        Bus { ram }
    }
}

impl Memory for Bus {
    fn read_8(&mut self, addr: u32) -> u8 {
        self.ram.read_8(addr)
    }

    fn write_8(&mut self, addr: u32, data: u8) {
        self.ram.write_8(addr, data);
    }
}