// Relative Modules
pub mod encoder;

// Standard Uses
use std::collections::HashMap;

// Crate Uses
use crate::packet::Packet;
use crate::router::encoder::Encoder;

// External Uses
// use pyo3::*;


pub type PacketId = u8;

// #[pyclass]
pub struct Router {
    mutators: Vec<Box<dyn Encoder>>,
    builders: HashMap<PacketId, fn(Vec<u8>) -> Box<dyn Packet>>,
    receivers: HashMap<PacketId, Vec<fn(&Box<dyn Packet>)>>
}

// #[pymethods]
impl Router {
    pub fn new(&self, mutators: Vec<Box<dyn Encoder>>) -> Self {
        Self {
            mutators,
            builders: Default::default(),
            receivers: Default::default()
        }
    }

    pub fn add_mutator(&mut self, mutator: Box<dyn Encoder>) {
        self.mutators.push(mutator);
    }

    pub fn route(&self, message: Vec<u8>) -> String {
        if message.len() < 1 {
            return format!("Message needs to contain at least {} bytes, which is the packet id",
                std::mem::size_of::<PacketId>()
            )
        }

        let id = &message[0];

        if !self.builders.contains_key(id) {
            return format!("Received message with packet id {}, which is not registered to route",
                id
            )
        }

        let packet = self.builders[id](message[1..].to_owned());

        for receiver in &self.receivers[id] {
            receiver(&packet)
        }

        "".to_owned()
    }
}

