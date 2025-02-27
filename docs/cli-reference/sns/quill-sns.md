# quill sns

The `quill sns` family of commands is used to interact with a Service Nervous System (SNS) canister array. For more information about SNS, see [the wiki page](https://internetcomputer.org/docs/current/tokenomics/sns/sns-intro-tokens).

`quill sns` commands use the same authentication flags as other `quill` commands, but require another `--canister-ids-file <FILE>` flag. This should contain the path to a file containing a JSON document describing the layout of an SNS. An example of such a file would be:

```json
{
    "governance_canister_id": "rrkah-fqaaa-aaaaa-aaaaq-cai",
    "ledger_canister_id": "ryjl3-tyaaa-aaaaa-aaaba-cai",
    "root_canister_id": "r7inp-6aaaa-aaaaa-aaabq-cai",
    "dapp_canister_id_list": [ "qoctq-giaaa-aaaaa-aaaea-cai" ]
}
```

## Basic usage

The basic syntax for running `quill sns` commands is:

```bash
quill sns [subcommand] [option] [flag]
```

To see the available subcommands, please refer to the [index page](../index.md) of the quill reference.
