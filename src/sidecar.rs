use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(String);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VVertex(u64);

#[derive(Debug)]
pub struct LocalVState {
    owned_vertices: BTreeSet<VVertex>,
}

#[derive(Debug)]
pub struct Sidecar {
    pub id: NodeId,
    v_state: LocalVState,
    peers: BTreeSet<NodeId>,
}

#[allow(dead_code)]
impl Sidecar {
    pub fn new(id: NodeId) -> Self {
        Self {
            id,
            v_state: LocalVState {
                owned_vertices: BTreeSet::new(),
            },
            peers: BTreeSet::new(),
        }
    }

    pub fn get_id(&self) -> NodeId {
        self.id.clone()
    }
}
