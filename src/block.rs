use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::linked_list::LinkedList;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(index: u32, data: String, previous_hash: String, transactions: Vec<Transaction>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            transactions,
        };
        block.hash = block.calculate_hash(); 
        block
    }

    pub fn calculate_hash(&self) -> String {
        let data_to_hash = format!("{}{}{}{}{:?}", self.index, self.timestamp, self.data, self.previous_hash, self.transactions);
        let mut hasher = Sha256::new();
        hasher.update(data_to_hash);
        let result = hasher.finalize();
        hex::encode(result)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
    pub txid: String,
    pub amount: u64, 
    pub sender: String,
    pub receiver: String,
}

impl Transaction {
    pub fn new(inputs: Vec<TxIn>, outputs: Vec<TxOut>, amount: u64, sender: &str, receiver: &str) -> Self {
        let mut transaction = Transaction {
            inputs,
            outputs,
            txid: String::new(), 
            amount,
            sender: sender.to_string(),
            receiver: receiver.to_string(),
        };
        transaction.txid = transaction.calculate_txid(); 
        transaction
    }

    pub fn calculate_txid(&self) -> String {
        let data_to_hash = format!("{:?}{:?}{}{}{}", self.inputs, self.outputs, self.amount, self.sender, self.receiver);
        let mut hasher = Sha256::new();
        hasher.update(data_to_hash);
        let result = hasher.finalize();
        hex::encode(result)
    }

    pub fn sample_transactions() -> (Vec<Self>, Vec<Self>, Vec<Self>) {
        let tx1 = Transaction::new(
            vec![TxIn::new("b1fea524fdd06e2ec2fdd4e4c1e14d4bdbfa1a0e7284c6e3b4d1dcb70758bd66", 0, "3045022100c6d470bb91d3f8008f8e27fa2b5c77b8022104d17b364f1021c82d7ea00ec8d7")],
            vec![TxOut::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", 50)],
            1000,
            "Alice",
            "Bob"
        );

        let tx2 = Transaction::new(
            vec![TxIn::new("b3c4b94d4a2f35e2a49b5b5baf6b303c47e1b2b02bfe06a28a41a014935b3d65", 1, "304402200d1966c7a60cf63f58a5ddfa6838a8725e6b0e7a8b320ae98e142c74b8b2447d")],
            vec![TxOut::new("1BoatSLRHtKNngkdXEeobR76b53LETtpyT", 30)],
            2000,
            "Charlie",
            "Dave"
        );

        let tx3 = Transaction::new(
            vec![TxIn::new("c1fea524fdd06e2ec2fdd4e4c1e14d4bdbfa1a0e7284c6e3b4d1dcb70758bd66", 0, "3045022100c6d470bb91d3f8008f8e27fa2b5c77b8022104d17b364f1021c82d7ea00ec8d7")],
            vec![TxOut::new("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", 70)],
            1500,
            "Eve",
            "Frank"
        );

        (vec![tx1.clone()], vec![tx2.clone()], vec![tx3.clone()])
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxIn {
    pub prev_txid: String,
    pub out: usize,
    pub signature: String, // to spend the output
}

impl TxIn {
    pub fn new(prev_txid: &str, out: usize, signature: &str) -> Self {
        TxIn {
            prev_txid: prev_txid.to_string(),
            out,
            signature: signature.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxOut {
    pub public_address: String,
    pub satoshis: u64,
}

impl TxOut {
    pub fn new(public_address: &str, satoshis: u64) -> Self {
        TxOut {
            public_address: public_address.to_string(),
            satoshis,
        }
    }
}

// Create a new blockchain with initial blocks
pub fn create_blockchain() -> LinkedList<Block> {
    let mut blockchain = LinkedList::new();

    // Get sample transactions for each block
    let (genesis_tx, second_tx, third_tx) = Transaction::sample_transactions();

    // Add the genesis block
    blockchain.push(Block::new(0, "Genesis Block".to_string(), "0".to_string(), genesis_tx));

    // Add the second block
    let previous_hash = blockchain.head.as_ref().unwrap().value.hash.clone();
    blockchain.push(Block::new(1, "Second Block".to_string(), previous_hash, second_tx));

    // Add the third block
    let previous_hash = blockchain.head.as_ref().unwrap().next.as_ref().unwrap().value.hash.clone();
    blockchain.push(Block::new(2, "Third Block".to_string(), previous_hash, third_tx));

    blockchain
}

pub fn serialize_block(block: &Block) -> String {
    serde_json::to_string_pretty(block).unwrap()
}

pub fn deserialize_block(data: &str) -> Block {
    serde_json::from_str(data).unwrap()
}
