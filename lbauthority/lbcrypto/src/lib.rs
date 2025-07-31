use sha2::{Sha256, Digest};

/// Double hash of a transaction to be used as the id
type Txid = [u8; 32];

/// Double hash of a public wallet address to signify a recipient.
/// Important to note that because a wallers value is actually held in
/// unspent outputs, it does not need to be unhashed. Anyone can calculate
/// their own wallets value by double hashing their address
/// and searching for it in the UTXO
type HashedPublic = [u8; 32];

#[derive(Debug, PartialEq, Eq)]
pub struct HashedBlock {
    pub bytes: [u8; 32],
}

impl HashedBlock {
    pub fn has_zero_bits(&self, n: u8) -> bool {
        let mut count: u8 = 0;

        for &b in &self.bytes {
            let zeros: u8 = b.leading_zeros() as u8;
            count += zeros;
            if zeros < 8 || count == n {
                break;
            }
        }
        
        count >= n
    }
}

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

/// The representation of a block within a localblocl
/// network. Similar to bitcoin's.
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    /// Requires a mut reference becuase it will increment the nonce until a valid hash is found.
    pub async fn search_for_nonce(&mut self, n: u8) {

        let mut attempt = 0;
        loop {
            let hash = self.header.hash();
            if hash.has_zero_bits(n) {
                return;
            }

            attempt += 1;
            self.header.increment_nonce();
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlockHeader {
    version: u8,
    previous_hash: HashedBlock,
    merkle_root: [u8; 32],
    nonce: u64,
}

impl BlockHeader {

    pub fn hash(&self) -> HashedBlock {
        let mut hasher = Sha256::new();
        hasher.update(&self.version.to_le_bytes());
        hasher.update(&self.previous_hash.bytes[..]);
        hasher.update(&self.merkle_root[..]);
        hasher.update(&self.nonce.to_le_bytes());

        let inter: HashedBlock = HashedBlock { bytes: hasher.finalize().into() };

        // Second go around ;)

        let mut hasher = Sha256::new();
        hasher.update(&inter.bytes[..]);
        
        HashedBlock {
            bytes: hasher.finalize().into()
        }
    
    }  

    pub fn new(version: u8, prev_block_header: Option<&BlockHeader>) -> Self {
        match prev_block_header {
            Some(b) => Self {
                version,
                previous_hash: b.hash(),
                merkle_root: [0; 32], // unitialized merkle root
                nonce: 0,
            },

            None => Self {
                version,
                previous_hash: HashedBlock { bytes: [0; 32] }, // None. Pretty much just useful for testing and the first block of a new network.
                merkle_root: [0; 32],
                nonce: 0,
            }
        }
    }

    pub fn compute_merkle_root(&mut self, transactions: &[Transaction]) {

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

    pub fn increment_nonce(&mut self) {
        self.nonce += 1;
    }

    pub fn set_nonce(&mut self, nonce: u64) {
        self.nonce = nonce;
    }
}

use p256::ecdsa::{Signature, VerifyingKey};
pub struct Transaction {
    txid: Txid,
    signature: Signature, // is signature on the inputs and outputs from the sender
    verifying_key: VerifyingKey,
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
}

use p256::ecdsa::signature::{Verifier, Signer};
use p256::elliptic_curve::rand_core::OsRng;
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

    pub fn test_dummy() -> Self {
        let private = p256::ecdsa::SigningKey::random(&mut OsRng);
        let public = private.verifying_key().to_owned();
        let message = b"Some bullshit";
        let signature = private.sign(message);

        Self {
            txid: [15; 32],
            signature,
            verifying_key: public,
            inputs: vec![TxInput { txid: [5; 32], index: 9 }, TxInput { txid: [30; 32], index: 2 }],
            outputs: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use p256::{ecdsa::Signature, elliptic_curve::rand_core::OsRng, pkcs8::PrivateKeyInfo};
    use p256::ecdsa::signature::Signer;

    #[test]
    fn merkle() {
        let private = p256::ecdsa::SigningKey::random(&mut OsRng);
        let public = private.verifying_key().to_owned();
        let message = b"Some bullshit";
        let signature = private.sign(message);

        let mut transactions = Vec::new();
        transactions.push(Transaction {
            txid: [2; 32],
            signature: signature,
            verifying_key: public,
            inputs: vec![TxInput { txid: [5; 32], index: 9 }],
            outputs: Vec::new(),
        });
        transactions.push(Transaction {
            txid: [6; 32],
            signature: signature,
            verifying_key: public,
            inputs: vec![TxInput { txid: [5; 32], index: 9 }, TxInput { txid: [30; 32], index: 2 }],
            outputs: Vec::new(),
        });
        transactions.push(Transaction {
            txid: [7; 32],
            signature: signature,
            verifying_key: public,
            inputs: vec![TxInput { txid: [5; 32], index: 9 }, TxInput { txid: [30; 32], index: 2 }],
            outputs: Vec::new(),
        });


        let mut transactions2 = Vec::new();
        transactions2.push(Transaction {
            txid: [2; 32],
            signature: signature,
            verifying_key: public,
            inputs: vec![TxInput { txid: [5; 32], index: 9 }],
            outputs: Vec::new(),
        });
        transactions2.push(Transaction {
            txid: [6; 32],
            signature: signature,
            verifying_key: public,
            inputs: vec![TxInput { txid: [5; 32], index: 9 }, TxInput { txid: [30; 32], index: 2 }],
            outputs: Vec::new(),
        });
        transactions2.push(Transaction {
            txid: [7; 32],
            signature: signature,
            verifying_key: public,
            inputs: vec![TxInput { txid: [5; 32], index: 9 }, TxInput { txid: [30; 32], index: 2 }],
            outputs: Vec::new(),
        });
        transactions2.push(Transaction {
            txid: [7; 32],
            signature: signature,
            verifying_key: public,
            inputs: vec![TxInput { txid: [5; 32], index: 13 }, TxInput { txid: [30; 32], index: 2 }],
            outputs: Vec::new(),
        });

        let mut b1 = BlockHeader::new(1, None);
        let mut b2 = BlockHeader::new(1, None);

        b1.compute_merkle_root(&transactions);
        b2.compute_merkle_root(&transactions2);

        assert_ne!(b1, b2);
    }
}