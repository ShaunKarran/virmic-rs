#[derive(Debug)]
pub struct MicroProcessor<'a> {
    r0: u8,
    r1: u8,
    pc: usize,
    memory: &'a [u8],
    is_running: bool,
}

impl<'a> MicroProcessor<'a> {
    pub fn new(memory: &'a [u8]) -> MicroProcessor<'a> {
        MicroProcessor { r0: 0, r1: 0, pc: 0, memory: memory, is_running: true }
    }

    pub fn cycle(&mut self) {
        let opcode = self.memory[self.pc];

        let pc_inc = match opcode {
            0x00 => { self.is_running = false; 0 },
            0x01 => { self.r0 = self.r0 + self.r1; 1 },
            0x02 => { self.r0 = self.r0 - self.r1; 1 },
            0x03 => { self.r0 = self.r0 + 1; 1 },
            0x04 => { self.r1 = self.r1 + 1; 1 },
            0x05 => { self.r0 = self.r0 - 1; 1 },
            0x06 => { self.r1 = self.r1 - 1; 1 },
            0x07 => { println!("{}", self.memory[self.pc + 1]); 2 },
            0x08 => { self.r0 = self.memory[self.pc + 1]; 2 },
            0x09 => { self.r1 = self.memory[self.pc + 1]; 2 },
            _ => { panic!("Could not decode instruction for opcode {}", opcode) }
        };

        self.pc += pc_inc;
    }
}

fn main() {
    println!("Welcome to the Virmic virtual microprocessor.");

    // Read program into 'memory'.
    let program: [u8; 4] = [0x09, 0x0A, 0x01, 0x00];

    let mut processor = MicroProcessor::new(&program);

    while processor.is_running {
        processor.cycle();
    }

    println!("{:?}", processor);
}
