use crate::block::Block;

/// A blockchain representation
pub struct BlockChain {
    /// List of blocks
    blocks: Vec<Block>,

    /// Book of data
    book: Vec<String>,

    /// Complexity necessary to add a block to the chain
    complexity: usize,
}

impl BlockChain {
    /// Generate a new blockchain
    /// ## Arguments
    /// * complexity : Complexity of the blockchain generated
    /// ## Results
    /// The blockchain generated
    pub fn new(complexity: usize) -> Self {
        let new_book = vec![];
        let genesis_block = Block::new(
            0,
            &BlockChain::book_stringified(new_book.clone()),
            &String::default(),
        );
        BlockChain {
            blocks: vec![genesis_block],
            book: new_book,
            complexity,
        }
    }

    /// Add a block to the chain
    /// ## Arguments
    /// * data : The data to add
    pub fn add_block(&mut self, block: Block) {
        if self.check_block_validity(&block) {
            println!(
                "New block added to the chain:\n\tData added : {}\n\tHash : {}\n\tProof: {}\n",
                block.data(),
                &block.hash(),
                &block.proof(),
            );
            self.book.push(block.data().to_owned());
            self.blocks.push(block);
        }
    }

    /// Check if the block given is valid
    pub fn check_block_validity(&self, block: &Block) -> bool {
        let last_block = self.blocks.get(self.blocks.len() - 1).unwrap();
        let hash_calculated = Block::calculate_hash(
            self.blocks.len() as u64,
            block.timestamp(),
            &block.data(),
            &last_block.hash(),
            block.proof(),
        );
        hash_calculated.starts_with(&"0".repeat(self.complexity))
    }

    /// Get the last hash
    pub fn last_hash(&self) -> String {
        let last_block = self.blocks.get(self.blocks.len() - 1).unwrap();
        last_block.hash()
    }

    /// Get the next block index
    pub fn next_idx(&self) -> usize {
        self.blocks.len()
    }

    /// Generate a book stringified
    fn book_stringified(book: Vec<String>) -> String {
        book.iter().fold("".to_string(), |cur, nxt| cur + nxt)
    }

    /// Get the book
    pub fn book(&self) -> Vec<String> {
        self.book.clone()
    }
}
