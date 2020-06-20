use leg_simulator::LegComputer;

#[test]
fn fibonacci() -> Result<(), String> {
    let source = "
    MOVC 0 => A
    MOVC 100 => D
    STOREP A => D
    ALU INCR D D => D
    MOVC 1 => B
    STOREP B => D

    LOADP D => B
    ALU DECR D D => D
    LOADP D => A
    ALU ADD A B => C

    JMPR Ou ? 14

    ALU INCR D D => D
    ALU INCR D D => D
    STOREP C => D

    MOVC 0 => A
    ALU ECHO A A => A
    JMPR Z ? -20

    HALT
    ";

    let computer: LegComputer = source.parse()?;
    let computer = computer.run();
    assert_eq!(
        computer.memory[100..=113],
        [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233]
    );

    Ok(())
}
