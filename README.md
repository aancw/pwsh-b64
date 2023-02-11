# pwsh-b64
Powershell implementation of ToBase64String UTF-16LE written in Rust

## Usage

```
❯ ./pwsh-b64-rs -h
pwsh-b64-rs 1.0
Petruknisme <me@petruknisme.com>
Powershell implementation of ToBase64String UTF-16LE written in Rust

USAGE:
pwsh_b64_rs --text <TEXT>

OPTIONS:
-h, --help           Print help information
-t, --text <TEXT>    Text to be encoded
    -V, --version        Print version information

❯ ./pwsh-b64-rs -t "hellow"
aABlAGwAbABvAHcA
```

## Motivation

Whenever i need to encode payload string with powershell base64 UTF-16LE, I don't need to turn on my windows os or use powershell again. Just run this tool and get the same encoded string like powershell did.

## License

MIT License