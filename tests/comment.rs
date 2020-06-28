use leg_simulator::assemble_program;
use leg_simulator::generate_code;
use leg_simulator::Word;

#[test]
fn comment() -> Result<(), String> {
    let source = "
    # This is a comment
    HALT
    ";

    let program: Vec<Word> = generate_code(&assemble_program(source)?);
    assert_eq!(program, vec![0, 0]);

    Ok(())
}
