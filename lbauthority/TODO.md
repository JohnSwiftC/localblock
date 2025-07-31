- Trait for to_bytes, i know it exists but i cannot think of the name

- Macro or crate for getting the size of a struct at compile time. I don't want a whole serde build but i need to serialize by blocks to bytes atleast. Honestly I could use serde and send shit over xml if i really wanted too

    - Specifically need to rewrite every to_bytes function in lib.rs, his a Ctrl-F