
use clap::Clap;

/// Convert picture into Emoji picture!
#[derive(Clap)]
#[clap(version = "1.0", author = "GLCraft")]
pub struct Opts {
    /// Picture path
    pub filename: String,
    /// Size of an emoji
    #[clap(short = 's', long = "size", default_value = "8")]
    pub size: u32,
    /// Emoji upscale
    #[clap(short = 'u', long = "upscale", default_value = "1")]
    pub upscale: u32,
    /// Emojis assets path
    #[clap(long = "assets", default_value = "deps/twemoji/assets/72x72")]
    pub assets_path: String,

}