use crate::rom::Rom;
use crate::ram::Ram;

const ADD:   u8 =   0x00;
const SUB:   u8 =   0x01;
const AND:   u8 =   0x02;
const OR:    u8 =   0x03;
const XOR:   u8 =   0x04;
const NOT:   u8 =   0x05;
const LDX_I: u8 =   0x06;
const LDY_I: u8 =   0x07;
const STX:   u8 =   0x08;
const STY:   u8 =   0x09;
const JMP:   u8 =   0x0A;
const JIC:   u8 =   0x0B;
const JIZ:   u8 =   0x0C;
const HLT:   u8 =   0x0D;
const LDX_A: u8 =   0x0E;
const LDY_A: u8 =   0x0F;

pub struct SPU{
    program_counter: u16,
    register_x: u16,
    register_y: u16,
    register_p: u16,
    carry_flag: bool,
    zero_flag: bool,
    rom: Rom,
    ram: Ram
}

impl SPU{
    pub fn new(rom: Rom, ram: Ram) -> Self{
        Self{
            rom: rom,
            ram: ram,
            program_counter: 0u16,
            register_x: 0u16,
            register_y: 0u16,
            register_p: 0u16,
            carry_flag: false,
            zero_flag: false
        } 
    }

    pub fn reset(&mut self){
        self.program_counter = 0u16;
        self.register_x = 0u16;
        self.register_y = 0u16;
        self.carry_flag = false;
        self.zero_flag = false;
    }

    fn get_rom_byte(&mut self, cycles: &mut u32) -> u16{
        *cycles += 1;
        self.rom.get(self.program_counter)
    }

    fn get_ram_value(&self, cycles: &mut u32) -> u16{
        *cycles += 4;
        self.ram.get(self.register_p)
    }

    fn set_ram_value(&mut self, value: u16, cycles: &mut u32){
        *cycles += 4;
        self.ram.set(value, self.register_p);
    }

    fn update_flags(&mut self, cycles: &mut u32){
        self.zero_flag = self.register_x == 0;
        *cycles += 1;
    }

    pub fn execute(&mut self, max_cycles: Option<u32>) -> u32{
        let mut cycles: u32 = 0;
        let max_cycles: u32 = *max_cycles.clone().get_or_insert(0);
        loop{
            if max_cycles > 0 && cycles >= max_cycles{
                break;
            }
            let instruction_data: u16 = self.get_rom_byte(&mut cycles);
            let instruction: u8 = (instruction_data >> 12) as u8;
            let argument: u16 = instruction_data & 0b0000111111111111u16;
            self.program_counter += 1;
            match instruction{
                ADD => {
                    self.carry_flag = (self.register_x as u32 + self.register_y as u32) > 0xFFFF;
                    self.register_x += self.register_y;
                    self.update_flags(&mut cycles);
                },
                SUB => {
                    self.carry_flag = (self.register_x as u32 - self.register_y as u32) > 0xFFFF;
                    self.register_x -= self.register_y;
                    self.update_flags(&mut cycles);
                },
                AND => {
                    self.register_x &= self.register_y;
                    self.update_flags(&mut cycles);
                },
                OR => {
                    self.register_x |= self.register_y;
                    self.update_flags(&mut cycles);
                },
                XOR => {
                    self.register_x ^= self.register_y;
                    self.update_flags(&mut cycles);
                },
                NOT => {
                    self.register_x = !self.register_x;
                    self.update_flags(&mut cycles);
                },
                LDX_I => {
                    self.register_x = argument;
                },
                LDY_I => {
                    self.register_y = argument;
                },
                STX => {
                    self.register_p = argument;
                    self.set_ram_value(self.register_x, &mut cycles);
                },
                STY => {
                    self.register_p = argument;
                    self.set_ram_value(self.register_y, &mut cycles);
                },
                JMP => {
                    self.program_counter = argument;
                    cycles += 1;
                },
                JIC => {
                    if self.carry_flag{
                        self.program_counter = argument;
                    }
                    cycles += 1;
                },
                JIZ => {
                    if self.zero_flag{
                        self.program_counter = argument;
                    }
                    cycles += 1;
                }
                HLT => {
                    break;
                },
                LDX_A => {
                    self.register_p = argument;
                    self.register_x = self.get_ram_value(&mut cycles);
                },
                LDY_A => {
                    self.register_p = argument;
                    self.register_y = self.get_ram_value(&mut cycles);
                },
                unknown => {
                    println!("Unknown instruction {}.", unknown);
                }
            }
        }
        cycles
    }

    pub fn get_ram(&self) -> &Ram{
        &self.ram
    }

    pub fn is_zero_flag(&self) -> bool{
        self.zero_flag
    }

    pub fn is_carry_flag(&self) -> bool{
        self.carry_flag
    }

    pub fn get_register_x(&self) -> u16{
        self.register_x
    }

    pub fn get_register_y(&self) -> u16{
        self.register_y
    }

    pub fn get_register_p(&self) -> u16{
        self.register_p
    }

    pub fn get_program_counter(&self) -> u16{
        self.program_counter
    }
}
