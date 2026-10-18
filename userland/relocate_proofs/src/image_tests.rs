// NONOS Operating System (AGPL-3.0-or-later)
//! The fixture PIE relocated at two bases: each pointer lands on its target
//! at its own base, and nothing else differs between the two images.

use crate::elf::parse;
use crate::loaded::Loaded;
use crate::relocate::rela::R_AARCH64_RELATIVE;

const PIE: &[u8] = include_bytes!("../fixtures/pie.elf");

#[test]
fn two_bases_give_the_same_image_moved() {
    let pie = parse(PIE);
    assert_eq!(pie.link_base, 0x4008_0000);
    assert_eq!(pie.relas.len(), 5);
    assert!(pie.relas.iter().all(|r| r.info & 0xffff_ffff == R_AARCH64_RELATIVE));
    let (a, b) = (Loaded::new(&pie, PIE), Loaded::new(&pie, PIE));
    assert_ne!(a.base, b.base);
    assert_eq!(a.relocate(&pie), Ok(5));
    assert_eq!(b.relocate(&pie), Ok(5));
    let places: Vec<u64> = pie.relas.iter().map(|r| r.offset - pie.link_base).collect();
    for (r, at) in pie.relas.iter().zip(&places) {
        // The pointer is its target's link address moved by each image's bias.
        assert_eq!(a.word(*at) - a.base, r.addend - pie.link_base);
        assert_eq!(b.word(*at) - b.base, r.addend - pie.link_base);
    }
    // Every byte that is not a relocated place is identical in both.
    let moved = |i: usize| places.iter().any(|p| (*p as usize..*p as usize + 8).contains(&i));
    let (x, y) = (a.bytes(), b.bytes());
    assert!((0..x.len()).filter(|i| !moved(*i)).all(|i| x[i] == y[i]));
    // The plain number after the pointers is not one of them.
    assert_eq!(a.word(0x18_0000 + 5 * 8), 0x1234);
}

#[test]
fn the_pointers_reach_what_they_name() {
    let pie = parse(PIE);
    let img = Loaded::new(&pie, PIE);
    let data = 0x18_0000; // .data, 2 MiB above the link base's block
                          // ld.lld leaves the places empty, so even at the link address the
                          // pointers exist only once the relocations are applied.
    assert!((0..5).all(|i| img.word(data + 8 * i) == 0));
    img.relocate(&pie).unwrap();
    let text = |p: u64| (p - img.base) as usize;
    // msg and msg + 3, read through the relocated pointers.
    assert_eq!(&img.bytes()[text(img.word(data + 8))..][..9], b"relocated");
    assert_eq!(&img.bytes()[text(img.word(data + 16))..][..6], b"ocated");
    // ptrs points at itself.
    assert_eq!(img.word(data + 24), img.base + data);
    // The instruction at func is RET.
    let func = text(img.word(data));
    assert_eq!(&img.bytes()[func..func + 4], &0xd65f_03c0u32.to_le_bytes());
}
