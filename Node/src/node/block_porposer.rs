pub struct BlockProposer {
    nonce: u64,
    block_id: u64,
    final_perconfirmation_count: u8,
}

impl BlockProposer {
    pub fn new() -> Self {
        Self {
            nonce: 0,
            block_id: 0,
            final_perconfirmation_count: 0,
        }
    }

    pub fn start_propose(&mut self, nonce: u64, block_id: u64) {
        self.nonce = nonce;
        self.block_id = block_id;
        self.final_perconfirmation_count = 0;
    }

    pub fn propose_next(&mut self) -> (u64, u64) {
        let nonce = self.nonce;
        let block_id = self.block_id;
        self.nonce += 1;
        self.block_id += 1;
        (nonce, block_id)
    }

    pub fn increment_final_perconfirmation(&mut self) {
        self.final_perconfirmation_count += 1;
    }

    pub fn is_final_perconfirmation(&self) -> bool {
        self.final_perconfirmation_count >= 3
    }
}
