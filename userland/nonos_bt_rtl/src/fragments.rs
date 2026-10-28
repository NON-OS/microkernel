// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! The image cut for vendor command 0xfc20 (btrtl.c, rtl_download_firmware;
//! btrtl.h, struct rtl_download_cmd): each command is one index octet and
//! up to 252 octets of the image. There are length / 252 + 1 fragments,
//! each of 252 octets but the last, which holds length % 252 (none, when
//! the length is a multiple of 252: an empty last fragment, as btrtl.c
//! sends it). Fragments are numbered 0 to 0x7f, then 1 to 0x7f again, and
//! so on; the last one's index also carries bit 7, "data end".

pub const FRAGMENT: usize = 252;
pub const DATA_END: u8 = 0x80;
const WRAP: usize = 0x7f;

/// The index octet of fragment `i`, without the data-end bit.
pub fn index(i: usize) -> u8 {
    if i <= WRAP {
        i as u8
    } else {
        ((i - WRAP - 1) % WRAP + 1) as u8
    }
}

pub fn count(len: usize) -> usize {
    len / FRAGMENT + 1
}

/// Each fragment's index octet, the data-end bit set on the last, and data.
pub fn fragments(image: &[u8]) -> impl Iterator<Item = (u8, &[u8])> {
    let n = count(image.len());
    (0..n).map(move |i| {
        let at = i * FRAGMENT;
        if i + 1 == n {
            (index(i) | DATA_END, &image[at..])
        } else {
            (index(i), &image[at..at + FRAGMENT])
        }
    })
}
