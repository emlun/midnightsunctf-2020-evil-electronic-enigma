use leg_simulator::LegComputer;

fn main() -> Result<(), String> {
    let test = "
        LOADC A 42
        LOADC B 43
        LOADC C 44
        LOADC D 45
    ";

    let fibonacci = "
LOADC A 0
LOADC D 100
STOREP A D
LOADC B 1
ALU ADD B D D
STOREP B D

LOADP B D
LOADC A 255
ALU ADD A D D
LOADP A D
ALU ADD A B C
LOADC A 2
ALU ADD A D D
STOREP C D

LOADC A 0
ALU ECHO A A A
JZ 12
    ";

    let source = fibonacci;

    let mut computer: LegComputer = source.parse()?;

    loop {
        println!("{}\n", computer);
        computer.step();
    }
}
