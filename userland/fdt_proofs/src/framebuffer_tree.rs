// NONOS Operating System (AGPL-3.0-or-later)
//! A tree with one `simple-framebuffer` node, placed where firmware puts it.

use crate::arch::fdt::find::framebuffer::{find, PixelFormat, Refused, SimpleFramebuffer};
use crate::arch::fdt::Fdt;
use crate::builder::{cells, text, Builder, END};

/// A 1920x1080 32-bit framebuffer at 0x3e40_0000, under `parent` (root when
/// empty), with `format` and `stride`; `ranges` on the parent when given.
pub(crate) fn tree(
    parent: &str,
    ranges: Option<&[u32]>,
    format: &str,
    stride: u32,
    status: &str,
) -> Vec<u8> {
    let mut b = Builder::default().begin("");
    b = b.prop("#address-cells", &cells(&[2])).prop("#size-cells", &cells(&[1]));
    if !parent.is_empty() {
        b = b.begin(parent).prop("#address-cells", &cells(&[2])).prop("#size-cells", &cells(&[1]));
        if let Some(r) = ranges {
            b = b.prop("ranges", &cells(r));
        }
    }
    b = b.begin("framebuffer@3e400000").prop("compatible", &text("simple-framebuffer"));
    b = b.prop("reg", &cells(&[0, 0x3e40_0000, 0x7e_9000]));
    b = b.prop("width", &cells(&[1920])).prop("height", &cells(&[1080]));
    b = b.prop("stride", &cells(&[stride])).prop("format", &text(format));
    b = b.prop("status", &text(status)).end();
    if !parent.is_empty() {
        b = b.end();
    }
    b.end().token(END).finish()
}

pub(crate) fn found(blob: &[u8]) -> Result<Option<SimpleFramebuffer>, Refused> {
    find(&Fdt::from_bytes(blob).unwrap())
}

pub(crate) const FB: SimpleFramebuffer = SimpleFramebuffer {
    base: 0x3e40_0000,
    size: 0x7e_9000,
    width: 1920,
    height: 1080,
    stride: 7680,
    format: PixelFormat::Bgrx,
};
