use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

pub type Word = u8;
pub type Memory = Vec<Word>;
pub type Address = Word;
pub type Value = Word;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Opcode {
    Nop = 0x0,

    Load = 0x1,
    LoadP = 0x2,

    Store = 0x3,
    StoreP = 0x4,

    Mov = 0x5,
    MovC = 0x6,

    Jmp = 0x7,
    JmpP = 0x8,
    JmpR = 0x9,
    JmpRP = 0xA,

    Stack = 0xB,

    Gpio = 0xC,

    Alu = 0xD,
}

impl TryFrom<Word> for Opcode {
    type Error = String;
    fn try_from(w: Word) -> Result<Self, Self::Error> {
        match w {
            0x0 => Ok(Self::Nop),

            0x1 => Ok(Self::Load),
            0x2 => Ok(Self::LoadP),

            0x3 => Ok(Self::Store),
            0x4 => Ok(Self::StoreP),

            0x5 => Ok(Self::Mov),
            0x6 => Ok(Self::MovC),

            0x7 => Ok(Self::Jmp),
            0x8 => Ok(Self::JmpP),
            0x9 => Ok(Self::JmpR),
            0xA => Ok(Self::JmpRP),

            0xB => Ok(Self::Stack),

            0xC => Ok(Self::Gpio),

            0xD => Ok(Self::Alu),

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
    /// First 8 flags
    FL = 12,
    /// Stack top pointer
    ST = 13,
    /// Stack frame base pointer
    BP = 14,
    /// Instruction pointer
    IP = 15,
}

impl TryFrom<Word> for RegisterRef {
    type Error = String;
    fn try_from(w: Word) -> Result<Self, Self::Error> {
        match w {
            0 => Ok(Self::A),
            1 => Ok(Self::B),
            2 => Ok(Self::C),
            3 => Ok(Self::D),
            12 => Ok(Self::FL),
            13 => Ok(Self::ST),
            14 => Ok(Self::BP),
            15 => Ok(Self::IP),
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
    Neg = 0b0101,
    Sub = 0b0110,
    Or = 0b1000,
    And = 0b1001,
    Nand = 0b1010,
    Nor = 0b1011,
    ShiftL = 0b1100,
    ShiftR = 0b1101,
    Echo = 0b1111,
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
            0b0101 => Ok(Self::Neg),
            0b0110 => Ok(Self::Sub),
            0b1000 => Ok(Self::Or),
            0b1001 => Ok(Self::And),
            0b1010 => Ok(Self::Nand),
            0b1011 => Ok(Self::Nor),
            0b1100 => Ok(Self::ShiftL),
            0b1101 => Ok(Self::ShiftR),
            0b1111 => Ok(Self::Echo),
            other => Err(format!("Invalid ALU opcode: {}", other)),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum AluFlagRef {
    EqZero = 0,
    OverflowUnsigned = 1,
    OverflowSigned = 2,
    Equal = 3,
    GreaterThan = 4,
    GreaterThanSigned = 5,
    GreaterOrEqual = 6,
    GreaterOrEqualSigned = 7,

    NotEqual = 8,
    LessThan = 9,
    LessThanSigned = 10,
    LessOrEqual = 11,
    LessOrEqualSigned = 12,

    False = 14,
    True = 15,
}

impl TryFrom<Word> for AluFlagRef {
    type Error = String;
    fn try_from(w: Word) -> Result<Self, Self::Error> {
        match w {
            0 => Ok(Self::EqZero),
            1 => Ok(Self::OverflowUnsigned),
            2 => Ok(Self::OverflowSigned),
            3 => Ok(Self::Equal),
            4 => Ok(Self::GreaterThan),
            5 => Ok(Self::GreaterThanSigned),
            6 => Ok(Self::GreaterOrEqual),
            7 => Ok(Self::GreaterOrEqualSigned),

            8 => Ok(Self::NotEqual),
            9 => Ok(Self::LessThan),
            10 => Ok(Self::LessThanSigned),
            11 => Ok(Self::LessOrEqual),
            12 => Ok(Self::LessOrEqualSigned),

            14 => Ok(Self::False),
            15 => Ok(Self::True),
            other => Err(format!("Invalid flag: {}", other)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AluFlags {
    pub eq_zero: bool,
    pub overflow_unsigned: bool,
    pub overflow_signed: bool,
    pub equal: bool,
    pub greater_than: bool,
    pub greater_than_signed: bool,
    pub greater_or_equal: bool,
    pub greater_or_equal_signed: bool,

    pub not_equal: bool,
    pub less_than: bool,
    pub less_than_signed: bool,
    pub less_or_equal: bool,
    pub less_or_equal_signed: bool,
}

impl AluFlags {
    fn new() -> AluFlags {
        AluFlags {
            eq_zero: false,
            overflow_unsigned: false,
            overflow_signed: false,
            equal: false,
            greater_than: false,
            greater_than_signed: false,
            greater_or_equal: false,
            greater_or_equal_signed: false,

            not_equal: false,
            less_than: false,
            less_than_signed: false,
            less_or_equal: false,
            less_or_equal_signed: false,
        }
    }

    fn get(&self, flag: &AluFlagRef) -> bool {
        match *flag {
            AluFlagRef::EqZero => self.eq_zero,
            AluFlagRef::OverflowUnsigned => self.overflow_unsigned,
            AluFlagRef::OverflowSigned => self.overflow_signed,
            AluFlagRef::Equal => self.equal,
            AluFlagRef::GreaterThan => self.greater_than,
            AluFlagRef::GreaterThanSigned => self.greater_than_signed,
            AluFlagRef::GreaterOrEqual => self.greater_or_equal,
            AluFlagRef::GreaterOrEqualSigned => self.greater_or_equal_signed,

            AluFlagRef::NotEqual => self.not_equal,
            AluFlagRef::LessThan => self.less_than,
            AluFlagRef::LessThanSigned => self.less_than_signed,
            AluFlagRef::LessOrEqual => self.less_or_equal,
            AluFlagRef::LessOrEqualSigned => self.less_or_equal_signed,
            AluFlagRef::False => false,
            AluFlagRef::True => true,
        }
    }

    fn as_word(&self) -> Word {
        (if self.eq_zero { 0x1 } else { 0 })
            | (if self.overflow_unsigned { 0x2 } else { 0 })
            | (if self.overflow_signed { 0x4 } else { 0 })
            | (if self.equal { 0x8 } else { 0 })
            | (if self.greater_than { 0x10 } else { 0 })
            | (if self.greater_than_signed { 0x20 } else { 0 })
            | (if self.greater_or_equal { 0x40 } else { 0 })
            | (if self.greater_or_equal_signed {
                0x80
            } else {
                0
            })
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
                ("EQ", self.equal),
                ("GT", self.greater_than),
                ("GTs", self.greater_than_signed),
                ("GE", self.greater_or_equal),
                ("GEs", self.greater_or_equal_signed),
                ("NE", self.not_equal),
                ("LT", self.less_than),
                ("LTs", self.less_than_signed),
                ("LE", self.less_or_equal),
                ("LEs", self.less_or_equal_signed),
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
        let mut values = HashMap::new();
        values.insert(RegisterRef::A, 0);
        values.insert(RegisterRef::B, 0);
        values.insert(RegisterRef::C, 0);
        values.insert(RegisterRef::D, 0);
        values.insert(RegisterRef::ST, 0);
        values.insert(RegisterRef::BP, 0);
        Registers { values }
    }

    pub fn get(&self, reg: &RegisterRef) -> Word {
        *self
            .values
            .get(reg)
            .expect(&format!("Register not set: {:?}", reg))
    }

    fn get_mut(&mut self, reg: RegisterRef) -> &mut Word {
        self.values.entry(reg).or_insert(0)
    }
}

impl Display for Registers {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "[A: {}, B: {}, C: {}, D: {}, ST: {}, BP: {}]",
            self.get(&RegisterRef::A),
            self.get(&RegisterRef::B),
            self.get(&RegisterRef::C),
            self.get(&RegisterRef::D),
            self.get(&RegisterRef::ST),
            self.get(&RegisterRef::BP),
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
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

    Stack(StackInstruction),

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

    Nop(NopOpcode),
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum StackOpcode {
    Ret = 0b0000,
    Push = 0b0001,
    Pop = 0b0010,
    Call = 0b0100,
    CallC = 0b0101,
    CallR = 0b0110,
    LoadA = 0b1000,
    LoadB = 0b1001,
    LoadC = 0b1010,
    LoadD = 0b1011,
}

impl TryFrom<Word> for StackOpcode {
    type Error = String;
    fn try_from(w: Word) -> Result<Self, Self::Error> {
        match w {
            0b0000 => Ok(Self::Ret),
            0b0001 => Ok(Self::Push),
            0b0010 => Ok(Self::Pop),

            0b0100 => Ok(Self::Call),
            0b0101 => Ok(Self::CallC),
            0b0110 => Ok(Self::CallR),

            0b1000 => Ok(Self::LoadA),
            0b1001 => Ok(Self::LoadB),
            0b1010 => Ok(Self::LoadC),
            0b1011 => Ok(Self::LoadD),

            other => Err(format!("Invalid stack opcode: {}", other)),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NopOpcode {
    Halt = 0x00,
    Nop = 0xff,
}

impl TryFrom<Word> for NopOpcode {
    type Error = String;
    fn try_from(w: Word) -> Result<Self, Self::Error> {
        match w {
            0x00 => Ok(Self::Halt),
            0xff => Ok(Self::Nop),
            other => Err(format!("Invalid NOP opcode: {}", other)),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum StackInstruction {
    Push { src: RegisterRef },
    Pop { dest: RegisterRef },
    Load { dest: RegisterRef, bp_diff: Word },
    Call { addr_reg: RegisterRef },
    CallC { addr: Word },
    CallR { diff: Word },
    Ret { src: RegisterRef },
}

impl Into<(Word, Word)> for &StackInstruction {
    fn into(self) -> (Word, Word) {
        fn cat(op: StackOpcode, arg: Word) -> (Word, Word) {
            (((Opcode::Stack as u8) << 4) | (op as u8), arg)
        }
        match self {
            StackInstruction::Push { src } => cat(StackOpcode::Push, *src as u8),
            StackInstruction::Pop { dest } => cat(StackOpcode::Pop, *dest as u8),
            StackInstruction::Call { addr_reg } => cat(StackOpcode::Call, *addr_reg as u8),
            StackInstruction::CallC { addr } => cat(StackOpcode::CallC, *addr),
            StackInstruction::CallR { diff } => cat(StackOpcode::CallR, *diff),
            StackInstruction::Ret { src } => cat(StackOpcode::Ret, *src as u8),
            StackInstruction::Load { dest, bp_diff } => {
                let opcode = match dest {
                    RegisterRef::A => StackOpcode::LoadA,
                    RegisterRef::B => StackOpcode::LoadB,
                    RegisterRef::C => StackOpcode::LoadC,
                    RegisterRef::D => StackOpcode::LoadD,
                    other => panic!("Cannot load from stack to register: {:?}", other),
                };
                cat(opcode, *bp_diff)
            }
        }
    }
}

impl TryFrom<(Word, Word)> for Instruction {
    type Error = String;
    fn try_from((word1, word2): (Word, Word)) -> Result<Instruction, String> {
        let opcode = Opcode::try_from(word1 >> 4)?;

        Ok(match opcode {
            Opcode::Load => Self::Load {
                dest: (word1 & 0xf).try_into()?,
                addr: word2,
            },
            Opcode::LoadP => Self::LoadP {
                dest: (word1 & 0xf).try_into()?,
                addr_src: (word2 & 0xf).try_into()?,
            },

            Opcode::Store => Self::Store {
                src: (word1 & 0xf).try_into()?,
                addr: word2,
            },
            Opcode::StoreP => Self::StoreP {
                src: (word1 & 0xf).try_into()?,
                addr_src: (word2 & 0xf).try_into()?,
            },

            Opcode::Mov => Self::Mov {
                dest: (word1 & 0xf).try_into()?,
                src: word2.try_into()?,
            },
            Opcode::MovC => Self::MovC {
                dest: (word1 & 0xf).try_into()?,
                val: word2,
            },

            Opcode::Jmp => Self::Jmp {
                flag: (word1 & 0xf).try_into()?,
                addr: word2,
            },
            Opcode::JmpP => Self::JmpP {
                flag: (word1 & 0xf).try_into()?,
                addr_src: (word2 & 0xf).try_into()?,
            },
            Opcode::JmpR => Self::JmpR {
                flag: (word1 & 0xf).try_into()?,
                diff: word2,
            },
            Opcode::JmpRP => Self::JmpRP {
                flag: (word1 & 0xf).try_into()?,
                diff_src: (word2 & 0xf).try_into()?,
            },

            Opcode::Stack => Self::Stack({
                let stack_opcode = StackOpcode::try_from(word1 & 0xf)?;
                match stack_opcode {
                    StackOpcode::Ret => StackInstruction::Ret {
                        src: word2.try_into()?,
                    },
                    StackOpcode::Push => StackInstruction::Push {
                        src: word2.try_into()?,
                    },
                    StackOpcode::Pop => StackInstruction::Pop {
                        dest: word2.try_into()?,
                    },
                    StackOpcode::Call => StackInstruction::Call {
                        addr_reg: word2.try_into()?,
                    },
                    StackOpcode::CallC => StackInstruction::CallC { addr: word2 },
                    StackOpcode::CallR => StackInstruction::CallR { diff: word2 },
                    StackOpcode::LoadA => StackInstruction::Load {
                        dest: RegisterRef::A,
                        bp_diff: word2,
                    },
                    StackOpcode::LoadB => StackInstruction::Load {
                        dest: RegisterRef::B,
                        bp_diff: word2,
                    },
                    StackOpcode::LoadC => StackInstruction::Load {
                        dest: RegisterRef::C,
                        bp_diff: word2,
                    },
                    StackOpcode::LoadD => StackInstruction::Load {
                        dest: RegisterRef::D,
                        bp_diff: word2,
                    },
                }
            }),

            Opcode::Gpio => match word1 & 0xf {
                0 => Self::Gpi {
                    dest: (word2 & 0xf).try_into()?,
                },
                1 => Self::Gpo {
                    src: (word2 & 0xf).try_into()?,
                },
                other => Err(format!("Invalid GPIO op: {}", other))?,
            },

            Opcode::Alu => Self::Alu {
                op: (word1 & 0xf).try_into()?,
                arg1: (word2 >> 6).try_into()?,
                arg2: ((word2 >> 4) & 0x3).try_into()?,
                out: (word2 & 0x3).try_into()?,
            },

            Opcode::Nop => Self::Nop(NopOpcode::try_from(word2)?),
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

            Instruction::Stack(stack_ins) => stack_ins.into(),

            Instruction::Gpi { dest } => ((Opcode::Gpio as u8) << 4 | 0x0, *dest as u8),
            Instruction::Gpo { src } => ((Opcode::Gpio as u8) << 4 | 0x1, *src as u8),

            Instruction::Alu {
                op,
                arg1,
                arg2,
                out,
            } => (
                ((Opcode::Alu as u8) << 4) | (*op as u8),
                ((*arg1 as u8) << 6) | ((*arg2 as u8) << 4) | (*out as u8),
            ),

            Instruction::Nop(nopcode) => cat(Opcode::Nop, *nopcode as u8),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LegComputer {
    pub eip: Word,
    pub program: Memory,
    pub memory: Memory,
    pub flags: AluFlags,
    pub registers: Registers,
    pub reg_i: Word,
    pub reg_o: Word,
}

impl Display for LegComputer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "{eip:03} {regs} {flags} [{reg_i} {reg_o}]\n",
            eip = self.eip,
            regs = self.registers,
            flags = self.flags,
            reg_i = self.reg_i,
            reg_o = self.reg_o,
        )?;

        let instruction = Instruction::try_from((
            self.program[self.eip as usize],
            self.program[self.eip as usize + 1],
        ))
        .unwrap();

        write!(f, "{:?}\n", instruction)?;

        for (i, v) in self.memory.iter().enumerate() {
            if i % 8 == 0 {
                write!(f, "\n{:>3}: ", i)?;
            }
            let is_sp = i == usize::from(self.read_register(&RegisterRef::ST));
            let is_bp = i == usize::from(self.read_register(&RegisterRef::BP));
            if is_sp && is_bp {
                write!(f, "[ {:>2} ]", v)?;
            } else if is_sp {
                write!(f, "[ {:>4}", v)?;
            } else if is_bp {
                write!(f, "{:>4} ]", v)?;
            } else {
                write!(f, "{:>6}", v)?;
            }
            if i < (self.memory.len() - 1) {
                write!(f, "  ")?;
            }
        }

        Ok(())
    }
}

fn to_bytes(a: u8) -> [bool; 8] {
    let mut o = [false; 8];
    for i in 0..8 {
        o[i] = ((a >> i) & 0x01) == 0x01;
    }
    o
}

fn from_bytes(a: [bool; 8]) -> u8 {
    let mut o = 0;
    for i in 0..8 {
        if a[i] {
            o |= 1 << i;
        }
    }
    o
}

fn full_add(a: bool, b: bool, c: bool) -> (bool, bool) {
    (a ^ b ^ c, (b && c) || (a && (b || c)))
}

fn add_8bit(a: [bool; 8], b: [bool; 8], mut carry: bool) -> ([bool; 8], bool, bool) {
    let mut o = [false; 8];
    let mut prev_carry = carry;
    for i in 0..8 {
        let (s, c) = full_add(a[i], b[i], carry);
        o[i] = s;
        prev_carry = carry;
        carry = c;
    }
    (o, carry, prev_carry ^ carry)
}

impl LegComputer {
    pub fn new(program: Vec<Word>, memory: Vec<Word>) -> LegComputer {
        LegComputer {
            eip: 0,
            program,
            memory,
            flags: AluFlags::new(),
            registers: Registers::new(),
            reg_i: 0,
            reg_o: 0,
        }
    }

    pub fn is_halted(&self) -> bool {
        let instruction = Instruction::try_from((
            self.program[self.eip as usize],
            self.program[self.eip as usize + 1],
        ))
        .unwrap();
        instruction == Instruction::Nop(NopOpcode::Halt)
    }

    pub fn run(mut self) -> Self {
        while !self.is_halted() {
            self.step();
        }
        self
    }

    pub fn read_register(&self, register: &RegisterRef) -> Word {
        match register {
            RegisterRef::FL => self.flags.as_word(),
            RegisterRef::IP => self.eip,
            _ => self.registers.get(register),
        }
    }

    fn stack_push(&mut self, value: Word) -> () {
        let new_st = ((self.read_register(&RegisterRef::ST) as u16 + 255) & 0xff) as u8;
        *self.registers.get_mut(RegisterRef::ST) = new_st;
        self.memory[new_st as usize] = value;
    }

    fn stack_pop(&mut self) -> Word {
        let current_st = self.read_register(&RegisterRef::ST);
        let result = self.memory[current_st as usize];
        *self.registers.get_mut(RegisterRef::ST) = ((current_st as u16 + 1) & 0xff) as u8;
        result
    }

    fn call(&mut self, addr: Word) -> () {
        self.stack_push(self.eip);
        self.stack_push(self.read_register(&RegisterRef::BP));
        let current_st = self.read_register(&RegisterRef::ST);
        *self.registers.get_mut(RegisterRef::BP) = current_st;
        self.eip = addr;
    }

    pub fn step(&mut self) -> () {
        let instruction = Instruction::try_from((
            self.program[self.eip as usize],
            self.program[self.eip as usize + 1],
        ))
        .unwrap();

        match instruction {
            Instruction::Load { dest, addr } => {
                *self.registers.get_mut(dest) = self.memory[addr as usize];
                self.eip += 2;
            }
            Instruction::LoadP { dest, addr_src } => {
                *self.registers.get_mut(dest) = self.memory[self.read_register(&addr_src) as usize];
                self.eip += 2;
            }

            Instruction::Store { src, addr } => {
                self.memory[addr as usize] = self.read_register(&src);
                self.eip += 2;
            }
            Instruction::StoreP { src, addr_src } => {
                let mem_index = self.read_register(&addr_src) as usize;
                self.memory[mem_index] = self.read_register(&src);
                self.eip += 2;
            }

            Instruction::Mov { src, dest } => {
                *self.registers.get_mut(dest) = self.read_register(&src);
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
                    self.eip = self.memory[self.read_register(&addr_src) as usize];
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
                        + self.memory[self.read_register(&diff_src) as usize] as i16)
                        as u8;
                } else {
                    self.eip += 2;
                }
            }

            Instruction::Stack(stack_ins) => match stack_ins {
                StackInstruction::Ret { src } => {
                    let current_bp = self.read_register(&RegisterRef::BP);
                    *self.registers.get_mut(RegisterRef::ST) = current_bp;

                    let stored_bp = self.stack_pop();
                    let stored_ip = self.stack_pop();
                    *self.registers.get_mut(RegisterRef::BP) = stored_bp;
                    self.stack_push(self.read_register(&src));
                    self.eip = stored_ip + 2;
                }
                StackInstruction::Push { src } => {
                    self.stack_push(self.read_register(&src));
                    self.eip += 2;
                }
                StackInstruction::Pop { dest } => {
                    let value = self.stack_pop();
                    *self.registers.get_mut(dest) = value;
                    self.eip += 2;
                }
                StackInstruction::Call { addr_reg } => {
                    self.call(self.read_register(&addr_reg));
                }
                StackInstruction::CallC { addr } => {
                    self.call(addr);
                }
                StackInstruction::CallR { diff } => {
                    self.call((self.eip as i16 + diff as i16) as u8);
                }
                StackInstruction::Load { dest, bp_diff } => {
                    let current_bp = self.read_register(&RegisterRef::BP);
                    let load_addr = ((current_bp as i16 + bp_diff as i16 + 256) % 256) as u8;
                    *self.registers.get_mut(dest) = self.memory[load_addr as usize];
                    self.eip += 2;
                }
            },

            Instruction::Gpi { dest } => {
                *self.registers.get_mut(dest) = self.reg_i;
                self.eip += 2;
            }
            Instruction::Gpo { src } => {
                self.reg_o = self.read_register(&src);
                self.eip += 2;
            }

            Instruction::Alu {
                op,
                arg1: arg1_addr,
                arg2: arg2_addr,
                out,
            } => {
                let arg1: [bool; 8] = to_bytes(self.registers.get(&arg1_addr));
                let arg2: [bool; 8] = to_bytes(self.registers.get(&arg2_addr));

                match op {
                    AluOpcode::Add => {
                        let (o, ofl_u, ofl_s) = add_8bit(arg1, arg2, false);

                        *self.registers.get_mut(out) = from_bytes(o) as u8;
                        self.flags.overflow_unsigned = ofl_u;
                        self.flags.overflow_signed = ofl_s;
                    }

                    AluOpcode::AddCarry => {
                        let (o, ofl_u, ofl_s) = add_8bit(arg1, arg2, true);

                        *self.registers.get_mut(out) = from_bytes(o) as u8;
                        self.flags.overflow_unsigned = ofl_u;
                        self.flags.overflow_signed = ofl_s;
                    }

                    AluOpcode::Incr => {
                        let (o, ofl_u, ofl_s) = add_8bit(
                            arg1,
                            [true, false, false, false, false, false, false, false],
                            false,
                        );

                        *self.registers.get_mut(out) = from_bytes(o) as u8;
                        self.flags.overflow_unsigned = ofl_u;
                        self.flags.overflow_signed = ofl_s;
                    }

                    AluOpcode::Decr => {
                        let (o, ofl_u, ofl_s) = add_8bit(
                            arg1,
                            [true, true, true, true, true, true, true, true],
                            false,
                        );

                        *self.registers.get_mut(out) = from_bytes(o) as u8;
                        self.flags.overflow_unsigned = ofl_u;
                        self.flags.overflow_signed = ofl_s;
                    }

                    AluOpcode::Xor => {
                        let mut o = [false; 8];
                        for i in 0..8 {
                            o[i] = arg1[i] ^ arg2[i];
                        }
                        *self.registers.get_mut(out) = from_bytes(o);
                    }

                    AluOpcode::Neg => {
                        let mut o = [false; 8];
                        for i in 0..8 {
                            o[i] = !arg2[i];
                        }
                        *self.registers.get_mut(out) = from_bytes(o) as u8;
                    }

                    AluOpcode::Sub => {
                        let mut not2 = [false; 8];
                        for i in 0..8 {
                            not2[i] = !arg2[i];
                        }
                        let (o, ofl_u, ofl_s) = add_8bit(arg1, not2, true);

                        *self.registers.get_mut(out) = from_bytes(o) as u8;
                        self.flags.overflow_unsigned = ofl_u;
                        self.flags.overflow_signed = ofl_s;
                    }

                    AluOpcode::Or => {
                        let mut o = [false; 8];
                        for i in 0..8 {
                            o[i] = arg1[i] || arg2[i];
                        }
                        *self.registers.get_mut(out) = from_bytes(o);
                    }
                    AluOpcode::And => {
                        let mut o = [false; 8];
                        for i in 0..8 {
                            o[i] = arg1[i] && arg2[i];
                        }
                        *self.registers.get_mut(out) = from_bytes(o);
                    }
                    AluOpcode::Nand => {
                        let mut o = [false; 8];
                        for i in 0..8 {
                            o[i] = !(arg1[i] && arg2[i]);
                        }
                        *self.registers.get_mut(out) = from_bytes(o);
                    }
                    AluOpcode::Nor => {
                        let mut o = [false; 8];
                        for i in 0..8 {
                            o[i] = !(arg1[i] || arg2[i]);
                        }
                        *self.registers.get_mut(out) = from_bytes(o);
                    }

                    AluOpcode::ShiftL => {
                        let o = match arg2 {
                            [false, false, false, ..] => arg1,
                            [true, false, false, ..] => [
                                false, arg1[0], arg1[1], arg1[2], arg1[3], arg1[4], arg1[5],
                                arg1[6],
                            ],
                            [false, true, false, ..] => [
                                false, false, arg1[0], arg1[1], arg1[2], arg1[3], arg1[4], arg1[5],
                            ],
                            [true, true, false, ..] => [
                                false, false, false, arg1[0], arg1[1], arg1[2], arg1[3], arg1[4],
                            ],
                            [false, false, true, ..] => [
                                false, false, false, false, arg1[0], arg1[1], arg1[2], arg1[3],
                            ],
                            [true, false, true, ..] => {
                                [false, false, false, false, false, arg1[0], arg1[1], arg1[2]]
                            }
                            [false, true, true, ..] => {
                                [false, false, false, false, false, false, arg1[0], arg1[1]]
                            }
                            [true, true, true, ..] => {
                                [false, false, false, false, false, false, false, arg1[0]]
                            }
                        };
                        *self.registers.get_mut(out) = from_bytes(o);
                    }

                    AluOpcode::ShiftR => {
                        let o = match arg2 {
                            [false, false, false, ..] => arg1,
                            [true, false, false, ..] => [
                                arg1[1], arg1[2], arg1[3], arg1[4], arg1[5], arg1[6], arg1[7],
                                arg1[7],
                            ],
                            [false, true, false, ..] => [
                                arg1[2], arg1[3], arg1[4], arg1[5], arg1[6], arg1[7], arg1[7],
                                arg1[7],
                            ],
                            [true, true, false, ..] => [
                                arg1[3], arg1[4], arg1[5], arg1[6], arg1[7], arg1[7], arg1[7],
                                arg1[7],
                            ],
                            [false, false, true, ..] => [
                                arg1[4], arg1[5], arg1[6], arg1[7], arg1[7], arg1[7], arg1[7],
                                arg1[7],
                            ],
                            [true, false, true, ..] => [
                                arg1[5], arg1[6], arg1[7], arg1[7], arg1[7], arg1[7], arg1[7],
                                arg1[7],
                            ],
                            [false, true, true, ..] => [
                                arg1[6], arg1[7], arg1[7], arg1[7], arg1[7], arg1[7], arg1[7],
                                arg1[7],
                            ],
                            [true, true, true, ..] => [
                                arg1[7], arg1[7], arg1[7], arg1[7], arg1[7], arg1[7], arg1[7],
                                arg1[7],
                            ],
                        };
                        *self.registers.get_mut(out) = from_bytes(o);
                    }

                    AluOpcode::Echo => {
                        *self.registers.get_mut(out) = self.registers.get(&arg1_addr);
                    }
                };

                self.flags.eq_zero = self.registers.get(&out) == 0;

                self.flags.equal = arg1[0] == arg2[0];
                for i in 1..8 {
                    self.flags.equal = self.flags.equal && (arg1[i] == arg2[i]);
                }
                self.flags.not_equal = !self.flags.equal;

                self.flags.greater_than = false;
                let mut not_greater_than = false;
                for i in 0..8 {
                    self.flags.greater_than = self.flags.greater_than
                        || (arg1[7 - i] && !arg2[7 - i] && !not_greater_than);
                    not_greater_than = not_greater_than || (!arg1[7 - i] && arg2[7 - i]);
                }

                self.flags.greater_than_signed = !arg1[7] && arg2[7];
                let mut not_greater_than = arg1[7] && !arg2[7];
                for i in 1..8 {
                    self.flags.greater_than_signed = self.flags.greater_than_signed
                        || (arg1[7 - i] && !arg2[7 - i] && !not_greater_than);
                    not_greater_than = not_greater_than || (!arg1[7 - i] && arg2[7 - i]);
                }

                self.flags.greater_or_equal = self.flags.greater_than || self.flags.equal;
                self.flags.greater_or_equal_signed =
                    self.flags.greater_than_signed || self.flags.equal;
                self.flags.less_than = !self.flags.greater_or_equal;
                self.flags.less_than_signed = !self.flags.greater_or_equal_signed;
                self.flags.less_or_equal = !self.flags.greater_than;
                self.flags.less_or_equal_signed = !self.flags.greater_than_signed;

                self.eip += 2;
            }

            Instruction::Nop(NopOpcode::Nop) => {
                self.eip += 2;
            }
            Instruction::Nop(NopOpcode::Halt) => {}
        };
    }
}
