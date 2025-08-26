# localblock

A fully custom implementation of a block-chain currency that can be hosted locally by a network administrator.

# Intention

This project is made entirely out of curiosity. While the applications are low, this could still be used to host very robust currencies controlled by a central organization.

In my effort to learn as much as possible about block-chain development, I have written almost all of the implementation from the ground up, notable exceptions include:

- ECC with p256. I am no where *near* qualified enough to implement solid cryptography by myself.

- SHA256

- The standard library

- The Tokio Runtime as a whole

- That's it! The rest is completely custom

The idea is a block-chain technology that only accepts so-called *approved nodes*. That is, this is **NOT** decentralized such as Bitcoin. This provides some interesting effects:

- An organization is solely responsible for keeping nodes secure.

- It is much harder, but still technically possible, to perform a 51% attack on the network.

> A key feature for the first full version is the ability for legitimate nodes to detect *poisoned nodes*, nodes that behave strangly because they are compromised.

# Details of the current development version

The project is structured into two crates and one library,

- `lbauthority`, a node within the network which handles mining and verification tasks

- `lbclient`, a client application which allows for wallet management and interaction with nodes in a network

- `lbcrypto`, the definitions and functions for blockchain datatypes, serialization, deserialization, and cryptography implementations.

The bulk of the technical details are in `lbcrypto`, so we will start there:

- `Block`s, `BlockHeader`s, `Transaction`s

Blocks consist of a `BlockHeader`, and a list of transactions for the current block.

The `BlockHeader` contains important information, such as the **Previous Hashed Block Header**, the **Merkle Root**, the **Nonce**, and the previous version.

- The previous hashed block header is as it sounds, details are found within `lib.rs`.

- The merkle root, just like BitCoin, is the root of a merkle tree formed from all the transactions in a block. This is the method of saving cryptographic security for transactions in the block header while maintaining constant space usage within the header.

- The nonce, changed in the process of searching for a hash that fits the current network challenge (more on that later).

Transactions consist of `TxInput` and `TxOutput` types, as well as the sender's signature and verifying key.

- `TxInput` consists of the `Txid` of a previous transaction, and the index of the output.

- `TxOutput` consists of the `HashedPublic` of the recipient, as well as an amount of currency to be sent to that address.

- `HashedPublic`, as described in the doc comment, is a twice SHA256'ed public address.

- A signature of the inputs and outputs is required, and is verified by trusted nodes with the verifying key.

Each type provides some serialization method, allowing for the type to be represented as bytes. Each of these methods also includes a method to reconstruct the type from bytes.

> Some structs, such as `Transaction`, include extra data in their serialization for the sole purpose of being deserialized. These are noted especially if the struct has a dedicated hashing impl.

> Any struct that includes a hash of some of its own contents also provide methods to verify these hashes. For example, `Transaction` and its `Txid`.

> Any struct that includes a signature includes methods to verify this signature. Yet again, `Transaction`.

Mining is currently implemented for `Block`.

## How to perform a mining test

Currently, `lbauthority` includes a test module that will hash a block until it finds a valid nonce for the challenge of 27 zero bits. Nativate to the `lbauthority` directory and run `cargo test -- --nocapture` to watch as the time exponentially increases as the challenge continues through challenges from 1-27 bits!

# Future Features

This project is far closer to being finished than not. Currently, these are the the key points that need to be finished:

The authority server, which must be able to do the following things:

- Keep a UTXO

- Keep the current longest block-chain

- Concurrently search for a new valid block, and propogate it to other nodes.

- Automatically detect *poisoned nodes* as described before.

- Accept new transactrions from the client

The client is practically finished, it must now only be able to:

- Form valid transactions to be sent to a node (already handled for the most part by `lbcrypto`)

- Get a current balance by checking against a node's UTXO.

# Conclusion

The difficult cryptography and backbone of localblock are completely finished. The remaining portions should come soon, and hopefully this will be a great learning experience!
