// The markers the sealed tree holds, each with why, so the hygiene check
// fails on any new one and on a row the tree no longer matches. The 0.9.3
// image was sealed from this tree: these files cannot change until the next
// seal, which is when each row is fixed and removed.

use std::collections::BTreeMap;

/// Where the rows live, from the repository root.
pub const FILE: &str = "nonos-verify/hygiene-baseline.txt";

type Key = (String, String);

/// How many lines of one kind a file holds, and why they are there.
pub struct Row {
    pub count: usize,
    pub reason: String,
}

/// Rows of `path kind count`, sorted, each followed by one or more indented
/// lines saying why. Blank lines and lines starting `#` are comments.
pub fn parse(text: &str) -> Result<BTreeMap<Key, Row>, String> {
    let mut rows: BTreeMap<Key, Row> = BTreeMap::new();
    let mut last: Option<Key> = None;
    for (i, raw) in text.lines().enumerate() {
        let at = i + 1;
        if raw.trim().is_empty() || raw.starts_with('#') {
            continue;
        }
        if raw.starts_with(char::is_whitespace) {
            let row = last
                .as_ref()
                .and_then(|k| rows.get_mut(k))
                .ok_or(format!("line {at}: a reason with no row"))?;
            if !row.reason.is_empty() {
                row.reason.push(' ');
            }
            row.reason.push_str(raw.trim());
            continue;
        }
        let words: Vec<&str> = raw.split_whitespace().collect();
        let &[path, kind, count] = words.as_slice() else {
            return Err(format!("line {at}: a row is `path kind count`"));
        };
        let count = count.parse().map_err(|_| format!("line {at}: {count:?} is not a count"))?;
        let key = (path.to_string(), kind.to_string());
        if let Some(prev) = &last {
            if rows.get(prev).is_some_and(|r| r.reason.is_empty()) {
                return Err(format!("{} {}: no reason", prev.0, prev.1));
            }
            if *prev >= key {
                return Err(format!("line {at}: rows out of order or repeated"));
            }
        }
        rows.insert(key.clone(), Row { count, reason: String::new() });
        last = Some(key);
    }
    match last {
        Some(prev) if rows.get(&prev).is_some_and(|r| r.reason.is_empty()) => {
            Err(format!("{} {}: no reason", prev.0, prev.1))
        }
        _ => Ok(rows),
    }
}

/// What disagrees between the rows and the violations found, each
/// `path:line:kind`: a violation no row names, a count a row does not say,
/// and a row naming nothing found.
pub fn compare(rows: &BTreeMap<Key, Row>, violations: &[String]) -> Vec<String> {
    let mut found: BTreeMap<Key, Vec<&String>> = BTreeMap::new();
    for v in violations {
        let mut parts = v.rsplitn(3, ':');
        if let (Some(kind), Some(_), Some(path)) = (parts.next(), parts.next(), parts.next()) {
            found.entry((path.to_string(), kind.to_string())).or_default().push(v);
        }
    }
    let mut out = Vec::new();
    for (key, hits) in &found {
        match rows.get(key) {
            None => out.extend(hits.iter().map(|h| format!("{h}: no row in {FILE}"))),
            Some(r) if r.count != hits.len() => out.push(format!(
                "{} {}: {} found, the row says {}",
                key.0,
                key.1,
                hits.len(),
                r.count
            )),
            Some(_) => {}
        }
    }
    for (key, r) in rows {
        if !found.contains_key(key) {
            out.push(format!("{} {}: the row says {}, none found; remove it", key.0, key.1, r.count));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{compare, parse};

    #[test]
    fn a_row_needs_its_reason_and_its_order() {
        assert!(parse("a.rs unwrap 1\n    why\nb.rs panic 2\n    why\n").is_ok());
        assert!(parse("a.rs unwrap 1\nb.rs panic 2\n    why\n").is_err());
        assert!(parse("b.rs panic 2\n    why\na.rs unwrap 1\n    why\n").is_err());
        assert!(parse("a.rs unwrap\n    why\n").is_err());
    }

    #[test]
    fn new_miscounted_and_stale_all_disagree() {
        let rows = parse("a.rs unwrap 1\n    why\nb.rs panic 2\n    why\nc.rs todo 1\n    why\n")
            .unwrap_or_default();
        let v: Vec<String> =
            ["a.rs:3:unwrap", "b.rs:4:panic", "d.rs:9:expect"].map(String::from).to_vec();
        let found = compare(&rows, &v);
        assert_eq!(found.len(), 3, "{found:?}");
        assert!(found[0].starts_with("b.rs panic: 1 found"));
        assert!(found[1].starts_with("d.rs:9:expect: no row"));
        assert!(found[2].starts_with("c.rs todo: the row says 1"));
    }
}
