use gb_core::gameboy::GameBoy;
use clap::Parser;

/// A GameBoy and GameBoy color emulator
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	/// Path of the ROM
	#[arg(short, long)]
	rom: String,

	/// Type of gameboy to emulate
	#[arg(short, long, default_value = "color")]
	screen: ScreenType,

	/// Skip the boot animation
	#[arg(long)]
	skip_animation: bool
}

#[derive(Debug, PartialEq, Clone, clap::ValueEnum)]
enum	ScreenType {
	Classic,
	Color
}

fn	main()
{
	#[cfg(feature = "verbose-logs")]
	env_logger::Builder::new().parse_filters("trace").init();
	#[cfg(not(feature = "verbose-logs"))]
	env_logger::Builder::new().parse_filters("info").init();

	let	args = Args::parse();

	let mut	gb: GameBoy;
	match args.skip_animation {
		false => match args.screen {
			ScreenType::Classic => gb = GameBoy::new(false),
			ScreenType::Color => gb = GameBoy::new(true)
		},
		true => match args.screen {
			ScreenType::Classic => gb = GameBoy::new_without_boot(false),
			ScreenType::Color => gb = GameBoy::new_without_boot(true)
		}
	}

	gb.load_cartridge(&args.rom);
}
