// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The decisions behind the GOP mode the loader scans out, with no UEFI
//! types in them so the host proofs run this exact file: which pixel
//! layouts a linear framebuffer can be written in, what the panel's EDID
//! says its native timing and physical size are, and which offered mode
//! to keep or set.

/// One linear mode the firmware offers, reduced to what the choice needs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Offered {
    pub index: u32,
    pub width: u32,
    pub height: u32,
}

/// What to do with the GOP: leave the mode the firmware set, or set one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Choice {
    Keep,
    Set(u32),
}

/// Why a mode was chosen, for the line the splash and the log print.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    /// A development build pinned the mode (NONOS_GOP_PREF).
    Pinned,
    /// The panel's EDID preferred timing, its native resolution.
    Native,
    /// The mode the firmware had set at handoff.
    Current,
    /// No native mode is known and the current one is a low fallback.
    Largest,
}

/// The panel's native mode and size from its EDID base block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Edid {
    pub width: u32,
    pub height: u32,
    /// Physical image size in millimetres, 0 when the panel does not say.
    pub width_mm: u32,
    pub height_mm: u32,
}

/// Below this many lines the mode the firmware left is taken as its
/// text-console default (640x480, 800x600) rather than a panel's native
/// mode, and a larger offered mode is preferred when no EDID names one.
pub const KEEP_MIN_HEIGHT: u32 = 720;

/// The largest frame the fallback picks when nothing names the panel's
/// native mode: 3840x2400, about 35 MiB at four bytes a pixel. The kernel
/// copy and the compositor's backing each hold one frame; the native or
/// current mode is honoured past this, since that is the panel's own.
pub const FALLBACK_MAX_PIXELS: u64 = 3840 * 2400;

/// Largest width or height accepted at all; the kernel refuses more.
pub const MAX_DIM: u32 = 8192;

/// The GOP pixel layout of a PixelBitMask mode, as a byte order a 32 bit
/// writer can use: Some(true) for blue in the low byte (the same bytes as
/// PixelBlueGreenRedReserved8BitPerColor), Some(false) for red in the low
/// byte, None for anything else (16 bpp, 10 bit channels, 24 bpp packed).
/// UEFI 2.10 section 12.9 gives the depth as the bits the four masks
/// cover; Linux's EFI stub (libstub/gop.c) sums the four mask widths the
/// same way, so a mode only counts as 32 bpp when the reserved mask holds
/// the top byte.
pub fn bitmask_bgr(red: u32, green: u32, blue: u32, reserved: u32) -> Option<bool> {
    if green != 0x0000_FF00 || reserved != 0xFF00_0000 {
        return None;
    }
    match (red, blue) {
        (0x00FF_0000, 0x0000_00FF) => Some(true),
        (0x0000_00FF, 0x00FF_0000) => Some(false),
        _ => None,
    }
}

const EDID_HEADER: [u8; 8] = [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00];
const EDID_BLOCK: usize = 128;
const DTD_START: usize = 54;
const DTD_LEN: usize = 18;

/// Parse the native timing and physical size from an EDID base block, as
/// EFI_EDID_ACTIVE_PROTOCOL or EFI_EDID_DISCOVERED_PROTOCOL hands it over.
/// The header and the block checksum must hold, as Linux's drm_edid
/// requires of block 0. EDID 1.3 and later make the first detailed timing
/// descriptor the preferred, native one; display descriptors (pixel clock
/// zero) are skipped. The descriptor's image size is in millimetres; when
/// it is zero the base block's centimetre size stands in.
pub fn parse_edid(raw: &[u8]) -> Option<Edid> {
    let b = raw.get(..EDID_BLOCK)?;
    if b[..8] != EDID_HEADER {
        return None;
    }
    if b.iter().fold(0u8, |s, &v| s.wrapping_add(v)) != 0 {
        return None;
    }
    let mut dtd = None;
    for i in 0..4 {
        let d = &b[DTD_START + i * DTD_LEN..DTD_START + (i + 1) * DTD_LEN];
        if d[0] != 0 || d[1] != 0 {
            dtd = Some(d);
            break;
        }
    }
    let d = dtd?;
    let width = d[2] as u32 | ((d[4] as u32 & 0xF0) << 4);
    let mut height = d[5] as u32 | ((d[7] as u32 & 0xF0) << 4);
    if d[17] & 0x80 != 0 {
        height = height.saturating_mul(2);
    }
    if width == 0 || height == 0 {
        return None;
    }
    let mut width_mm = d[12] as u32 | ((d[14] as u32 & 0xF0) << 4);
    let mut height_mm = d[13] as u32 | ((d[14] as u32 & 0x0F) << 8);
    if width_mm == 0 || height_mm == 0 {
        // Bytes 21 and 22: the maximum image size in centimetres. Either
        // being zero means the pair encodes an aspect ratio, not a size.
        let (w_cm, h_cm) = (b[21] as u32, b[22] as u32);
        if w_cm != 0 && h_cm != 0 {
            width_mm = w_cm * 10;
            height_mm = h_cm * 10;
        } else {
            width_mm = 0;
            height_mm = 0;
        }
    }
    Some(Edid { width, height, width_mm, height_mm })
}

fn usable(w: u32, h: u32) -> bool {
    w != 0 && h != 0 && w <= MAX_DIM && h <= MAX_DIM
}

fn find(offered: &[Offered], want: (u32, u32)) -> Option<u32> {
    offered.iter().find(|m| (m.width, m.height) == want).map(|m| m.index)
}

/// Choose the mode to scan out. `offered` holds the linear modes only,
/// `current` the mode the firmware had set when it is linear too,
/// `native` the EDID preferred timing and `pinned` a development pin.
///
/// The firmware's current mode is kept unless something better is known,
/// as Linux (efifb, simpledrm) and systemd-boot do: on a laptop it is the
/// panel's native mode, and setting anything else makes the panel scaler
/// stretch or blur it. The EDID preferred timing wins when the firmware
/// left a different mode set (a 1024x768 logo mode on a 1920x1080 panel).
/// Only when neither is known and the current mode is a low text-console
/// default does the largest offered mode win, bounded by the native size
/// when the EDID gives one and by FALLBACK_MAX_PIXELS otherwise.
pub fn choose(
    offered: &[Offered],
    current: Option<(u32, u32)>,
    native: Option<(u32, u32)>,
    pinned: Option<(u32, u32)>,
) -> Option<(Choice, Source)> {
    let current = current.filter(|&(w, h)| usable(w, h));
    let native = native.filter(|&(w, h)| usable(w, h));
    for (want, source) in [(pinned, Source::Pinned), (native, Source::Native)] {
        let Some(want) = want else { continue };
        if current == Some(want) {
            return Some((Choice::Keep, source));
        }
        if let Some(index) = find(offered, want) {
            return Some((Choice::Set(index), source));
        }
    }
    if let Some((_, h)) = current {
        if h >= KEEP_MIN_HEIGHT {
            return Some((Choice::Keep, Source::Current));
        }
    }
    let current_area = current.map_or(0, |(w, h)| w as u64 * h as u64);
    let mut best: Option<(u32, u64)> = None;
    for m in offered.iter().filter(|m| usable(m.width, m.height)) {
        if let Some((nw, nh)) = native {
            if m.width > nw || m.height > nh {
                continue;
            }
        }
        let area = m.width as u64 * m.height as u64;
        if area > FALLBACK_MAX_PIXELS || area <= current_area {
            continue;
        }
        if best.is_none_or(|(_, a)| area > a) {
            best = Some((m.index, area));
        }
    }
    match (best, current) {
        (Some((index, _)), _) => Some((Choice::Set(index), Source::Largest)),
        (None, Some(_)) => Some((Choice::Keep, Source::Current)),
        (None, None) => None,
    }
}

/// The physical size packed for the handoff's framebuffer record: width
/// in millimetres in the low half, height in the high half, 0 for unknown.
pub fn pack_mm(width_mm: u32, height_mm: u32) -> u32 {
    if width_mm == 0 || height_mm == 0 || width_mm > 0xFFFF || height_mm > 0xFFFF {
        return 0;
    }
    width_mm | (height_mm << 16)
}
