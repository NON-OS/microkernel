// NONOS Operating System (AGPL-3.0-or-later)
//! A relocation the kernel does not apply, or one outside the image, refuses
//! the whole image before anything is written; so does a load that would
//! break the 2 MiB layout.

use crate::elf::parse;
use crate::loaded::Loaded;
use crate::relocate::apply::apply;
use crate::relocate::rela::{Placement, Refused};

const PIE: &[u8] = include_bytes!("../fixtures/pie.elf");

fn refused(edit: impl Fn(&mut crate::elf::Pie)) -> (Refused, bool) {
    let mut pie = parse(PIE);
    let img = Loaded::new(&pie, PIE);
    let before = img.bytes().to_vec();
    edit(&mut pie);
    let err = img.relocate(&pie).unwrap_err();
    (err, img.bytes() == before.as_slice())
}

#[test]
fn a_bad_entry_anywhere_leaves_the_image_untouched() {
    // R_AARCH64_ABS64 (257) at the last entry, after four good ones.
    assert_eq!(refused(|p| p.relas[4].info = 257), (Refused::NotRelative, true));
    assert_eq!(refused(|p| p.relas[4].offset = p.link_base - 8), (Refused::OutsideImage, true));
    assert_eq!(
        refused(|p| p.relas[4].offset = p.link_base + p.len - 4),
        (Refused::OutsideImage, true)
    );
    assert_eq!(refused(|p| p.relas[4].offset += 4), (Refused::MisalignedPlace, true));
}

#[test]
fn a_bias_that_is_not_a_multiple_of_2_mib_is_refused() {
    let pie = parse(PIE);
    let img = Loaded::new(&pie, PIE);
    let at =
        Placement { link_base: pie.link_base, load_base: img.base + 0x1000, len: pie.len - 0x1000 };
    // SAFETY: refused before any write, so nothing outside the image is touched.
    assert_eq!(unsafe { apply(&pie.relas, at) }, Err(Refused::LoadMisaligned));
}
