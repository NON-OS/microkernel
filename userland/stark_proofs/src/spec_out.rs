// NONOS Operating System (AGPL-3.0-or-later)
//! Where the ignored vector generators write: the contracts tree's spec
//! folder, named by `NOX_SPEC_DIR`. A generator run without it fails and
//! names the variable, so no generator writes to a fixed machine path.

use alloc::string::String;

/// The variable that names the contracts tree's spec folder.
pub const SPEC_DIR_VAR: &str = "NOX_SPEC_DIR";

/// `name` under the spec folder `NOX_SPEC_DIR` names.
pub fn spec_path(name: &str) -> String {
    let dir = std::env::var(SPEC_DIR_VAR)
        .unwrap_or_else(|_| panic!("set {} to the contracts tree's spec folder", SPEC_DIR_VAR));
    join(&dir, name)
}

/// `name` under `dir`, with one separator between them.
pub fn join(dir: &str, name: &str) -> String {
    alloc::format!("{}/{}", dir.trim_end_matches('/'), name.trim_start_matches('/'))
}

#[test]
fn a_name_is_joined_under_the_folder_with_one_separator() {
    assert_eq!(join("spec", "a.json"), "spec/a.json");
    assert_eq!(join("spec/", "a.json"), "spec/a.json");
    assert_eq!(join("spec", "reference/i.json"), "spec/reference/i.json");
    assert_eq!(join("spec//", "/a.json"), "spec/a.json");
}
