// NONOS Operating System (AGPL-3.0-or-later)
//! Structure blocks that are not one balanced tree never become an `Fdt`.

use crate::arch::fdt::{Fdt, FdtError};
use crate::blobs::err;
use crate::builder::{Builder, END};

fn root() -> Builder {
    Builder::default().begin("")
}

#[test]
fn a_tree_must_end_with_fdt_end() {
    assert_eq!(err(&root().end().finish()), FdtError::MissingEnd);
    assert_eq!(err(&root().token(END).finish()), FdtError::Unbalanced);
}

#[test]
fn closing_more_than_was_opened_is_refused() {
    assert_eq!(err(&root().end().end().token(END).finish()), FdtError::Unbalanced);
}

#[test]
fn exactly_one_unnamed_root() {
    let two = root().end().begin("").end().token(END).finish();
    assert_eq!(err(&two), FdtError::BadRoot);
    let named = Builder::default().begin("x").end().token(END).finish();
    assert_eq!(err(&named), FdtError::BadRoot);
    let none = Builder::default().token(END).finish();
    assert_eq!(err(&none), FdtError::BadRoot);
    let stray = Builder::default().prop("a", &[]).begin("").end().token(END).finish();
    assert_eq!(err(&stray), FdtError::BadRoot);
}

#[test]
fn properties_must_precede_subnodes() {
    let late = root().begin("a").end().prop("x", &[1, 2, 3, 4]).end().token(END).finish();
    assert_eq!(err(&late), FdtError::PropertyAfterNode);
}

#[test]
fn nesting_stops_at_sixteen() {
    let nest = |levels: usize| {
        let mut b = root();
        for _ in 1..levels {
            b = b.begin("n");
        }
        for _ in 0..levels {
            b = b.end();
        }
        b.token(END).finish()
    };
    assert!(Fdt::from_bytes(&nest(16)).is_ok());
    assert_eq!(err(&nest(17)), FdtError::TooDeep);
}

#[test]
fn unknown_tokens_and_bad_payloads_are_refused() {
    assert_eq!(err(&root().token(7).end().token(END).finish()), FdtError::UnknownToken);
    // A property whose length runs past the block.
    let long = root().token(3).token(0x7fff_fff0).token(0).end().token(END).finish();
    assert_eq!(err(&long), FdtError::TruncatedProperty);
    // A property whose name offset is past the strings block.
    let unnamed = root().token(3).token(0).token(4096).end().token(END).finish();
    assert_eq!(err(&unnamed), FdtError::StringMissing);
    // A node name that runs to the end of the block with no terminator.
    let mut open = root().token(1).finish();
    let n = open.len();
    open.extend_from_slice(b"abcd");
    let size = u32::from_be_bytes(open[36..40].try_into().unwrap()) + 4;
    open[36..40].copy_from_slice(&size.to_be_bytes());
    open[4..8].copy_from_slice(&(n as u32 + 4).to_be_bytes());
    open[12..16].copy_from_slice(&(n as u32 + 4).to_be_bytes());
    assert_eq!(err(&open), FdtError::TruncatedNodeName);
}
