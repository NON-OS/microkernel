// NONOS Operating System (AGPL-3.0-or-later)

//! The rows of Linux v6.12 pcie/drv.c `iwl_dev_info_table` for the Qu and
//! QuZ CNVi ids with an HR radio, and Linux's choice rebuilt from them
//! (`subsystems` compares the driver's against it).

use crate::gen3::select::Image;

pub(super) const HR1: u16 = 0x10C;
pub(super) const HR2: u16 = 0x10A;

// The listed subsystems, with the image of each row's config: `qu` for
// iwl_ax201_cfg_qu_hr and the killer qu_b0 configs, `quz` for the quz ones.
pub(super) fn listed(device: u16) -> (&'static [u16], Image) {
    const BASE: [u16; 8] = [0x0070, 0x0074, 0x0078, 0x007C, 0x1651, 0x1652, 0x2074, 0x4070];
    const WITH_0310: [u16; 9] =
        [0x0070, 0x0074, 0x0078, 0x007C, 0x0310, 0x1651, 0x1652, 0x2074, 0x4070];
    const WITH_6074: [u16; 10] =
        [0x0070, 0x0074, 0x0078, 0x007C, 0x0310, 0x1651, 0x1652, 0x2074, 0x4070, 0x6074];
    const A0F0: [u16; 10] =
        [0x0070, 0x0074, 0x0078, 0x007C, 0x0A10, 0x1651, 0x1652, 0x2074, 0x4070, 0x6074];
    match device {
        0x43F0 => (&BASE, Image::QuB0Hr),
        0xA0F0 => (&A0F0, Image::QuB0Hr),
        0x02F0 => (&WITH_6074, Image::QuZHr),
        0x06F0 => (&WITH_0310, Image::QuZHr),
        0x4DF0 => (&WITH_6074, Image::QuB0Hr),
        _ => (&WITH_0310, Image::QuB0Hr),
    }
}

// The MAC rows of `iwl_dev_info_table` for an HR radio: their image, if one
// matches (`no160` is `IWL_SUBDEVICE_NO_160`).
fn mac_row(mac: u16, step: u8, rf: u16, no160: bool) -> Option<Image> {
    match (mac, step, rf) {
        (0x33, 1, HR1) => Some(Image::QuB0Hr),
        (0x33, 1, HR2) if no160 => Some(Image::QuB0Hr),
        (0x33, 2, _) => Some(Image::QuC0Hr),
        (0x35, _, HR1) => Some(Image::QuZHr),
        (0x35, 1, HR2) => Some(Image::QuZHr),
        _ => None,
    }
}

pub(super) fn linux(device: u16, sub: u16, hw_rev: u32, rf: u16) -> Option<Image> {
    let (mac, step) = (((hw_rev >> 4) & 0xFFF) as u16, (hw_rev & 0xF) as u8);
    if let Some(i) = mac_row(mac, step, rf, sub & 0x0200 != 0) {
        return Some(i);
    }
    let (subs, image) = listed(device);
    subs.contains(&sub).then_some(match (image, hw_rev) {
        (Image::QuB0Hr, 0x332) => Image::QuC0Hr,
        (Image::QuB0Hr, 0x351) => Image::QuZHr,
        (image, _) => image,
    })
}

pub(super) fn own(mac: u16, image: Image) -> bool {
    matches!((mac, image), (0x33, Image::QuB0Hr | Image::QuC0Hr) | (0x35, Image::QuZHr))
}
