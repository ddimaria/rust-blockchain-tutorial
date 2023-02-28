use ethereum_types::H256;

#[derive(Debug)]
pub(crate) struct WorldState {
    state_trie: H256,
}

impl WorldState {
    pub(crate) fn new() -> Self {
        WorldState {
            state_trie: H256::zero(),
        }
    }

    pub(crate) fn update_state_trie(&mut self, hash: H256) {
        self.state_trie = hash;
    }
}
