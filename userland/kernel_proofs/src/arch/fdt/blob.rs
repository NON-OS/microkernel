// NONOS Operating System (AGPL-3.0-or-later)
//! A flattened device tree built in memory, laid out as the Devicetree
//! Specification's blob: header, empty reservation map, structure, strings.

use std::vec::Vec;

const MAGIC: u32 = 0xD00D_FEED;
const HEADER: usize = 40;
const RESERVE: usize = 16;

#[derive(Default)]
pub struct Tree {
    structure: Vec<u8>,
    strings: Vec<u8>,
}

impl Tree {
    pub fn begin(&mut self, name: &str) -> &mut Self {
        self.word(1);
        self.structure.extend_from_slice(name.as_bytes());
        self.structure.push(0);
        self.pad()
    }

    pub fn end(&mut self) -> &mut Self {
        self.word(2);
        self
    }

    pub fn prop(&mut self, name: &str, data: &[u8]) -> &mut Self {
        let off = self.strings.len() as u32;
        self.strings.extend_from_slice(name.as_bytes());
        self.strings.push(0);
        self.word(3).word(data.len() as u32).word(off);
        self.structure.extend_from_slice(data);
        self.pad()
    }

    pub fn cells(&mut self, name: &str, cells: &[u32]) -> &mut Self {
        let data: Vec<u8> = cells.iter().flat_map(|c| c.to_be_bytes()).collect();
        self.prop(name, &data)
    }

    pub fn blob(&mut self) -> Vec<u8> {
        self.word(9);
        let (s, t) = (self.structure.len(), self.strings.len());
        let off_s = HEADER + RESERVE;
        let words = [MAGIC, (off_s + s + t) as u32, off_s as u32, (off_s + s) as u32];
        let rest = [HEADER as u32, 17, 16, 0, t as u32, s as u32];
        let mut out: Vec<u8> = words.iter().chain(&rest).flat_map(|w| w.to_be_bytes()).collect();
        out.resize(off_s, 0);
        out.extend_from_slice(&self.structure);
        out.extend_from_slice(&self.strings);
        out
    }

    fn word(&mut self, w: u32) -> &mut Self {
        self.structure.extend_from_slice(&w.to_be_bytes());
        self
    }

    fn pad(&mut self) -> &mut Self {
        while !self.structure.len().is_multiple_of(4) {
            self.structure.push(0);
        }
        self
    }
}
