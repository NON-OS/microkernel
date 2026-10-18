//! Finding the pieces of a JSON reply: a key's value, a quoted run, objects, an array of strings.

/// The slice just past a key, or nothing if the key is absent.
pub(crate) fn after<'a>(s: &'a str, key: &str) -> Option<&'a str> {
    let at = s.find(key)?;
    s.get(at.checked_add(key.len())?..)
}

/// The first double-quoted run in a slice, without the quotes.
pub(crate) fn quoted(s: &str) -> Option<&str> {
    let open = s.find('"')?;
    let rest = s.get(open.checked_add(1)?..)?;
    let close = rest.find('"')?;
    rest.get(..close)
}

/// The top level objects of the first array in a slice, split on brace depth
/// with string literals respected so a brace inside a string does not count.
pub(crate) fn objects(s: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let bytes = s.as_bytes();
    let (mut depth, mut start, mut in_str, mut esc, mut seen_array) =
        (0i32, 0usize, false, false, false);
    for (i, &b) in bytes.iter().enumerate() {
        if in_str {
            if esc {
                esc = false;
            } else if b == b'\\' {
                esc = true;
            } else if b == b'"' {
                in_str = false;
            }
            continue;
        }
        match b {
            b'"' => in_str = true,
            b'[' if !seen_array => seen_array = true,
            b'{' => {
                if depth == 0 {
                    start = i;
                }
                depth = depth.saturating_add(1);
            }
            b'}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    if let Some(slice) = s.get(start..=i) {
                        out.push(slice);
                    }
                }
            }
            b']' if depth == 0 && seen_array => break,
            _ => {}
        }
    }
    out
}

/// The double-quoted strings of the first bracketed array in a slice.
pub(super) fn strings_in_array(s: &str) -> Option<Vec<&str>> {
    let open = s.find('[')?;
    let rest = s.get(open.checked_add(1)?..)?;
    let close = rest.find(']')?;
    let inner = rest.get(..close)?;
    let mut out = Vec::new();
    let mut cursor = inner;
    while let Some(q) = quoted(cursor) {
        out.push(q);
        let past = cursor.find(q)?.checked_add(q.len())?.checked_add(1)?;
        cursor = cursor.get(past..)?;
    }
    Some(out)
}
