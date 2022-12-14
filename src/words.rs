/// words library to support passphrase cli.
///
use crate::config::Config;
use crate::{BIP39_LENGTH, BIP39_WORDS};
use log::info;

#[derive(Debug, Default, Clone)]
pub struct Phrase {
    pub line_number: usize,
    pub index_list: Vec<usize>,
    pub word_list: Vec<String>,
}

/// a collection of generated phrases
#[derive(Debug, Default, Clone)]
pub struct Phrases {
    pub config: Config,
    pub phrase_list: Vec<Phrase>,
}

impl Phrases {
    /// create a new Phrases struct with default values
    pub fn new(config: Config, phrase_list: Vec<Phrase>) -> Phrases {
        Phrases {
            config,
            phrase_list,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct PassPhrase {}

impl PassPhrase {
    pub fn new() -> PassPhrase {
        PassPhrase {}
    }

    /// generate a list of words following config rules for seed, words-per-line-count, offset and limit
    pub fn generate_list(&self, config: Config) -> Phrases {
        info!("generate list with config: {:?}", &config);

        let rng = match config.seed {
            Some(seed) => fastrand::Rng::with_seed(seed as u64),
            _ => fastrand::Rng::new(),
        };
        let len = config.limit;

        let mut phrases = Vec::with_capacity(len);

        (1..=len).for_each(|n| {
            let index_list = self.generate_idx(&rng, config.word_count);
            let word_list = self.get_words(index_list.clone());

            phrases.push(Phrase {
                line_number: n,
                index_list,
                word_list,
            });
        });
        // do a skip for the offset

        Phrases {
            config,
            phrase_list: phrases,
        }
    }

    /// generate a single phrase/line of word_count words
    pub fn generate_idx(&self, rng: &fastrand::Rng, word_count: u8) -> Vec<usize> {
        (0..word_count).map(|_| rng.usize(..BIP39_LENGTH)).collect()
    }

    /// fetch the words from the source using the random list
    pub fn get_words(&self, nlist: Vec<usize>) -> Vec<String> {
        nlist
            .iter()
            .map(|n| BIP39_WORDS.lines().nth(*n).unwrap().to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_list() {
        let config = Config::new();
        let pp = PassPhrase::new();
        let p = pp.generate_list(config.clone());

        println!("{:?}", p);

        assert_eq!(p.config, config);
    }

    #[test]
    fn new_phrases() {
        let config = Config::new();
        let list: Vec<Phrase> = vec![];
        let p = Phrases::new(config, list);
        assert_eq!(p.config.seed, None);
    }

    #[test]
    fn new_config() {
        let config = Config::new();
        assert_eq!(config.seed, None);
        assert_eq!(config.word_count, 12);
        assert_eq!(config.limit, 20);
    }
}
