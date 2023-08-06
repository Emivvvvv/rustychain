use crate::blockchain::Chain;

pub struct Node {
    chain: Chain
}

impl Node {
    pub fn new(chain: Chain) -> Self {
        Node {
            chain
        }
    }

    pub fn watch(&self) {
        self.chain.print_current()
    }
}