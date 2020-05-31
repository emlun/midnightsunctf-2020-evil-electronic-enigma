pub type Word = u8;
pub type Memory = Vec<Word>;

pub type Opcode = u8;
pub type RegisterRef = u8;
pub type Address = Word;
pub type Value = Word;
pub type AluOpcode = u8;

const OP_LOAD: Opcode = 0x0;
const OP_LOADP: Opcode = 0x1;
const OP_LOADC: Opcode = 0x2;
const OP_STORE: Opcode = 0x3;
const OP_STOREP: Opcode = 0x4;
const OP_JZ: Opcode = 0x5;
const OP_JZP: Opcode = 0x6;
const OP_JOS: Opcode = 0x7;
const OP_JOSP: Opcode = 0x8;
const OP_JUS: Opcode = 0x9;
const OP_JUSP: Opcode = 0xA;
const OP_SHIFTL: Opcode = 0xB;
const OP_SHIFTR: Opcode = 0xC;
const OP_GPI: Opcode = 0xD;
const OP_GPO: Opcode = 0xE;
const OP_ALU: Opcode = 0xF;

const OP_ALU_ADD: AluOpcode = 0b0000;
const OP_ALU_ADD_CARRY: AluOpcode = 0b0001;
const OP_ALU_XOR: AluOpcode = 0b0100;
const OP_ALU_OR: AluOpcode = 0b1000;
const OP_ALU_AND: AluOpcode = 0b1001;
const OP_ALU_NAND: AluOpcode = 0b1010;
const OP_ALU_NOR: AluOpcode = 0b1011;
const OP_ALU_ECHO: AluOpcode = 0b1100;

#[derive(Clone, Debug)]
pub struct LegComputer {
    pub eip: Word,
    pub prog: Memory,
    pub flags: AluFlags,
    pub registers: Registers,
    pub reg_i: Word,
    pub reg_o: Word,
}

impl std::fmt::Display for LegComputer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
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

impl std::fmt::Display for AluFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
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
    pub a: Word,
    pub b: Word,
    pub c: Word,
    pub d: Word,
}

impl Registers {
    fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        }
    }

    fn get(&self, reg: RegisterRef) -> Word {
        match reg {
            0x0 => self.a,
            0x1 => self.b,
            0x2 => self.c,
            0x3 => self.d,
            _ => panic!("Invalid register reference: {}", reg),
        }
    }

    fn get_mut(&mut self, reg: RegisterRef) -> &mut Word {
        match reg {
            0x0 => &mut self.a,
            0x1 => &mut self.b,
            0x2 => &mut self.c,
            0x3 => &mut self.d,
            _ => panic!("Invalid register reference: {}", reg),
        }
    }
}

impl std::fmt::Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "[A: {}, B: {}, C: {}, D: {}]",
            self.a, self.b, self.c, self.d,
        )
    }
}

pub enum Instruction {
    LOAD {
        reg: RegisterRef,
        addr: Address,
    },
    LOADP {
        reg: RegisterRef,
        addr_reg: RegisterRef,
    },
    LOADC {
        reg: RegisterRef,
        val: Value,
    },
    STORE {
        reg: RegisterRef,
        addr: Address,
    },
    STOREP {
        reg: RegisterRef,
        addr_reg: RegisterRef,
    },
    JZ {
        addr: Address,
    },
    JZP {
        addr_reg: RegisterRef,
    },
    JOS {
        addr: Address,
    },
    JOSP {
        addr_reg: RegisterRef,
    },
    JUS {
        addr: Address,
    },
    JUSP {
        addr_reg: RegisterRef,
    },
    SHIFTL {
        reg_in: RegisterRef,
        reg_out: RegisterRef,
    },
    SHIFTR {
        reg_in: RegisterRef,
        reg_out: RegisterRef,
    },
    GPI {
        reg: RegisterRef,
    },
    GPO {
        reg: RegisterRef,
    },
    ALU {
        op: AluOpcode,
        arg1: RegisterRef,
        arg2: RegisterRef,
        out: RegisterRef,
    },
}

impl From<(Word, Word)> for Instruction {
    fn from((word1, word2): (Word, Word)) -> Instruction {
        let opcode = word1 >> 4;
        match opcode {
            OP_LOAD => Self::LOAD {
                reg: word1 & 0x3,
                addr: word2,
            },
            OP_LOADP => Self::LOADP {
                reg: word1 & 0x3,
                addr_reg: word2 & 0x3,
            },
            OP_LOADC => Self::LOADC {
                reg: word1 & 0x3,
                val: word2,
            },
            OP_STORE => Self::STORE {
                reg: word1 & 0x3,
                addr: word2,
            },
            OP_STOREP => Self::STOREP {
                reg: word1 & 0x3,
                addr_reg: word2 & 0x3,
            },
            OP_JZ => Self::JZ { addr: word2 },
            OP_JZP => Self::JZP {
                addr_reg: word2 & 0x3,
            },
            OP_JOS => Self::JOS { addr: word2 },
            OP_JOSP => Self::JOSP {
                addr_reg: word2 & 0x3,
            },
            OP_JUS => Self::JUS { addr: word2 },
            OP_JUSP => Self::JUSP {
                addr_reg: word2 & 0x3,
            },
            OP_SHIFTL => Self::SHIFTL {
                reg_in: (word2 >> 4) & 0x3,
                reg_out: word2 & 0x3,
            },
            OP_SHIFTR => Self::SHIFTR {
                reg_in: (word2 >> 4) & 0x3,
                reg_out: word2 & 0x3,
            },
            OP_GPI => Self::GPI { reg: word2 & 0x3 },
            OP_GPO => Self::GPO { reg: word2 & 0x3 },
            OP_ALU => Self::ALU {
                op: word1 & 0xf,
                arg1: word2 >> 6,
                arg2: (word2 >> 4) & 0x3,
                out: word2 & 0x3,
            },

            _ => panic!("Invalid opcode: {}", opcode),
        }
    }
}

impl Into<(Word, Word)> for &Instruction {
    fn into(self) -> (Word, Word) {
        match self {
            Instruction::LOAD { reg, addr } => ((OP_LOAD << 4) | reg, *addr),
            Instruction::LOADP { reg, addr_reg } => ((OP_LOADP << 4) | reg, *addr_reg),
            Instruction::LOADC { reg, val } => ((OP_LOADC << 4) | reg, *val),
            Instruction::STORE { reg, addr } => ((OP_STORE << 4) | reg, *addr),
            Instruction::STOREP { reg, addr_reg } => ((OP_STOREP << 4) | reg, *addr_reg),
            Instruction::JZ { addr } => (OP_JZ << 4, *addr),
            Instruction::JZP { addr_reg } => (OP_JZP << 4, *addr_reg),
            Instruction::JOS { addr } => (OP_JOS << 4, *addr),
            Instruction::JOSP { addr_reg } => (OP_JOSP << 4, *addr_reg),
            Instruction::JUS { addr } => (OP_JUS << 4, *addr),
            Instruction::JUSP { addr_reg } => (OP_JUSP << 4, *addr_reg),
            Instruction::SHIFTL { reg_in, reg_out } => (OP_SHIFTL << 4, (reg_in << 4) | reg_out),
            Instruction::SHIFTR { reg_in, reg_out } => (OP_SHIFTR << 4, (reg_in << 4) | reg_out),
            Instruction::GPI { reg } => (OP_GPI << 4, *reg),
            Instruction::GPO { reg } => (OP_GPO << 4, *reg),
            Instruction::ALU {
                op,
                arg1,
                arg2,
                out,
            } => ((OP_ALU << 4) | op, (arg1 << 6) | (arg2 << 6) | out),
        }
    }
}

impl std::str::FromStr for Instruction {
    type Err = String;
    fn from_str(line: &str) -> Result<Instruction, Self::Err> {
        let line_words: Vec<&str> = line.split(" ").collect();

        fn to_reg_ref(s: &str) -> Result<RegisterRef, String> {
            match s {
                "A" => Ok(0x0),
                "B" => Ok(0x1),
                "C" => Ok(0x2),
                "D" => Ok(0x3),
                _ => Err(format!("Innvalid register: {}", s)),
            }
        }

        Ok(match line_words[0] {
            "LOAD" => Self::LOAD {
                reg: to_reg_ref(line_words[1])?,
                addr: line_words[2]
                    .parse()
                    .map_err(|_| format!("Invalid memory address: {}", line_words[2]))?,
            },
            "LOADP" => Self::LOADP {
                reg: to_reg_ref(line_words[1])?,
                addr_reg: to_reg_ref(line_words[2])?,
            },
            "LOADC" => Self::LOADC {
                reg: to_reg_ref(line_words[1])?,
                val: line_words[2]
                    .parse()
                    .map_err(|_| format!("Invalid constant value: {}", line_words[2]))?,
            },
            "STORE" => Self::STORE {
                reg: to_reg_ref(line_words[1])?,
                addr: line_words[2]
                    .parse()
                    .map_err(|_| format!("Invalid memory address: {}", line_words[2]))?,
            },
            "STOREP" => Self::STOREP {
                reg: to_reg_ref(line_words[1])?,
                addr_reg: to_reg_ref(line_words[2])?,
            },
            "JZ" => Self::JZ {
                addr: line_words[1]
                    .parse()
                    .map_err(|_| format!("Invalid memory address: {}", line_words[1]))?,
            },
            "JZP" => Self::JZP {
                addr_reg: to_reg_ref(line_words[1])?,
            },
            "JOS" => Self::JOS {
                addr: line_words[1]
                    .parse()
                    .map_err(|_| format!("Invalid memory address: {}", line_words[1]))?,
            },
            "JOSP" => Self::JOSP {
                addr_reg: to_reg_ref(line_words[1])?,
            },
            "JUS" => Self::JUS {
                addr: line_words[1]
                    .parse()
                    .map_err(|_| format!("Invalid memory address: {}", line_words[1]))?,
            },
            "JUSP" => Self::JUSP {
                addr_reg: to_reg_ref(line_words[1])?,
            },
            "SHIFTL" => Self::SHIFTL {
                reg_in: to_reg_ref(line_words[1])?,
                reg_out: to_reg_ref(line_words[2])?,
            },
            "SHIFTR" => Self::SHIFTR {
                reg_in: to_reg_ref(line_words[1])?,
                reg_out: to_reg_ref(line_words[2])?,
            },
            "GPI" => Self::GPI {
                reg: to_reg_ref(line_words[1])?,
            },
            "GPO" => Self::GPO {
                reg: to_reg_ref(line_words[1])?,
            },
            "ALU" => Self::ALU {
                op: match line_words[1] {
                    "ADD" => Ok(OP_ALU_ADD),
                    "ADDC" => Ok(OP_ALU_ADD_CARRY),
                    "XOR" => Ok(OP_ALU_XOR),
                    "OR" => Ok(OP_ALU_OR),
                    "AND" => Ok(OP_ALU_AND),
                    "NAND" => Ok(OP_ALU_NAND),
                    "NOR" => Ok(OP_ALU_NOR),
                    "ECHO" => Ok(OP_ALU_ECHO),
                    _ => Err(format!("Invalid ALU operation: {}", line_words[1])),
                }?,
                arg1: to_reg_ref(line_words[2])?,
                arg2: to_reg_ref(line_words[3])?,
                out: to_reg_ref(line_words[4])?,
            },
            _ => panic!("Invalid instruction name: {}", line_words[0]),
        })
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
        let instruction = Instruction::from((
            self.prog[self.eip as usize],
            self.prog[self.eip as usize + 1],
        ));

        match instruction {
            Instruction::LOAD { reg, addr } => {
                *self.registers.get_mut(reg) = self.prog[addr as usize];
                self.eip += 2;
            }
            Instruction::LOADP { reg, addr_reg } => {
                *self.registers.get_mut(reg) = self.prog[self.registers.get(addr_reg) as usize];
                self.eip += 2;
            }
            Instruction::LOADC { reg, val } => {
                *self.registers.get_mut(reg) = val;
                self.eip += 2;
            }
            Instruction::STORE { reg, addr } => {
                self.prog[addr as usize] = self.registers.get(reg);
                self.eip += 2;
            }
            Instruction::STOREP { reg, addr_reg } => {
                self.prog[self.registers.get(addr_reg) as usize] = self.registers.get(reg);
                self.eip += 2;
            }
            Instruction::JZ { addr } => {
                if self.flags.eq_zero {
                    self.eip = addr;
                } else {
                    self.eip += 2;
                }
            }
            Instruction::JZP { addr_reg } => {
                if self.flags.eq_zero {
                    self.eip = self.prog[self.registers.get(addr_reg) as usize];
                } else {
                    self.eip += 2;
                }
            }
            Instruction::JOS { addr } => {
                if self.flags.overflow_signed {
                    self.eip = addr;
                } else {
                    self.eip += 2;
                }
            }
            Instruction::JOSP { addr_reg } => {
                if self.flags.overflow_signed {
                    self.eip = self.prog[self.registers.get(addr_reg) as usize];
                } else {
                    self.eip += 2;
                }
            }
            Instruction::JUS { addr } => {
                if self.flags.overflow_unsigned {
                    self.eip = addr;
                } else {
                    self.eip += 2;
                }
            }
            Instruction::JUSP { addr_reg } => {
                if self.flags.overflow_unsigned {
                    self.eip = self.prog[self.registers.get(addr_reg) as usize];
                } else {
                    self.eip += 2;
                }
            }
            Instruction::SHIFTL { reg_in, reg_out } => {
                *self.registers.get_mut(reg_out) = self.registers.get(reg_in) << 1;
                self.eip += 2;
            }
            Instruction::SHIFTR { reg_in, reg_out } => {
                *self.registers.get_mut(reg_out) = self.registers.get(reg_in) >> 1;
                self.eip += 2;
            }
            Instruction::GPI { reg } => {
                *self.registers.get_mut(reg) = self.reg_i;
                self.eip += 2;
            }
            Instruction::GPO { reg } => {
                self.reg_o = self.registers.get(reg);
                self.eip += 2;
            }
            Instruction::ALU {
                op,
                arg1,
                arg2,
                out,
            } => {
                match op {
                    OP_ALU_ADD => {
                        let arg1_unsigned: u16 = self.registers.get(arg1) as u16;
                        let arg2_unsigned: u16 = self.registers.get(arg2) as u16;
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

                    OP_ALU_ADD_CARRY => {
                        let arg1_unsigned: u16 = self.registers.get(arg1) as u16;
                        let arg2_unsigned: u16 = self.registers.get(arg2) as u16;
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

                    OP_ALU_XOR => {
                        *self.registers.get_mut(out) =
                            self.registers.get(arg1) ^ self.registers.get(arg2);
                    }
                    OP_ALU_OR => {
                        *self.registers.get_mut(out) =
                            self.registers.get(arg1) | self.registers.get(arg2);
                    }
                    OP_ALU_AND => {
                        *self.registers.get_mut(out) =
                            self.registers.get(arg1) & self.registers.get(arg2);
                    }
                    OP_ALU_NAND => {
                        *self.registers.get_mut(out) =
                            !(self.registers.get(arg1) & self.registers.get(arg2));
                    }
                    OP_ALU_NOR => {
                        *self.registers.get_mut(out) =
                            !(self.registers.get(arg1) | self.registers.get(arg2));
                    }

                    OP_ALU_ECHO => {
                        *self.registers.get_mut(out) = self.registers.get(arg1);
                    }

                    _ => panic!("Invalid ALU opcode: {}", op),
                };
                self.flags.eq_zero = self.registers.get(out) == 0;
                self.eip += 2;
            }
        };
    }
}

impl std::str::FromStr for LegComputer {
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
