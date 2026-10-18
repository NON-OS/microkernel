// NONOS Operating System (AGPL-3.0-or-later)
//! A bound device behind `Nic` for the frame service's proofs: what it is
//! given to send is kept, and what it receives is scripted.

use crate::nic::Nic;
use crate::station::Taken;

pub struct Fake {
    pub sent: Vec<Vec<u8>>,
    pub frames: Vec<Result<Option<Vec<u8>>, i32>>,
}

impl Nic for Fake {
    fn mac(&self) -> [u8; 6] {
        [2, 0, 0, 0, 0, 1]
    }
    fn taken(&self) -> Taken {
        Taken::Programmed
    }
    fn link_up(&self) -> bool {
        true
    }
    fn send(&mut self, frame: &[u8]) -> Result<(), i32> {
        self.sent.push(frame.to_vec());
        Ok(())
    }
    fn recv(&mut self, out: &mut [u8]) -> Result<Option<usize>, i32> {
        match self.frames.remove(0)? {
            Some(f) => Ok(Some({
                out[..f.len()].copy_from_slice(&f);
                f.len()
            })),
            None => Ok(None),
        }
    }
}
