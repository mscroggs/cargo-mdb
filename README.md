# cargo-mdb

Cargo extension for running the [mdb](https://github.com/TomMelt/mdb) debugger.

## Installing

The latest release of cargo-mdb can be installed by running:

```bash
cargo install cargo-mdb
```

## Runnning

Once installed the mdb debugger can be started by running:

```bash
cargo mdb -n 4 --example example_name
```

The `mdb attach` command that then be used to attach the debugger to another
terminal window will be output in terminal.

