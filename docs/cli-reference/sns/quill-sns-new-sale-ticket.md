# quill sns new-sale-ticket

Attempt to create a new sale ticket. If there is already an open ticket, it will return the details of the existing ticket.


## Basic usage

The basic syntax for running `quill sns new-sale-ticket` commands is:

```bash
quill sns new-sale-ticket --amount-icp-e8s <AMOUNT_ICP_E8S> [option]
```

## Flags

| Flag            | Description                 |
|-----------------|-----------------------------|
| `-h`, `--help`  | Displays usage information. |

## Options

| Option                      | Description                                               |
|-----------------------------|-----------------------------------------------------------|
| `--amount-icp-e8s <AMOUNT>` | The amount of ICP tokens to participate in this sns sale. |
| `--subaccount <SUBACCOUNT>` | The subaccount you will use to pay for this ticket.       |
