use crate::render::theme::{FG, HINT};
use crate::render::{self, widgets::lines, widgets::rows};
use crate::server::step::{default_key, list_nav, Outcome, K_ENTER, K_ENTER_LF};
use crate::state::Context;

/// Rows after the first, which is amnesic, defined beside their rule.
pub use crate::keep::{INSTALL, USB_LIVE};

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
        b"Opens the installer. It writes this boot image and a new store",
        b"to a disk you name, carrying your name, keyboard, time zone,",
        b"wallpaper, Qwen model, apps and signed programs, so setup does",
        b"not run there. The disk this boot came from keeps no answers:",
        b"started from it again, NONOS is amnesic and runs setup.",
    ],
];

/// Whether this mode keeps state across boots: Install, on the disk the
/// installer writes. Its answers stay out of this boot's store (`keep::answers_for`).
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
    let l = render::layout_of(ctx);
    let (cv, x) = (&mut render::canvas(ctx), l.col_x);
    let y = rows::list(cv, x, l.body_y, MODES, ctx.mode_sel as usize);
    let why = WHY[ctx.mode_sel as usize % WHY.len()];
    let color = if ctx.mode_sel == USB_LIVE { HINT } else { FG };
    lines::text(cv, x, y + l.gap, why, color);
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
