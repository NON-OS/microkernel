// NONOS Operating System (AGPL-3.0-or-later)
//! What a test reads of the rig's memory, and the payloads it puts there.

use super::rig::Rig;
use crate::constants::ata::STRUCT_REGION_BYTES;
use crate::engine::region::DmaRegion;

/// The first `n` bytes of `r`.
fn bytes(r: &DmaRegion, n: usize) -> Vec<u8> {
    assert!(n as u64 <= r.len());
    // SAFETY: the grant is live while the port holds it, and `n` long.
    unsafe { std::slice::from_raw_parts(r.user_va() as *const u8, n) }.to_vec()
}

impl Rig {
    /// The command table as the driver last built it.
    pub fn table(&self) -> Vec<u8> {
        bytes(&self.port.ctba, STRUCT_REGION_BYTES as usize)
    }

    /// Slot 0's command header, as the HBA last left it.
    pub fn header(&self) -> Vec<u8> {
        bytes(&self.port.clb, 32)
    }

    /// Where the command table and the data buffer lie, as the HBA sees them.
    pub fn table_and_data_addr(&self) -> (u64, u64) {
        (self.port.ctba.device_addr(), self.port.data.device_addr())
    }

    /// The first `n` bytes of the data buffer, by the driver's own copy.
    pub fn data(&self, n: usize) -> Vec<u8> {
        let mut out = vec![0; n];
        self.port.take(&mut out).expect("no more than the data buffer holds");
        out
    }

    /// Put a write's payload in the data buffer, as the write handler does.
    pub fn load(&mut self, payload: &[u8]) {
        self.port.load(payload).expect("a payload the buffer holds, on a working port");
    }
}
