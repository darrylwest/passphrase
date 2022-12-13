/// generate a list of random passphrase words

pub mod words;

/// the current app version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// bitcoin improvement proposal # 39 list of curated words for passphrases
pub const BIP39_WORDS: &str = include_str!("../assets/bip39-english.txt");
