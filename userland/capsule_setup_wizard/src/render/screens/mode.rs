use crate::render::theme::{FG, HINT};
use crate::render::{self, widgets::lines, widgets::rows};
use crate::server::step::{default_key, list_nav, Outcome, K_ENTER, K_ENTER_LF};
use crate::state::Context;

/// Rows after the first, which is amnesic.
pub const USB_LIVE: u8 = 1;
pub const INSTALL: u8 = 2;

/*
 * Setup's exit status in its low byte: 0 starts the desktop, and 3 hands the
 * whole screen to the installer with no desktop behind it. The apps turned
 * off ride in the byte above (nonos_policy_proto::apps::exit_status).
 */
const EXIT_DESKTOP: u8 = 0;
pub const EXIT_INSTALLER: u8 = 3;

const MODES: &[&[u8]] =
    &[b"Amnesic (default)", b"USB live (unavailable)", b"Install to this computer"];

const WHY: [&[&[u8]]; 3] = [
    &[
        b"RAM only. Nothing is written to any disk.",
        b"Setup runs again on the next boot. Install NONOS later from the",
        b"dock and the installed disk starts with these answers.",
    ],
    &[
        b"State kept encrypted on the boot stick.",
        b"Unavailable: NONOS keeps state only on an NVMe, SATA or",
        b"virtio disk today, and has no passphrase-keyed volume.",
    ],
    &[
        b"Keeps your name, keyboard, time zone, wallpaper, Qwen model and",
        b"apps in the store on this boot's NONOS disk, so setup does not",
        b"run again, then opens the installer. The installer writes this",
        b"boot image and a new store to a disk you name. That store",
        b"carries those answers and signed programs, so setup does not",
        b"run there either.",
    ],
];

/// Whether this mode keeps state across boots.
pub fn keeps(ctx: &Context) -> bool {
    ctx.mode_sel == INSTALL
}

pub fn exit_code(ctx: &Context) -> i32 {
    let what = if ctx.mode_sel == INSTALL { EXIT_INSTALLER } else { EXIT_DESKTOP };
    nonos_policy_proto::apps::exit_status(what, ctx.apps_off)
}

pub fn draw(ctx: &Context) {
    let sub: &[u8] = if ctx.install_boot {
        b"You chose Install NONOS at boot; Amnesic installs nothing"
    } else {
        b"Where this machine keeps what you do"
    };
    render::frame(ctx, b"Mode", sub, b"ENTER NEXT  ESC BACK");
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let l = render::layout_of(ctx);
    let (buf, x) = (render::buffer(ctx), l.col_x);
    let y = rows::list(buf, spx, w, h, x, l.body_y, MODES, ctx.mode_sel as usize);
    let why = WHY[ctx.mode_sel as usize % WHY.len()];
    let color = if ctx.mode_sel == USB_LIVE { HINT } else { FG };
    lines::text(buf, spx, w, h, x, y + l.gap, why, color);
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
