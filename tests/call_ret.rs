use leg_simulator::LegComputer;
use leg_simulator::RegisterRef;

#[test]
fn call() -> Result<(), String> {
    let source = "
    MOVC 1 => A
    MOVC 3 => B
    MOVC 5 => C
    MOVC 7 => D

    PUSH A
    PUSH B
    PUSH C
    PUSH D

    MOVC 50 => D
    CALL D

    HALT
    HALT
    HALT
    HALT
    HALT
    HALT
    HALT
    HALT
    HALT
    HALT
    HALT
    HALT
    HALT
    HALT
    HALT

    SLOAD 5 => A
    SLOAD 2 => B
    ALU ADD A B => C
    PUSH A
    PUSH B
    PUSH C
    PUSH D
    HALT
    ";

    let computer: LegComputer = source.parse()?;
    let computer = computer.run();

    assert_eq!(computer.read_register(&RegisterRef::ST), 246);
    assert_eq!(computer.read_register(&RegisterRef::BP), 250);
    assert_eq!(computer.eip, 64);

    assert_eq!(computer.memory[255], 1); // A
    assert_eq!(computer.memory[254], 3); // B
    assert_eq!(computer.memory[253], 5); // C
    assert_eq!(computer.memory[252], 7); // D

    assert_eq!(computer.memory[251], 18); // Stored instruction pointer
    assert_eq!(computer.memory[250], 0); // Stored stack frame base pointer
    assert_eq!(computer.memory[249], 1); // A := BP -5
    assert_eq!(computer.memory[248], 7); // B := BP -2
    assert_eq!(computer.memory[247], 8); // C := A + B
    assert_eq!(computer.memory[246], 50); // D

    assert_eq!(computer.memory[245], 0); // Outside stack

    Ok(())
}

#[test]
fn call_ret() -> Result<(), String> {
    let source = "
    MOVC 1 => A
    MOVC 3 => B
    MOVC 5 => C
    MOVC 7 => D

    PUSH A
    PUSH B
    PUSH C
    PUSH D

    CALLC 50

    NOP
    HALT
    HALT
    HALT
    HALT
    HALT
    HALT
    HALT
    HALT
    HALT
    HALT
    HALT
    HALT
    HALT
    HALT
    HALT

    SLOAD 5 => A
    SLOAD 2 => B
    ALU ADD A B => C
    PUSH A
    PUSH B
    PUSH C
    PUSH D
    RET C
    ";

    let computer: LegComputer = source.parse()?;
    let computer = computer.run();

    assert_eq!(computer.read_register(&RegisterRef::ST), 251);
    assert_eq!(computer.read_register(&RegisterRef::BP), 0);
    assert_eq!(computer.eip, 20);

    assert_eq!(computer.memory[255], 1);
    assert_eq!(computer.memory[254], 3);
    assert_eq!(computer.memory[253], 5);
    assert_eq!(computer.memory[252], 7);

    assert_eq!(computer.memory[251], 8);

    Ok(())
}
