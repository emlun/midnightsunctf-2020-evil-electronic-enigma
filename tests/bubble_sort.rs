use leg_simulator::assemble_program;
use leg_simulator::generate_code;
use leg_simulator::LegComputer;
use leg_simulator::Word;
use rand::Rng;

// Memory address 2, 3 contain start (inclusive), end (inclusive) of list
// List is sorted in place
const SOURCE: &str = "
JMP T ? 8
HALT
HALT
HALT

LOAD 3 => D

LOAD 2 => C
ALU XOR C D => A
JMP Z ? 6

ALU XOR C D => A
JMPR Z ? 4
JMPR T ? 8
ALU DECR D D => D
STORE D => 3
JMP T ? 10

LOADP C => A
MOV C => B
ALU INCR B B => B
LOADP B => B
ALU ECHO A B => A
JMPR GT ? 6

ALU INCR C C => C
JMP T ? 16

LOADP C => A
ALU INCR C B => B
LOADP B => B
STOREP B => C
ALU INCR C C => C
STOREP A => C
JMPR T ? 4

ALU INCR C C => C
JMP T ? 16
";

#[test]
fn bubble_sort() -> Result<(), String> {
    let mut program: Vec<Word> = generate_code(&assemble_program(SOURCE)?);

    let start_list = 104;
    let list_len = 128;
    let end_list = start_list + list_len - 1;

    program[2] = start_list;
    program[3] = end_list;

    while program.len() < start_list.into() {
        program.push(0);
    }
    for i in (0..list_len).rev() {
        program.push(i);
    }
    while program.len() < 256 {
        program.push(0);
    }

    let computer = LegComputer::new(program).run();

    assert_eq!(
        *(0..list_len).collect::<Vec<u8>>().as_slice(),
        computer.memory[start_list.into()..=end_list.into()]
    );

    Ok(())
}

#[test]
fn bubble_sort_random() -> Result<(), String> {
    let mut program: Vec<Word> = generate_code(&assemble_program(SOURCE)?);

    let start_list = 104;
    let list_len = 8;
    let end_list = start_list + list_len - 1;

    program[2] = start_list;
    program[3] = end_list;

    let mut rng = rand::thread_rng();
    let mut input = Vec::new();
    while input.len() < list_len.into() {
        input.push(rng.gen());
    }

    while program.len() < start_list.into() {
        program.push(0);
    }
    for i in (0..list_len).rev() {
        program.push(input[usize::from(i)]);
    }
    while program.len() < 256 {
        program.push(0);
    }

    let computer = LegComputer::new(program).run();

    input.sort();

    assert_eq!(
        *input.as_slice(),
        computer.memory[start_list.into()..=end_list.into()]
    );

    Ok(())
}
