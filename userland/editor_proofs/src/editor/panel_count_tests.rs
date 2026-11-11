// NONOS Operating System (AGPL-3.0-or-later)
//! The Word Count panel as the shared card of rows, at 1x to 2x: it shows
//! the document's counts, and no row lights or picks under the pointer.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::{Controls, Step};

use super::panel_tests::{paint, view, H};
use super::square_tests::{centre, lock};
use super::strip_tests::{changed, W};
use crate::doc::counts::char_count;
use crate::doc::text_bridge::doc_from_text;
use crate::editor::panel::Panel;
use crate::editor::wordcount::count_rows;

#[test]
fn word_count_shows_the_documents_counts_and_picks_nothing() {
    let _g = lock();
    let text = b"one two\n\nthree four five\n";
    let owned = count_rows(text);
    let chars = char_count(&doc_from_text(text)).to_string();
    let want = [("Words", "5".to_string()), ("Characters", chars), ("Paragraphs", "2".into())];
    assert_eq!(owned, want);
    let rows: Vec<(&str, &str)> = owned.iter().map(|(l, v)| (*l, v.as_str())).collect();
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let (v, c) = (view(Panel::WordCount, &rows), &mut Controls::acting_on_press());
        let still = paint(&v, c);
        for (_, at) in v.rows_at(v.rect(W, H, 26)) {
            let (x, y) = centre(at);
            (c.moved(x, y), assert_eq!(c.pressed(x, y), Step::Idle, "no pick at {q}"));
            (c.released(), assert_eq!(paint(&v, c), still, "nothing lights at {q}"));
        }
        let other = [rows[0], rows[1], ("Paragraphs", "7")];
        assert!(!changed(&still, &paint(&view(Panel::WordCount, &other), c)).is_empty());
    }
    set_scale(4);
}
