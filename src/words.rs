
use crate::BIP39_WORDS;

pub struct PassPhrase {
}

impl PassPhrase {
    pub fn generate_idx(rng: &fastrand::Rng, len: usize) -> Vec<usize> {
        (0..len).map(|_| rng.usize(..2048) ).collect()
    }

    pub fn get_words(nlist: Vec<usize>) -> Vec<String> {
        nlist.iter().map(|n| BIP39_WORDS.lines().nth(*n).unwrap().to_string() ).collect()
    }


}
