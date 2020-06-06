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
    LoadC = 0x2,

    Store = 0x3,
    StoreP = 0x4,

    Jz = 0x5,
    JzP = 0x6,
    Jos = 0x7,
    JosP = 0x8,
    Jus = 0x9,
    JusP = 0xA,

    ShiftL = 0xB,
    ShiftR = 0xC,
    Gpi = 0xD,
    Gpo = 0xE,
    Alu = 0xF,
}

impl TryFrom<Word> for Opcode {
    type Error = String;
    fn try_from(w: Word) -> Result<Self, Self::Error> {
        match w {
            0x0 => Ok(Self::Load),
            0x1 => Ok(Self::LoadP),
            0x2 => Ok(Self::LoadC),

            0x3 => Ok(Self::Store),
            0x4 => Ok(Self::StoreP),

            0x5 => Ok(Self::Jz),
            0x6 => Ok(Self::JzP),
            0x7 => Ok(Self::Jos),
            0x8 => Ok(Self::JosP),
            0x9 => Ok(Self::Jus),
            0xA => Ok(Self::JusP),

            0xB => Ok(Self::ShiftL),
            0xC => Ok(Self::ShiftR),
            0xD => Ok(Self::Gpi),
            0xE => Ok(Self::Gpo),
            0xF => Ok(Self::Alu),

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
#[derive(Clone, Copy, Debug)]
pub enum AluOpcode {
    Add = 0b0000,
    AddCarry = 0b0001,
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

#[derive(Clone, Debug)]
pub struct AluFlags {
    pub eq_zero: bool,
    pub overflow_signed: bool,
    pub overflow_unsigned: bool,
}

impl AluFlags {
    fn new() -> AluFlags {
        AluFlags {
            eq_zero: false,
            overflow_signed: false,
            overflow_unsigned: false,
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
                ("Os", self.overflow_signed),
                ("Ou", self.overflow_unsigned),
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

pub enum Instruction {
    Load {
        reg: RegisterRef,
        addr: Address,
    },
    LoadP {
        reg: RegisterRef,
        addr_reg: RegisterRef,
    },
    LoadC {
        reg: RegisterRef,
        val: Value,
    },

    Store {
        reg: RegisterRef,
        addr: Address,
    },
    StoreP {
        reg: RegisterRef,
        addr_reg: RegisterRef,
    },

    Jz {
        addr: Address,
    },
    JzP {
        addr_reg: RegisterRef,
    },
    Jos {
        addr: Address,
    },
    JosP {
        addr_reg: RegisterRef,
    },
    Jus {
        addr: Address,
    },
    JusP {
        addr_reg: RegisterRef,
    },

    ShiftL {
        reg_in: RegisterRef,
        reg_out: RegisterRef,
    },
    ShiftR {
        reg_in: RegisterRef,
        reg_out: RegisterRef,
    },
    Gpi {
        reg: RegisterRef,
    },
    Gpo {
        reg: RegisterRef,
    },
    Alu {
        op: AluOpcode,
        arg1: RegisterRef,
        arg2: RegisterRef,
        out: RegisterRef,
    },
}

impl TryFrom<(Word, Word)> for Instruction {
    type Error = String;
    fn try_from((word1, word2): (Word, Word)) -> Result<Instruction, String> {
        let opcode = Opcode::try_from(word1 >> 4)?;

        Ok(match opcode {
            Opcode::Load => Self::Load {
                reg: (word1 & 0x3).try_into()?,
                addr: word2,
            },
            Opcode::LoadP => Self::LoadP {
                reg: (word1 & 0x3).try_into()?,
                addr_reg: (word2 & 0x3).try_into()?,
            },
            Opcode::LoadC => Self::LoadC {
                reg: (word1 & 0x3).try_into()?,
                val: word2,
            },

            Opcode::Store => Self::Store {
                reg: (word1 & 0x3).try_into()?,
                addr: word2,
            },
            Opcode::StoreP => Self::StoreP {
                reg: (word1 & 0x3).try_into()?,
                addr_reg: (word2 & 0x3).try_into()?,
            },

            Opcode::Jz => Self::Jz { addr: word2 },
            Opcode::JzP => Self::JzP {
                addr_reg: (word2 & 0x3).try_into()?,
            },
            Opcode::Jos => Self::Jos { addr: word2 },
            Opcode::JosP => Self::JosP {
                addr_reg: (word2 & 0x3).try_into()?,
            },
            Opcode::Jus => Self::Jus { addr: word2 },
            Opcode::JusP => Self::JusP {
                addr_reg: (word2 & 0x3).try_into()?,
            },

            Opcode::ShiftL => Self::ShiftL {
                reg_in: ((word2 >> 4) & 0x3).try_into()?,
                reg_out: (word2 & 0x3).try_into()?,
            },
            Opcode::ShiftR => Self::ShiftR {
                reg_in: ((word2 >> 4) & 0x3).try_into()?,
                reg_out: (word2 & 0x3).try_into()?,
            },
            Opcode::Gpi => Self::Gpi {
                reg: (word2 & 0x3).try_into()?,
            },
            Opcode::Gpo => Self::Gpo {
                reg: (word2 & 0x3).try_into()?,
            },
            Opcode::Alu => Self::Alu {
                op: (word1 & 0xf).try_into()?,
                arg1: (word2 >> 6).try_into()?,
                arg2: ((word2 >> 4) & 0x3).try_into()?,
                out: (word2 & 0x3).try_into()?,
            },
        })
    }
}

impl Into<(Word, Word)> for &Instruction {
    fn into(self) -> (Word, Word) {
        fn pack(opcode: Opcode, word1_tail: &RegisterRef, word2: Word) -> (Word, Word) {
            (((opcode as u8) << 4) | (*word1_tail as u8), word2)
        }

        fn pack2(
            opcode: Opcode,
            word2_front: &RegisterRef,
            word2_tail: &RegisterRef,
        ) -> (Word, Word) {
            (
                ((opcode as u8) << 4),
                ((*word2_front as u8) << 4) | (*word2_tail as u8),
            )
        }

        fn cat(opcode: Opcode, word2: Word) -> (Word, Word) {
            (((opcode as u8) << 4), word2)
        }

        match self {
            Instruction::Load { reg, addr } => pack(Opcode::Load, reg, *addr),
            Instruction::LoadP { reg, addr_reg } => pack(Opcode::LoadP, reg, *addr_reg as u8),
            Instruction::LoadC { reg, val } => pack(Opcode::LoadC, reg, *val),

            Instruction::Store { reg, addr } => pack(Opcode::Store, reg, *addr),
            Instruction::StoreP { reg, addr_reg } => pack(Opcode::StoreP, reg, *addr_reg as u8),

            Instruction::Jz { addr } => cat(Opcode::Jz, *addr),
            Instruction::JzP { addr_reg } => cat(Opcode::JzP, *addr_reg as u8),
            Instruction::Jos { addr } => cat(Opcode::Jos, *addr),
            Instruction::JosP { addr_reg } => cat(Opcode::JosP, *addr_reg as u8),
            Instruction::Jus { addr } => cat(Opcode::Jus, *addr),
            Instruction::JusP { addr_reg } => cat(Opcode::JusP, *addr_reg as u8),

            Instruction::ShiftL { reg_in, reg_out } => pack2(Opcode::ShiftL, reg_in, reg_out),
            Instruction::ShiftR { reg_in, reg_out } => pack2(Opcode::ShiftR, reg_in, reg_out),
            Instruction::Gpi { reg } => cat(Opcode::Gpi, *reg as u8),
            Instruction::Gpo { reg } => cat(Opcode::Gpo, *reg as u8),
            Instruction::Alu {
                op,
                arg1,
                arg2,
                out,
            } => (
                ((Opcode::Alu as u8) << 4) | (*op as u8),
                ((*arg1 as u8) << 6) | ((*arg2 as u8) << 6) | (*out as u8),
            ),
        }
    }
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(line: &str) -> Result<Instruction, Self::Err> {
        let line_words: Vec<&str> = line.split(" ").collect();

        fn parse_addr(s: &str) -> Result<Word, String> {
            s.parse()
                .map_err(|_| format!("Invalid memory address: {}", s))
        }

        Ok(match line_words[0] {
            "LOAD" => Self::Load {
                reg: line_words[1].parse()?,
                addr: parse_addr(line_words[2])?,
            },
            "LOADP" => Self::LoadP {
                reg: line_words[1].parse()?,
                addr_reg: line_words[2].parse()?,
            },
            "LOADC" => Self::LoadC {
                reg: line_words[1].parse()?,
                val: parse_addr(line_words[2])?,
            },

            "STORE" => Self::Store {
                reg: line_words[1].parse()?,
                addr: parse_addr(line_words[2])?,
            },
            "STOREP" => Self::StoreP {
                reg: line_words[1].parse()?,
                addr_reg: line_words[2].parse()?,
            },

            "JZ" => Self::Jz {
                addr: parse_addr(line_words[1])?,
            },
            "JZP" => Self::JzP {
                addr_reg: line_words[1].parse()?,
            },
            "JOS" => Self::Jos {
                addr: parse_addr(line_words[1])?,
            },
            "JOSP" => Self::JosP {
                addr_reg: line_words[1].parse()?,
            },
            "JUS" => Self::Jus {
                addr: parse_addr(line_words[1])?,
            },
            "JUSP" => Self::JusP {
                addr_reg: line_words[1].parse()?,
            },

            "SHIFTL" => Self::ShiftL {
                reg_in: line_words[1].parse()?,
                reg_out: line_words[2].parse()?,
            },
            "SHIFTR" => Self::ShiftR {
                reg_in: line_words[1].parse()?,
                reg_out: line_words[2].parse()?,
            },
            "GPI" => Self::Gpi {
                reg: line_words[1].parse()?,
            },
            "GPO" => Self::Gpo {
                reg: line_words[1].parse()?,
            },
            "ALU" => Self::Alu {
                op: line_words[1].parse()?,
                arg1: line_words[2].parse()?,
                arg2: line_words[3].parse()?,
                out: line_words[4].parse()?,
            },
            other => panic!("Invalid instruction name: {:?}", other),
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

    pub fn step(&mut self) -> () {
        let instruction = Instruction::try_from((
            self.prog[self.eip as usize],
            self.prog[self.eip as usize + 1],
        ))
        .unwrap();

        match instruction {
            Instruction::Load { reg, addr } => {
                *self.registers.get_mut(reg) = self.prog[addr as usize];
                self.eip += 2;
            }
            Instruction::LoadP { reg, addr_reg } => {
                *self.registers.get_mut(reg) = self.prog[self.registers.get(&addr_reg) as usize];
                self.eip += 2;
            }
            Instruction::LoadC { reg, val } => {
                *self.registers.get_mut(reg) = val;
                self.eip += 2;
            }

            Instruction::Store { reg, addr } => {
                self.prog[addr as usize] = self.registers.get(&reg);
                self.eip += 2;
            }
            Instruction::StoreP { reg, addr_reg } => {
                self.prog[self.registers.get(&addr_reg) as usize] = self.registers.get(&reg);
                self.eip += 2;
            }

            Instruction::Jz { addr } => {
                if self.flags.eq_zero {
                    self.eip = addr;
                } else {
                    self.eip += 2;
                }
            }
            Instruction::JzP { addr_reg } => {
                if self.flags.eq_zero {
                    self.eip = self.prog[self.registers.get(&addr_reg) as usize];
                } else {
                    self.eip += 2;
                }
            }
            Instruction::Jos { addr } => {
                if self.flags.overflow_signed {
                    self.eip = addr;
                } else {
                    self.eip += 2;
                }
            }
            Instruction::JosP { addr_reg } => {
                if self.flags.overflow_signed {
                    self.eip = self.prog[self.registers.get(&addr_reg) as usize];
                } else {
                    self.eip += 2;
                }
            }
            Instruction::Jus { addr } => {
                if self.flags.overflow_unsigned {
                    self.eip = addr;
                } else {
                    self.eip += 2;
                }
            }
            Instruction::JusP { addr_reg } => {
                if self.flags.overflow_unsigned {
                    self.eip = self.prog[self.registers.get(&addr_reg) as usize];
                } else {
                    self.eip += 2;
                }
            }

            Instruction::ShiftL { reg_in, reg_out } => {
                *self.registers.get_mut(reg_out) = self.registers.get(&reg_in) << 1;
                self.eip += 2;
            }
            Instruction::ShiftR { reg_in, reg_out } => {
                *self.registers.get_mut(reg_out) = self.registers.get(&reg_in) >> 1;
                self.eip += 2;
            }
            Instruction::Gpi { reg } => {
                *self.registers.get_mut(reg) = self.reg_i;
                self.eip += 2;
            }
            Instruction::Gpo { reg } => {
                self.reg_o = self.registers.get(&reg);
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
