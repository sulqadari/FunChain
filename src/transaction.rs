use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Transaction {
	id:   Vec<u8>,
	vin:  Vec<TXInput>,
	vout: Vec<TXOutput>
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TXInput {
	txid:      Vec<u8>,	// transaction ID
	vout:      usize,	// 
	signature: Vec<u8>,	// 
	pub_key:   Vec<u8>	// 
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TXOutput {
	value:        i32,		//
	pub_key_hash: Vec<u8>	// 
}

impl Transaction {
	pub fn get_id(&self) -> &Vec<u8> {
		return &self.id;
	}
}