// NONOS Operating System (AGPL-3.0-or-later)
//! The prompt's branch and job count: read right, looked for where git keeps
//! them, and dropped before the path when the line runs out.

use super::prompt_bits::{branch, fit, heads, jobs_label, UP};

#[test]
fn head_names_the_branch_or_the_detached_commit() {
    assert_eq!(branch(b"ref: refs/heads/main\n"), Some(&b"main"[..]));
    assert_eq!(branch(b"ref: refs/heads/feat/panes"), Some(&b"feat/panes"[..]));
    let id = b"0123456789abcdef0123456789abcdef01234567\n";
    assert_eq!(branch(id), Some(&b"0123456"[..]));
    assert_eq!(branch(b"ref: refs/heads/\n"), None);
    assert_eq!(branch(b"ref: refs/heads/a b"), None, "no space in a branch name");
    assert_eq!(branch(b"0123"), None);
    assert_eq!(branch(b"hello"), None);
    assert_eq!(branch(b"not a head at all"), None);
    assert_eq!(branch(b"0123456789abcdef"), None, "a commit id is 40 characters");
}

#[test]
fn the_work_tree_is_looked_for_upward_and_no_further() {
    let p: Vec<String> =
        heads(b"/home/nonos/w/proj").iter().map(|v| String::from_utf8(v.clone()).unwrap()).collect();
    assert_eq!(p[0], "/home/nonos/w/proj/.git/HEAD");
    assert_eq!(p[1], "/home/nonos/w/.git/HEAD");
    assert_eq!(p.last().unwrap(), "/home/.git/HEAD", "stops below the root");
    assert_eq!(heads(b"/a/b/c/d/e/f/g").len(), UP + 1);
    assert_eq!(heads(b"/home/").len(), 1);
}

#[test]
fn jobs_are_counted_in_words() {
    let mut b = [0u8; 12];
    assert_eq!(jobs_label(0, &mut b), b"");
    assert_eq!(jobs_label(1, &mut b), b"1 job");
    assert_eq!(jobs_label(12, &mut b), b"12 jobs");
}

#[test]
fn jobs_give_way_first_and_the_path_never_does() {
    assert_eq!(fit(40, 10, 4, 6), (true, true), "both fit");
    assert_eq!(fit(17, 10, 4, 6), (true, false), "room for the branch alone");
    assert_eq!(fit(17, 10, 0, 6), (false, true), "no branch: the jobs fit");
    assert_eq!(fit(12, 10, 4, 6), (false, false), "the path keeps its room");
    assert_eq!(fit(40, 10, 0, 0), (false, false), "nothing to say, nothing drawn");
}
