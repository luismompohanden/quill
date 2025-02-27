# quill ckbtc retrieve-btc-status

Sends a message to check the status of a ckBTC-to-BTC conversion.

This conversion can be performed with the [`quill ckbtc retrieve-btc`](./quill-ckbtc-retrieve-btc.md) command.

## Basic usage

The basic syntax for running `quill ckbtc retrieve-btc-status` commands is:

```bash
quill ckbtc retrieve-btc-status [option] <BLOCK_INDEX>
```

## Arguments

| Argument        | Description               |
|-----------------|---------------------------|
| `<BLOCK_INDEX>` | The block index to check. |

## Flags

| Flag                 | Description                                        |
|----------------------|----------------------------------------------------|
| `--dry-run`          | Will display the query, but not send it.           |
| `-h`, `--help`       | Displays usage information.                        |
| `--testnet`          | Uses ckTESTBTC instead of ckBTC.                   |
| `--yes`              | Skips confirmation and sends the message directly. |
