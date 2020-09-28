use leg_simulator::assemble_program;
use leg_simulator::generate_code;
use leg_simulator::LegComputer;
use leg_simulator::RegisterRef;
use leg_simulator::Word;

// Memory address 0, 1 contain start (inclusive), end (exclusive) of list
// Memory address 2 contains start (inclusive) of correct result
// Memory address 4, 5 contain start (inclusive), end (inclusive) of output for correct
// Memory address 6, 7 contain start (inclusive), end (inclusive) of output for incorrect
// List is copied to range immediately following it,
// then the copy is sorted in place,
// then the XOR of the two lists is compared against the correct result.
// Writes output for correct at address 0 if equal, or output for incorrect otherwise.
const CHALLENGE_PROG: &str = "
LOAD 0 => C
LOAD 1 => D
PUSH C
PUSH D
PUSH D
CALLR 72
POP C
POP D
POP D
POP D

ALU SUB C D => D
ALU ADD C D => D
ALU DECR D D => D
PUSH C
PUSH D
CALLR 128
POP A
POP D
ALU INCR D D => D
PUSH D

LOAD 0 => B
PUSH B
LOAD 2 => B
PUSH B
CALLR 58
POP A

ALU ECHO A A => A
JMPR Z ? 8
LOAD 6 => C
LOAD 7 => D
JMPR T ? 8
LOAD 4 => C
LOAD 5 => D
NOP

MOVC 0 => B
LOADP C => A
STOREP A => B
ALU INCR B B => B
ALU INCR C D => C
JMPR LT ? -8
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
JMPR LT ? 6
SLOAD 2 => C
RET C

LOADP A => D
STOREP D => C
ALU INCR A A => A
ALU INCR C C => C
JMPR T ? -16
";

// Check the xor of two lists against a predefined correct result.
// Stack offset 5, 4 contain start (inclusive), end (exclusive) of list 1
// Stack offset 3 contains start (inclusive) of list 2
// offset 2 contains start (inclusive) of the correct xor template
// Returns zero if xor result was equal to correct xor template
const XOR_LIST_CHECK_FN: &str = "
# Function: xor list check
SLOAD 5 => C
SLOAD 3 => B
SLOAD 2 => D
PUSH D
MOVC 0 => D
PUSH D

SLOAD 4 => D

ALU ECHO C D => C
JMPR LT ? 6
POP A
RET A

LOADP B => A
LOADP C => D
ALU XOR A D => A

SLOAD -1 => D
LOADP D => D
ALU XOR A D => A

POP D
ALU OR A D => A

ALU INCR B B => B
ALU INCR C C => C
POP D
ALU INCR D D => D
PUSH D
PUSH A

JMPR T ? -38
";

// Stack offset 3, 2 contain start (inclusive), end (inclusive) of list
// List is sorted in place
const QUICKSORT_FN: &str = "
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
JMPR NE ? 38

PUSH C

SLOAD 3 => A
PUSH A
ALU DECR C C => A
PUSH A
CALLR -28
POP A
POP B
POP B

ALU INCR A A => A
PUSH A
SLOAD 2 => A
PUSH A
CALLR -44
POP A
POP A
POP A

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
JMPR T ? -62
";

#[test]
fn xor_fn() -> Result<(), String> {
    let source = &format!(
        "
MOVC 32 => A
PUSH A
MOVC 64 => A
PUSH A

PUSH A

MOVC 0 => A
PUSH A

CALLR 4
HALT

{}",
        XOR_LIST_CHECK_FN,
    );
    let program: Vec<Word> = generate_code(&assemble_program(source)?);
    assert!(program.len() <= 256);
    let mut memory: Vec<Word> = Vec::with_capacity(256);

    let input = b"midnight{f1ddlin_wi_m4_bi75}";
    let sorted_input = {
        let mut v = input.to_vec();
        v.sort();
        v
    };
    let solution_xor: Vec<u8> = input
        .iter()
        .zip(sorted_input.iter())
        .map(|(a, b)| a ^ b)
        .collect();

    let start_list = 32 as u8;
    let end_list = start_list + input.len() as u8;

    memory.extend(&solution_xor);
    memory.resize(start_list.into(), 0);
    memory.extend(input);
    memory.resize(64, 0);
    memory.extend(&sorted_input);
    memory.resize(256, 0);

    let computer = LegComputer::new(program, memory).run();
    println!("{}", computer);

    assert_eq!(
        0,
        computer.memory[computer.registers.get(&RegisterRef::ST) as usize]
    );
    assert_eq!(
        input[..],
        computer.memory[start_list.into()..end_list.into()]
    );
    assert_eq!(
        sorted_input[..],
        computer.memory
            [(usize::from(start_list) + 32)..(usize::from(start_list) + 32 + input.len())]
    );
    assert_eq!(solution_xor[..], computer.memory[0..solution_xor.len()]);

    Ok(())
}

fn run_ctf(input: &[u8]) -> Result<LegComputer, String> {
    let source = &format!(
        "{}\n{}\n{}\n{}",
        CHALLENGE_PROG, COPY_LIST_FN, XOR_LIST_CHECK_FN, QUICKSORT_FN
    );
    let program: Vec<Word> = generate_code(&assemble_program(source)?);
    assert!(program.len() <= 256);
    let mut memory: Vec<Word> = Vec::with_capacity(256);

    let sorted_input = {
        let mut v = input.to_vec();
        v.sort();
        v
    };

    let correct_input = b"midnight{f1D)l3n_w/_M4_bi75~}";
    let sorted_correct_input = {
        let mut v = correct_input.to_vec();
        v.sort();
        v
    };
    let solution_xor: Vec<u8> = correct_input
        .iter()
        .zip(sorted_correct_input.iter())
        .map(|(a, b)| a ^ b)
        .collect();

    let start_ok = 8;
    let start_err = start_ok + 4;
    let start_solution = start_err + 4;
    let start_list = start_solution + solution_xor.len();
    let end_list = start_list + input.len();

    memory.resize(start_ok, 0);
    memory.extend(b"OK!");
    memory.resize(start_err, 0);
    memory.extend(b"ERR");

    memory[0] = start_list as u8;
    memory[1] = end_list as u8;
    memory[2] = start_solution as u8;
    memory[4] = start_ok as u8;
    memory[5] = (start_ok + 3) as u8;
    memory[6] = start_err as u8;
    memory[7] = (start_err + 3) as u8;

    memory.resize(start_solution, 0);
    memory.extend(&solution_xor);

    memory.resize(start_list.into(), 0);
    memory.extend(input);
    memory.resize(256, 0);

    let computer = LegComputer::new(program, memory).run();
    println!("{}", computer);

    assert_eq!(
        input[..],
        computer.memory[start_list.into()..end_list.into()]
    );
    assert_eq!(
        sorted_input[..],
        computer.memory
            [(usize::from(start_list) + input.len())..(usize::from(end_list) + input.len())]
    );
    assert_eq!(
        solution_xor[..],
        computer.memory[start_solution..(start_solution + solution_xor.len())]
    );

    Ok(computer)
}

#[test]
fn test_ctf_correct() -> Result<(), String> {
    let input = b"midnight{f1D)l3n_w/_M4_bi75~}";
    let computer = run_ctf(input)?;
    assert_eq!(b"OK!"[..], computer.memory[0..3]);

    Ok(())
}

#[test]
fn test_ctf_incorrect() -> Result<(), String> {
    let input = b"midnight{fiddlin_wi_ma_bits}";
    let computer = run_ctf(input)?;
    assert_eq!(b"ERR"[..], computer.memory[0..3]);

    Ok(())
}

#[test]
fn test_ctf_offset() -> Result<(), String> {
    let input: Vec<u8> = b"midnight{f1D)l3n_w/_M4_bi75~}"
        .iter()
        .map(|b| b + 1)
        .collect();
    let computer = run_ctf(&input)?;
    assert_eq!(b"ERR"[..], computer.memory[0..3]);

    Ok(())
}
