use leg_simulator::assemble_program;
use leg_simulator::generate_code;
use leg_simulator::LegComputer;
use leg_simulator::Word;
use rand::Rng;

// Memory address 2, 3 contain start (inclusive), end (inclusive) of list
// List is sorted in place
const BUBBLE_SORT: &str = "
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

// Memory address 2, 3 contain start (inclusive), end (inclusive) of list
// List is sorted in place
const QUICKSORT: &str = "
JMP T ? 4
HALT

LOAD 2 => C
LOAD 3 => D
PUSH C
PUSH D

CALLC 16
HALT

# Subroutine: Carry the pivot forward

# If start is past end, return
SLOAD 3 => C
SLOAD 2 => D
ALU ECHO C D => C
JMPR LT ? 4
RET C

MOV C => D

# LOOP1:

# If next was last element, recurse then return
SLOAD 2 => A
ALU ECHO D A => D
JMPR NE ? 28

PUSH C

SLOAD 3 => A
PUSH A
ALU DECR C C => A
PUSH A
CALLC 16

POP A
ALU INCR A A => A
PUSH A
SLOAD 2 => A
PUSH A
CALLC 16

RET C


# Set next location
ALU INCR D D => D

# Load pivot and next
LOADP C => A
LOADP D => B

# If pivot is greater than next, swap places
ALU ECHO A B => A
JMPR LE ? 12
STOREP B => C
# Put pivot after the swapped element
ALU INCR C C => C
LOADP C => B
STOREP B => D
STOREP A => C

# Loop: LOOP1
JMP T ? 28
";

fn test_reversed_range(source: &str, range_len: Word) -> Result<(), String> {
    let mut program: Vec<Word> = generate_code(&assemble_program(source)?);

    let start_list = (program.len() + 8 - (program.len() % 8)) as u8;
    let end_list = start_list + range_len - 1;

    program[2] = start_list;
    program[3] = end_list;

    while program.len() < start_list.into() {
        program.push(0);
    }
    for i in (0..range_len).rev() {
        program.push(i);
    }
    while program.len() < 256 {
        program.push(0);
    }

    let computer = LegComputer::new(program).run();

    assert_eq!(
        *(0..range_len).collect::<Vec<u8>>().as_slice(),
        computer.memory[start_list.into()..=end_list.into()]
    );

    Ok(())
}

fn test_random_list(source: &str, list_len: Word) -> Result<(), String> {
    let mut program: Vec<Word> = generate_code(&assemble_program(source)?);

    let start_list = (program.len() + 8 - (program.len() % 8)) as u8;
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

#[test]
fn bubble_sort() -> Result<(), String> {
    test_reversed_range(BUBBLE_SORT, 128)
}

#[test]
fn bubble_sort_random() -> Result<(), String> {
    test_random_list(BUBBLE_SORT, 128)
}

#[test]
fn quicksort() -> Result<(), String> {
    test_reversed_range(QUICKSORT, 27)
}

#[test]
fn quicksort_random() -> Result<(), String> {
    test_random_list(QUICKSORT, 27)
}
