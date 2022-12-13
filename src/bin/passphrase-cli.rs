use anyhow::Result;
use log::info;
use passphrase::words::{Config, PassPhrase};
use std::env;

fn main() -> Result<()> {
    // command line arg: --seed
    let pp = PassPhrase::new();

    if env::args().len() >= 2 {
        fastrand::seed(19501103);
    }

    // command line arg: --show-indexes
    // let show_indexes = false;

    // command line arg: --size
    // let size = 12_usize;

    // command line arg: --count
    // let count = 20_usize;

    // the default
    let config = Config::new();

    let phrases = pp.generate_list(config);
    info!("{:?}", &phrases);
    for phrase in phrases.phrase_list {
        println!("{} {:?}", &phrase.line_number, &phrase.index_list);
        println!("{} {:?}", &phrase.line_number, &phrase.word_list);
    }

    Ok(())
}
