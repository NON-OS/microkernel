use crate::render::{self, widgets::rows};
use crate::server::step::{default_key, list_nav, Outcome};
use crate::state::Context;

// Names shown in the list, and the wallpaper catalog index each one sets.
const WALLS: &[&[u8]] = &[b"Circuit", b"Emblem", b"Halo", b"Grid", b"Tiles", b"Lattice"];
const CATALOG: [u8; 6] = [55, 13, 20, 27, 33, 60];

// The catalog index for the chosen row; the first row is the system default.
pub fn wallpaper(sel: u8) -> u8 {
    CATALOG.get(sel as usize).copied().unwrap_or(CATALOG[0])
}

pub fn draw(ctx: &Context) {
    render::frame(ctx, b"Appearance", b"j/k to choose a wallpaper", b"ENTER NEXT  ESC BACK");
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let buf = render::buffer(ctx);
    rows::list(buf, spx, w, h, render::content_x(w), 110, WALLS, ctx.wall_sel as usize);
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    if let Some(o) = list_nav(&mut ctx.wall_sel, WALLS.len() as u8, code) {
        return o;
    }
    default_key(code)
}
