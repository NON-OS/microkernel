// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::boot_fb_rule::{contains, holds_boot_fb, MMIO};

const PIO: u8 = 2;
/// QEMU's std VGA framebuffer as OVMF hands it over: 1280x800 at 16 MiB BAR 0.
const FB: (u64, u64) = (0x8000_0000, 1280 * 4 * 800);
type Win = (u8, u64, u64);

/// The claimed devices' windows, each a list; a device not claimed is not in it.
fn held(claimed: &[Vec<Win>]) -> bool {
    holds_boot_fb(FB, claimed.iter().map(|d| d.iter().copied()), |w: &Win| *w)
}

/// vmware-svga: an I/O BAR first, the framebuffer in BAR 1, its FIFO in BAR 2.
fn svga() -> Vec<Win> {
    vec![(PIO, 0xC000, 0x10), (MMIO, 0x8000_0000, 16 << 20), (MMIO, 0x8400_0000, 2 << 20)]
}

/// virtio-vga: the framebuffer in BAR 0.
fn virtio_vga() -> Vec<Win> {
    vec![(MMIO, 0x8000_0000, 8 << 20), (MMIO, 0x8180_0000, 0x4000)]
}

/// A NIC whose windows lie elsewhere.
fn nic() -> Vec<Win> {
    vec![(MMIO, 0x8180_8000, 0x2_0000), (PIO, 0xC040, 0x20)]
}

#[test]
fn nothing_claimed_leaves_gop_writable() {
    assert!(!held(&[]));
    assert!(!held(&[nic()]), "a device elsewhere holds nothing");
}

#[test]
fn the_gpu_behind_the_framebuffer_holds_it_whichever_bar() {
    assert!(held(&[virtio_vga()]), "BAR 0");
    assert!(held(&[nic(), svga()]), "BAR 1, behind an I/O BAR, on the second device");
}

#[test]
fn a_window_must_contain_the_whole_frame() {
    assert!(!contains((MMIO, 0x8000_0000, 1 << 20), FB.0, FB.1), "ends inside the frame");
    assert!(!contains((MMIO, 0x8000_1000, 16 << 20), FB.0, FB.1), "starts past the frame's first byte");
    assert!(contains((MMIO, 0x8000_0000, FB.1), FB.0, FB.1), "exactly the frame");
    assert!(!contains((PIO, 0x8000_0000, 16 << 20), FB.0, FB.1), "port space is no memory");
    assert!(!contains((MMIO, 0x8000_0000, 0), FB.0, FB.1), "an empty window");
    assert!(!contains((MMIO, 0x8000_0000, 16 << 20), FB.0, 0), "an empty frame");
    assert!(!contains((MMIO, u64::MAX - 0xFFF, 0x2000), u64::MAX - 0xFFF, 0x1000), "a window that wraps");
}

#[test]
fn the_release_makes_gop_writable_again() {
    let mut claims = vec![svga()];
    assert!(held(&claims), "claimed: EBUSY");
    claims.clear();
    assert!(!held(&claims), "released: GOP is written again");
    claims.push(virtio_vga());
    assert!(held(&claims), "and a later claim holds it again");
}
