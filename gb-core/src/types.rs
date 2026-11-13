pub type Byte = u8;
pub type Word = u16;
pub type Address = u16;

pub trait ByteOperation {
	fn	get_high(self) -> Byte;
	fn	get_low(self) -> Byte;
}

impl ByteOperation for Byte {
	fn	get_high(self) -> Byte {
		self >> 4
	}

	fn	get_low(self) -> Byte {
		self & 0x0F
	}
}

pub trait WordOperation {
	fn	get_high(self) -> Byte;
	fn	get_low(self) -> Byte;
	fn	set_high(&self, byte: Byte) -> Word;
	fn	set_low(&self, byte: Byte) -> Word;

	fn	from_bytes(high: Byte, low: Byte) -> Word;
}

impl WordOperation for Word {
	fn	get_high(self) -> Byte
	{
		(self >> 8) as Byte
	}

	fn	get_low(self) -> Byte
	{
		(self & 0xFF) as Byte
	}

	fn	set_high(&self, byte: Byte) -> Word
	{
		let mut word = self & 0x00FF;
		word |= (byte as Word) << 8;
		word
	}
	
	fn	set_low(&self, byte: Byte) -> Word
	{
		let mut word = self & 0xFF00;
		word |= byte as Word;
		word
	}

	fn	from_bytes(high: Byte, low: Byte) -> Word
	{
		(low as Word).set_high(high)
	}
}
