/*
*   Here is all the tests for this emulator
*/

use crate::spu::SPU;
use crate::rom::Rom;
use crate::ram::Ram;

fn execute_spu(program: Vec<u16>, max_cycles: Option<u32>) -> (SPU, u32){
    let mut rom: Rom = Rom::new();
    rom.load(program);
    let mut spu: SPU = SPU::new(rom, Ram::new());
    let cycles: u32 = spu.execute(max_cycles);
    (spu, cycles)
}

#[test]
fn test_add(){
    let execute_context: (SPU, u32) = execute_spu(vec![0x6002, 0x7002, 0x0000], Some(4));
    assert_eq!(4, execute_context.0.get_register_x());
}

#[test]
fn test_sub(){
    let execute_context: (SPU, u32) = execute_spu(vec![0x6002, 0x7002, 0x1000], Some(4));
    assert_eq!(0, execute_context.0.get_register_x());
}

#[test]
fn test_and(){
    let execute_context: (SPU, u32) = execute_spu(vec![0x6002, 0x7003, 0x2000], Some(4));
    assert_eq!(2, execute_context.0.get_register_x());
}

#[test]
fn test_or(){
    let execute_context: (SPU, u32) = execute_spu(vec![0x60F0, 0x700F, 0x3000], Some(4));
    assert_eq!(0x00FF, execute_context.0.get_register_x());
}

#[test]
fn test_xor(){
    let execute_context: (SPU, u32) = execute_spu(vec![0x60FF, 0x700F, 0x4000], Some(4));
    assert_eq!(0x00F0, execute_context.0.get_register_x());
}

#[test]
fn test_not(){
    let execute_context: (SPU, u32) = execute_spu(vec![0x6FFF, 0x5000], Some(3));
    assert_eq!(0xF000, execute_context.0.get_register_x());
}

#[test]
fn test_ldx_im(){
    let execute_context: (SPU, u32) = execute_spu(vec![0x6FFF], Some(1));
    assert_eq!(0x0FFF, execute_context.0.get_register_x());
}

#[test]
fn test_ldy_im(){
    let execute_context: (SPU, u32) = execute_spu(vec![0x7FFF], Some(1));
    assert_eq!(0x0FFF, execute_context.0.get_register_y());
}

#[test]
fn test_stx(){
    let execute_context: (SPU, u32) = execute_spu(vec![0x60F0, 0x8001], Some(5));
    assert_eq!(0x00F0, execute_context.0.get_ram().get(0x0001));
}

#[test]
fn test_sty(){
    let execute_context: (SPU, u32) = execute_spu(vec![0x70FF, 0x900F], Some(5));
    assert_eq!(0x00FF, execute_context.0.get_ram().get(0x000F));
}

#[test]
fn test_jmp(){
    let execute_context: (SPU, u32) = execute_spu(vec![0xAFFF], Some(1));
    assert_eq!(0x0FFF, execute_context.0.get_program_counter());
}

#[test]
fn test_jic(){
    let execute_context: (SPU, u32) = execute_spu(vec![0x6000, 0x7001, 0x1000, 0xBFF0, 0xD000], Some(5));
    assert_eq!(0x0FF0, execute_context.0.get_program_counter());
    assert_eq!(true, execute_context.0.is_carry_flag());
}

#[test]
fn test_jiz(){
    let execute_context: (SPU, u32) = execute_spu(vec![0x6FFF, 0x7FFF, 0x1000, 0xCFF0], Some(5));
    assert_eq!(0x0FF0, execute_context.0.get_program_counter());
    assert_eq!(true, execute_context.0.is_zero_flag());
}

#[test]
fn test_hlt(){
    let execute_context: (SPU, u32) = execute_spu(vec![0xD000], Some(10));
    assert_eq!(1, execute_context.1);
}

#[test]
fn test_ldx_ad(){
    let execute_context: (SPU, u32) = execute_spu(vec![0x70FF, 0x9000, 0xE000], Some(9));
    assert_eq!(0x00FF, execute_context.0.get_register_x());
}

#[test]
fn test_ldy_ad(){
    let execute_context: (SPU, u32) = execute_spu(vec![0x60FF, 0x8000, 0xF000], Some(9));
    assert_eq!(0x00FF, execute_context.0.get_register_y());
}
