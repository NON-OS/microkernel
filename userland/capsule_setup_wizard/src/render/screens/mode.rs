use crate::render::theme::{FG, HINT};
use crate::render::{self, widgets::lines, widgets::rows};
use crate::server::step::{default_key, list_nav, Outcome, K_ENTER, K_ENTER_LF};
use crate::state::Context;

/// Rows after the first, which is amnesic.
pub const USB_LIVE: u8 = 1;
pub const INSTALL: u8 = 2;

/// What setup's exit status asks of the kernel: start the desktop, and for
/// 3 open the installer beside it.
const EXIT_DESKTOP: i32 = 0;
const EXIT_INSTALLER: i32 = 3;

const MODES: &[&[u8]] =
    &[b"Amnesic (default)", b"USB live (unavailable)", b"Install to this computer"];

const WHY: [&[&[u8]]; 3] = [
    &[b"RAM only. Nothing is written to any disk.", b"Setup runs again on the next boot."],
    &[
        b"State kept encrypted on the boot stick.",
        b"Unavailable: NONOS keeps state only on an NVMe, SATA or",
        b"virtio disk today, and has no passphrase-keyed volume.",
    ],
    &[
        b"Keeps these answers in the store on this boot's NONOS disk,",
        b"so setup does not run again, then opens the installer.",
        b"The installer copies this boot image (loader, kernel, boot",
        b"config) to a disk you name. It does not copy these answers.",
    ],
];

/// Whether this mode keeps state across boots.
pub fn keeps(ctx: &Context) -> bool {
    ctx.mode_sel == INSTALL
}

pub fn exit_code(ctx: &Context) -> i32 {
    if ctx.mode_sel == INSTALL {
        EXIT_INSTALLER
    } else {
        EXIT_DESKTOP
    }
}

pub fn draw(ctx: &Context) {
    render::frame(ctx, b"Mode", b"Where this machine keeps what you do", b"ENTER NEXT  ESC BACK");
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let (buf, x) = (render::buffer(ctx), render::content_x(w));
    rows::list(buf, spx, w, h, x, 110, MODES, ctx.mode_sel as usize);
    let why = WHY[ctx.mode_sel as usize % WHY.len()];
    let color = if ctx.mode_sel == USB_LIVE { HINT } else { FG };
    lines::text(buf, spx, w, h, x, 110 + 30 * MODES.len() as u32 + 16, why, color);
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    if let Some(o) = list_nav(&mut ctx.mode_sel, MODES.len() as u8, code) {
        return o;
    }
    if ctx.mode_sel == USB_LIVE && (code == K_ENTER || code == K_ENTER_LF) {
        return Outcome::Stay;
    }
    default_key(code)
}
