// NONOS Operating System (AGPL-3.0-or-later)
/*
 * The rows `help` prints, rendered by the terminal's own layout code from
 * its own tables, so the tests read what a user sees.
 */

/*
 * The tool table, read from the file that runs the tools: that file spawns
 * processes, so it is parsed rather than compiled here.
 */
pub fn tool_table() -> Vec<(Vec<u8>, Vec<u8>)> {
    let src = include_str!("../../capsule_terminal/src/command/builtin/tool.rs");
    let mut out = Vec::new();
    for line in src.lines() {
        let Some(rest) = line.trim().strip_prefix("(b\"") else { continue };
        let Some((typed, rest)) = rest.split_once("\", b\"") else { continue };
        let Some((service, _)) = rest.split_once('"') else { continue };
        out.push((typed.as_bytes().to_vec(), service.as_bytes().to_vec()));
    }
    out
}

/*
 * Every table of rows `help` prints, each laid out by the function the
 * terminal lays it out with, as (the table's name, its rows).
 */
pub fn help_tables() -> Vec<(&'static str, Vec<String>)> {
    use crate::help_layout::*;
    let render = |rows: &[(&[u8], &[u8])], pad| -> Vec<String> {
        rows.iter().map(|(n, t)| String::from_utf8(plain_row(n, t, pad)).unwrap()).collect()
    };
    let owned = tool_table();
    let tools: Vec<(&[u8], &[u8])> =
        owned.iter().map(|(a, b)| (a.as_slice(), b.as_slice())).collect();
    let list = crate::tool_list::tool_list(&tools);
    let mut groups: Vec<(&[u8], &[u8])> = GROUPS.to_vec();
    groups.push((b"tools", &list));
    vec![
        ("groups", render(&groups, GROUP_PAD)),
        ("deeper", render(DEEPER, DEEPER_PAD)),
        ("keys", render(&crate::help_pages::KEYS, PAGE_PAD)),
        ("shell", render(&crate::help_pages::SHELL, PAGE_PAD)),
    ]
}

pub fn help_rows() -> Vec<String> {
    let mut rows = vec![String::from_utf8(crate::help_layout::INTRO.to_vec()).unwrap()];
    rows.extend(help_tables().into_iter().flat_map(|(_, r)| r));
    rows
}
