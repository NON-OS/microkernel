// NONOS Operating System (AGPL-3.0-or-later)
//! Tables that would route a device's DMA or MSIs somewhere they do not
//! say, each refused with its reason.

use crate::arch::aarch64::iort::{root_complexes, IortError};
use crate::records_tests::its;
use crate::table::{Map, Table, NODES_AT};

fn read(build: impl FnOnce(&mut Table)) -> Result<(), IortError> {
    let mut t = Table::new();
    build(&mut t);
    root_complexes(&t.finish(), its).map(|_| ())
}

fn rc_to(maps: &[Map]) -> Result<(), IortError> {
    read(|t| {
        t.root(0, maps);
    })
}

#[test]
fn mappings_that_name_nothing_or_wrap() {
    // Between nodes, and past the table.
    assert_eq!(rc_to(&[(0, 1, 0, NODES_AT + 4, 0)]), Err(IortError::BadMapping));
    assert_eq!(rc_to(&[(0, 1, 0, 0x1_0000, 0)]), Err(IortError::BadMapping));
    // To itself: a root complex is no unit to send ids to.
    assert_eq!(rc_to(&[(0, 1, 0, NODES_AT, 0)]), Err(IortError::BadMapping));
    let wraps = |input, ids, output| {
        read(|t| {
            let g = t.its_group(&[0]);
            t.root(0, &[(input, ids, output, g, 0)]);
        })
    };
    assert_eq!(wraps(u32::MAX, 2, 0), Err(IortError::BadMapping));
    assert_eq!(wraps(0, 2, u32::MAX), Err(IortError::BadMapping));
    assert_eq!(wraps(0, 0, 0), Err(IortError::BadMapping));
    assert_eq!(wraps(1, u32::MAX, 0), Ok(()));
}

#[test]
fn a_root_complex_with_a_single_mapping() {
    let single = read(|t| {
        let g = t.its_group(&[0]);
        t.root(0, &[(0, 1, 0, g, 1)]);
    });
    assert_eq!(single, Err(IortError::BadMapping));
}

#[test]
fn two_smmus_or_two_its_groups_for_one_root_complex() {
    let smmus = read(|t| {
        let (a, b) =
            (t.smmu(0x1000_0000, 0, [0; 4], 0, &[]), t.smmu(0x2000_0000, 0, [0; 4], 0, &[]));
        t.root(0, &[(0, 0x10, 0, a, 0), (0x10, 0x10, 0, b, 0)]);
    });
    assert_eq!(smmus, Err(IortError::Ambiguous));
    let groups = read(|t| {
        let (a, b) = (t.its_group(&[0]), t.its_group(&[0]));
        t.root(0, &[(0, 0x10, 0, a, 0), (0x10, 0x10, 0, b, 0)]);
    });
    assert_eq!(groups, Err(IortError::Ambiguous));
}
