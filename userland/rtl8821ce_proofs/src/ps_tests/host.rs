// NONOS Operating System (AGPL-3.0-or-later)
//! The host's Link Control, found through the capability list, and the bits
//! of it the chip follows.

use crate::pcie::{link_control, Aspm, LNKCTL_ASPM_L1, LNKCTL_CLKREQ_EN};

// A configuration space: status with the list bit, a power management
// capability at 0x40, PCI Express at 0x50 with Link Control 0x0142.
fn space() -> [u8; 256] {
    let mut c = [0u8; 256];
    c[0x06] = 1 << 4;
    c[0x34] = 0x40;
    c[0x40..0x42].copy_from_slice(&[0x01, 0x50]);
    c[0x50..0x52].copy_from_slice(&[0x10, 0x00]);
    c[0x60..0x62].copy_from_slice(&0x0142u16.to_le_bytes());
    c
}

fn reader(c: [u8; 256]) -> impl Fn(u32, u32) -> Option<u32> {
    move |off, width| {
        let b = c.get(off as usize..(off + width) as usize)?;
        Some(b.iter().rev().fold(0, |v, &x| v << 8 | x as u32))
    }
}

#[test]
fn link_control_is_found_past_another_capability() {
    assert_eq!(link_control(reader(space())), Some(0x0142));
    let aspm = Aspm::from_link_control(0x0142);
    assert_eq!((aspm.l1, aspm.clkreq), (true, true));
    assert_eq!(Aspm::from_link_control(!(LNKCTL_ASPM_L1 | LNKCTL_CLKREQ_EN)), Aspm::default());
}

#[test]
fn a_card_without_the_capability_or_with_a_bad_list_gives_none() {
    let mut no_list = space();
    no_list[0x06] = 0;
    assert_eq!(link_control(reader(no_list)), None, "no capability list");
    let mut looping = space();
    looping[0x41] = 0x40;
    assert_eq!(link_control(reader(looping)), None, "a list that loops");
    let mut low = space();
    low[0x41] = 0x20;
    assert_eq!(link_control(reader(low)), None, "a link into the header");
    assert_eq!(link_control(|_, _| None), None, "every read refused");
}
