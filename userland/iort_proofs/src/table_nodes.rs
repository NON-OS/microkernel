// NONOS Operating System (AGPL-3.0-or-later)
//! The three node types the reader takes, each with its type-specific body
//! laid out as DEN0049 gives it.

use crate::table::{Map, Table};

impl Table {
    pub fn its_group(&mut self, ids: &[u32]) -> u32 {
        let mut body = (ids.len() as u32).to_le_bytes().to_vec();
        ids.iter().for_each(|id| body.extend_from_slice(&id.to_le_bytes()));
        self.add(0, 1, &body, &[])
    }

    /// An SMMUv3, revision 4: base, flags, VATOS, model, the four GSIVs,
    /// proximity domain, DeviceID mapping index.
    pub fn smmu(
        &mut self,
        base: u64,
        flags: u32,
        gsivs: [u32; 4],
        index: u32,
        maps: &[Map],
    ) -> u32 {
        let mut body = base.to_le_bytes().to_vec();
        body.extend_from_slice(&flags.to_le_bytes());
        body.extend_from_slice(&[0; 4 + 8 + 4]);
        gsivs.iter().for_each(|g| body.extend_from_slice(&g.to_le_bytes()));
        body.extend_from_slice(&0u32.to_le_bytes());
        body.extend_from_slice(&index.to_le_bytes());
        self.add(4, 4, &body, maps)
    }

    /// A root complex: memory access properties, ATS, then the segment.
    pub fn root(&mut self, segment: u32, maps: &[Map]) -> u32 {
        let mut body = vec![0u8; 8 + 4];
        body.extend_from_slice(&segment.to_le_bytes());
        self.add(2, 1, &body, maps)
    }
}
