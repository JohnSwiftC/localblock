- Trait for to_bytes, i know it exists but i cannot think of the name

- Macro or crate for getting the size of a struct at compile time. I don't want a whole serde build but i need to serialize by blocks to bytes atleast. Honestly I could use serde and send shit over xml if i really wanted too

    - Specifically need to rewrite every to_bytes function in lib.rs, his a Ctrl-F

- Serialization to and from bytes for the following types. Those done are marked with an X.
    - HashedBlock X
        - bytes() X
        - from_bytes() X
        - rewrote hashing N/A
    - Block UNFINISHED
        - serialize() X
        - from_bytes() UNFINISHED
    - BlockHeader X
        - bytes() X
        - from_bytes() X
        - rewrote hashing X
    - Transaction X
        - bytes() N/A is now serialize(), is done X
        - from_bytes() X
        - rewrote hashing N/A doing so might cause a slowdown, lets just use bytes for serialization. Also outputs a vec because of an unknown size at compile time. Block will have to do the same

- Things an auth node needs to do

    - Accept connections with new blocks
        - Validate that the merkle root is correct
        - Validate that the BlockHeader's hash fits the required amount of 0 bits
        - Hold tie branches until a tie is broken
        - Stop the current work for a new block and start on a new one
    - Accept connections with new transactions
        - Validate the signature of the transaction
        - Validate that the txid is correct
        - Restart work on new block with new merkle root
    - Work on a new block
        - Self explanatory
    - Be able to both request and send the current full blockchain
        - Allows for servers to be on the same page, accept longest valid chain always


- General improvements
    - Organization of structs and functions is annoying currently
    - Some hashed values are types, some are structs, and some are just a [u8; 32] => all structs, or atleast no raw [u8; 32]
    - Hashing functions for struct will likely reuse code once that same struct has a bytes() method. Instead of doing it again, hash the bytes
    from bytes()