// NONOS Operating System (AGPL-3.0-or-later)
//! IORT tables built field by field as DEN0049 lays them out. A node is
//! added after the nodes its mappings name: `add` returns its offset.

/// One ID mapping: input base, ID count, output base, target offset, flags.
pub type Map = (u32, u32, u32, u32, u32);

pub struct Table {
    nodes: Vec<u8>,
    count: u32,
}

pub const NODES_AT: u32 = 48;

impl Table {
    pub fn new() -> Self {
        Table { nodes: Vec::new(), count: 0 }
    }

    pub(crate) fn add(&mut self, kind: u8, revision: u8, body: &[u8], maps: &[Map]) -> u32 {
        let at = NODES_AT + self.nodes.len() as u32;
        let array = 16 + body.len() as u32;
        let length = array as usize + maps.len() * 20;
        self.nodes.push(kind);
        self.nodes.extend_from_slice(&(length as u16).to_le_bytes());
        self.nodes.push(revision);
        for word in [0, maps.len() as u32, if maps.is_empty() { 0 } else { array }] {
            self.nodes.extend_from_slice(&word.to_le_bytes());
        }
        self.nodes.extend_from_slice(body);
        for &(input, ids, output, target, flags) in maps {
            // The table stores the count minus one; zero becomes 0xFFFF_FFFF.
            for word in [input, ids.wrapping_sub(1), output, target, flags] {
                self.nodes.extend_from_slice(&word.to_le_bytes());
            }
        }
        self.count += 1;
        at
    }

    pub fn finish(&self) -> Vec<u8> {
        let length = NODES_AT as usize + self.nodes.len();
        let mut t = b"IORT".to_vec();
        t.extend_from_slice(&(length as u32).to_le_bytes());
        t.extend_from_slice(&[3, 0]);
        t.extend_from_slice(&[0; 26]);
        for word in [self.count, NODES_AT, 0] {
            t.extend_from_slice(&word.to_le_bytes());
        }
        t.extend_from_slice(&self.nodes);
        t
    }
}
