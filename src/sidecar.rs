use std::collections::{BTreeSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(String);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct VVertex(u64);

#[derive(Debug)]
pub struct LocalVState {
    owned_vertices: BTreeSet<VVertex>,
    p_value: u64,
}

#[derive(Clone, Debug)]
pub enum RepairOp {
    AddVirtualVertex { vertex: VVertex },
    RemoveVirtualVertex { vertex: VVertex },
    AddPeer { peer: NodeId },
    RemovePeer { peer: NodeId },
}

#[derive(Clone, Debug)]
pub enum Message {
    ApplyRepairOp(RepairOp),
}

#[derive(Debug)]
pub struct Envelope {
    from: NodeId,
    to: NodeId,
    message: Message,
}

#[derive(Debug)]
pub struct Sidecar {
    pub id: NodeId,
    v_state: LocalVState,
    peers: BTreeSet<NodeId>,

    // Only added for simulation purposes
    // TODO: remove after we move on after simulation
    inbox: VecDeque<Envelope>,
}

#[allow(dead_code)]
impl Sidecar {
    pub fn new(id: NodeId) -> Self {
        Self {
            id,
            v_state: LocalVState {
                owned_vertices: BTreeSet::new(),
                p_value: 5, // TODO: make this dynamic
            },
            peers: BTreeSet::new(),
            inbox: VecDeque::new(),
        }
    }

    pub fn push_msg(&mut self, envelope: Envelope) {
        self.inbox.push_back(envelope);
    }

    pub fn tick(&mut self) {
        while let Some(envelope) = self.inbox.pop_front() {
            self.handle_msg(envelope);
        }
    }

    pub fn handle_msg(&mut self, envelope: Envelope) {
        // Something here
        match envelope.message {
            Message::ApplyRepairOp(x) => {}
        }
    }

    pub fn get_id(&self) -> NodeId {
        self.id.clone()
    }
}
