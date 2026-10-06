//! How one event's history is read in pages: as few as the server allows. A page starts as
//! wide as `widest` and halves each time the server refuses a range, down to `narrowest`, below
//! which the refusal stands and fails the scan. Every page is whole or refused, so no gap is
//! ever left, and the pages that came stay in order. Pure over its fetch, so the rule is
//! tested without a network.

use crate::error::NetError;

/// Whether a failed page is a server refusing the range, which a narrower page may pass. A rate
/// limit is not one: more pages would only be refused faster.
fn range_refused(e: NetError) -> bool {
    match e {
        NetError::ReplyShape => true,
        NetError::Rejected { code } => code != 429,
        _ => false,
    }
}

/// Read `from_block..=to_block` with `fetch(from, to)`, a page at a time. `seen(to)` is told
/// the last block of each page read, for the progress the wallet shows.
pub fn paged<T>(
    from_block: u64,
    to_block: u64,
    widest: u64,
    narrowest: u64,
    mut fetch: impl FnMut(u64, u64) -> Result<Vec<T>, NetError>,
    mut seen: impl FnMut(u64),
) -> Result<Vec<T>, NetError> {
    if narrowest == 0 || widest < narrowest {
        return Err(NetError::EndpointRefused);
    }
    let mut window = widest;
    let mut out = Vec::new();
    let mut from = from_block;
    while from <= to_block {
        let to = from.saturating_add(window - 1).min(to_block);
        match fetch(from, to) {
            Ok(page) => {
                out.extend(page);
                seen(to);
                from = match to.checked_add(1) {
                    Some(next) => next,
                    None => break,
                };
            }
            Err(e) if range_refused(e) && window > narrowest => {
                window = (window / 2).max(narrowest);
            }
            Err(e) => return Err(e),
        }
    }
    Ok(out)
}
