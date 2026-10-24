// NONOS Operating System (AGPL-3.0-or-later)
//! The `simple-framebuffer` finder: the node where firmware puts it, each
//! format string the kernel can draw, and every node it must refuse.

use crate::arch::fdt::find::framebuffer::{PixelFormat, Refused};
use crate::builder::{cells, text, Builder, END};
use crate::fixtures::ALL;
use crate::framebuffer_tree::{found, tree, FB};

#[test]
fn the_node_is_found_at_the_root_and_under_chosen_with_or_without_ranges() {
    assert_eq!(found(&tree("", None, "a8r8g8b8", 7680, "okay")), Ok(Some(FB)));
    assert_eq!(found(&tree("chosen", None, "a8r8g8b8", 7680, "okay")), Ok(Some(FB)));
    assert_eq!(found(&tree("chosen", Some(&[]), "a8r8g8b8", 7680, "okay")), Ok(Some(FB)));
    // A bus that moves it: translated, not read as a CPU address.
    let soc = [0, 0x3e00_0000, 0x1, 0x3e00_0000, 0x0100_0000];
    let moved = found(&tree("soc", Some(&soc), "a8r8g8b8", 7680, "okay")).unwrap().unwrap();
    assert_eq!(moved.base, 0x1_3e40_0000);
    // Not under /chosen and no `ranges`: not mapped, as the spec says.
    assert!(matches!(found(&tree("soc", None, "a8r8g8b8", 7680, "okay")), Err(Refused::Tree(_))));
}

#[test]
fn each_format_names_its_bytes_in_memory() {
    let format = |f| found(&tree("", None, f, 7680, "okay")).unwrap().unwrap().format;
    assert_eq!(format("x8r8g8b8"), PixelFormat::Bgrx);
    assert_eq!(format("x8b8g8r8"), PixelFormat::Rgbx);
    assert_eq!(format("a8b8g8r8"), PixelFormat::Rgbx);
    let packed = found(&tree("", None, "r8g8b8", 5760, "okay")).unwrap().unwrap();
    assert_eq!((packed.format, packed.format.bytes_per_pixel()), (PixelFormat::Bgr, 3));
}

#[test]
fn absent_disabled_and_incomplete_nodes() {
    assert_eq!(found(&tree("", None, "a8r8g8b8", 7680, "disabled")), Ok(None));
    for blob in ALL {
        assert_eq!(found(blob), Ok(None));
    }
    let mut b =
        Builder::default().begin("").begin("fb").prop("compatible", &text("simple-framebuffer"));
    b = b.prop("reg", &cells(&[0x1000, 0x1000])).prop("width", &cells(&[16]));
    let blob = b.end().end().token(END).finish();
    assert_eq!(found(&blob), Err(Refused::Missing));
}
