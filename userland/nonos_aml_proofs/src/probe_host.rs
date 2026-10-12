// NONOS Operating System (AGPL-3.0-or-later)
//! The probe as the interpreter's host.

use crate::exec::{Host, RegionAccess};
use crate::object::Object;
use crate::probe::Probe;

impl Host for Probe {
    fn sleep(&mut self, ms: u64) {
        self.slept_ms.push(ms);
    }
    fn stall(&mut self, us: u64) {
        self.stalled_us.push(us);
    }
    fn timer(&mut self) -> u64 {
        self.ticks += 10;
        self.ticks
    }
    fn debug(&mut self, value: &Object) {
        self.debug.push(format!("{value:?}"));
    }
    fn fatal(&mut self, kind: u8, code: u32, arg: u64) {
        self.fatal.push((kind, code, arg));
    }
    fn region_read(&mut self, a: &RegionAccess<'_>) -> Option<u64> {
        self.read(a)
    }
    fn region_write(&mut self, a: &RegionAccess<'_>, value: u64) -> bool {
        self.write(a, value)
    }
    fn transaction(
        &mut self,
        a: &RegionAccess<'_>,
        access: [u8; 3],
        connection: Option<&[u8]>,
        data: &mut Vec<u8>,
    ) -> bool {
        self.transactions.push((
            a.space.0,
            a.address,
            access,
            connection.map(|c| c.to_vec()),
            data.clone(),
        ));
        // The modelled device answers with a status byte, then what it got.
        data.insert(0, 0x00);
        !self.refuse
    }
    fn gpio(&mut self, _: &[u8], pin: u32, count: u32, write: Option<u64>) -> Option<u64> {
        if let Some(v) = write {
            self.pins.insert(pin, v & ((1u64 << count) - 1));
        }
        Some(*self.pins.get(&pin).unwrap_or(&0))
    }
    fn table(&mut self, signature: &[u8], _: &[u8], _: &[u8]) -> Option<Vec<u8>> {
        self.tables.iter().find(|t| &t[..4] == signature).cloned()
    }
    fn lock_word(&mut self, old: u32, new: u32) -> Option<u32> {
        self.cas(old, new)
    }
    fn lock_released(&mut self) {
        self.released += 1;
    }
}
