// NONOS Operating System (AGPL-3.0-or-later)
//! Just enough ELF64 to load a static PIE: its PT_LOAD segments and the
//! `.rela.dyn` its PT_DYNAMIC names (ELF gABI, program header and dynamic
//! section; DT_RELA 7, DT_RELASZ 8).

use crate::relocate::rela::Rela;

pub struct Segment {
    pub vaddr: u64,
    pub file: std::ops::Range<usize>,
    pub mem: u64,
}

pub struct Pie {
    pub link_base: u64,
    pub len: u64,
    pub segments: Vec<Segment>,
    pub relas: Vec<Rela>,
}

fn u64_at(b: &[u8], at: usize) -> u64 {
    u64::from_le_bytes(b[at..at + 8].try_into().unwrap())
}

fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
}

pub fn parse(b: &[u8]) -> Pie {
    assert_eq!(&b[..4], b"\x7fELF");
    let (phoff, phnum) = (u64_at(b, 0x20) as usize, u16::from_le_bytes([b[0x38], b[0x39]]));
    let mut segments = Vec::new();
    let mut dynamic = None;
    for i in 0..phnum as usize {
        let ph = phoff + i * 56;
        let (kind, offset, vaddr) = (u32_at(b, ph), u64_at(b, ph + 8) as usize, u64_at(b, ph + 16));
        let (filesz, memsz) = (u64_at(b, ph + 32) as usize, u64_at(b, ph + 40));
        match kind {
            1 => segments.push(Segment { vaddr, file: offset..offset + filesz, mem: memsz }),
            2 => dynamic = Some(offset..offset + filesz),
            _ => {}
        }
    }
    let link_base = segments.iter().map(|s| s.vaddr).min().unwrap();
    let end = segments.iter().map(|s| s.vaddr + s.mem).max().unwrap();
    let (mut rela, mut relasz) = (0, 0);
    for d in b[dynamic.unwrap()].chunks_exact(16) {
        match u64_at(d, 0) {
            7 => rela = u64_at(d, 8),
            8 => relasz = u64_at(d, 8),
            _ => {}
        }
    }
    let seg = segments.iter().find(|s| (s.vaddr..s.vaddr + s.mem).contains(&rela)).unwrap();
    let at = seg.file.start + (rela - seg.vaddr) as usize;
    let relas = b[at..at + relasz as usize]
        .chunks_exact(24)
        .map(|e| Rela { offset: u64_at(e, 0), info: u64_at(e, 8), addend: u64_at(e, 16) })
        .collect();
    Pie { link_base, len: end - link_base, segments, relas }
}
