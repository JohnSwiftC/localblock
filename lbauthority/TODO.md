- Trait for to_bytes, i know it exists but i cannot think of the name

- Macro or crate for getting the size of a struct at compile time. I don't want a whole serde build but i need to serialize by blocks to bytes atleast. Honestly I could use serde and send shit over xml if i really wanted too

    - Specifically need to rewrite every to_bytes function in lib.rs, his a Ctrl-F

- Serialization to and from bytes for the following types. Those done are marked with an X.
    - HashedBlock X
        - bytes() X
        - from_bytes() X
        - rewrote hashing N/A
    - Block UNFINISHED
        - bytes() UNFINISHED
        - from_bytes() UNFINISHED
        - rewrote hashing UNFINISHED
    - BlockHeader X
        - bytes() X
        - from_bytes() X
        - rewrote hashing X
    - Transaction UNFINISHED
        - bytes() X
        - from_bytes() UNFINISHED
        - rewrote hashing UNFINISHED

- General improvements
    - Organization of structs and functions is annoying currently
    - Some hashed values are types, some are structs, and some are just a [u8; 32] => all structs, or atleast no raw [u8; 32]
    - Hashing functions for struct will likely reuse code once that same struct has a bytes() method. Instead of doing it again, hash the bytes
    from bytes()