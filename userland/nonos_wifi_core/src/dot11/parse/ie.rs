// NONOS Operating System (AGPL-3.0-or-later)

//! Information elements found in a tagged-parameter area.

/// Find an information element by id in a tagged-parameter area. Walks
/// `[id][len][data]` tuples with a bounded index and returns the element's data
/// slice, or `None` if it is absent or a length runs past the buffer.
pub fn find_ie(tags: &[u8], id: u8) -> Option<&[u8]> {
    let mut off = 0usize;
    while off + 2 <= tags.len() {
        let eid = tags[off];
        let len = tags[off + 1] as usize;
        let start = off + 2;
        let end = start.checked_add(len)?;
        if end > tags.len() {
            return None;
        }
        if eid == id {
            return Some(&tags[start..end]);
        }
        off = end;
    }
    None
}
