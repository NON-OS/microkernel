// NONOS Operating System (AGPL-3.0-or-later)

//! Reading the two file layouts, with every bound checked: a patch's
//! sections must be of the information type and lie in the file, one to
//! eight of them; a RAM image's regions must end before their records, one
//! to sixteen of them. Linux checks the type and trusts the rest.

use crate::fw::patch::{sections, Section};
use crate::fw::ram::{regions, Region};
use crate::model::images::{patch, pattern, ram, ram_7922};

#[test]
fn a_patch_reads_as_its_sections() {
    let img = patch(&[(0x900000, pattern(1, 100), 0x0100_0000), (0x200000, pattern(2, 7), 0)]);
    let s = sections(&img).unwrap();
    assert_eq!(s[0], Section { offs: 96 + 128, addr: 0x900000, len: 100, info: 0x0100_0000 });
    assert_eq!(s[1], Section { offs: 96 + 128 + 100, addr: 0x200000, len: 7, info: 0 });
}

#[test]
fn a_patch_out_of_shape_is_refused() {
    let one = patch(&[(0x900000, pattern(1, 100), 0)]);
    let mut wrong_type = one.clone();
    wrong_type[96 + 3] = 3;
    assert!(sections(&wrong_type).is_none());
    let mut past_end = one.clone();
    past_end[96 + 19] += 1; // the length, one byte more than the file holds
    assert!(sections(&past_end).is_none());
    let mut none = one.clone();
    none[47] = 0;
    assert!(sections(&none).is_none());
    let nine = patch(&vec![(0x900000, pattern(1, 4), 0); 9]);
    assert!(sections(&nine).is_none(), "more than eight");
    let eight = patch(&vec![(0x900000, pattern(1, 4), 0); 8]);
    assert_eq!(sections(&eight).map(|s| s.len()), Some(8));
    assert!(sections(&one[..50]).is_none());
}

#[test]
fn ram_code_reads_as_its_regions_back_to_back() {
    let r = regions(&ram_7922()).unwrap();
    assert_eq!(r.len(), 4);
    assert_eq!(r[0], Region { offs: 0, addr: 0x91b000, len: 5000, feature: 0x21 });
    assert_eq!(r[1], Region { offs: 5000, addr: 0x2015c00, len: 4096, feature: 0x01 });
    assert_eq!(r[3].offs, 5000 + 4096 + 100);
}

#[test]
fn ram_code_out_of_shape_is_refused() {
    let img = ram(&[(0x91b000, pattern(1, 64), 0x21)]);
    let mut over = img.clone();
    let rec = img.len() - 36 - 40;
    over[rec + 20] = 64 + 17; // past the records' start
    assert!(regions(&over).is_none());
    let mut zero = img.clone();
    let n = zero.len() - 34;
    zero[n] = 0;
    assert!(regions(&zero).is_none());
    let many = ram(&vec![(0, pattern(1, 4), 0); 17]);
    assert!(regions(&many).is_none(), "more than sixteen");
    assert!(regions(&ram(&vec![(0, pattern(1, 4), 0); 16])).is_some());
    assert!(regions(&img[..30]).is_none());
}
