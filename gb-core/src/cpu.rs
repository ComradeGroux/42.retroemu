use crate::types::Byte;
use crate::types::Word;
use crate::types::WordOperation;

pub struct Cpu {
	pub a: Byte,
	pub b: Byte,
	pub c: Byte,
	pub d: Byte,
	pub e: Byte,
	pub h: Byte,
	pub l: Byte,
	pub f: Byte,	// flag
	pub sp: Word,	// stack pointer
	pub pc: Word,	// program counter
	pub halt: bool,	// halt flag
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Register {
	A,
	B,
	C,
	D,
	E,
	H,
	L,
	HL,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum	Register16 {
	BC,
	DE,
	HL,
	SP,
	AF,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum	Flag {
	Zero,
	Substraction,
	HalfCarry,
	Carry
}


#[derive(Debug, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
enum	Instructions {
	// Load
	LD_R_R,
	LD_R_N,
	LD_RR_RR,
	LD_HL_R,
	LD_HL_N,
	LD_R_HL,
	LD_RR_A,
	LD_NN_A,
	LDH_NN_A,
	LDH_C_A,
	LD_A_RR,
	LD_A_NN,
	LDH_A_NN,
	LDH_A_C,
	LD_HLI_A,
	LD_HLD_A,
	LD_A_HLI,
	LD_A_HLD,
	// 8-bit arithmetic
	ADC_A_R,
	ADC_A_HL,
	ADC_A_N,
	ADD_A_R,
	ADD_A_HL,
	ADD_A_N,
	CP_A_R,
	CP_A_HL,
	CP_A_N,
	DEC_R,
	DEC_HL,
	INC_R,
	INC_HL,
	SBC_A_R,
	SBC_A_HL,
	SBC_A_N,
	SUB_A_R,
	SUB_A_HL,
	SUB_A_N,
	// 16-bit arithmetic
	ADD_HL_RR,
	DEC_RR,
	INC_RR,
	// Bitwise logic
	AND_A_R,
	AND_A_HL,
	AND_A_N,
	CPL,
	OR_A_R,
	OR_A_HL,
	OR_A_N,
	XOR_A_R,
	XOR_A_HL,
	XOR_A_N,
	// Bit flag
	BIT_U_R,
	BIT_U_HL,
	RES_U_R,
	RES_U_HL,
	SET_U_R,
	SET_U_HL,
	// Bit shift
	RL_R,
	RL_HL,
	RL_A,
	RLC_R,
	RLC_HL,
	RLC_A,
	RR_R,
	RR_HL,
	RR_A,
	RRC_R,
	RRC_HL,
	RRC_A,
	SLA_R,
	SLA_HL,
	SRA_R,
	SRA_HL,
	SRL_R,
	SRL_HL,
	SWAP_R,
	SWAP_HL,
	// Jump and subroutine
	CALL_NN,
	CALL_CC_NN,
	JP_HL,
	JP_NN,
	JP_CC_NN,
	JR_NN,
	JR_CC_NN,
	RET_CC,
	RET,
	RETI,
	RST_VEC,
	// Carry flag
	CCF,
	SCF,
	// Stack manipulation
	ADD_HL_SP,
	ADD_SP_E,
	DEC_SP,
	INC_SP,
	LD_SP_NN,
	LD_NN_SP,
	LD_HL_SP_E,
	LD_SP_HL,
	POP_AF,
	POP_RR,
	PUSH_AF,
	PUSH_RR,
	// Interrupt-related
	DI,
	EI,
	HALT,
	// Miscellaneous
	DAA,
	NOP,
	STOP
}

impl Cpu
{
	pub fn	new() -> Self
	{
		Cpu {
			a: 0x00,
			b: 0x00,
			c: 0x00,
			d: 0x00,
			e: 0x00,
			h: 0x00,
			l: 0x00,
			f: 0x00,
			sp: 0x00,
			pc: 0x00,
			halt: false,
		}
	}

	pub fn	get_register(&self, reg: Register) -> Byte
	{
		match reg
		{
			Register::A => self.a,
			Register::B => self.b,
			Register::C => self.c,
			Register::D => self.d,
			Register::E => self.e,
			Register::H => self.h,
			Register::L => self.l,
			Register::HL => core::panic!("Unknown register HL")
		}
	}

	pub fn	set_register(&mut self, reg: Register, byte: Byte)
	{
		match reg
		{
			Register::A => self.a = byte,
			Register::B => self.b = byte,
			Register::C => self.c = byte,
			Register::D => self.d = byte,
			Register::E => self.e = byte,
			Register::H => self.h = byte,
			Register::L => self.l = byte,
			Register::HL => core::panic!("Unknown register HL")
		}
	}

	pub fn	get_register16(&self, reg: Register16) -> Word
	{
		match reg
		{
			Register16::SP => self.sp,
			Register16::BC => bytes2word(self.b, self.c),
			Register16::DE => bytes2word(self.d, self.e),
			Register16::AF => bytes2word(self.a, self.f),
			Register16::HL => bytes2word(self.h, self.l),
		}
	}

	pub fn	set_register16(&mut self, reg: Register16, word: Word)
	{
		match reg
		{
			Register16::SP => self.sp = word,
			Register16::BC => {
				self.b = word.get_high();
				self.c = word.get_low();
			},
			Register16::DE => {
				self.d = word.get_high();
				self.e = word.get_low();
			},
			Register16::AF => {
				self.a = word.get_high();
				self.f = word.get_low();
			},
			Register16::HL => {
				self.h = word.get_high();
				self.l = word.get_low();
			},
		}
	}



}

fn	bytes2word(high: Byte, low: Byte) -> Word
{
	(low as Word).set_high(high)
}
