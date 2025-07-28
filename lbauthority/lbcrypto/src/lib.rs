use sha2::{Sha256, Digest};

/// The representation of a block within a localblocl
/// network. Similar to bitcoin's.
pub struct Block {

}

/// Double hash of a transaction to be used as the id
type Txid = [u8; 32];

/// Double hash of a public wallet address to signify a recipient.
/// Important to note that because a wallers value is actually held in
/// unspent outputs, it does not need to be unhashed. Anyone can calculate
/// their own wallets value by double hashing their address
/// and searching for it in the UTXO
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
