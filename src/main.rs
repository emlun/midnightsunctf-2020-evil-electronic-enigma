use leg_simulator::LegComputer;

fn main() -> Result<(), String> {
    let fibonacci = "
MOVC 0 INTO A
MOVC 100 INTO D
STOREP A AT D
ALU INCR D D D
MOVC 1 INTO B
STOREP B AT D

LOADP D TO B
ALU DECR D D D
LOADP D TO A
ALU ADD A B C

JMPR IF Ou BY 14

ALU INCR D D D
ALU INCR D D D
STOREP C AT D

MOVC 0 INTO A
ALU ECHO A A A
JMPR IF Z BY -20

HALT
    ";

    let source = fibonacci;

    let mut computer: LegComputer = source.parse()?;

    while !computer.is_halted() {
        println!("{}\n", computer);
        computer.step();
    }
    Ok(())
}
