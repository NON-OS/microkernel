// NONOS Operating System (AGPL-3.0-or-later)
//! The menu bar's rows as menu items: every title keeps the labels and the
//! actions it had, in the order the dispatcher switches on; a row shows the
//! keys that send its action; only the rows that delete are danger rows;
//! and a ribbon pill's values tick only the value in effect.

use nonos_toolkit::widgets::Item;

use crate::editor::menu_view::ticked;
use crate::editor::menubar::items::MenuCmd::{self, *};
use crate::editor::menubar::items::{rows, TITLES};

const KEPT: [&[(&str, MenuCmd)]; 8] = [
    &[("New Tab", NewTab), ("Open...", Ctrl(0x4F, false)), ("Save", Ctrl(0x53, false))],
    &[("Undo", Ctrl(0x5A, false)), ("Redo", Ctrl(0x59, false)), ("Cut", Ctrl(0x58, false))],
    &[("Zoom In", Ctrl(0x3D, false)), ("Zoom Out", Ctrl(0x2D, false))],
    &[("Image", Markup(true)), ("Link", Markup(false)), ("Page Break", PageBreak)],
    &[("Bold", Ctrl(0x42, true)), ("Italic", Ctrl(0x49, false))],
    &[("Duplicate Line", Ctrl(0x44, false)), ("Delete Line", Ctrl(0x4B, true))],
    &[("Insert Table", Table(0)), ("Insert Row", Table(1)), ("Insert Column", Table(2))],
    &[("Keyboard Shortcuts", Info(0)), ("About NONOS Docs", Info(1))],
];
#[rustfmt::skip]
const LATER: [(usize, usize, &str, MenuCmd); 17] = [
    (0, 3, "Save As...", Ctrl(0x53, true)), (0, 4, "Export...", Ctrl(0x45, false)),
    (0, 5, "Close Tab", CloseTab), (1, 3, "Copy", Ctrl(0x43, false)),
    (1, 4, "Paste", Ctrl(0x56, false)), (1, 5, "Select All", Ctrl(0x41, false)),
    (1, 6, "Find...", Ctrl(0x46, false)), (1, 7, "Replace...", Ctrl(0x48, false)),
    (1, 8, "Replace All", Ctrl(0x48, true)), (2, 2, "Reset Zoom", Ctrl(0x30, false)),
    (2, 3, "Toggle Theme", Theme), (2, 4, "Toggle Sidebar", ToggleSidebar),
    (3, 3, "Special Character", Special), (4, 2, "Underline", Ctrl(0x55, false)),
    (4, 3, "Toggle Comment", Ctrl(0x2F, false)), (5, 2, "Word Count", WordCount),
    (6, 3, "Delete Table", Table(3)),
];
const COUNTS: [usize; 8] = [6, 9, 5, 4, 4, 3, 4, 2];

/// The keys that send `cmd`, worked out from the action, not the table.
fn keys(cmd: MenuCmd) -> String {
    match cmd {
        Ctrl(c, s) => format!("Ctrl+{}{}", ["", "Shift+"][usize::from(s)], c as u8 as char),
        NewTab => "Ctrl+N".into(),
        CloseTab => "Ctrl+W".into(),
        ToggleSidebar => "Ctrl+B".into(),
        Theme => "Ctrl+K T".into(),
        _ => String::new(),
    }
}

fn kept(t: usize, i: usize, label: &str, cmd: MenuCmd) {
    let k = keys(cmd);
    let it = Item::new(label).shortcut(&k);
    let it = if label.starts_with("Delete") { it.danger() } else { it };
    assert!(rows(t)[i].0 == it && rows(t)[i].1 == cmd, "{} row {i}: {label}", TITLES[t]);
}

#[test]
fn every_row_keeps_its_label_action_and_keys() {
    let row =
        |t: usize| move |(i, &(label, cmd)): (usize, &(&'static str, MenuCmd))| (t, i, label, cmd);
    let firsts = KEPT.iter().enumerate().flat_map(|(t, r)| r.iter().enumerate().map(row(t)));
    firsts.chain(LATER).for_each(|(t, i, label, cmd)| kept(t, i, label, cmd));
    let all: usize = KEPT.iter().map(|r| r.len()).sum::<usize>() + LATER.len();
    assert_eq!((0..8).map(|t| rows(t).len()).collect::<Vec<_>>(), COUNTS);
    assert_eq!(all, COUNTS.iter().sum::<usize>(), "every row is pinned");
}

#[test]
fn a_pill_ticks_only_the_value_in_effect() {
    let labels: Vec<String> = ["10", "16", "48"].map(String::from).to_vec();
    for (cur, on) in [("16", Some(1)), ("13", None)] {
        let got = ticked(&labels, cur);
        let want = |i: usize| Item::new(&labels[i]).checked(on == Some(i));
        assert!(got.len() == 3 && (0..3).all(|i| got[i] == want(i)), "{cur}");
    }
}
