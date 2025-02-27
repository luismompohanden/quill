# quill send

Sends a signed message or a set of messages to the IC.

## Basic usage

The basic syntax for running `quill send` commands is:

``` bash
quill send [option] <file name>
```

## Arguments

| Argument      | Description                                   |
|---------------|-----------------------------------------------|
| `<file name>` | Path to the signed message, or `-` for stdin. |

## Flags

| Flag           | Description                                        |
|----------------|----------------------------------------------------|
| `--dry-run`    | Will display the signed message, but not send it.  |
| `-h`, `--help` | Displays usage information.                        |
| `-y`, `--yes`  | Skips confirmation and sends the message directly. |

## Examples

The `quill send` command is used to send messages previously generated and signed by another quill command. This is most frequently done to make use of quill on a non-airgapped machine. 

For example, to send a transfer of 5 ICP to the anonymous principal `2vxsx-fae`:

```sh
quill transfer 1c7a48ba6a562aa9eaa2481a9049cdf0433b9738c992d698c31d8abf89cadc79 --amount 5 > transfer.json
quill send transfer.json
```

If the provided filename is `-`, the message can be piped in from stdin. The above pair of commands can be shortened to:

```sh
quill transfer 1c7a48ba6a562aa9eaa2481a9049cdf0433b9738c992d698c31d8abf89cadc79 --amount 5 > transfer.json | quill send --yes -
```

## Remarks

As `quill send` is so frequently piped to, `<file name>` will be inferred to be `-` if another command is being piped to it.
