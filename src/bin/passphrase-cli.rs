
use passphrase::words::PassPhrase;
use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    // command line arg: --seed
    if env::args().len() >= 2 {
        fastrand::seed(19501103);
    }

    // command line arg: --show-indexes
    let show_indexes = false;
    
    // command line arg: --size
    let size = 12_usize;

    // command line arg: --count
    let count = 20_usize;

    let rng = fastrand::Rng::new();
    (1..=count).for_each(|x| {
        let idx_list = PassPhrase::generate_idx(&rng, size);
        if show_indexes {
            println!("{} {:?}", x, &idx_list);
        }

        let words = PassPhrase::get_words(idx_list);

        println!("{:02} {}", x, words.join("-"));
    });

    Ok(())
}

