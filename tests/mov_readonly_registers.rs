use leg_simulator::LegComputer;
use leg_simulator::RegisterRef;

#[test]
fn mov_readonly_registers() -> Result<(), String> {
    let source = "
    MOVC 2 => A
    MOVC 1 => B
    ALU ADD A B => B
    MOV IP => C
    MOV FL => D
    HALT
    ";

    let computer: LegComputer = source.parse()?;
    let computer = computer.run();
    assert_eq!(computer.registers.get(&RegisterRef::A), 2);
    assert_eq!(computer.registers.get(&RegisterRef::B), 3);
    assert_eq!(computer.registers.get(&RegisterRef::C), 6);
    assert_eq!(computer.registers.get(&RegisterRef::D), 0b11110000);

    Ok(())
}
