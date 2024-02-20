use crate::{consts::*, display::Display};
use rand::Rng;
use std::{fs::File, io::Read, path::Path};

pub struct Cpu {
    // Config
    pub speed: u8, // CPU speed
    pub debug: bool,

    // Interaction
    pub pixels: [[bool; WIDTH]; HEIGHT],
    pub keypad: [u8; 16],

    // CPU
    opcode: u16,
    memory: Box<[u8; 4096]>,
    vreg: [u8; 16],   // 8-bit general purpose registers
    ireg: u16,        // Index register
    pc: u16,          // Program Counter
    stack: [u16; 16], // Interpeter returns to the address on the top of the stack after a subroutine is called
    sp: u8,           // Stack Pointer
    delay_timer: u8,
    pub sound_timer: u8,
    sttick: f32, // Sound timer tick
    tick: f32,   // CPU timer tick
}

impl Cpu {
    pub fn new(speed: u8, debug: bool) -> Cpu {
        let mut memory = [0; 4096];

        for i in 0..240 {
            memory[i] = FONT[i];
        }

        Cpu {
            speed: speed,
            debug: debug,

            pixels: [[false; WIDTH]; HEIGHT],
            keypad: [0; 16],

            opcode: 0,
            memory: Box::new(memory),
            vreg: [0; 16],
            ireg: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            sttick: 0.0,
            tick: 0.0,
        }
    }

    pub fn load_rom(&mut self, file_name: &str) {
        let path = Path::new(file_name);
        let mut file =
            File::open(&path).expect(format!("Failed to open ROM: {}", file_name).as_str());
        let mut buf: Vec<u8> = Vec::new();

        file.read_to_end(&mut buf)
            .expect(format!("Failed to read ROM: {}", file_name).as_str());

        if buf.len() >= 3585 {
            panic!("ROM is too large, size: {} > 3584", buf.len());
        }

        let buf_len = buf.len();

        for i in 0..buf_len {
            self.memory[i + 512] = buf[i];
        }

        println!("Loaded ROM: {} ({} bytes) ", file_name, buf_len);
    }

    pub fn tick(&mut self, display: &mut Display) {
        self.opcode = (self.memory[self.pc as usize] as u16) << 8
            | (self.memory[self.pc as usize + 1] as u16);

        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;
        let nn: u8 = (self.opcode & 0x00FF) as u8; // u8, byte 8-bit value

        let nnn: u16 = (self.opcode & 0x0FFF) as u16; // addr 12-bit value

        if self.debug {
            println!(
                "Opcode: 0x{:X} Vx: 0x{:X} Vy: 0x{:X} N: 0x{:X} NNN: 0x{:X}",
                self.opcode, vx, vy, nn, nnn
            );
        }
        match self.opcode & 0xF000 {
            0x0000 => match self.opcode & 0x00FF {
                // 00E0 Clear screen
                0x00E0 => {
                    self.pixels = [[false; WIDTH]; HEIGHT];
                    self.pc += 2;
                }

                // 00EE Return from subroutine
                0x00EE => {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                    self.pc += 2;
                }
                _ => println!("Unknown opcode: 00{:X}", self.opcode),
            },

            // 1NNN Jump to location
            0x1000 => {
                self.pc = nnn;
            }

            // 2NNN Call subroutine
            0x2000 => {
                self.stack[self.sp as usize] = self.pc;

                self.sp = self.sp.wrapping_add(1);
                self.pc = nnn;
            }

            // 3XNN Skip next instruction if Vx = nn
            0x3000 => {
                if self.vreg[vx] == nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            // 4XNN Skip next instruction if Vx != nn
            0x4000 => {
                if self.vreg[vx] != nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            // 5XY0 Skip next instruction if Vx = Vy
            0x5000 => {
                if self.vreg[vx] == self.vreg[vy] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            // 6XNN Set Vx = nn
            0x6000 => {
                self.vreg[vx] = nn;
                self.pc += 2;
            }

            // 7XNN Set Vx = Vx + nn
            0x7000 => {
                self.vreg[vx] = self.vreg[vx].wrapping_add(nn);
                self.pc += 2;
            }

            // 9XE0 Skip next instruction if Vx != Vy
            0x9000 => {
                if self.vreg[vx] != self.vreg[vy] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            // DXYN
            0xD000 => {
                self.draw(display);
                self.pc += 2;
            }

            0x8000 => {
                match self.opcode & 0x000F {
                    /* Logical and arithmetic instructions */
                    // 8XY0 Set Vx = Vy
                    0x0000 => {
                        self.vreg[vx] = self.vreg[vy];
                        self.pc += 2;
                    }

                    // 8XY1 Set Vx = Vx | Vy (Binary OR)
                    0x0001 => {
                        self.vreg[vx] = self.vreg[vx] | self.vreg[vy];
                        self.pc += 2;
                    }

                    // 8XY2 Set Vx = Vx & Vy (Binary AND)
                    0x0002 => {
                        self.vreg[vx] = self.vreg[vx] & self.vreg[vy];
                        self.pc += 2;
                    }

                    // 8XY3 Set Vx = Vx ^ Vy (Binary XOR)
                    0x0003 => {
                        self.vreg[vx] = self.vreg[vx] ^ self.vreg[vy];
                        self.pc += 2;
                    }

                    // 8XY4 Set Vx = Vx + Vy
                    0x0004 => {
                        self.vreg[vx] = self.vreg[vx].wrapping_add(self.vreg[vy]);
                        self.pc += 2;
                    }

                    // 8XY5 Set Vx = Vx - Vy
                    0x0005 => {
                        self.vreg[vx] = self.vreg[vx].wrapping_sub(self.vreg[vy]);
                        self.vreg[0xF] = if self.vreg[vy] > self.vreg[vx] { 0 } else { 1 };
                        self.pc += 2;
                    }

                    // 8XY6 Set Vx = Vx >> 1 (WARNING: AMBIGUOUS)
                    0x0006 => {
                        self.vreg[0xF] = self.vreg[vx] & 1;
                        self.vreg[vx] >>= 1;
                        self.pc += 2;
                    }

                    // 8XY7 Set Vx = Vy - Vx
                    0x0007 => {
                        self.vreg[vx] = self.vreg[vy].wrapping_sub(self.vreg[vx]);
                        self.vreg[0xF] = if self.vreg[vx] > self.vreg[vy] { 0 } else { 1 };
                        self.pc += 2;
                    }

                    // 8XYE Set Vx = Vx << 1 (WARNING: AMBIGUOUS)
                    0x000E => {
                        self.vreg[0xF] = (self.vreg[vx] >> 7) & 1;
                        self.vreg[vx] <<= 1;
                        self.pc += 2;
                    }

                    _ => println!("Unknown opcode: 8X{:X}", self.opcode),
                }
            }

            // ANNN Set I = NNN
            0xA000 => {
                self.ireg = nnn;
                self.pc += 2;
            }

            // BNNN Jump to address NNN + V0 (WARNING: AMBIGUOUS)
            0xB000 => {
                self.pc = nnn + self.vreg[0] as u16;
            }

            // CXNN Random
            0xC000 => {
                let mut rng = rand::thread_rng();
                self.vreg[vx] = rng.gen::<u8>() & nn;
                self.pc += 2;
            }

            /* Skip if key */
            0xE000 => {
                match self.opcode & 0x00FF {
                    // EX9E Skip next instruction if key with the value of Vx is pressed
                    0x009E => {
                        if self.keypad[self.vreg[vx] as usize] != 0 {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }

                    // EXA1 Skip next instruction if key with the value of Vx is not pressed
                    0x00A1 => {
                        if self.keypad[self.vreg[vx] as usize] == 0 {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    _ => println!("Unknown opcode: 00{:#X}", self.opcode),
                }
            }

            0xF000 => match self.opcode & 0x00FF {
                // FX07 Set Vx = delay timer
                0x0007 => {
                    self.vreg[vx] = self.delay_timer;
                    self.pc += 2;
                }

                // FX15 Set delay timer = Vx
                0x0015 => {
                    self.delay_timer = self.vreg[vx];
                    self.pc += 2;
                }

                // FX18 Set sound timer = Vx
                0x0018 => {
                    self.sound_timer = self.vreg[vx];
                    self.pc += 2;
                }

                // FX1E Set I = I + Vx
                0x001E => {
                    self.ireg += self.vreg[vx] as u16;
                    self.vreg[0xF] = (self.ireg > 0xFFF) as u8;
                    self.pc += 2;
                }

                // FX0A Block until a key is pressed
                0x000A => {
                    for i in 0..0xF {
                        if self.keypad[i] != 0 {
                            self.vreg[vx] = i as u8;
                            break;
                        }
                    }

                    self.pc += 2;
                }

                // FX29 Set I = location of sprite for digit Vx
                0x0029 => {
                    self.ireg = (self.vreg[vx] as u16).wrapping_mul(0x5);
                    self.pc += 2;
                }

                // Fx33 Store BCD representation of Vx in memory locations I, I+1, and I+2
                0x0033 => {
                    self.memory[self.ireg as usize] = self.vreg[vx] / 100;
                    self.memory[self.ireg as usize + 1] = (self.vreg[vx] % 100) / 10;
                    self.memory[self.ireg as usize + 2] = self.vreg[vx] % 10;
                    self.pc += 2;
                }

                // FX55 Store registers V0 through Vx in memory starting at location I
                0x0055 => {
                    for i in 0..=vx {
                        self.memory[self.ireg as usize + i] = self.vreg[i];
                    }
                    self.pc += 2;
                }

                // FX65 Fill registers V0 through Vx from memory starting at location I
                0x0065 => {
                    for i in 0..=vx {
                        self.vreg[i] = self.memory[self.ireg as usize + i];
                    }
                    self.pc += 2;
                }

                _ => println!("Unknown opcode: FX{:#X}", self.opcode),
            },
            _ => println!("Unknown opcode: {:#X}", self.opcode),
        }
    }

    pub fn draw(&mut self, display: &mut Display) {
        let sprite_w: usize = 8;
        let sprite_h = (self.opcode & 0x000F) as usize;
        let sprite_x = usize::from(self.vreg[((self.opcode & 0x0F00) >> 8) as usize]);
        let sprite_y = usize::from(self.vreg[((self.opcode & 0x00F0) >> 4) as usize]);

        self.vreg[0xF] = 0;

        for col in 0..sprite_h {
            let pixel = self.memory[self.ireg as usize + col as usize] as u16;

            for row in 0..sprite_w {
                let x = (sprite_x + row) % WIDTH;
                let y = (sprite_y + col) % HEIGHT;

                if pixel & if false { 0x8000 } else { 0x80 } >> row != 0 {
                    self.vreg[0xF] |= self.pixels[y % HEIGHT][x % WIDTH] as u8;
                    self.pixels[y][x] ^= true;
                }
            }
        }

        display.draw(&self.pixels);
    }

    pub fn reset(&mut self) {
        println!("Resetting CPU");

        for v in &mut self.vreg {
            *v = 0;
        }

        self.ireg = 0;
        self.pc = 0x200;
        self.sp = 0;
        self.stack = [0; 16];
        self.sttick = 0.0;
        self.tick = 0.0;

        self.delay_timer = 0;
        self.sound_timer = 0;

        self.pixels = [[false; WIDTH]; HEIGHT];
    }

    pub fn update_timers(&mut self, dt: f32) {
        if self.delay_timer > 0 {
            self.tick -= dt;

            if self.tick <= 0.0 {
                self.delay_timer -= 1;
                self.tick = 1.0 / 60.0;
            }
        }

        if self.sound_timer > 0 {
            self.sttick -= dt;

            if self.sttick <= 0.0 {
                self.sound_timer -= 1;
                self.sttick = 1.0 / 60.0;
            }
        }
    }
}
