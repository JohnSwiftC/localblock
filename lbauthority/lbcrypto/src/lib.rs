use sha2::{Digest, Sha256};

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
/// The representation of a block within a localblocl
/// network. Similar to bitcoin's.
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}
impl Block {
    /// Requires a mut reference becuase it will increment the nonce until a valid hash is found.
    pub async fn search_for_nonce(&mut self, n: u8) {
        loop {
            let hash = self.header.hash().await;
            if hash.has_zero_bits(n).await {
                return;
            }

            self.header.increment_nonce();
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HashedBlock {
    bytes: [u8; 32],
}

impl HashedBlock {
    pub async fn has_zero_bits(&self, n: u8) -> bool {
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

    pub fn from_bytes(bytes: &[u8]) -> Result<HashedBlock, SerialError> {
        let fixed_b: [u8; 32] = bytes.try_into().map_err(|_| SerialError::IncorrectSize {
            expected: HashedBlock::raw_size(),
            provided: bytes.len(),
        })?;

        Ok(HashedBlock { bytes: fixed_b })
    }

    /// Panics if not given the appropritate amount of bytes
    pub fn from_bytes_unchecked(bytes: &[u8]) -> HashedBlock {
        HashedBlock {
            bytes: bytes.try_into().unwrap(),
        }
    }

    pub fn bytes(&self) -> [u8; 32] {
        self.bytes
    }

    fn raw_size() -> usize {
        32
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
    pub async fn hash(&self) -> HashedBlock {
        let mut hasher = Sha256::new();
        hasher.update(self.bytes());

        let inter: HashedBlock = HashedBlock {
            bytes: hasher.finalize().into(),
        };

        // Second go around ;)

        let mut hasher = Sha256::new();
        hasher.update(&inter.bytes[..]);

        HashedBlock {
            bytes: hasher.finalize().into(),
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(BlockHeader::raw_size());
        bytes.extend_from_slice(&self.version.to_le_bytes());
        bytes.extend_from_slice(&self.previous_hash.bytes[..]);
        bytes.extend_from_slice(&self.merkle_root[..]);
        bytes.extend_from_slice(&self.nonce.to_le_bytes());

        bytes
    }

    /// Need some internet for this
    pub fn from_bytes(bytes: &[u8]) -> Result<BlockHeader, SerialError> {
        if bytes.len() != BlockHeader::raw_size() {
            return Err(SerialError::IncorrectSize {
                expected: BlockHeader::raw_size(),
                provided: bytes.len(),
            });
        }

        // Unwrapping because we know from the previous check that we will be able to create every type. This does not check to see if that
        // type really makes sense, but thats a problem for a later day
        let version: u8 = u8::from_le_bytes([bytes[0]]);
        let previous_hash: HashedBlock = HashedBlock::from_bytes_unchecked(&bytes[1..33]);
        let merkle_root: [u8; 32] = bytes[33..65].try_into().unwrap();
        let nonce: u64 = u64::from_le_bytes(bytes[65..73].try_into().unwrap());

        Ok(BlockHeader {
            version,
            previous_hash,
            merkle_root,
            nonce,
        })
    }

    pub async fn new(version: u8, prev_block_header: Option<&BlockHeader>) -> Self {
        match prev_block_header {
            Some(b) => Self {
                version,
                previous_hash: b.hash().await,
                merkle_root: [0; 32], // unitialized merkle root
                nonce: 0,
            },

            None => Self {
                version,
                previous_hash: HashedBlock { bytes: [0; 32] }, // None. Pretty much just useful for testing and the first block of a new network.
                merkle_root: [0; 32],
                nonce: 0,
            },
        }
    }

    pub async fn compute_merkle_root(&mut self, transactions: &[Transaction]) {
        let odd = {
            if transactions.len() % 2 == 0 {
                false
            } else {
                true
            }
        };

        let mut hashes: Vec<[u8; 32]> = Vec::with_capacity(transactions.len());
        for transaction in transactions {
            hashes.push(transaction.hash().await);
        }

        if odd {
            hashes.push(transactions[transactions.len() - 1].hash().await);
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

    fn raw_size() -> usize {
        73
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

use p256::ecdsa::signature::{Signer, Verifier};
use p256::elliptic_curve::rand_core::OsRng;
impl Transaction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.txid);
        bytes.extend_from_slice(&self.signature.to_bytes());
        bytes.extend_from_slice(&self.verifying_key.to_sec1_bytes());

        for input in &self.inputs {
            bytes.extend_from_slice(&input.txid);
        }

        bytes
    }

    pub async fn verify_signature(&self) -> bool {
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

        self.verifying_key
            .verify(&bytes[..], &self.signature)
            .is_ok()
    }

    pub async fn verify_txid(&self) -> bool {
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

    pub async fn hash(&self) -> [u8; 32] {
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
            inputs: vec![
                TxInput {
                    txid: [5; 32],
                    index: 9,
                },
                TxInput {
                    txid: [30; 32],
                    index: 2,
                },
            ],
            outputs: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum SerialError {
    IncorrectSize { expected: usize, provided: usize },
}

impl std::fmt::Display for SerialError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            SerialError::IncorrectSize { expected, provided } => {
                write!(f, "Expected {} bytes, got {}", expected, provided)
            }
        }
    }
}
impl std::error::Error for SerialError {}

#[cfg(test)]
mod tests {
    use super::*;
    use p256::ecdsa::signature::Signer;
    use p256::{ecdsa::Signature, elliptic_curve::rand_core::OsRng, pkcs8::PrivateKeyInfo};

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
            inputs: vec![TxInput {
                txid: [5; 32],
                index: 9,
            }],
            outputs: Vec::new(),
        });
        transactions.push(Transaction {
            txid: [6; 32],
            signature: signature,
            verifying_key: public,
            inputs: vec![
                TxInput {
                    txid: [5; 32],
                    index: 9,
                },
                TxInput {
                    txid: [30; 32],
                    index: 2,
                },
            ],
            outputs: Vec::new(),
        });
        transactions.push(Transaction {
            txid: [7; 32],
            signature: signature,
            verifying_key: public,
            inputs: vec![
                TxInput {
                    txid: [5; 32],
                    index: 9,
                },
                TxInput {
                    txid: [30; 32],
                    index: 2,
                },
            ],
            outputs: Vec::new(),
        });

        let mut transactions2 = Vec::new();
        transactions2.push(Transaction {
            txid: [2; 32],
            signature: signature,
            verifying_key: public,
            inputs: vec![TxInput {
                txid: [5; 32],
                index: 9,
            }],
            outputs: Vec::new(),
        });
        transactions2.push(Transaction {
            txid: [6; 32],
            signature: signature,
            verifying_key: public,
            inputs: vec![
                TxInput {
                    txid: [5; 32],
                    index: 9,
                },
                TxInput {
                    txid: [30; 32],
                    index: 2,
                },
            ],
            outputs: Vec::new(),
        });
        transactions2.push(Transaction {
            txid: [7; 32],
            signature: signature,
            verifying_key: public,
            inputs: vec![
                TxInput {
                    txid: [5; 32],
                    index: 9,
                },
                TxInput {
                    txid: [30; 32],
                    index: 2,
                },
            ],
            outputs: Vec::new(),
        });
        transactions2.push(Transaction {
            txid: [7; 32],
            signature: signature,
            verifying_key: public,
            inputs: vec![
                TxInput {
                    txid: [5; 32],
                    index: 13,
                },
                TxInput {
                    txid: [30; 32],
                    index: 2,
                },
            ],
            outputs: Vec::new(),
        });

        let mut b1 = BlockHeader::new(1, None);
        let mut b2 = BlockHeader::new(1, None);

        b1.compute_merkle_root(&transactions);
        b2.compute_merkle_root(&transactions2);

        assert_ne!(b1, b2);
    }
}
