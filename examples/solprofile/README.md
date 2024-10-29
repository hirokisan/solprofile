# Examples

## Run on localnet

### Setup

#### Deploy Program

See [Run on localnet](https://github.com/hirokisan/solprofile?tab=readme-ov-file#run-on-localnet).

#### Copy IDL

Copy `../../target/idl/solprofile.json` to `idls/solprofile.json`.

### Create a profile

```console
$ cargo run --quiet --bin create
```

### Get a profile

```console
$ cargo run --quiet --bin get
```

Tips.

```console
$ anchor account solprofile.Profile {ADDRESS} --idl=idls/solprofile.json
```

### Update a profile

```console
$ cargo run --quiet --bin update
```
