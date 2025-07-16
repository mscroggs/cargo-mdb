# cargo-mdb

Cargo extension for running the [mdb](https://github.com/TomMelt/mdb) debugger.

## Installing

The latest development version of cargo-mdb can be installed by running:

```bash
git clone https://github.com/mscroggs/cargo-mdb.git
cd cargo-mdb
cargo install --path .
```

## Runnning

Once installed the mdb debugger can be started by running:

```bash
cargo mdb -n 4 --example example_name
```

The `mdb attach` command that then be used to attach the debugger to another
terminal window will be output in terminal.

