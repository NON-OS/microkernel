extern crate alloc;
use crate::viewer::budget::{refuse_bytes, READ_LIMIT};
use crate::viewer::decode::decode;
use crate::viewer::gallery::layout::{grid, kept, visible};
use crate::viewer::gallery::state::{Entry, GalleryState, THUMB_H, THUMB_W};
use crate::viewer::says::{decoder_trouble, SILENT, THUMB_QUIET_MS};
use crate::viewer::scale::{draw_bilinear, Dst};
use alloc::vec;
use nonos_app_skeleton::clients::vfs::read_file;

pub fn fit_thumb(sw: u32, sh: u32) -> (u32, u32) {
    if sw == 0 || sh == 0 {
        return (1, 1);
    }
    let sw_s = THUMB_W as f32 / sw as f32;
    let sh_s = THUMB_H as f32 / sh as f32;
    let mut s = if sw_s < sh_s { sw_s } else { sh_s };
    if s > 1.0 {
        s = 1.0;
    }
    (((sw as f32 * s) as u32).max(1), ((sh as f32 * s) as u32).max(1))
}

/// One step of thumbnail work for a `win_w` by `win_h` gallery: drop the
/// thumbnails that have scrolled well away, then decode the first missing one
/// on screen, or failing that the first in the row either side. True when
/// something changed and the gallery should be drawn again.
pub fn decode_next(g: &mut GalleryState, owner_pid: u32, win_w: u32, win_h: u32) -> bool {
    let now = nonos_libc::mk_uptime_ms().max(0) as u64;
    if now < g.quiet_until_ms {
        return false;
    }
    let gr = grid(win_w);
    let count = g.entries.len();
    let (keep_from, keep_to) = kept(count, g.scroll, win_h, &gr);
    for (i, e) in g.entries.iter_mut().enumerate() {
        if (i < keep_from || i >= keep_to) && e.thumb.is_some() {
            e.thumb = None;
        }
    }
    let (show_from, show_to) = visible(count, g.scroll, win_h, &gr);
    let wanted = |e: &Entry| e.thumb.is_none() && !e.failed;
    let next = (show_from..show_to)
        .find(|&i| wanted(&g.entries[i]))
        .or_else(|| (keep_from..keep_to).find(|&i| wanted(&g.entries[i])));
    let Some(i) = next else { return false };
    match make_thumb(&mut g.entries[i], owner_pid) {
        Thumb::Made | Thumb::Failed(None) => {}
        Thumb::Failed(Some(trouble)) => g.decoder_trouble = Some(trouble),
        // The tile is left to try again: the file is not at fault.
        Thumb::Silent => g.quiet_until_ms = now.saturating_add(THUMB_QUIET_MS),
    }
    true
}

enum Thumb {
    Made,
    /// The tile is marked; the decoder's own trouble, when it is that.
    Failed(Option<&'static str>),
    /// The store did not answer the read.
    Silent,
}

/// Decode one tile's thumbnail. A failure marks the tile; one that is the
/// decoder's rather than the file's is handed back for the gallery to say.
fn make_thumb(e: &mut Entry, owner_pid: u32) -> Thumb {
    let bytes = match read_file(owner_pid, e.path.as_bytes(), READ_LIMIT) {
        Ok(b) if refuse_bytes(b.len() as u64).is_none() => b,
        Err(SILENT) => return Thumb::Silent,
        _ => {
            e.failed = true;
            return Thumb::Failed(None);
        }
    };
    let d = match decode(&bytes, e.path.as_bytes()) {
        Ok(d) => d,
        Err(err) => {
            e.failed = true;
            return Thumb::Failed(decoder_trouble(err));
        }
    };
    let (tw, th) = fit_thumb(d.w, d.h);
    let mut thumb = vec![0u32; (tw * th) as usize];
    let mut dst = Dst { px: &mut thumb, stride: tw, w: tw, h: th };
    draw_bilinear(&mut dst, &d.px, d.w, d.h, 0, 0, tw, th);
    e.tw = tw;
    e.th = th;
    e.thumb = Some(thumb);
    Thumb::Made
}
