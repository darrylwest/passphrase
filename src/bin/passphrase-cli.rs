//! CLI for passphrase
//!
use anyhow::Result;
use clap::Parser;
use log::info;
use passphrase::config::Config;
use passphrase::words::PassPhrase;

#[derive(Debug, Default, Parser)]
#[clap(name = "passphrase-cli")]
#[command(author, version, long_about = None)]
#[clap(about = "passphrase-cli\nGenerate one or more strong passphrases.")]
pub struct Cli {
    /// set a seed for the ring generator
    #[clap(long, value_parser)]
    pub seed: Option<usize>,

    /// show the index numbers with each phrase
    #[clap(short, long, value_parser, default_value_t = false)]
    pub show_indexes: bool,

    /// set the number of words for each phrase
    #[clap(short, long, value_parser, default_value_t = 12_u8)]
    pub phrase_words: u8,

    /// set the number of phrases to generate
    #[clap(short, long, value_parser, default_value_t = 20_usize)]
    pub lines: usize,

    /// read the gernator configuration from the specified Toml file (overrides other settings)
    #[clap(short, long, value_parser)]
    pub config_file: Option<String>,
}

impl Cli {
    pub fn new() -> Cli {
        Cli::parse()
    }
}

fn main() -> Result<()> {
    // command line arg: --seed
    let cli = Cli::new();
    info!("{:?}", cli);

    // the default
    let pp = PassPhrase::new();
    let config = if let Some(file) = cli.config_file {
        Config::read_config(&file)?
    } else {
        let name = Config::default_name();
        Config::with_values(&name, cli.seed, cli.phrase_words, cli.lines)
    };

    let phrases = pp.generate_list(config);
    info!("{:?}", &phrases);
    for phrase in phrases.phrase_list {
        let mut buf = vec![];
        if cli.show_indexes {
            if cli.lines > 1 {
                buf.push(format!("[{:02}] ", &phrase.line_number));
            }

            buf.push(format!("{:?}\n", &phrase.index_list));
        }

        if cli.lines > 1 {
            buf.push(format!("[{:02}] ", &phrase.line_number));
        }

        buf.push(format!("{:?}", &phrase.word_list.join("-")));

        println!("{}", buf.join(""));
    }

    Ok(())
}
