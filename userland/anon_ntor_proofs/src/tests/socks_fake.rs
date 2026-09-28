// NONOS Operating System (AGPL-3.0-or-later)
/* A tunnel whose far end the test plays, so each exit answer is chosen
 * here rather than hoped for, and the frames a caller sends. */

use alloc::vec::Vec;

use super::socks::tunnel::{Far, Tunnel, Unsent};

pub struct Fake {
    pub refuse: Option<u8>,
    pub opened: Vec<(Vec<u8>, u16)>,
    pub far: Far,
    pub blocked: bool,
    pub sent: Vec<u8>,
    pub arrived: Vec<u8>,
    pub closed: Vec<u16>,
}

pub fn fake() -> Fake {
    Fake {
        refuse: None,
        opened: Vec::new(),
        far: Far::Opening,
        blocked: false,
        sent: Vec::new(),
        arrived: Vec::new(),
        closed: Vec::new(),
    }
}

impl Tunnel for Fake {
    fn open(&mut self, host: &[u8], port: u16) -> Result<u16, u8> {
        self.refuse.map_or(Ok(7), Err).inspect(|_| self.opened.push((host.to_vec(), port)))
    }
    fn far(&self, _id: u16) -> Far {
        self.far
    }
    fn send(&mut self, _id: u16, data: &[u8]) -> Result<usize, Unsent> {
        match (self.far, self.blocked) {
            (Far::Ended(_) | Far::Gone, _) => Err(Unsent::Over),
            (_, true) => Err(Unsent::Blocked),
            _ => Ok(data.len()).inspect(|_| self.sent.extend_from_slice(data)),
        }
    }
    fn take(&mut self, _id: u16, max: usize) -> Vec<u8> {
        let n = self.arrived.len().min(max);
        self.arrived.drain(..n).collect()
    }
    fn waiting(&self, _id: u16) -> usize {
        self.arrived.len()
    }
    fn close(&mut self, id: u16) {
        self.closed.push(id);
    }
}
