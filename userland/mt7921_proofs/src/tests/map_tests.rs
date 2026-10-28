// NONOS Operating System (AGPL-3.0-or-later)

//! The address map onto BAR0 (`__mt7921_reg_addr`): below 1 MiB direct,
//! the fixed windows in Linux's order with Linux's inclusive end, the L1
//! remap's three ranges, and nothing else.

use crate::hw::map::{fixed, remapped};

#[test]
fn below_one_mebibyte_an_address_is_its_own_offset() {
    assert_eq!(fixed(0xd4208), Some(0xd4208));
    assert_eq!(fixed(0xfe24c), Some(0xfe24c));
    assert_eq!(fixed(0xfffff), Some(0xfffff));
}

#[test]
fn the_fixed_windows_the_driver_uses_land_where_linux_puts_them() {
    assert_eq!(fixed(0x7c060010), Some(0xe0010), "MT_CONN_ON_LPCTL");
    assert_eq!(fixed(0x7c0600f0), Some(0xe00f0), "MT_CONN_ON_MISC");
    assert_eq!(fixed(0x7c026004), Some(0xd6004), "MT_DMASHDL_SW_CONTROL");
    assert_eq!(fixed(0x54000120), Some(0x02120), "MT_WFDMA_DUMMY_CR");
    assert_eq!(fixed(0x0041f23c), Some(0x9f23c), "MT_SWDEF_MODE");
    assert_eq!(fixed(0x820cd004), Some(0x0f004), "MT_MDP_DCR1");
    assert_eq!(fixed(0x820e7000), Some(0x21e00), "MT_DMA_DCR0(0)");
}

#[test]
fn the_first_window_holding_an_address_wins_and_its_end_counts() {
    // Linux bounds a window by `ofs > size`, so its one-past-the-end
    // address is the window's too.
    assert_eq!(fixed(0x7c010000), Some(0xf0000 + 0x10000), "the end is inclusive, as Linux's");
    assert_eq!(fixed(0x820fd800), Some(0xa4800 + 0x800), "the last window's end");
    assert_eq!(fixed(0x820fd801), None);
}

#[test]
fn addresses_no_window_holds_go_through_l1_or_nowhere() {
    for a in [0x18060010, 0x18000140, 0x70010200, 0x70010020, 0x7c300000] {
        assert!(remapped(a), "{a:#x}");
    }
    assert_eq!(fixed(0x70010200), None);
    assert!(!remapped(0x18c00000) && !remapped(0x78000000) && !remapped(0x7c400000));
    assert!(!remapped(0x17ffffff) && !remapped(0x6fffffff));
}
