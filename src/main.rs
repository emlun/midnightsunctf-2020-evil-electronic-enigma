use leg_simulator::LegComputer;

fn main() -> Result<(), String> {
    let fibonacci = "
    MOVC 0 => A
    MOVC 100 => D
    STOREP A => D
    ALU INCR D D D
    MOVC 1 => B
    STOREP B => D

    LOADP D => B
    ALU DECR D D D
    LOADP D => A
    ALU ADD A B C

    JMPR Ou ? 14

    ALU INCR D D D
    ALU INCR D D D
    STOREP C => D

    MOVC 0 => A
    ALU ECHO A A A
    JMPR Z ? -20

    HALT
    ";

    let source = fibonacci;

    let computer: LegComputer = source.parse()?;
    let computer = computer.run();
    println!("{}\n", computer);

    Ok(())
}
