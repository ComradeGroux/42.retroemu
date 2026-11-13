use std::process::exit;

const HEADER_START: usize = 0x100;
const HEADER_END: usize = 0x150;

pub enum	CartType {
	ROMONLY,
	MBC1,
	MBC1_RAM,
	MBC1_RAM_BATTERY,
	MBC2,
	MBC5,
	MBC5_RAM,
	MBC5_RAM_BATTERY
}
impl		TryFrom<u8> for CartType {
	type Error = HeaderError;

	fn	try_from(value: u8) -> Result<Self, Self::Error>
	{
		match value {
			0x00 => Ok(CartType::ROMONLY),
			0x01 => Ok(CartType::MBC1),
			0x02 => Ok(CartType::MBC1_RAM),
			0x03 => Ok(CartType::MBC1_RAM_BATTERY),
			0x05 => Ok(CartType::MBC2),
			0x19 => Ok(CartType::MBC5),
			0x1A => Ok(CartType::MBC5_RAM),
			0x1B => Ok(CartType::MBC5_RAM_BATTERY),
			other => Err(HeaderError::UnknownCartType(other))
		}	
	}
}

#[derive(Debug)]
pub enum	HeaderError {
	TooShort,
	UnknownCartType(u8)
}
impl 		std::fmt::Display for HeaderError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		match self
		{
			HeaderError::TooShort => write!(f, "ROM too small to contains a header"),
			HeaderError::UnknownCartType(b) => write!(f, "Unknown cartridge type: {b:#04x}"),
		}
	}
}

struct		Reader<'a> {
	data: &'a [u8]
}
impl<'a>	Reader<'a> {
	fn	new(data: &'a [u8]) -> Self
	{
		Self { data }
	}

	fn array<const N: usize>(&mut self) -> Result<[u8; N], HeaderError>
	{
		let (chunk, rest) = self.data.split_first_chunk::<N>().ok_or(HeaderError::TooShort)?;
		self.data = rest;
		Ok(*chunk)
	}

	fn u8(&mut self) -> Result<u8, HeaderError>
	{
		Ok(self.array::<1>()?[0])
	}

	fn u16_be(&mut self) -> Result<u16, HeaderError>
	{
		Ok(u16::from_be_bytes(self.array()?))
	}
}

struct	RomHeader {
	entry: [u8; 4],
	logo: [u8; 0x30],

	title: [u8; 16],
	new_lic_code: u16,
	sgb_flag: u8,
	cartridge_type: CartType,
	rom_size: u8,
	ram_size: u8,
	dest_code: u8,
	lic_code: u8,
	version: u8,
	header_checksum: u8,
	global_checksum: u16
}
impl	RomHeader {
	pub fn	parse(bytes: &[u8]) -> Result<RomHeader, HeaderError>
	{
		let mut r = Reader::new(bytes);

		Ok(RomHeader {
			entry: r.array()?,
			logo: r.array()?,
			title: r.array()?,
			new_lic_code: r.u16_be()?,
			sgb_flag: r.u8()?,
			cartridge_type: CartType::try_from(r.u8()?)?,
			rom_size: r.u8()?,
			ram_size: r.u8()?,
			dest_code: r.u8()?,
			lic_code: r.u8()?,
			version: r.u8()?,
			header_checksum: r.u8()?,
			global_checksum: r.u16_be()?
		})
	}
}

pub struct	Cartridge {
	filename: String,
	rom_size: u32,
	rom_data: Vec<u8>,
	header: RomHeader
}
impl		Cartridge {
	pub fn	new() -> Cartridge
	{
		Self {
			filename: "test".to_string(),
			rom_size: 0,
			rom_data: Vec::new(),
			header: RomHeader {
				entry: [0, 0, 0, 0],
				logo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
				title: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
				new_lic_code: 0,
				sgb_flag: 0,
				cartridge_type: CartType::ROMONLY,
				rom_size: 0,
				ram_size: 0,
				dest_code: 0,
				lic_code: 0,
				version: 0,
				header_checksum: 0,
				global_checksum: 0
			}
		}
	}

	pub fn	load(&mut self, cart_path: &str)
	{
		let	data = std::fs::read(cart_path);
		match data {
			Ok(data) => {
				let	header_bytes = match data.get(HEADER_START..HEADER_END)
				{
					Some(b) => b,
					None => {
						log::error!("{}", HeaderError::TooShort);
						exit(1);
					}
				};

				match RomHeader::parse(header_bytes)
				{
					Ok(header) => self.header = header,
					Err(err) => {
						log::error!("{}", err);
						exit(1);
					}
				}

				self.rom_size = data.len() as u32;
				self.rom_data = data;
			},
			Err(err) => {
				log::error!("{}", err);
				exit(1);
			}
		}

		if checksum_ok(&self.rom_data, self.header.header_checksum) == false
		{
			log::error!("ROM's checksum does not match");
			exit(1);
		}
	}
}

fn	checksum_ok(rom: &[u8], checksum: u8) -> bool
{
	let	sum = rom[0x134..=0x14C].iter().fold(0u8, |acc, &b| acc.wrapping_sub(b).wrapping_sub(1));
	sum == checksum
}
