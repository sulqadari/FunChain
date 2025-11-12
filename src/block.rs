use bincode;
use serde::{Deserialize, Serialize};
use sled::IVec;

use crate::proof_of_work::ProofOfWork;
use crate::transaction::Transaction;

#[derive(Clone, Deserialize, Serialize)]
pub struct Block {
	timestamp:      i64,			// block creation time
	pre_block_hash: String,			// the hash of the previous block
	hash:           String,			// the hash of the current block
	transactions: Vec<Transaction>,	// the list of transactions in this block
	nonce:          i64,			// used in block mining
	height:         usize			// number of previous blocks
}

impl Block {
	/// Creates a new block object for a blockchain.
	pub fn new_block(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Block {
		let mut block: Block = Block {
			timestamp: crate::current_timestamp(),
			pre_block_hash,
			hash: String::new(),
			transactions: transactions.to_vec(),
			nonce: 0,
			height
		};
	
		let pow = ProofOfWork::new_proof_of_work(block.clone());
		let (nonce, hash) = pow.run();
		block.nonce = nonce;
		block.hash = hash;	
		return block;
	}

	/// Takes a slice of bytes and returns a Block object that has been deserialized
	/// from the binary data.
	pub fn deserialize(bytes: &[u8]) -> Block {
		return bincode::deserialize(bytes).unwrap();
	}

	/// Takes a reference to a Block object and returns a vector of bytes that represents
	/// the object in a serialized binary format.
	pub fn serialize(&self) -> Vec<u8> {
		return bincode::serialize(self).unwrap().to_vec();
	}

	/// Returns a borrowed reference to a slice of Transaction objects
	pub fn get_transaction(&self) -> &[Transaction] {
		return self.transactions.as_slice();
	}

	/// Returns a cloned copy of the 'pre_block_hash' string.
	pub fn get_pre_block_hash(&self) -> String {
		return self.pre_block_hash.clone();
	}

	/// returns the hash of the current block as a primitive immutable string.
	pub fn get_hash(&self) -> &str {
		return self.hash.as_str();
	}

	/// Returns a vector of bytes
	pub fn get_hash_bytes(&self) -> Vec<u8> {
		return self.hash.as_bytes().to_vec();
	}

	/// Returns the timestamp value held within the struct instance as an i64 type.
	pub fn get_timestamp(&self) -> i64 {
		return self.timestamp;
	}

	/// Returns the height value held within the struct instance as an usize type.
	pub fn get_height(&self) -> usize {
		return self.height;
	}

	/// hashes the collection on transactions by means of SHA256 hash algorithm.
	pub fn hash_transactions(&self) -> Vec<u8> {
		let mut txhash = vec![];
		for transaction in &self.transactions {
			txhash.extend(transaction.get_id());
		}
		return crate::sha256_digest(txhash.as_slice());
	}

	/// Generates the genesis block
	pub fn generate_genesis_block(transaction: &Transaction) -> Block {
		let transactions = vec![transaction.clone()];
		return Block::new_block(String::from("None"), &transactions, 0);
	}
}

/// An implementation of the 'From' trait for a 'Block' struct and an IVec struct.
/// the 'From' trait allows us to create an instance of one type from an instance
/// of another type.
impl From<Block> for IVec {
	fn from(b: Block) -> Self {
		let bytes = bincode::serialize(&b).unwrap();	// serialize the Block instance into a byte array
		return Self::from(bytes);						// Convert the byte array into an instance of IVec
	}
}