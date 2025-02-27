# quill sns register-vote

Signs a ManageNeuron message to register a vote for a proposal. Registering a vote will update the ballot of the given proposal and could trigger followees to vote. When enough votes are cast or enough time passes, the proposal will either be rejected or adopted and executed.

## Basic usage

The basic syntax for running `quill sns register-vote` commands is:

```bash
quill sns register-vote <NEURON_ID> --proposal-id <PROPOSAL_ID> --vote <VOTE> [option]
```

## Arguments

| Argument      | Description                        |
|---------------|------------------------------------|
| `<NEURON_ID>` | The ID of the neuron to configure. |

## Flags

| Flag           | Description                 |
|----------------|-----------------------------|
| `-h`, `--help` | Displays usage information. |

## Options

| Option                        | Description                                |
|-------------------------------|--------------------------------------------|
| `--proposal-id <PROPOSAL_ID>` | The ID of the proposal to be voted on.     |
| `--vote <VOTE>`               | The vote to be cast on the proposal (y/n). |
