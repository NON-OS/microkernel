// NONOS Operating System (AGPL-3.0-or-later)
//! The status bar is one shared `Row` in the footer: its body line fits the
//! footer at 1x, as the old fixed status size did.

use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::design::text_role::TextRole;
use nonos_toolkit::font::ttf::line_height;

use super::square_tests::lock;
use crate::layout::footer_h;

#[test]
fn the_status_line_fits_the_footer_at_1x() {
    let _g = lock();
    set_scale(4);
    assert!(line_height(scale().font(TextRole::Body.px())) < footer_h() as i32);
}
