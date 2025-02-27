# quill claim-neurons

Claim seed neurons from the Genesis Token Canister.

## Basic usage

The basic syntax for running `quill claim-neurons` commands is:

``` bash
quill claim-neurons [flag]
```

## Flags

| Flag           | Description                 |
|----------------|-----------------------------|
| `-h`, `--help` | Displays usage information. |

## Examples

The `quill claim-neurons` command is used to claim rewards for the original Genesis stakers. If your twelve-word phrase is stored in the file `seed.txt`, you would say:

```sh
quill claim-neurons --seed-file seed.txt
```

This request would generate a response like:

```candid
(variant { Ok = vec { 2313380519530470538 : nat64, 4966884161088437903 : nat64 } })
```

Each number in the `vec` is the ID of one of your neurons.

## Remarks

As this is an update call, it will not actually make the request, but rather generate a signed and packaged request that can be sent from anywhere. You can use the `--qr` flag to display it as a QR code, or if you are not working with an air-gapped machine, you can pipe it to `quill send -`.

For more information about genesis rewards, see the [How-To guide]. For more information about neurons, see [Neurons]. For creating non-genesis neurons, consult the documentation for [`quill neuron-stake`].

[How-To guide]: https://wiki.internetcomputer.org/wiki/How-To:_Claim_neurons_for_seed_participants
[Neurons]: https://internetcomputer.org/docs/current/tokenomics/nns/nns-intro#neurons
[`quill neuron-stake`]: quill-neuron-stake.md
