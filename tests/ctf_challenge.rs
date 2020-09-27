use leg_simulator::assemble_program;
use leg_simulator::generate_code;
use leg_simulator::LegComputer;
use leg_simulator::Word;
use rand::Rng;

// Memory address 2, 3 contain start (inclusive), end (exclusive) of list
// List is copied to range immediately following it,
// then the copy is sorted in place,
// then TODO
const CHALLENGE_PROG: &str = "
JMP T ? 4
HALT

LOAD 0 => C
LOAD 1 => D
PUSH C
PUSH D
PUSH D
CALLR 52
POP C
POP D
POP D
POP D

ALU SUB C D => D
ALU ADD C D => D
ALU DECR D D => D
PUSH C
PUSH D
CALLR 56
POP A
POP D
POP C

LOAD 0 => B

LOAD 1 => D
ALU ECHO B D => B
JMPR LT ? 4
HALT

LOADP B => D
ALU XOR A D => A
LOADP C => D
ALU XOR A D => A
ALU INCR B B => B
ALU INCR C C => C

JMPR T ? -20
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
fn test_ctf() -> Result<(), String> {
    let source = &format!("{}\n{}\n{}", CHALLENGE_PROG, COPY_LIST_FN, QUICKSORT_FN);
    let program: Vec<Word> = generate_code(&assemble_program(source)?);
    let mut memory: Vec<Word> = Vec::with_capacity(256);

    let input = b"midnight{f1ddlin_wi_m4_bi75}";
    let sorted_input = {
        let mut v = input.to_vec();
        v.sort();
        v
    };
    let start_list = 8;
    let end_list = start_list + input.len() as u8;

    memory.resize(start_list.into(), 0);
    memory[0] = start_list;
    memory[1] = end_list;

    memory.append(&mut input.to_vec());
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
    assert!(false);

    Ok(())
}
