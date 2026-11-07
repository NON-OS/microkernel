//! An image's list and chunks from the NONOS package mirror, over the route
//! route_pick.rs chose. On the mixnet each GET goes through route_link as
//! every install's does (http/route.rs), one at a time on its kept stream;
//! on the fast route, several at once to the mirror's address (fast.rs).
//! Whatever comes is held to its digest by the caller before it is used.

use alloc::vec::Vec;
use core::ops::Range;

use super::super::http::resume::whole;
use super::super::http::route::exchange;
use super::super::http_reply::complete;
use super::super::timing;
use super::route_pick::Route;
use super::span;

/// Where the mirror is and how it is reached.
pub struct Mirror<'a> {
    pub at: &'a str,
    pub port: u16,
    pub host: &'a str,
    pub route: Route,
}

/// Room for a reply's status and headers beside a chunk.
const HEAD: usize = 16 << 10;

/// The chunk list at `path`, whole.
pub fn list(m: &Mirror, path: &str) -> Option<Vec<u8>> {
    match m.route {
        Route::Mixnet => mixnet_list(m, path),
        Route::Fast => fast_list(m, path),
    }
}

/// Bytes `r` of the image at `path`, of `total` bytes.
pub fn chunk(m: &Mirror, path: &str, r: &Range<u64>, total: u64) -> Option<Vec<u8>> {
    match m.route {
        Route::Mixnet => {
            let max = usize::try_from(r.end - r.start).ok()?.saturating_add(HEAD);
            let t0 = timing::now();
            let req = span::request(path, m.host, r);
            #[cfg(feature = "dev-mirror")]
            let raw = match crate::linux::net::route::is_local(m.at) {
                true => super::super::http::direct::direct(m.at, m.port, req.as_bytes()),
                false => exchange(m.at, m.port, req.as_bytes(), max, &complete),
            };
            #[cfg(not(feature = "dev-mirror"))]
            let raw = exchange(m.at, m.port, req.as_bytes(), max, &complete);
            timing::got(raw.as_ref().map_or(0, Vec::len), timing::now().saturating_sub(t0));
            span::chunk(raw?, r, total)
        }
        Route::Fast => many(m, path, core::slice::from_ref(r), total).pop().flatten(),
    }
}

/// Several chunks at once where the route allows it, each Some only when
/// it came whole; in the order asked.
pub fn many(m: &Mirror, path: &str, ranges: &[Range<u64>], total: u64) -> Vec<Option<Vec<u8>>> {
    match m.route {
        Route::Mixnet => ranges.iter().map(|r| chunk(m, path, r, total)).collect(),
        #[cfg(feature = "install-fast-route")]
        Route::Fast => super::fast::many(m, path, ranges, total),
        #[cfg(not(feature = "install-fast-route"))]
        Route::Fast => ranges.iter().map(|_| None).collect(),
    }
}

/* The list over the route; in a development image from its relay on the
 * local network, dialled directly as every dev-mirror GET is (http/get.rs). */
fn mixnet_list(m: &Mirror, path: &str) -> Option<Vec<u8>> {
    #[cfg(feature = "dev-mirror")]
    if crate::linux::net::route::is_local(m.at) {
        return super::super::http::get::get_as(m.at, m.port, m.host, path);
    }
    whole(m.at, m.port, m.host, path)
}

#[cfg(feature = "install-fast-route")]
fn fast_list(m: &Mirror, path: &str) -> Option<Vec<u8>> {
    super::fast::list(m, path)
}

/* A build without the fast route has no way to take it. */
#[cfg(not(feature = "install-fast-route"))]
fn fast_list(_: &Mirror, _: &str) -> Option<Vec<u8>> {
    None
}
