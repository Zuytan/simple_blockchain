mod block;
mod blockchain;

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use blockchain::BlockChain;

use crate::block::Block;

fn main() {
    // Number of thread to use
    let nb_thread = 3;
    // Proof of work
    let complexity = 4;
    // NB of block expected
    let nb_block_expected = 10;

    let blockchain = Arc::new(Mutex::new(BlockChain::new(complexity)));

    for thread_i in 0..nb_thread {
        let blockchain_thread_i = blockchain.clone();
        thread::spawn(move || {
            let bc = blockchain_thread_i.lock().unwrap();
            let idx = bc.next_idx();
            drop(bc);
            let mut block_index = idx;
            while block_index < nb_block_expected {
                let bc = blockchain_thread_i.lock().unwrap();
                block_index = bc.next_idx();
                let previous_hash = &bc.last_hash();
                drop(bc);
                let data = format!("Block {} mined by thread {}", block_index, thread_i);
                let mut new_block = Block::new(block_index as u64, &data, previous_hash);
                new_block.mine(complexity);
                let mut bc = blockchain_thread_i.lock().unwrap();
                bc.add_block(new_block);
            }
        });
    }
    let mut is_finished = false;
    while !is_finished {
        thread::sleep(Duration::from_secs(1));

        let bc = blockchain.lock().unwrap();
        if bc.next_idx() >= nb_block_expected {
            is_finished = true
        }
    }
    let bc = blockchain.lock().unwrap();
    println!("Book result : {:?}", bc.book());
}
