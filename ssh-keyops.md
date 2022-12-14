# Ed 25519 Keys

## Create and Maintain ed25519 key pair

* create a new strong passphrase: `passphrase-cli [options]`
* copy and save the passphrase in a safe location: `print <key> | pbcopy`
* create the new key: `ssh-keygen -a 250 -t ed25519 -f ~/.ssh/ed25519`
* paste in the passphrase (or leave blank while testing)

## Start the ssh-agent

* eval $(ssh-agent) (add this to .zshrc)
* `ssh-add ~/.ssh/ed25519`
* run `ssh-add -l` to see if it took

## Add to remote machines

* copy the new public key to remote .ssh/authorized_keys
* or, use ssh-copy command
* test the connection to remote machine (ssh-agent must be running)

## Change a PassPhrase

You should rotate the passphrase often, at least once per month especially if you are on the road...

* generate a new passphrase: `passphrase-cli [options]
* get a copy of your current passphrase: `print <key> | pbcopy`
* run this `ssh-keygen -f ~/.ssh/ed25519 -t ed25519 -p`
* paste in the current password
* copy/paste in the new password (twice)

_you may have to re-add this to the ssh-agent after changing passwords_

###### darryl.west | 2022-12-14
