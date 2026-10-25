// NONOS Operating System (AGPL-3.0-or-later)
//! A reserved range is the device's only when its `memory-region` names the
//! child back (the C-41 ruling): a one-sided `iommu-addresses` entry is
//! left out and said, and the named ones beside it are kept.

use crate::reserved_dma_tree::{read, tree_naming, PLATFORM};

#[test]
fn an_entry_not_named_back_is_left_out_and_said() {
    let children = [
        (Some((0x9000_0000, 0x1000)), vec![PLATFORM, 0, 0x9000_0000, 0, 0x1000]),
        (Some((0xa000_0000, 0x1000)), vec![PLATFORM, 0, 0xa000_0000, 0, 0x1000]),
    ];
    let (got, unkeyed) = read(&tree_naming(&children, None, &[0])).unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].range.iova, 0xa000_0000);
    assert_eq!(unkeyed, Some(&b"an entry its device's memory-region does not name back"[..]));
    let (got, unkeyed) = read(&tree_naming(&children, None, &[0, 1])).unwrap();
    assert!(got.is_empty() && unkeyed.is_some());
    let (got, unkeyed) = read(&tree_naming(&children, None, &[])).unwrap();
    assert_eq!((got.len(), unkeyed), (2, None));
}
