use leg_simulator::LegComputer;

#[test]
fn stack() -> Result<(), String> {
    let source = "
    MOVC 1 => A
    MOVC 2 => B
    MOVC 3 => C
    MOVC 4 => D

    PUSH A
    PUSH B
    PUSH C
    PUSH D

    PUSH ST
    PUSH BP

    PUSH A
    PUSH B
    PUSH C
    PUSH D

    HALT
    ";

    let computer: LegComputer = source.parse()?;
    let computer = computer.run();
    assert_eq!(computer.memory[255], 1); // A
    assert_eq!(computer.memory[254], 2); // B
    assert_eq!(computer.memory[253], 3); // C
    assert_eq!(computer.memory[252], 4); // D

    assert_eq!(computer.memory[251], 252); // ST
    assert_eq!(computer.memory[250], 0); // BP

    assert_eq!(computer.memory[249], 1); // A
    assert_eq!(computer.memory[248], 2); // B
    assert_eq!(computer.memory[247], 3); // C
    assert_eq!(computer.memory[246], 4); // D

    Ok(())
}
