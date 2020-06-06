use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::str::FromStr;

pub type Word = u8;
pub type Memory = Vec<Word>;
pub type Address = Word;
pub type Value = Word;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Opcode {
    Load = 0x0,
    LoadP = 0x1,

    Store = 0x2,
    StoreP = 0x3,

    Mov = 0x4,
    MovC = 0x5,

    Jmp = 0x6,
    JmpP = 0x7,
    JmpR = 0x8,
    JmpRP = 0x9,

    ShiftL = 0xA,
    ShiftR = 0xB,

    Gpi = 0xC,
    Gpo = 0xD,

    Alu = 0xE,

    Halt = 0xF,
}

impl TryFrom<Word> for Opcode {
    type Error = String;
    fn try_from(w: Word) -> Result<Self, Self::Error> {
        match w {
            0x0 => Ok(Self::Load),
            0x1 => Ok(Self::LoadP),

            0x2 => Ok(Self::Store),
            0x3 => Ok(Self::StoreP),

            0x4 => Ok(Self::Mov),
            0x5 => Ok(Self::MovC),

            0x6 => Ok(Self::Jmp),
            0x7 => Ok(Self::JmpP),
            0x8 => Ok(Self::JmpR),
            0x9 => Ok(Self::JmpRP),

            0xA => Ok(Self::ShiftL),
            0xB => Ok(Self::ShiftR),

            0xC => Ok(Self::Gpi),
            0xD => Ok(Self::Gpo),

            0xE => Ok(Self::Alu),

            0xF => Ok(Self::Halt),

            other => Err(format!("Invalid opcode: {}", other)),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum RegisterRef {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

impl TryFrom<Word> for RegisterRef {
    type Error = String;
    fn try_from(w: Word) -> Result<Self, Self::Error> {
        match w {
            0 => Ok(Self::A),
            1 => Ok(Self::B),
            2 => Ok(Self::C),
            3 => Ok(Self::D),
            other => Err(format!("Invalid register: {}", other)),
        }
    }
}

impl FromStr for RegisterRef {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            other => Err(format!("Invalid register: {}", other)),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AluOpcode {
    Add = 0b0000,
    AddCarry = 0b0001,
    Incr = 0b0010,
    Decr = 0b0011,
    Xor = 0b0100,
    Or = 0b1000,
    And = 0b1001,
    Nand = 0b1010,
    Nor = 0b1011,
    Echo = 0b1100,
}

impl TryFrom<Word> for AluOpcode {
    type Error = String;
    fn try_from(w: Word) -> Result<Self, Self::Error> {
        match w {
            0b0000 => Ok(Self::Add),
            0b0001 => Ok(Self::AddCarry),
            0b0010 => Ok(Self::Incr),
            0b0011 => Ok(Self::Decr),
            0b0100 => Ok(Self::Xor),
            0b1000 => Ok(Self::Or),
            0b1001 => Ok(Self::And),
            0b1010 => Ok(Self::Nand),
            0b1011 => Ok(Self::Nor),
            0b1100 => Ok(Self::Echo),
            other => Err(format!("Invalid ALU opcode: {}", other)),
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
            "OR" => Ok(AluOpcode::Or),
            "AND" => Ok(AluOpcode::And),
            "NAND" => Ok(AluOpcode::Nand),
            "NOR" => Ok(AluOpcode::Nor),
            "ECHO" => Ok(AluOpcode::Echo),
            other => Err(format!("Invalid ALU operation: {}", other)),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum AluFlagRef {
    Zero = 0,
    OverflowUnsigned = 1,
    OverflowSigned = 2,
}

impl TryFrom<Word> for AluFlagRef {
    type Error = String;
    fn try_from(w: Word) -> Result<Self, Self::Error> {
        match w {
            0 => Ok(Self::Zero),
            1 => Ok(Self::OverflowUnsigned),
            2 => Ok(Self::OverflowSigned),
            other => Err(format!("Invalid flag: {}", other)),
        }
    }
}

impl FromStr for AluFlagRef {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Z" => Ok(AluFlagRef::Zero),
            "Ou" => Ok(AluFlagRef::OverflowUnsigned),
            "Os" => Ok(AluFlagRef::OverflowSigned),
            other => Err(format!("Invalid flag: {}", other)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AluFlags {
    pub eq_zero: bool,
    pub overflow_unsigned: bool,
    pub overflow_signed: bool,
}

impl AluFlags {
    fn new() -> AluFlags {
        AluFlags {
            eq_zero: false,
            overflow_unsigned: false,
            overflow_signed: false,
        }
    }

    fn get(&self, flag: &AluFlagRef) -> bool {
        match *flag {
            AluFlagRef::Zero => self.eq_zero,
            AluFlagRef::OverflowUnsigned => self.overflow_unsigned,
            AluFlagRef::OverflowSigned => self.overflow_signed,
        }
    }
}

impl Display for AluFlags {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "[{}]",
            vec![
                ("Z", self.eq_zero),
                ("Ou", self.overflow_unsigned),
                ("Os", self.overflow_signed),
            ]
            .into_iter()
            .filter(|(_, b)| *b)
            .map(|(s, _)| s)
            .collect::<Vec<&str>>()
            .join(", "),
        )
    }
}

#[derive(Clone, Debug)]
pub struct Registers {
    values: HashMap<RegisterRef, Word>,
}

impl Registers {
    fn new() -> Registers {
        Registers {
            values: HashMap::new(),
        }
    }

    fn get(&self, reg: &RegisterRef) -> Word {
        *self.values.get(reg).unwrap_or(&0)
    }

    fn get_mut(&mut self, reg: RegisterRef) -> &mut Word {
        self.values.entry(reg).or_insert(0)
    }
}

impl Display for Registers {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "[A: {}, B: {}, C: {}, D: {}]",
            self.get(&RegisterRef::A),
            self.get(&RegisterRef::B),
            self.get(&RegisterRef::C),
            self.get(&RegisterRef::D),
        )
    }
}

#[derive(Eq, PartialEq)]
pub enum Instruction {
    Load {
        dest: RegisterRef,
        addr: Address,
    },
    LoadP {
        dest: RegisterRef,
        addr_src: RegisterRef,
    },

    Store {
        src: RegisterRef,
        addr: Address,
    },
    StoreP {
        src: RegisterRef,
        addr_src: RegisterRef,
    },

    Mov {
        dest: RegisterRef,
        src: RegisterRef,
    },
    MovC {
        dest: RegisterRef,
        val: Value,
    },

    Jmp {
        flag: AluFlagRef,
        addr: Address,
    },
    JmpP {
        flag: AluFlagRef,
        addr_src: RegisterRef,
    },
    JmpR {
        flag: AluFlagRef,
        diff: Value,
    },
    JmpRP {
        flag: AluFlagRef,
        diff_src: RegisterRef,
    },

    ShiftL {
        src: RegisterRef,
        dest: RegisterRef,
        amount: Value,
    },
    ShiftR {
        src: RegisterRef,
        dest: RegisterRef,
        amount: Value,
    },

    Gpi {
        dest: RegisterRef,
    },
    Gpo {
        src: RegisterRef,
    },

    Alu {
        op: AluOpcode,
        arg1: RegisterRef,
        arg2: RegisterRef,
        out: RegisterRef,
    },

    Halt,
}

impl TryFrom<(Word, Word)> for Instruction {
    type Error = String;
    fn try_from((word1, word2): (Word, Word)) -> Result<Instruction, String> {
        let opcode = Opcode::try_from(word1 >> 4)?;

        Ok(match opcode {
            Opcode::Load => Self::Load {
                dest: (word1 & 0x3).try_into()?,
                addr: word2,
            },
            Opcode::LoadP => Self::LoadP {
                dest: (word1 & 0x3).try_into()?,
                addr_src: (word2 & 0x3).try_into()?,
            },

            Opcode::Store => Self::Store {
                src: (word1 & 0x3).try_into()?,
                addr: word2,
            },
            Opcode::StoreP => Self::StoreP {
                src: (word1 & 0x3).try_into()?,
                addr_src: (word2 & 0x3).try_into()?,
            },

            Opcode::Mov => Self::Mov {
                src: (word1 & 0x3).try_into()?,
                dest: word2.try_into()?,
            },
            Opcode::MovC => Self::MovC {
                dest: (word1 & 0x3).try_into()?,
                val: word2,
            },

            Opcode::Jmp => Self::Jmp {
                flag: (word1 & 0x3).try_into()?,
                addr: word2,
            },
            Opcode::JmpP => Self::JmpP {
                flag: (word1 & 0x3).try_into()?,
                addr_src: (word2 & 0x3).try_into()?,
            },
            Opcode::JmpR => Self::JmpR {
                flag: (word1 & 0x3).try_into()?,
                diff: word2,
            },
            Opcode::JmpRP => Self::JmpRP {
                flag: (word1 & 0x3).try_into()?,
                diff_src: (word2 & 0x3).try_into()?,
            },

            Opcode::ShiftL => Self::ShiftL {
                src: ((word1 >> 2) & 0x3).try_into()?,
                dest: (word1 & 0x3).try_into()?,
                amount: word2,
            },
            Opcode::ShiftR => Self::ShiftR {
                src: ((word1 >> 2) & 0x3).try_into()?,
                dest: (word1 & 0x3).try_into()?,
                amount: word2,
            },

            Opcode::Gpi => Self::Gpi {
                dest: (word2 & 0x3).try_into()?,
            },
            Opcode::Gpo => Self::Gpo {
                src: (word2 & 0x3).try_into()?,
            },

            Opcode::Alu => Self::Alu {
                op: (word1 & 0xf).try_into()?,
                arg1: (word2 >> 6).try_into()?,
                arg2: ((word2 >> 4) & 0x3).try_into()?,
                out: (word2 & 0x3).try_into()?,
            },

            Opcode::Halt => Self::Halt,
        })
    }
}

impl Into<(Word, Word)> for &Instruction {
    fn into(self) -> (Word, Word) {
        fn pack(opcode: Opcode, word1_tail: &RegisterRef, word2: Word) -> (Word, Word) {
            (((opcode as u8) << 4) | (*word1_tail as u8), word2)
        }

        fn packf(opcode: Opcode, word1_tail: &AluFlagRef, word2: Word) -> (Word, Word) {
            (((opcode as u8) << 4) | (*word1_tail as u8), word2)
        }

        fn cat(opcode: Opcode, word2: Word) -> (Word, Word) {
            (((opcode as u8) << 4), word2)
        }

        match self {
            Instruction::Load { dest, addr } => pack(Opcode::Load, dest, *addr),
            Instruction::LoadP { dest, addr_src } => pack(Opcode::LoadP, dest, *addr_src as u8),

            Instruction::Store { src, addr } => pack(Opcode::Store, src, *addr),
            Instruction::StoreP { src, addr_src } => pack(Opcode::StoreP, src, *addr_src as u8),

            Instruction::Mov { dest, src } => pack(Opcode::Mov, dest, *src as u8),
            Instruction::MovC { dest, val } => pack(Opcode::MovC, dest, *val),

            Instruction::Jmp { flag, addr } => packf(Opcode::Jmp, flag, *addr),
            Instruction::JmpP { flag, addr_src } => packf(Opcode::JmpP, flag, *addr_src as u8),
            Instruction::JmpR { flag, diff } => packf(Opcode::JmpR, flag, *diff),
            Instruction::JmpRP { flag, diff_src } => packf(Opcode::JmpRP, flag, *diff_src as u8),

            Instruction::ShiftL { src, dest, amount } => (
                ((Opcode::ShiftL as u8) << 4) | ((*src as u8) << 2) | (*dest as u8),
                *amount,
            ),
            Instruction::ShiftR { src, dest, amount } => (
                ((Opcode::ShiftL as u8) << 4) | ((*src as u8) << 2) | (*dest as u8),
                *amount,
            ),

            Instruction::Gpi { dest } => cat(Opcode::Gpi, *dest as u8),
            Instruction::Gpo { src } => cat(Opcode::Gpo, *src as u8),

            Instruction::Alu {
                op,
                arg1,
                arg2,
                out,
            } => (
                ((Opcode::Alu as u8) << 4) | (*op as u8),
                ((*arg1 as u8) << 6) | ((*arg2 as u8) << 6) | (*out as u8),
            ),

            Instruction::Halt => cat(Opcode::Halt, 0),
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

        Ok(match &line_words[..] {
            ["LOAD", addr, "=>", dest] => Self::Load {
                addr: parse_word(addr)?,
                dest: dest.parse()?,
            },
            ["LOADP", addr_src, "=>", dest] => Self::LoadP {
                addr_src: addr_src.parse()?,
                dest: dest.parse()?,
            },

            ["STORE", src, "=>", addr] => Self::Store {
                src: src.parse()?,
                addr: parse_word(addr)?,
            },
            ["STOREP", src, "=>", addr_src] => Self::StoreP {
                src: src.parse()?,
                addr_src: addr_src.parse()?,
            },

            ["MOV", src, "=>", dest] => Self::Mov {
                src: src.parse()?,
                dest: dest.parse()?,
            },
            ["MOVC", val, "=>", dest] => Self::MovC {
                val: parse_word(val)?,
                dest: dest.parse()?,
            },

            ["JMP", flag, "?", addr] => Self::Jmp {
                flag: flag.parse()?,
                addr: parse_word(addr)?,
            },
            ["JMPP", flag, "?", addr_src] => Self::JmpP {
                flag: flag.parse()?,
                addr_src: addr_src.parse()?,
            },
            ["JMPR", flag, "?", diff] => Self::JmpR {
                flag: flag.parse()?,
                diff: parse_word(diff)?,
            },
            ["JMPRP", flag, "?", diff_src] => Self::JmpRP {
                flag: flag.parse()?,
                diff_src: diff_src.parse()?,
            },

            ["SHIFTL", amount, "OF", src, "=>", dest] => Self::ShiftL {
                amount: parse_word(amount)?,
                src: src.parse()?,
                dest: dest.parse()?,
            },
            ["SHIFTR", amount, "OF", src, "=>", dest] => Self::ShiftR {
                amount: parse_word(amount)?,
                src: src.parse()?,
                dest: dest.parse()?,
            },

            ["GPI", dest, "<="] => Self::Gpi {
                dest: dest.parse()?,
            },
            ["GPO", src, "=>"] => Self::Gpo { src: src.parse()? },

            ["ALU", op, arg1, arg2, out] => Self::Alu {
                op: op.parse()?,
                arg1: arg1.parse()?,
                arg2: arg2.parse()?,
                out: out.parse()?,
            },

            ["HALT"] => Self::Halt,

            other => panic!("Invalid instruction: {:?}", other),
        })
    }
}

#[derive(Clone, Debug)]
pub struct LegComputer {
    pub eip: Word,
    pub prog: Memory,
    pub flags: AluFlags,
    pub registers: Registers,
    pub reg_i: Word,
    pub reg_o: Word,
}

impl Display for LegComputer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "{eip:03} {regs} {flags} [{reg_i} {reg_o}]\n{prog:?}",
            eip = self.eip,
            regs = self.registers,
            flags = self.flags,
            reg_i = self.reg_i,
            reg_o = self.reg_o,
            prog = self.prog,
        )
    }
}

impl LegComputer {
    pub fn new(program: Vec<Word>) -> LegComputer {
        LegComputer {
            eip: 0,
            prog: program,
            flags: AluFlags::new(),
            registers: Registers::new(),
            reg_i: 0,
            reg_o: 0,
        }
    }

    pub fn is_halted(&self) -> bool {
        let instruction = Instruction::try_from((
            self.prog[self.eip as usize],
            self.prog[self.eip as usize + 1],
        ))
        .unwrap();
        instruction == Instruction::Halt
    }

    pub fn run(mut self) -> Self {
        while !self.is_halted() {
            self.step();
        }
        self
    }

    pub fn step(&mut self) -> () {
        let instruction = Instruction::try_from((
            self.prog[self.eip as usize],
            self.prog[self.eip as usize + 1],
        ))
        .unwrap();

        match instruction {
            Instruction::Load { dest, addr } => {
                *self.registers.get_mut(dest) = self.prog[addr as usize];
                self.eip += 2;
            }
            Instruction::LoadP { dest, addr_src } => {
                *self.registers.get_mut(dest) = self.prog[self.registers.get(&addr_src) as usize];
                self.eip += 2;
            }

            Instruction::Store { src, addr } => {
                self.prog[addr as usize] = self.registers.get(&src);
                self.eip += 2;
            }
            Instruction::StoreP { src, addr_src } => {
                self.prog[self.registers.get(&addr_src) as usize] = self.registers.get(&src);
                self.eip += 2;
            }

            Instruction::Mov { src, dest } => {
                *self.registers.get_mut(dest) = self.registers.get(&src);
                self.eip += 2;
            }
            Instruction::MovC { dest, val } => {
                *self.registers.get_mut(dest) = val;
                self.eip += 2;
            }

            Instruction::Jmp { flag, addr } => {
                if self.flags.get(&flag) {
                    self.eip = addr;
                } else {
                    self.eip += 2;
                }
            }
            Instruction::JmpP { flag, addr_src } => {
                if self.flags.get(&flag) {
                    self.eip = self.prog[self.registers.get(&addr_src) as usize];
                } else {
                    self.eip += 2;
                }
            }
            Instruction::JmpR { flag, diff } => {
                if self.flags.get(&flag) {
                    self.eip = (self.eip as i16 + diff as i16) as u8;
                } else {
                    self.eip += 2;
                }
            }
            Instruction::JmpRP { flag, diff_src } => {
                if self.flags.get(&flag) {
                    self.eip = (self.eip as i16
                        + self.prog[self.registers.get(&diff_src) as usize] as i16)
                        as u8;
                } else {
                    self.eip += 2;
                }
            }

            Instruction::ShiftL { src, dest, amount } => {
                *self.registers.get_mut(dest) = self.registers.get(&src) << amount;
                self.eip += 2;
            }
            Instruction::ShiftR { src, dest, amount } => {
                *self.registers.get_mut(dest) = self.registers.get(&src) >> amount;
                self.eip += 2;
            }

            Instruction::Gpi { dest } => {
                *self.registers.get_mut(dest) = self.reg_i;
                self.eip += 2;
            }
            Instruction::Gpo { src } => {
                self.reg_o = self.registers.get(&src);
                self.eip += 2;
            }

            Instruction::Alu {
                op,
                arg1,
                arg2,
                out,
            } => {
                match op {
                    AluOpcode::Add => {
                        let arg1_unsigned: u16 = self.registers.get(&arg1) as u16;
                        let arg2_unsigned: u16 = self.registers.get(&arg2) as u16;
                        let result_unsigned: u16 = arg1_unsigned + arg2_unsigned;

                        let arg1_signed: i16 = if arg1_unsigned > 0x7f {
                            (arg1_unsigned as i16) - 256
                        } else {
                            arg1_unsigned as i16
                        };
                        let arg2_signed: i16 = if arg2_unsigned > 0x7f {
                            (arg2_unsigned as i16) - 256
                        } else {
                            arg2_unsigned as i16
                        };
                        let result_signed: i16 = arg1_signed + arg2_signed;

                        *self.registers.get_mut(out) = (result_unsigned & 0xff) as u8;
                        if result_unsigned > 0xff {
                            self.flags.overflow_unsigned = true;
                        }
                        if result_signed < -128 || result_signed > 127 {
                            self.flags.overflow_signed = true;
                        }
                    }

                    AluOpcode::AddCarry => {
                        let arg1_unsigned: u16 = self.registers.get(&arg1) as u16;
                        let arg2_unsigned: u16 = self.registers.get(&arg2) as u16;
                        let result_unsigned: u16 = arg1_unsigned + arg2_unsigned + 1;

                        let arg1_signed: i16 = if arg1_unsigned > 0x7f {
                            (arg1_unsigned as i16) - 256
                        } else {
                            arg1_unsigned as i16
                        };
                        let arg2_signed: i16 = if arg2_unsigned > 0x7f {
                            (arg2_unsigned as i16) - 256
                        } else {
                            arg2_unsigned as i16
                        };
                        let result_signed: i16 = arg1_signed + arg2_signed + 1;

                        *self.registers.get_mut(out) = (result_unsigned & 0xff) as u8;
                        if result_unsigned > 0xff {
                            self.flags.overflow_unsigned = true;
                        }
                        if result_signed < -128 || result_signed > 127 {
                            self.flags.overflow_signed = true;
                        }
                    }

                    AluOpcode::Incr => {
                        let arg1: u16 = self.registers.get(&arg1) as u16;
                        let result: u16 = (arg1 + 1) & 0xff;

                        *self.registers.get_mut(out) = result as u8;
                        self.flags.overflow_unsigned = result == 0x00;
                        self.flags.overflow_signed = result == 0x80;
                    }

                    AluOpcode::Decr => {
                        let arg1: u16 = self.registers.get(&arg1) as u16;
                        let result: u16 = (arg1 + 256 - 1) & 0xff;

                        *self.registers.get_mut(out) = result as u8;
                        self.flags.overflow_unsigned = result == 0xff;
                        self.flags.overflow_signed = result == 0x7f;
                    }

                    AluOpcode::Xor => {
                        *self.registers.get_mut(out) =
                            self.registers.get(&arg1) ^ self.registers.get(&arg2);
                    }
                    AluOpcode::Or => {
                        *self.registers.get_mut(out) =
                            self.registers.get(&arg1) | self.registers.get(&arg2);
                    }
                    AluOpcode::And => {
                        *self.registers.get_mut(out) =
                            self.registers.get(&arg1) & self.registers.get(&arg2);
                    }
                    AluOpcode::Nand => {
                        *self.registers.get_mut(out) =
                            !(self.registers.get(&arg1) & self.registers.get(&arg2));
                    }
                    AluOpcode::Nor => {
                        *self.registers.get_mut(out) =
                            !(self.registers.get(&arg1) | self.registers.get(&arg2));
                    }

                    AluOpcode::Echo => {
                        *self.registers.get_mut(out) = self.registers.get(&arg1);
                    }
                };
                self.flags.eq_zero = self.registers.get(&out) == 0;
                self.eip += 2;
            }

            Instruction::Halt => {}
        };
    }
}

impl FromStr for LegComputer {
    type Err = String;
    fn from_str(source: &str) -> Result<LegComputer, Self::Err> {
        let mut program = generate_code(&assemble_program(&source.lines().collect::<Vec<&str>>())?);

        let mut memory: Memory = Vec::with_capacity(256);
        memory.append(&mut program);
        while memory.len() < 256 {
            memory.push(0);
        }

        Ok(LegComputer::new(memory))
    }
}

pub fn assemble_program(lines: &[&str]) -> Result<Vec<Instruction>, String> {
    lines
        .iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
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
