# quill

The `quill` is a minimalistic ledger and governance toolkit for cold wallets.

You can use the `quill` parent command with different flags and subcommands to perform different types of operations.

## Basic usage

The basic syntax for running `quill` commands is:

``` bash
quill [option] [subcommand] [flag]
```

To see the available subcommands, please refer to the [index page](index.md) of the quill reference.

## Flags

You can use the following optional flags with the `quill` parent command or with any of the `quill` subcommands.

| Flag                        | Description                                     |
|-----------------------------|-------------------------------------------------|
| `-h`, `--help`              | Displays usage information.                     |
| `--hsm`                     | Enables HSM functionality.                      |
| `--insecure-local-dev-mode` | Enter local testing mode.                       |
| `--qr`                      | Output the result(s) as UTF-8 QR codes.         |
| `-V`, `--version`           | Displays version information.                   |

## Options

You can use the following options with the `quill` command.

| Option                        | Description                                 |
|-------------------------------|---------------------------------------------|
| `--hsm-id <HSM_ID>`           | Specifies the HSM key identifier.           |
| `--hsm-libpath <HSM_LIBPATH>` | Specifies the path to the HSM library.      |
| `--hsm-slot <HSM_SLOT>`       | Specifies the HSM slot to use.              |
| `--pem-file <PEM_FILE>`       | Path to your PEM file (use "-" for STDIN).  |
| `--seed-file <SEED_FILE>`     | Path to your seed file (use "-" for STDIN). |

## Examples

Quill authentication can be performed either via a key file, or a hardware key. A principal is controlled by exactly one key; don't lose it! 

See [`quill generate`] to generate a new key file, though Quill should be compatible with any secp256k1 (aka K-256) key in the SEC1 format or ed25519 key in the PKCS#8 format. To authenticate using the key `identity.pem`:

```sh
quill list-neurons --pem-file identity.pem
```

Some commands that do not require your key will still be more useful with it; for example, `quill account-balance` doesn't require authentication, but providing your key prevents you from having to provide your principal or account ID:

```sh
quill account-balance --pem-file identity.pem
```

Quill can also be used with a seed phrase directly, though using [`quill generate`] to convert it into a private key should be preferred instead. To authenticate using a `seed.txt` file containing your seed phrase:

```sh
quill list-neurons --seed-file seed.txt
```

Both of these files can be specified to come from stdin:

```sh
cat identity.pem | quill list-neurons --pem-file -
```

Quill can also sign transactions using a hardware key (HSM) such as Nitrokey or Yubikey. It will need to have been configured beforehand with a secp256r1 (aka P-256) key, and you will need OpenSC or an equivalent installed. Assuming the HSM is in slot 0 (`pkcs11-tool --list-slots`), and you are signing with the first key it holds, such a signing command might look like:

```sh
quill list-neurons --hsm-slot 0 --hsm-id 01
```

Other PKCS#11 modules than OpenSC can be used as well. For example, to make use of Yubikey slots other than its default four, you would need `yubico-piv-tool`. A command to make use of this might look like:

```sh
quill list-neurons --hsm-slot 0 --hsm-id 05 --hsm-libpath /usr/local/lib/libykcs11.so
```

## Remarks

HSM commands ask for your PIN interactively, and for security cannot be piped. To use them in a script, you can instead pass the PIN via the `QUILL_HSM_PIN` environment variable. The other three flags can also be specified via `QUILL_HSM_SLOT`, `QUILL_HSM_LIBPATH`, and `QUILL_HSM_ID`.

Quill will by default use the well-known public key of the Internet Computer. However, for local development of canisters, Quill, or the IC, you may want to run it against a local replica. In such a case, you can use the `--insecure-local-dev-mode` flag to fetch the root key and trust it. Never use this flag if attempting a real transaction; never use this flag with your real keys. The URL that sent messages go to can be set via the `IC_URL` environment variable.

For a description of `--qr`, see [`quill qr-code`].

[`quill generate`]: quill-generate.md
[`quill qr-code`]: quill-qr-code.md
