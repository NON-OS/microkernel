// NONOS Operating System (AGPL-3.0-or-later)
//! MSI controllers and maps the kernel must refuse rather than drive.

use crate::arch::fdt::FdtError;
use crate::msi_tree::{kind, tree, Props, ITS, V2M};

#[test]
fn controllers_that_cannot_be_driven_safely_are_refused() {
    let bad = |host: Props, unit| kind(&tree(host, &[unit]));
    let parent: Props = &[("msi-parent", &[1])];
    let two_cells: Props = &[("#msi-cells", &[2])];
    let small = ("arm,gic-v3-its", 0x1_0000, ITS.2);
    assert_eq!(bad(parent, small), Err(FdtError::BadCells), "no translation frame");
    assert_eq!(bad(parent, ("arm,gic-v3-its", 0x2_0000, &[])), Err(FdtError::BadCells));
    assert_eq!(bad(parent, ("arm,gic-v3-its", 0x2_0000, two_cells)), Err(FdtError::BadCells));
    assert_eq!(bad(parent, ("arm,gic-v2m-frame", 0x800, &[])), Err(FdtError::BadCells));
    assert_eq!(bad(parent, ("vendor,own-msi", 0x1000, &[])), Ok(None), "not driven");
    assert_eq!(bad(&[("msi-parent", &[1, 7])], ITS), Err(FdtError::BadCells));
    assert_eq!(bad(&[("msi-parent", &[3])], ITS), Err(FdtError::NoSuchNode));
}

#[test]
fn a_map_naming_two_controllers_or_a_broken_entry_is_refused() {
    let two = [("msi-map", &[0u32, 1, 0, 0x100, 0x100, 2, 0, 0x100][..])];
    assert_eq!(kind(&tree(&two, &[ITS, V2M])), Err(FdtError::BadCells));
    for map in [&[0u32, 1, 0][..], &[0xff00, 1, 0, 0x200], &[0, 1, 0xffff_ff00, 0x200]] {
        assert_eq!(kind(&tree(&[("msi-map", map)], &[ITS])), Err(FdtError::BadCells));
    }
    assert_eq!(kind(&tree(&[("msi-map", &[])], &[ITS])), Ok(None), "an empty map");
}
