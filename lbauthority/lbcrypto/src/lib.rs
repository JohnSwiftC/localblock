use sha2::{Sha256, Digest};

/// The representation of a block within a localblocl
/// network. Similar to bitcoin's.
pub struct Block {
    header: BlockHeader,
    transactions: Vec<Transaction>,
}

pub struct BlockHeader {
    version: u8,
    merkle_root: [u8; 32],
}

impl BlockHeader {
    pub fn compute_merkle_root(&mut self, mut transactions: &[Transaction]) {

        let odd = {
            if transactions.len() % 2 == 0 {
                false
            } else {
                true
            }
        };

        let mut hashes: Vec<[u8; 32]> = Vec::with_capacity(transactions.len());
        for transaction in transactions {
            hashes.push(transaction.hash());
        }

        if odd {
            hashes.push(transactions[transactions.len() - 1].hash());
        }

        let mut steps = hashes.len();
        while steps > 1 {
            let mut i = 0;
            for k in 0..steps / 2 {
                let mut hasher = Sha256::new();
                hasher.update(hashes[i]);
                hasher.update(hashes[i + 1]);
                hashes[k] = hasher.finalize().into();
                i += 2;
            }

            steps /= 2;
        }

        self.merkle_root = hashes[0];
    }
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

use p256::ecdsa::{Signature, VerifyingKey};
pub struct Transaction {
    txid: Txid,
    signature: Signature, // is signature on the inputs and outputs from the sender
    verifying_key: VerifyingKey,
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
}

use p256::ecdsa::signature::Verifier;
impl Transaction {
    pub fn verify_signature(&self) -> bool {
        // Get size of buffer needed to match against
        let byte_count = self.inputs.len() * 33 + self.outputs.len() * 36;
        let mut bytes: Vec<u8> = Vec::with_capacity(byte_count);

        for input in &self.inputs {
            bytes.extend_from_slice(&input.txid[..]);
            bytes.extend_from_slice(&input.index.to_le_bytes()[..]);
        }

        for output in &self.outputs {
            bytes.extend_from_slice(&output.recip[..]);
            bytes.extend_from_slice(&output.amount.to_le_bytes()[..]);
        }

        self.verifying_key.verify(&bytes[..], &self.signature).is_ok()
    }

    pub fn verify_txid(&self) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(self.signature.to_bytes());
        hasher.update(self.verifying_key.to_sec1_bytes());

        for input in &self.inputs {
            hasher.update(&input.txid[..]);
            hasher.update(&input.index.to_le_bytes());
        }

        for output in &self.outputs {
            hasher.update(&output.recip[..]);
            hasher.update(&output.amount.to_le_bytes()[..]);
        }

        let hash: [u8; 32] = hasher.finalize().into();
        &hash[..] == &self.txid[..]
    }

    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&self.txid);
        hasher.update(&self.signature.to_bytes()[..]);
        hasher.update(&self.verifying_key.to_sec1_bytes()[..]);


        for input in &self.inputs {
            hasher.update(&input.txid[..]);
            hasher.update(&input.index.to_le_bytes());
        }

        for output in &self.outputs {
            hasher.update(&output.recip[..]);
            hasher.update(&output.amount.to_le_bytes()[..]);
        }

        let hash: [u8; 32] = hasher.finalize().into();
        hash
    }
}