# patchy
patch substrate / polkadot

alternatives: diener - That's propper because it uses cargo metadata but doesn't work if cargo metadata doesn't work.

It will alter the Cargo.toml in the current dir.

It assumes that you have a dir structure:

   * parent dir /
      * substrate /
      * polkadot /
      * cumulus /

you run `patchy` in the cumulus and polkadot dirs to patch those branches.

## licensing
unlicense
