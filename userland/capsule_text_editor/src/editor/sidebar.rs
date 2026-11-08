//! The file-explorer sidebar: an EXPLORER label over the explorer's rows,
//! with any failed file operation's reason beside the label, and the rows'
//! pointer fed to their controls.

use alloc::vec::Vec;

use nonos_app_skeleton::{feed, InputEvent, PaintBuffer};
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::rect::{inset, split_left};
use nonos_toolkit::widgets::{draw, Ink, Label, Rect, Step};

use super::app::Editor;
use super::explorer::{Entry, Explorer};
use super::layout::{ACTIVITY_W, FOOTER_H, ROW_H, SIDEBAR_W, TABBAR_H, TITLEBAR_H};
use super::shell::pane_y;
use super::tree::{empty_tree_line, explorer_note};

const HEAD: &str = "EXPLORER";
const PAD: u32 = 14;

impl Editor {
    /// Paints the sidebar while it is open; closed, it lays out no rows.
    pub(super) fn paint_sidebar(&mut self, fb: &mut PaintBuffer) {
        self.ex_ctl.clear();
        if !self.sidebar_open {
            return;
        }
        let (r, height) = (roles(), fb.height);
        let side = Rect { x: ACTIVITY_W, y: 0, w: SIDEBAR_W, h: height };
        draw::fill(fb, side, r.sunken);
        let edge = scale().hair();
        draw::fill(fb, Rect { x: side.x + side.w - edge, w: edge, ..side }, r.border);
        let band = Rect { y: TITLEBAR_H, h: TABBAR_H, ..inset(side, PAD) };
        let head = Label::new(HEAD).ink(Ink::Muted);
        head.draw(fb, band);
        let tree = &self.tree;
        if let (false, Some(note)) = (tree.visible.is_empty(), explorer_note(tree.status)) {
            let (_, rest) = split_left(band, head.size().0.saturating_add(10));
            Label::new(note).ink(Ink::Accent).draw(fb, rest);
        }
        let top = pane_y(self.ribbon_shown()) + if self.entry.is_some() { ROW_H } else { 0 };
        let rows = Rect { y: top, h: height.saturating_sub(top + FOOTER_H), ..side };
        if tree.visible.is_empty() {
            let line = Rect { y: top, h: ROW_H, ..band };
            Label::new(empty_tree_line(tree.status)).ink(Ink::Muted).draw(fb, line);
            return;
        }
        let entry = |n: &usize| {
            let node = &tree.nodes[*n];
            let open = node.is_dir && tree.expanded.contains(&node.path);
            Entry { name: &node.name, depth: node.depth, dir: node.is_dir, open }
        };
        let entries: Vec<Entry> = tree.visible.iter().map(entry).collect();
        let first = tree.scroll as usize;
        let (selected, row_h) = (tree.selected, ROW_H);
        Explorer { entries: &entries, first, selected, band: rows, row_h }.paint(fb, &mut self.ex_ctl);
    }

    /// Feeds a pointer event to the explorer's rows. While covered the pointer
    /// has left them, so no row lights or acts beneath a menu.
    pub(super) fn explorer_step(&mut self, event: &InputEvent) -> Step<usize> {
        if self.covered() {
            return self.ex_ctl.left();
        }
        feed(&mut self.ex_ctl, event)
    }
}
