# nearx

NEAR TX Swiss Army Knife

> Script-friendly CLI for interacting with NEAR blockchain

Features:

- Proper delineation between (results: stdout) & (error + logs: stderr).
- Structured output for easy parsing.

Implemented actions:

- [x] Mutable calls
- [x] Immutable calls
- [x] Sign and export transactions

## Usage

<details>

<summary> <code>nearx --help</code> </summary>

```console
NEAR TX Swiss Army Knife

Usage: nearx <COMMAND>

Commands:
  call  Calls a method on a contract
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

Hint:
  nearx call METHOD [with '{}'] on CONTRACT [as ACCOUNT with SECRET [gas GAS] [deposit DEPOSIT] [display]] through RPC_URL [with TOKEN]

Examples:
  # Immutably call `add(1, 2)` on `adder.testnet`
  $ nearx call "add" with "[1, 2]" on "adder.testnet" through "https://rpc.testnet.near.org"

  # Mutably call `addGreeting("Hello, World!")` on `greeter.testnet` as `bob.testnet` with `300TGas` and no deposit.
  $ nearx call "addGreeting" with '["Hello World"]' on "greeter.testnet" as "bob.testnet" with "ed25519:52CwWhWHzgaSZRx..bMFSyXn9hao4YNXuz" through "https://rpc.testnet.near.org"

  # Authenticated, immutable call to `add(1, 2)` on `adder.testnet` as `carol.testnet`
  $ nearx call "add" with "[1, 2]" on "adder.testnet" through "https://rpc.testnet.near.org" with "5a28cd2041c1780f5d64fa6dca4b22bd"

  # Read `NEAR_RPC_URL` and `NEAR_RPC_API_KEY` from the environment
  $ nearx call "add" with "[1, 2]" on "adder.testnet"

  # Display a transaction that calls `addGreeting("Hello, World!")` on `greeter.testnet` as `derek.testnet` with `100 TGas` and `5 Ⓝ` deposit.
  $ nearx call "addGreeting" with '["Hello World"]' on "greeter.testnet" as "derek.testnet" with "ed25519:52CwWhWHzgaSZRx..bMFSyXn9hao4YNXuz" gas "100Tgas" deposit "5N" display
```

</details>

### Examples

#### Immutable call

```console
$ nearx call "get" with '{"keys":["miraclx.near/**"]}' on "social.near" through "https://rpc.mainnet.near.org"
{
  "miraclx.near": {
    "profile": {
      "description": "🦀 Tech Nerd.\n\nEx-Engineering @pagodaplatform.near + @near.",
      "horizon_tnc": "true",
      "name": "Miraculous"
    }
  }
}
```

#### Execute a mutable function call

```console
$ nearx call "rate" with '{"account_id":"a.miraclx.testnet","rating":5}' on "nosedive.testnet" as "miraclx.testnet" with "$KEY" through "https://rpc.testnet.near.org"
   info │ transaction hash: FvkVeTgRsECmN7JawSqZZz4acSvq5u74DJMAovsExaQE
   info │       block hash: G5Ty46YCVojRz6wmWXXNXhFdtvrtLnLjo8GZf7yn8CcQ
   info │   execution cost: 2.4280 TGas

```

#### Prepare and export a signed transaction

```console
$ nearx call "rate" with '{"account_id":"a.miraclx.testnet","rating":5}' on "nosedive.testnet" as "miraclx.testnet" with "$KEY" display through "https://rpc.testnet.near.org"
   info │ transaction hash: FNRUdCHhVD5gv5QD3Qr9GKuDio2ebtYYEcDoW4mPycSJ
DwAAAG1pcmFjbHgudGVzdG5ldACUEocRG+w4BSVfvY29Bd6PtyDhZqsmZiWlkKdzWTWmqyc+HQnCQgAAEAAAAG5vc2VkaXZlLnRlc3RuZXSKW5lagbERyoCbxmf3HbThkMcqxHcb+FCOWynY2XptNAEAAAACBAAAAHJhdGUtAAAAeyJhY2NvdW50X2lkIjoiYS5taXJhY2x4LnRlc3RuZXQiLCJyYXRpbmciOjV9AMBuMdkQAQAAAAAAAAAAAAAAAAAAAAAAAPAnfaaeCrNnMX32mVeizLLsonqQW94lxF+XpzC2dVOLtacqrVabea58agY/O9wranWZqsLoBTNs2QM8oR7h8Q0=
```

#### Errors

```console
$ nearx call "rate" with '{"account_id":"a.miraclx.testnet"}' on "nosedive.testnet" as "miraclx.testnet" with "$KEY" through "https://rpc.testnet.near.org"
   info │ transaction hash: Dr8gnQP4RHKT6zQHW1Mi4YhF2Spy6FkfsbyaVq4wXftS
   info │       block hash: qmaQShnWzP1WhpD3nkmjtu1SG5pz6x7MAEAgeWGvtPX
   info │   execution cost: 2.4280 TGas
  error │ transaction failed: ActionError(
    ActionError {
        index: Some(
            0,
        ),
        kind: FunctionCallError(
            ExecutionError(
                "Smart contract panicked: panicked at 'Failed to deserialize input from JSON.: Error(\"missing field `rating`\", line: 1, column: 34)', src/lib.rs:83:1",
            ),
        ),
    },
)
```

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as below, without any additional terms or conditions.

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
