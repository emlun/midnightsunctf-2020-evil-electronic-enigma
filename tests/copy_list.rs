use leg_simulator::assemble_program;
use leg_simulator::generate_code;
use leg_simulator::LegComputer;
use leg_simulator::Word;
use rand::Rng;

// Memory address 2, 3 contain start (inclusive), end (exclusive) of list
// List is copied to memory segment immediately following it
const COPY_LIST_PROG: &str = "
JMPR T ? 4
HALT
LOAD 2 => A
LOAD 3 => B
PUSH A
PUSH B
PUSH B
CALLC 18
HALT
";

// Stack offset 4, 3 contain start (inclusive), end (exclusive) of list
// offset 2 contains start (inclusive) of copy destination
const COPY_LIST_FN: &str = "
# Function: copy list
SLOAD 4 => A
SLOAD 3 => B
SLOAD 2 => C

ALU ECHO A B => A
JMPR LT ? 4
RET A

LOADP A => D
STOREP D => C
ALU INCR A A => A
ALU INCR C C => C
JMPR T ? -14
";

fn test_reversed_range(source: &str, range_len: Word) -> Result<(), String> {
    let mut program: Vec<Word> = generate_code(&assemble_program(source)?);

    let start_list = (program.len() + 8 - (program.len() % 8)) as u8;
    let end_list = start_list + range_len;

    program[2] = start_list;
    program[3] = end_list;

    program.resize(start_list.into(), 0);
    program.append(&mut (0..range_len).rev().collect());
    program.resize(256, 0);

    let computer = LegComputer::new(program).run();

    assert_eq!(
        *(0..range_len).rev().collect::<Vec<u8>>().as_slice(),
        computer.memory[start_list.into()..end_list.into()]
    );

    Ok(())
}

fn test_random_list(source: &str, list_len: Word) -> Result<(), String> {
    let mut program: Vec<Word> = generate_code(&assemble_program(source)?);

    let start_list = (program.len() + 8 - (program.len() % 8)) as u8;
    let end_list = start_list + list_len;

    program[2] = start_list;
    program[3] = end_list;

    let mut rng = rand::thread_rng();
    let mut input = Vec::new();
    input.resize_with(list_len.into(), || rng.gen());

    program.resize(start_list.into(), 0);
    program.append(&mut input.clone());
    program.resize(256, 0);

    let computer = LegComputer::new(program).run();

    assert_eq!(
        input[..],
        computer.memory[start_list.into()..end_list.into()]
    );
    assert_eq!(
        input[..],
        computer.memory[(start_list + list_len).into()..(end_list + list_len).into()]
    );

    Ok(())
}

#[test]
fn copy_list() -> Result<(), String> {
    test_reversed_range(&format!("{}\n{}", COPY_LIST_PROG, COPY_LIST_FN), 16)
}

#[test]
fn copy_list_random() -> Result<(), String> {
    test_random_list(&format!("{}\n{}", COPY_LIST_PROG, COPY_LIST_FN), 16)
}
