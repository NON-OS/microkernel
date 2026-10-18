// NONOS Operating System (AGPL-3.0-or-later)
//! What the records cannot hold, and ITSs MADT does not name.

use crate::arch::aarch64::iort::{root_complexes, IortError};
use crate::records_tests::its;
use crate::table::{Map, Table};

fn read(build: impl FnOnce(&mut Table)) -> Result<(), IortError> {
    let mut t = Table::new();
    build(&mut t);
    root_complexes(&t.finish(), its).map(|_| ())
}

#[test]
fn more_than_the_records_hold() {
    let entries = |n: u32| {
        read(|t| {
            let g = t.its_group(&[0]);
            let maps: Vec<Map> = (0..n).map(|i| (i * 0x10, 0x10, i, g, 0)).collect();
            t.root(0, &maps);
        })
    };
    assert_eq!(entries(8), Ok(()));
    assert_eq!(entries(9), Err(IortError::TooMany));
    let roots = |n: u32| {
        read(|t| {
            for segment in 0..n {
                t.root(segment, &[]);
            }
        })
    };
    assert_eq!(roots(4), Ok(()));
    assert_eq!(roots(5), Err(IortError::TooMany));
}

#[test]
fn an_its_madt_does_not_name() {
    let unknown = read(|t| {
        let g = t.its_group(&[7]);
        t.root(0, &[(0, 1, 0, g, 0)]);
    });
    assert_eq!(unknown, Err(IortError::UnknownIts));
    let empty = read(|t| {
        let g = t.its_group(&[]);
        t.root(0, &[(0, 1, 0, g, 0)]);
    });
    assert_eq!(empty, Err(IortError::BadNode));
}
