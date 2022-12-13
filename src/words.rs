
use log::info;
// use serde::
use crate::{BIP39_WORDS, BIP39_LENGTH};

#[derive(Debug, Default, Clone)]
pub struct Phrases {
    pub seed: Option<usize>,
    pub index_list: Vec<usize>,
    pub phrase_list: Vec<String>,
    pub limit: usize,
    pub offset: usize,
}

impl Phrases {
    pub fn new() -> Phrases {
        Phrases {
            seed: None,
            index_list: vec![],
            phrase_list: vec![],
            limit: BIP39_LENGTH,
            offset: 0_usize,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct PassPhrase {
}

impl PassPhrase {
    pub fn new() -> PassPhrase {
        PassPhrase {
        }
    }

    pub fn generate_list(&self, seed: Option<usize>, offset: usize, limit: usize) -> Phrases {
        let limit = if limit == 0 || limit > BIP39_LENGTH {
            BIP39_LENGTH
        } else {
            limit
        };

        info!("generate list with seed: {:?}, offset: {}, limit: {}", seed, offset, limit);
        let mut ilist = vec![];
        let mut plist = vec![];

        Phrases { seed, index_list: ilist, phrase_list: plist, limit, offset }
    }

    pub fn generate_idx(&self, rng: &fastrand::Rng, len: usize) -> Vec<usize> {
        (0..len).map(|_| rng.usize(..BIP39_LENGTH )).collect()
    }

    pub fn get_words(&self, nlist: Vec<usize>) -> Vec<String> {
        nlist.iter().map(|n| BIP39_WORDS.lines().nth(*n).unwrap().to_string() ).collect()
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_phrases() {
        let pp = Phrases::new();
        assert_eq!(pp.seed, None);
    }
}