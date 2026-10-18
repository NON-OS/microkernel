// NONOS Operating System (AGPL-3.0-or-later)
//! `msi-map`, `msi-parent` and the controllers they name, built to pin each
//! rule no board's tree exercises.

use crate::arch::fdt::find::msi::MsiKind;
use crate::arch::fdt::FdtError;
use crate::msi_tree::{kind, tree, Props, ITS, V2M};

#[test]
fn a_bare_msi_parent_maps_every_rid_to_itself() {
    assert_eq!(kind(&tree(&[("msi-parent", &[2])], &[ITS, V2M])), Ok(Some((V2M_KIND, 0x105))));
    let map = [("msi-map", &[0x100u32, 1, 0x40, 0x100][..]), ("msi-map-mask", &[0xfff8][..])];
    assert_eq!(kind(&tree(&map, &[ITS])), Ok(Some((MsiKind::Its, 0x40))));
    assert_eq!(kind(&tree(&[], &[ITS])), Ok(None), "no map, no parent");
}

const V2M_KIND: MsiKind = MsiKind::V2m { spi_base: 0, spi_count: 0 };

#[test]
fn a_v2m_range_from_the_node_is_whole_and_inside_the_spis() {
    let ranged = |base: u32, count: u32| {
        let props: Props = &[("arm,msi-base-spi", &[base]), ("arm,msi-num-spis", &[count])];
        kind(&tree(&[("msi-parent", &[1])], &[("arm,gic-v2m-frame", 0x1000, props)]))
    };
    let at = |spi_base, spi_count| Ok(Some((MsiKind::V2m { spi_base, spi_count }, 0x105)));
    assert_eq!(ranged(80, 64), at(80, 64));
    assert_eq!(ranged(32, 988), at(32, 988));
    for (base, count) in [(31, 1), (80, 0), (1000, 21), (0, 0xffff_ffff)] {
        assert_eq!(ranged(base, count), Err(FdtError::BadCells), "{base} {count}");
    }
    let half: Props = &[("arm,msi-base-spi", &[80])];
    let blob = tree(&[("msi-parent", &[1])], &[("arm,gic-v2m-frame", 0x1000, half)]);
    assert_eq!(kind(&blob), Err(FdtError::BadCells));
}
