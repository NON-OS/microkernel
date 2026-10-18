// NONOS Operating System (AGPL-3.0-or-later)
//! A write-back cache beside a device that reads and writes memory only, as
//! one without `dma-coherent` does. A Non-cacheable read goes to memory, but
//! a line a cacheable alias brought in may answer it (mismatched attributes).

use std::collections::BTreeMap;

pub const LINE: u64 = 64;

#[derive(Clone, Copy)]
struct Line {
    data: [u8; LINE as usize],
    dirty: bool,
}

#[derive(Default)]
pub struct System {
    pub memory: BTreeMap<u64, u8>,
    cache: BTreeMap<u64, Line>,
}

impl System {
    fn mem(&self, at: u64) -> u8 {
        *self.memory.get(&at).unwrap_or(&0)
    }

    fn fill(&mut self, tag: u64) -> &mut Line {
        let data = std::array::from_fn(|i| self.mem(tag * LINE + i as u64));
        self.cache.entry(tag).or_insert(Line { data, dirty: false })
    }

    pub fn cpu_write(&mut self, at: u64, value: u8) {
        let line = self.fill(at / LINE);
        line.data[(at % LINE) as usize] = value;
        line.dirty = true;
    }

    pub fn cpu_read(&mut self, at: u64) -> u8 {
        self.fill(at / LINE).data[(at % LINE) as usize]
    }

    /// A Non-cacheable read: memory, unless a line is present (mismatch).
    pub fn cpu_read_uncached(&self, at: u64) -> u8 {
        self.cache.get(&(at / LINE)).map_or(self.mem(at), |l| l.data[(at % LINE) as usize])
    }

    pub fn device_read(&self, at: u64) -> u8 {
        self.mem(at)
    }

    pub fn device_write(&mut self, at: u64, value: u8) {
        self.memory.insert(at, value);
    }

    pub fn clean(&mut self, tag: u64) {
        if let Some(line) = self.cache.get_mut(&tag).filter(|l| l.dirty) {
            line.dirty = false;
            let data = line.data;
            for i in 0..LINE {
                self.memory.insert(tag * LINE + i, data[i as usize]);
            }
        }
    }

    pub fn clean_invalidate(&mut self, tag: u64) {
        self.clean(tag);
        self.cache.remove(&tag);
    }

    /// A plain invalidate, dirty or not: the kernel has none.
    pub fn invalidate(&mut self, tag: u64) {
        self.cache.remove(&tag);
    }
}
