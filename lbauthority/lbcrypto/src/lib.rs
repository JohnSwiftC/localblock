use sha2::{Sha256, Digest};

/// The representation of a block within a localblocl
/// network. Similar to bitcoin's.
pub struct Block {

}

type Txid = [u8; 32]; // Is just a sha256 hash
type HashedPublic = [u8; 32];

/// represents an input in a transaction
/// references a previous transaction, and the index of the specific
/// output that is trying to be spent
struct TxInput {
    txid: Txid,
    index: u8,
}

struct TxOutput {
    recip: HashedPublic,
    amount: u32,
}
pub struct Transaction {
    txid: Txid,
    inputs: Vec<TxInput>,
    output: Vec<TxOutput>,
}


#[cfg(test)]
mod tests {
    use super::*;
}
