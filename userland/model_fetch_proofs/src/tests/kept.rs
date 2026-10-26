// NONOS Operating System (AGPL-3.0-or-later)
//! The host tool's catalogue at load: admitted as the fetcher admits it, a
//! name too long to keep refused, and the hand-named tiers' files kept.

use super::{published, written};
use crate::catalogue::admit::admit;
use crate::catalogue::parse::parse;
use crate::catalogue::types::File;
use crate::pins::all;

/* The host tool's catalogue is admitted as the fetcher admits it at load. */
#[test]
fn the_catalogue_is_admitted_and_a_name_too_long_to_keep_is_refused_at_load() {
    let blob = written();
    let mut cat = parse(&blob[..blob.len() - 64]).unwrap();
    assert_eq!(admit(&cat), Ok(()));
    assert!(cat.tiers.iter().flat_map(|t| &t.files).all(File::keepable));
    let small = cat.tiers.iter_mut().find(|t| t.word == "small").unwrap();
    small.memory += 1;
    assert_eq!(
        admit(&cat),
        Err("the model catalogue gives a tier a memory need its shape does not")
    );
    let mut cat = parse(&blob[..blob.len() - 64]).unwrap();
    let coder = cat.tiers.iter_mut().find(|t| t.word == "coder-7b").unwrap();
    coder.files[0].name = published(&coder.files[0].name);
    assert_eq!(coder.files[0].name.len(), 52);
    assert_eq!(admit(&cat), Err("the model catalogue names a file the data volume cannot keep"));
}

/* The tiers named by hand: their files fit, and every Coder part does now. */
#[test]
fn the_small_4b_and_coder_files_are_kept() {
    for tier in ["small", "qwen3-0.6b", "qwen3-4b", "coder-7b", "coder-14b", "coder-32b"] {
        let files: Vec<_> = all().filter(|p| p.tier == tier).collect();
        assert!(!files.is_empty(), "{tier}");
        for p in files {
            assert!(crate::pins::keepable(p.name), "{tier}");
            assert!(p.name.len() <= 49, "{tier}");
        }
    }
}
