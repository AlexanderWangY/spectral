use std::collections::{BTreeMap, HashMap};

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
    }

    pub fn delete_node(&mut self, node_id: NodeId) {
        // Implement deletion logic
    }

    pub fn tick(&mut self) {
        // Apply mutations and consume inboxes
        self.epoch += 1;
    }
}
