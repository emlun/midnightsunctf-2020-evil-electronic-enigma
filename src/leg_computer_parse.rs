use super::leg_computer::AluFlagRef;
use super::leg_computer::AluOpcode;
use super::leg_computer::Instruction;
use super::leg_computer::LegComputer;
use super::leg_computer::NopOpcode;
use super::leg_computer::RegisterRef;
use super::leg_computer::StackInstruction;
use super::leg_computer::Word;
use std::str::FromStr;

impl FromStr for RegisterRef {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            "FL" => Ok(Self::FL),
            "ST" => Ok(Self::ST),
            "BP" => Ok(Self::BP),
            "IP" => Ok(Self::IP),
            other => Err(format!("Invalid register: {}", other)),
        }
    }
}

impl FromStr for AluOpcode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ADD" => Ok(AluOpcode::Add),
            "ADDC" => Ok(AluOpcode::AddCarry),
            "INCR" => Ok(AluOpcode::Incr),
            "DECR" => Ok(AluOpcode::Decr),
            "XOR" => Ok(AluOpcode::Xor),
            "NEG" => Ok(AluOpcode::Neg),
            "SUB" => Ok(AluOpcode::Sub),
            "OR" => Ok(AluOpcode::Or),
            "AND" => Ok(AluOpcode::And),
            "NAND" => Ok(AluOpcode::Nand),
            "NOR" => Ok(AluOpcode::Nor),
            "SHIFTL" => Ok(AluOpcode::ShiftL),
            "SHIFTR" => Ok(AluOpcode::ShiftR),
            "ECHO" => Ok(AluOpcode::Echo),
            other => Err(format!("Invalid ALU operation: {}", other)),
        }
    }
}

impl FromStr for AluFlagRef {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Z" => Ok(AluFlagRef::EqZero),
            "Ou" => Ok(AluFlagRef::OverflowUnsigned),
            "Os" => Ok(AluFlagRef::OverflowSigned),
            "EQ" => Ok(AluFlagRef::Equal),
            "GT" => Ok(AluFlagRef::GreaterThan),
            "GTs" => Ok(AluFlagRef::GreaterThanSigned),
            "GE" => Ok(AluFlagRef::GreaterOrEqual),
            "GEs" => Ok(AluFlagRef::GreaterOrEqualSigned),

            "NE" => Ok(AluFlagRef::NotEqual),
            "LT" => Ok(AluFlagRef::LessThan),
            "LTs" => Ok(AluFlagRef::LessThanSigned),
            "LE" => Ok(AluFlagRef::LessOrEqual),
            "LEs" => Ok(AluFlagRef::LessOrEqualSigned),
            "F" => Ok(AluFlagRef::False),
            "T" => Ok(AluFlagRef::True),
            other => Err(format!("Invalid flag: {}", other)),
        }
    }
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(line: &str) -> Result<Instruction, Self::Err> {
        let line_words: Vec<&str> = line.split(" ").collect();

        fn parse_word(s: &str) -> Result<Word, String> {
            let w: i16 = s.parse().map_err(|_| format!("Invalid word: {}", s))?;
            Ok(((w + 256) & 0xff) as Word)
        }

        match &line_words[..] {
            ["LOAD", addr, "=>", dest] => Ok(Self::Load {
                addr: parse_word(addr)?,
                dest: dest.parse()?,
            }),
            ["LOADP", addr_src, "=>", dest] => Ok(Self::LoadP {
                addr_src: addr_src.parse()?,
                dest: dest.parse()?,
            }),

            ["STORE", src, "=>", addr] => Ok(Self::Store {
                src: src.parse()?,
                addr: parse_word(addr)?,
            }),
            ["STOREP", src, "=>", addr_src] => Ok(Self::StoreP {
                src: src.parse()?,
                addr_src: addr_src.parse()?,
            }),

            ["MOV", src, "=>", dest] => Ok(Self::Mov {
                src: src.parse()?,
                dest: dest.parse()?,
            }),
            ["MOVC", val, "=>", dest] => Ok(Self::MovC {
                val: parse_word(val)?,
                dest: dest.parse()?,
            }),

            ["JMP", flag, "?", addr] => Ok(Self::Jmp {
                flag: flag.parse()?,
                addr: parse_word(addr)?,
            }),
            ["JMPP", flag, "?", addr_src] => Ok(Self::JmpP {
                flag: flag.parse()?,
                addr_src: addr_src.parse()?,
            }),
            ["JMPR", flag, "?", diff] => Ok(Self::JmpR {
                flag: flag.parse()?,
                diff: parse_word(diff)?,
            }),
            ["JMPRP", flag, "?", diff_src] => Ok(Self::JmpRP {
                flag: flag.parse()?,
                diff_src: diff_src.parse()?,
            }),

            ["PUSH", src] => Ok(Self::Stack(StackInstruction::Push { src: src.parse()? })),
            ["POP", dest] => Ok(Self::Stack(StackInstruction::Pop {
                dest: dest.parse()?,
            })),
            ["CALL", addr_reg] => Ok(Self::Stack(StackInstruction::Call {
                addr_reg: addr_reg.parse()?,
            })),
            ["CALLC", addr] => Ok(Self::Stack(StackInstruction::CallC {
                addr: parse_word(addr)?,
            })),
            ["CALLR", diff] => Ok(Self::Stack(StackInstruction::CallR {
                diff: parse_word(diff)?,
            })),
            ["RET", src] => Ok(Self::Stack(StackInstruction::Ret { src: src.parse()? })),
            ["SLOAD", bp_diff, "=>", dest] => Ok(Self::Stack(StackInstruction::Load {
                dest: dest.parse()?,
                bp_diff: parse_word(bp_diff)?,
            })),

            ["GPI", dest, "<="] => Ok(Self::Gpi {
                dest: dest.parse()?,
            }),
            ["GPO", src, "=>"] => Ok(Self::Gpo { src: src.parse()? }),

            ["ALU", op, arg1, arg2, "=>", out] => Ok(Self::Alu {
                op: op.parse()?,
                arg1: arg1.parse()?,
                arg2: arg2.parse()?,
                out: out.parse()?,
            }),

            ["NOP"] => Ok(Self::Nop(NopOpcode::Nop)),
            ["HALT"] => Ok(Self::Nop(NopOpcode::Halt)),

            other => Err(format!("Invalid instruction: {:?}", other)),
        }
    }
}

impl FromStr for LegComputer {
    type Err = String;
    fn from_str(source: &str) -> Result<LegComputer, Self::Err> {
        let program = generate_code(&assemble_program(source)?);
        Ok(LegComputer::new(program, vec![0; 256]))
    }
}

pub fn assemble_program(source: &str) -> Result<Vec<Instruction>, String> {
    source
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .filter(|s| !s.starts_with("#"))
        .map(|s| s.parse())
        .collect()
}

pub fn generate_code(program: &[Instruction]) -> Vec<Word> {
    let mut result = Vec::with_capacity(program.len());
    for ins in program {
        let (word1, word2): (Word, Word) = ins.into();
        result.push(word1);
        result.push(word2);
    }
    result
}
