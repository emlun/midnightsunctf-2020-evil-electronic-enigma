use leg_simulator::LegComputer;
use leg_simulator::RegisterRef;

#[test]
fn nop() -> Result<(), String> {
    let source = "
    MOVC 1 => A
    NOP
    NOP
    NOP
    NOP
    NOP
    NOP
    NOP
    NOP
    NOP
    NOP
    NOP
    NOP
    NOP
    NOP
    NOP
    NOP
    NOP
    MOVC 2 => B
    HALT
    ";

    let computer: LegComputer = source.parse()?;
    let computer = computer.run();

    assert_eq!(computer.memory[40..], [0; 216][..]);
    assert_eq!(computer.read_register(&RegisterRef::A), 1);
    assert_eq!(computer.read_register(&RegisterRef::B), 2);
    assert_eq!(computer.eip, 38);

    Ok(())
}

#[test]
fn halt() -> Result<(), String> {
    let source = "
    HALT
    MOVC 1 => A
    ";

    let computer: LegComputer = source.parse()?;
    let computer = computer.run();

    assert_eq!(computer.memory[4..], [0; 252][..]);
    assert_eq!(computer.read_register(&RegisterRef::A), 0);
    assert_eq!(computer.eip, 0);

    Ok(())
}
