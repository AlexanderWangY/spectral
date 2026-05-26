use std::collections::HashMap;

use crate::sidecar::{NodeId, Sidecar};

#[derive(Default, Debug)]
pub struct SimWorld {
    nodes: HashMap<NodeId, Sidecar>,
    size: u32,
    epoch: u64,
}

impl SimWorld {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, node: Sidecar) {
        self.nodes.insert(node.id.clone(), node);
        self.size += 1;
    }

    pub fn delete_node(&mut self, node_id: NodeId) {
        self.nodes.remove(&node_id);
        self.size -= 1;
    }

    pub fn tick(&mut self) {
        // Tick all nodes
        for (key, val) in &mut self.nodes {
            val.tick();
        }
        self.epoch += 1;
    }
}
