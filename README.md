# solprofile

solprofile gives profile to the wallet on solana.

## Setup

- Install Rust
- Install Solana CLI
- Install Anchor CLI

## Run on localnet

### [Optional] Create a Keypair

If `~/.config/solana/id.json` does not exist, run.

```console
$ solana-keygen new --no-bip39-passphrase
```

### [Optional] Set Config

If not set, run.

```console
$ solana config set --keypair ~/.config/solana/id.json
$ solana config set --url localhost
```

### Run test validator

```console
$ just run-test-validator
```

### Airdrop enough SOL to pay for deploy fee

```console
$ solana airdrop 2
```

### Build

```console
$ just build
```

This will generate a keypair in `target/deploy`.

### Check Program ID

```console
$ just keys
```

Set the Program ID to `declare_id!` and `programs.localnet.solprofile` on Anchor.toml.

### Build again

```console
$ just build
```

To include the new program id in the binary.

### Deploy

```console
$ just deploy
```

### Check

https://solscan.io/account/{ProgramID}?cluster=custom&customUrl=http://localhost:8899

## Tips

### Keep keypair secret to reuse

Keep the keypair generated in `target/deploy` after `anchor build`.

### If for some reason a build doesn't pass or a test doesn't pass

```console
$ just clean
```
