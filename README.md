# QrCli

`qrcli` is a tool to generate QR codes on the command line and display
them there without the need for an X server. All error correction
modes are supported: `low`, `medium`, `quartile` and `high`.

```
$ ./qrcli-rs -h
qrcli 0.1.0
Sebastian Geisler <sgeisler@wh2.tu-dresden.de>

USAGE:
    qrcli-rs [OPTIONS] <text>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --error-correction <coding>    specifies the error correction code to use [default: high]

ARGS:
    <text>    text that will be encoded
```