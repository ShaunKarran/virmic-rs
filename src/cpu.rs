#[derive(Debug)]
pub struct CPU<'a> {
    r0: u8,
    r1: u8,
    pc: u8,
    memory: &'a mut [u8],
    is_running: bool,
}

impl<'a> CPU<'a> {
    pub fn new(memory: &'a mut [u8]) -> CPU<'a> {
        CPU { r0: 0, r1: 0, pc: 0, memory: memory, is_running: true }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn cycle(&mut self) {
        let opcode = self.memory[self.pc as usize];

        let pc_inc = match opcode {
            0x00 => { self.is_running = false; 0 },
            0x01 => { self.r0 = self.r0 + self.r1; 1 },
            0x02 => { self.r0 = self.r0 - self.r1; 1 },
            0x03 => { self.r0 = self.r0 + 1; 1 },
            0x04 => { self.r1 = self.r1 + 1; 1 },
            0x05 => { self.r0 = self.r0 - 1; 1 },
            0x06 => { self.r1 = self.r1 - 1; 1 },
            0x07 => { println!("{}", self.memory[self.fetch_byte() as usize]); 2 },
            0x08 => { self.r0 = self.fetch_byte(); 2 },
            0x09 => { self.r1 = self.fetch_byte(); 2 },
            0x0a => { self.memory[self.fetch_byte() as usize] = self.r0; 2 },
            0x0b => { self.memory[self.fetch_byte() as usize] = self.r1; 2 },
            0x0c => { self.pc = self.fetch_byte(); 0 },
            0x0d => {
                if self.r0 == 0 {
                    self.pc = self.fetch_byte();
                }
                0
            },
            0x0e => {
                if self.r0 != 0 {
                    self.pc = self.fetch_byte()
                }
                0
            },
            _ => { panic!("Could not decode instruction for opcode {}", opcode) }
        };

        self.pc += pc_inc;
    }

    fn fetch_byte(&self) -> u8 {
        self.memory[(self.pc + 1) as usize]
    }
}
