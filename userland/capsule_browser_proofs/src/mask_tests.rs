// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! mask-image: a url mask takes the box's background image slot and marks
//! it a mask, so paint shows the background color only through its alpha;
//! a gradient mask is not drawn and leaves the box as it was.

use crate::probe::Page;

const VP: (u32, u32) = (800, 600);

#[test]
fn a_url_mask_marks_the_box_and_keeps_its_color_and_size() {
    let html = "<style>.icon{display:inline-block;width:16px;height:16px;\
                background-color:#123456;mask-image:url(/icons/chevron.svg);mask-size:cover}</style>\
                <i class=icon id=i></i>";
    let p = Page::at(html, VP);
    let f = p.frag("i");
    assert!(f.mask);
    assert_eq!(f.bg_image.as_deref(), Some("/icons/chevron.svg"));
    assert_eq!(f.bg & 0xff_ffff, 0x12_3456);
    /* Cover: an 8x4 mask scaled by 4 to cover the 16px box, at 0 0. */
    assert_eq!(f.bg_layer.tile((8, 4), [0, 0, 16, 16]), [0, 0, 32, 16]);
}

#[test]
fn the_webkit_prefix_masks_as_well() {
    let html = "<i id=i style=\"display:block;height:9px;background:red;\
                -webkit-mask-image:url(m.svg)\"></i>";
    assert!(Page::at(html, VP).frag("i").mask);
}

#[test]
fn a_gradient_mask_leaves_the_box_unmasked() {
    let html = "<div id=d style=\"height:9px;background:red;\
                mask-image:linear-gradient(to bottom, #000, transparent)\"></div>";
    let p = Page::at(html, VP);
    assert!(!p.frag("d").mask);
    assert_eq!(p.frag("d").bg_image, None);
}
