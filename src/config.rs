use clap::Parser;

/// 2D Life simulation
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Output debug information
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
    /// Overrides both width/height if true
    #[arg(short, long, default_value_t = false)]
    pub fullscreen: bool,
    /// Window Width
    #[arg(short, long, default_value_t = 1200)]
    pub width: u32,
    /// Window Height
    #[arg(short, long, default_value_t = 900)]
    pub height: u32,
    /// Pixel size of each cell in the grid
    #[arg(short, long, default_value_t = 2)]
    pub resolution: u32,
    /// Speed factor, higher is slower
    #[arg(short, long, default_value_t = 1)]
    pub speed: u8,
}