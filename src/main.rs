use leg_simulator::LegComputer;
use std::io::Read;

fn main() -> () {
    let mut input = std::io::stdin();
    let source = {
        let mut source = String::new();
        input
            .read_to_string(&mut source)
            .expect("Failed to read source code");
        source
    };

    let computer: LegComputer = source.parse().expect("Failed to parse source code");
    let computer = computer.run();
    println!("{}\n", computer);
}
