use crate::block;
use num_bigint::BigInt;
use num_bigint::Sign;
use data_encoding::HEXLOWER;

const MAX_NONCE: i64 = 0x7FFF_FFFF_FFFF_FFFF;

pub struct ProofOfWork {
	block:  block::Block,
	target: BigInt
}

impl ProofOfWork {
	/// Performs the block mining.
	/// This fucntion is used to find a 'nonce' value that produces a hash of the block data
	/// that is lower than a specific target value.
	/// It takes no arguments but operates on the ProofOfWork object on which it is called.
	/// Returns a tuple containing the nonce and the hash value that was produced using that nonce.
	pub fn run(&self) -> (i64, String) {
		let mut nonce = 0;
		let mut hash  = Vec::new();
		println!("Mining the block...");
	
		while nonce < MAX_NONCE {
			let data = self.prepare_data(nonce);
			hash = crate::sha256_digest(data.as_slice());
			let hash_int = BigInt::from_bytes_be(Sign::Plus, hash.as_slice());
	
			if (hash_int.lt(self.target.borrow())) {
				println!("{}", HEXLOWER.encode(hash.as_slice()));
				break;
			} else {
				nonce += 1;
			}
		}
	
		println!("Done.");
		return (nonce, HEXLOWER.encode(hash.as_slice()));
	}
}