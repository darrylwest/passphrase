# PassPhrase

A library and CLI that generates BIP39 and other dictionary based random words used as strong passphrases.

```bash
 ______                     ______ __                           
|   __ \.---.-.-----.-----.|   __ \  |--.----.---.-.-----.-----.
|    __/|  _  |__ --|__ --||    __/     |   _|  _  |__ --|  -__|
|___|   |___._|_____|_____||___|  |__|__|__| |___._|_____|_____|

```

### Overview

Generate one or more lines of random passphrase words from BIP39 word list.  The CLI supports controlling the number of lines to output, the number of words per phrase and other attributes.

Example Output:

```bash
[01] "chat-problem-begin-immune-thought-install-nerve-hen-they-call-receive-brisk"
[02] "inject-cost-broken-clerk-fan-slim-actual-clump-special-time-garlic-priority"
[03] "acquire-afraid-erosion-clever-mammal-focus-define-awkward-yellow-quote-glue-mimic"
[04] "lobster-hood-shove-photo-clean-trap-shiver-escape-base-title-jaguar-welcome"
[05] "session-art-weather-thought-name-vapor-edge-alarm-priority-upon-panther-gun"
[06] "quit-boring-organ-stamp-sugar-level-elephant-update-simple-slab-chest-swamp"
[07] "correct-hour-blood-nerve-pair-culture-captain-resemble-tiger-response-any-keep"
[08] "spend-mask-change-record-pulse-mansion-pitch-obey-round-merge-tree-vintage"
[09] "short-obvious-step-sure-sun-explain-primary-describe-report-vocal-soon-fetch"
[10] "unable-ensure-cable-sausage-fitness-ginger-betray-ostrich-crater-raven-despair-emerge"
[11] "shop-major-kiwi-flat-pledge-prison-mountain-cereal-strike-hammer-chef-client"
[12] "sun-bind-cross-meat-debate-deposit-essay-fun-chapter-join-turkey-tone"
[13] "card-banner-hockey-vanish-grab-drive-bronze-logic-jar-symbol-sheriff-scare"
[14] "sphere-brick-cave-toddler-voice-pony-maple-off-job-witness-pass-fashion"
[15] "humble-cart-misery-claw-version-afraid-dream-guess-ketchup-slight-online-core"
[16] "worth-inherit-brother-good-room-differ-rapid-derive-blur-girl-hat-glimpse"
[17] "ribbon-spike-whisper-rookie-cherry-whip-educate-session-panel-ski-decide-swarm"
[18] "clean-antenna-tonight-subway-street-opinion-link-step-spot-carry-multiply-wool"
[19] "funny-plug-cry-connect-tribe-invest-tank-tank-seven-forget-slight-often"
[20] "round-ozone-purse-solution-belt-find-start-consider-slim-gas-fantasy-cycle"

```

### CLI

Usage:

```bash
passphrase --help

passphrase-cli
Generate one or more strong passphrases.

Usage: passphrase-cli [OPTIONS]

Options:
      --seed <SEED>                  set a seed for the ring generator
  -s, --show-indexes                 show the index numbers with each phrase
  -p, --phrase-words <PHRASE_WORDS>  set the number of words for each phrase [default: 12]
  -l, --lines <LINES>                set the number of phrases to generate [default: 20]
  -c, --config-file <CONFIG_FILE>    read the gernator configuration from the specified Toml file
  -h, --help                         Print help information
  -V, --version                      Print version information
```

_as of version 0.5.1_

### Lib Use

_TBD_

###### darryl.west | 2022.12.13
