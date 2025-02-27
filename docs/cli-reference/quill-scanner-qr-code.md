# quill scanner-qr-code

Print QR Scanner dapp QR code: scan to start dapp to submit QR results.

## Basic usage

The basic syntax for running `quill scanner-qr-code` commands is:

``` bash
quill scanner-qr-code
```

## Flags

| Flag           | Description                 |
|----------------|-----------------------------|
| `-h`, `--help` | Displays usage information. |

## Remarks

The `quill scanner-qr-code` command is used to provide an always-accessible shortcut to a QR scanner that can send the messages that Quill generates; you can also [click here to go there directly][qr]. Quill commands that sign update calls take a `--qr` parameter to output such a QR code; another tool's messages, if output in JSON format, can be turned into a QR code via [`quill qr-code`].

The QR code reader may not show field names of the response, but rather show field numbers, obscuring the meaning of the response. You can use the [didc] tool's `hash` command to convert a field name to a field number, in order to make sense of the output; for example, `didc hash 'voting_power'` produces 3871395629.

[didc]: https://github.com/dfinity/candid/releases
[qr]: https://p5deo-6aaaa-aaaab-aaaxq-cai.raw.icp0.io/
[`quill qr-code`]: quill-qr-code.md
