// NONOS Operating System (AGPL-3.0-or-later)
//! Platform nodes the finder must not publish as they stand.

use crate::platform_tests::{one, tree, SPI_9_LEVEL};

#[test]
fn unreadable_nodes_are_left_out() {
    // A dma-ranges that is not whole entries.
    assert_eq!(one(&tree(None, Some(&[0, 0]), SPI_9_LEVEL)), Ok(None));
    // A falling edge or active-low SPI is not a GIC trigger.
    for flags in [2, 8] {
        assert_eq!(one(&tree(None, None, &[0, 9, flags])), Ok(None));
    }
    // A PPI is a CPU's; the device is kept, without an interrupt.
    let d = one(&tree(None, None, &[1, 9, 4])).unwrap().unwrap();
    assert_eq!(d.irq, None);
}
