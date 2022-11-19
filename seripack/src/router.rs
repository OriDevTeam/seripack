// Standard Uses
use std::collections::HashMap;
use std::mem;

// Crate Uses
use crate::packet::Packet;
use crate::encoder::Encoder;

// External Uses
use anyhow::{Result, bail};


pub type PacketId = u8;
pub type BuilderFn = (usize, fn(Vec<u8>) -> Box<dyn Packet>);
pub type ReceiverFn = fn(Box<dyn Packet>);


// #[pyclass]
#[derive(Default)]
pub struct Router {
    // TODO: Conditional encoders can be a different concept where encoders only act on certain
    //       packet instead, but maybe instead of packet it there could be a sort of enum or string
    //       flag, doing by id could be less convenient (because more packet id specific code
    //       instead where probably only consts are necessary, or something similar)
    encoders: Vec<Box<dyn Encoder + Send + Sync>>,
    builders: HashMap<PacketId, BuilderFn>,
    receivers: HashMap<PacketId, Vec<ReceiverFn>>
}

// #[pymethods]
impl Router {

    pub fn add_encoder(&mut self, encoder: Box<dyn Encoder + Send + Sync>) {
        self.encoders.push(encoder);
    }

    pub fn add_builder(&mut self, id: PacketId, builder: BuilderFn) {
        self.builders.entry(id)
            .and_modify(|e| *e = builder)
            .or_insert(builder);
    }

    pub fn add_receiver(&mut self, id: PacketId, receiver: ReceiverFn) {
        self.receivers.entry(id).or_insert(vec![]).push(receiver);
    }

    pub fn packet_size(&mut self, id: &u8) -> Option<usize>{
        self.builders.get(id).map(|p| p.0)
    }

    pub fn has_packet(&self, id: u8) -> bool {
        self.builders.contains_key(&id)
    }

    pub fn route(&self, message: Vec<u8>) -> Result<()> {
        if message.len() < mem::size_of::<PacketId>() {
            bail!("Message needs to contain at least {} bytes, which is the packet id",
                std::mem::size_of::<PacketId>()
            )
        }

        self.route_id(message[0] as PacketId, message[mem::size_of::<PacketId>()..]
            .to_owned())
    }

    pub fn route_id(&self, id: PacketId, message: Vec<u8>) -> Result<()> {
        if !self.builders.contains_key(&id) {
            bail!("Received message with packet id {}, which is not registered", id)
        }

        let mut decoded = message;
        for encoder in &self.encoders {
            encoder.decode(&mut decoded);
        }

        for receiver in &self.receivers[&id] {
            receiver(self.builders[&id].1(decoded.clone()));
        }

        Ok(())
    }
}

