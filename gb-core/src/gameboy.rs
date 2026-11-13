use crate::cpu::Cpu;
use crate::bus::Bus;
use crate::memory::Memory;
use crate::cartridge::Cartridge;

pub struct	GameBoy {
	cpu: Cpu,
	memory: Memory,
	bus: Bus,
	cart: Cartridge
}
impl	GameBoy {
	pub fn	new(_is_color: bool) -> GameBoy
	{
		Self {
			cpu: Cpu::new(),
			memory: Memory::new(),
			bus: Bus::new(),
			cart: Cartridge::new()
		}
	}

	pub fn	new_without_boot(_is_color: bool) -> GameBoy
	{
		Self {
			cpu: Cpu::new_without_boot(),
			memory: Memory::new(),
			bus: Bus::new(),
			cart: Cartridge::new()
		}
	}

	pub fn	load_cartridge(&mut self, path: &String)
	{
		self.cart.load(path);
	}

	pub fn	display_reg(&self)
	{
		self.cpu.display_registers(true);
	}
}
