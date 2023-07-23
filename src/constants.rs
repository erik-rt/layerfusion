use console::Emoji;
// Engine input asset folder (layers)
pub const ASSETS_INPUT: &str = "assets";

// Engine output asset folder
pub const ASSETS_OUTPUT: &str = "outputs";

// Output metadata folder
pub const METADATA_OUTPUT: &str = "metadata";

// Fun emojis
pub const PALETTE_EMOJI: Emoji<'_, '_> = Emoji("🎨 ", "");

pub const CHECKMARK_EMOJI: Emoji<'_, '_> = Emoji("✅ ", "");

pub const ERROR_EMOJI: Emoji<'_, '_> = Emoji("❌ ", "");
