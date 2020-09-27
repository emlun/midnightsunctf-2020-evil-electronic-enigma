use leg_simulator::assemble_program;
use leg_simulator::generate_code;
use leg_simulator::LegComputer;
use leg_simulator::Word;

#[test]
fn add_bignum() -> Result<(), String> {
    // Memory address 2, 3 contain start, end address of number A (big endian)
    // Memory address 4, 5 contain start, end address of number B (big endian)
    // A and B must be of equal length
    // A + B (mod length) is written in place to memory range A
    let source = "
    JMP T ? 8
    HALT
    HALT
    HALT

    MOVC 0 => D

    LOAD 3 => A
    LOADP A => A
    LOAD 5 => B
    LOADP B => B
    ALU ECHO D D => D
    JMPR Z ? 6
    ALU ADDC A B => C
    JMPR T ? 4
    ALU ADD A B => C
    MOVC 1 => D
    JMPR Ou ? 4
    MOVC 0 => D

    LOAD 3 => A
    STOREP C => A
    MOVC 0 => C
    LOAD 5 => B
    STOREP C => B

    LOAD 2 => C
    ALU XOR A C => C
    JMP Z ? 6
    ALU DECR A A => A
    STORE A => 3

    LOAD 4 => C
    ALU XOR B C => C
    JMP Z ? 6
    ALU DECR B B => B
    STORE B => 5

    JMP T ? 10
    ";

    let program: Vec<Word> = generate_code(&assemble_program(source)?);
    let mut memory: Vec<Word> = Vec::with_capacity(256);

    let start_a = 100;
    let end_a = 113;
    let start_b = 200;
    let end_b = 213;

    memory.resize(100, 0);
    memory[2] = start_a;
    memory[3] = end_a;
    memory[4] = start_b;
    memory[5] = end_b;

    memory.resize(100, 0);
    memory.append(&mut vec![
        0xe0, 0xd0, 0xc0, 0xb0, 0xa0, 0x90, 0x80, 0x70, 0x60, 0x50, 0x40, 0x30, 0x20, 0x10,
    ]);

    memory.resize(200, 0);
    memory.append(&mut vec![
        0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
    ]);

    memory.resize(256, 0);

    let computer = LegComputer::new(program, memory).run();

    let expected: u128 = (0xe0d0c0b0a0908070605040302010 + 0x1111111111111111111111111111)
        & 0xffffffffffffffffffffffffffff;
    let mut actual: u128 = 0;
    for i in start_a..=end_a {
        actual <<= 8;
        actual += computer.memory[i as usize] as u128;
    }
    assert_eq!(actual, expected);

    Ok(())
}
