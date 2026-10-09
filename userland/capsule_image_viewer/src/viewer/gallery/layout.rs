pub const THUMB_W: u32 = 160;
pub const THUMB_H: u32 = 120;
pub const GAP: u32 = 12;
pub const HEADER_H: u32 = 28;
pub const LABEL_H: u32 = 16;

pub struct Grid {
    pub cols: u32,
    pub cell_w: u32,
    pub cell_h: u32,
}

pub fn grid(win_w: u32) -> Grid {
    let cell_w = THUMB_W + GAP;
    let cell_h = THUMB_H + LABEL_H + GAP;
    let cols = (win_w.saturating_sub(GAP) / cell_w).max(1);
    Grid { cols, cell_w, cell_h }
}

pub fn cell_rect(i: usize, scroll_rows: usize, g: &Grid) -> (i32, i32, u32, u32) {
    let col = (i as u32) % g.cols;
    let row = (i as u32) / g.cols;
    let x = (GAP + col * g.cell_w) as i32;
    let y = HEADER_H as i32 + GAP as i32 + (row as i32 - scroll_rows as i32) * g.cell_h as i32;
    (x, y, THUMB_W, THUMB_H + LABEL_H)
}

pub fn rows(count: usize, g: &Grid) -> usize {
    let cols = g.cols.max(1) as usize;
    count.div_ceil(cols)
}

/// Rows that fit whole below the header: what the keyboard keeps in view and
/// what the last page of `max_scroll` shows.
pub fn full_rows(win_h: u32, g: &Grid) -> usize {
    (win_h.saturating_sub(HEADER_H) / g.cell_h).max(1) as usize
}

pub fn max_scroll(count: usize, win_h: u32, g: &Grid) -> usize {
    rows(count, g).saturating_sub(full_rows(win_h, g))
}

/// The tiles on screen, `[start, end)`, when `scroll` rows are scrolled past:
/// every row that starts above the window's bottom edge, whole or not.
pub fn visible(count: usize, scroll: usize, win_h: u32, g: &Grid) -> (usize, usize) {
    let cols = g.cols.max(1) as usize;
    let body = win_h.saturating_sub(HEADER_H + GAP);
    let shown = body.div_ceil(g.cell_h) as usize;
    ((scroll * cols).min(count), ((scroll + shown) * cols).min(count))
}

/// The tiles whose thumbnails are kept: those on screen and one row either
/// side, so a one-row scroll shows pictures rather than empty tiles. The rest
/// are dropped, which is what lets the gallery list every image the store
/// holds without its thumbnails outgrowing the heap.
pub fn kept(count: usize, scroll: usize, win_h: u32, g: &Grid) -> (usize, usize) {
    let cols = g.cols.max(1) as usize;
    let (start, end) = visible(count, scroll, win_h, g);
    (start.saturating_sub(cols), (end + cols).min(count))
}

/// The scroll that brings tile `sel` into view, moving no further than it
/// must, so the keyboard's selection never walks off the window.
pub fn follow(scroll: usize, sel: usize, count: usize, win_h: u32, g: &Grid) -> usize {
    let cols = g.cols.max(1) as usize;
    let row = sel / cols;
    let shown = full_rows(win_h, g);
    let next = if row < scroll {
        row
    } else if row >= scroll + shown {
        row + 1 - shown
    } else {
        scroll
    };
    next.min(max_scroll(count, win_h, g))
}
