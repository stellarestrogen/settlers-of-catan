use super::objects::Transport;

pub mod bounds;
pub mod holder;
pub mod position;

pub struct Edge {
    transport: Option<Transport>
}

impl Edge {
    pub fn new() -> Self {
        Edge {
            transport: None
        }
    }

    pub fn set_transport(&mut self, transport: Transport) {
        self.transport = Some(transport)
    }

    pub fn unset_transport(&mut self) {
        self.transport = None
    }
}