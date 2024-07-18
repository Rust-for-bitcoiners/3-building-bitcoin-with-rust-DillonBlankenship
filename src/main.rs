mod block;
mod linked_list;

use block::{create_blockchain, serialize_block, deserialize_block};

fn main() {
    //format long strings (txid, prev_txid)
    fn format_string(data: &str) -> String {
        if data.len() > 10 {
            format!("{}...{}", &data[..5], &data[data.len()-5..])
        } else {
            data.to_string()
        }
    }

    // print block
    fn print_block_details(block: &block::Block) {
        println!(
            "----------------------------------\n\
            Block Index: {}\n\
            Timestamp: {}\n\
            Previous Hash: {}\n\
            Hash: {}",
            block.index, block.timestamp, block.previous_hash, block.hash
        );

        for transaction in &block.transactions {
            println!(
                "\nTransaction ID: {}\n\
                Amount: {} sats\n\
                Sender: {}\n\
                Receiver: {}\n\
                Inputs:",
                format_string(&transaction.txid), transaction.amount, transaction.sender, transaction.receiver
            );

            for input in &transaction.inputs {
                println!(
                    "Previous TxID: {}\n\
                    Out Index: {}\n\
                    Signature: {}",
                    format_string(&input.prev_txid), input.out, format_string(&input.signature)
                );
            }

            println!("Outputs:");
            for output in &transaction.outputs {
                println!(
                    "Address: {}\n\
                    Amount: {} sats",
                    output.public_address, output.satoshis
                );
            }
        }
        println!("----------------------------------");
    }

    // Create the blockchain
    let blockchain = create_blockchain();
    
    // Print the blockchain
    let mut current = &blockchain.head;
    while let Some(node) = current {
        print_block_details(&node.value);
        current = &node.next;
        if current.is_some() {
            println!("    |\n    V");
        }
    }

    // Serialize the first block 
    let _serialized_block = serialize_block(&blockchain.head.as_ref().unwrap().value);

    // Deserialize the first block
    let _deserialized_block: block::Block = deserialize_block(&_serialized_block);
}
