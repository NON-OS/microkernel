//! Which route an image's chunks come over, and how many at once. The
//! mixnet route (Anyone, as every install) is the only one a build takes
//! unless it was built with `install-fast-route` AND the person turned the
//! fast route on in /etc/nonos-install-fast-route. No release profile
//! builds the feature: it waits for the project's sign-off (
//! ). Either route's bytes are held to the same digests
//! by the same code. Pure, so the host proofs hold it.

/// How chunks travel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Route {
    /// Over Anyone, through route_link, named and resolved at the exit.
    Mixnet,
    /// Straight to the mirror's address, off the mixnet.
    Fast,
}

/// What the person set: the word `on` alone turns the fast route on;
/// anything else, or no file, leaves it off.
pub fn asked(setting: Option<&[u8]>) -> bool {
    setting.is_some_and(|s| s.split(|b| b.is_ascii_whitespace()).find(|w| !w.is_empty()) == Some(b"on"))
}

/// The route for this install: Fast only when the build carries it, the
/// person asked for it, and the build names a mirror address for it.
pub fn route(built: bool, asked: bool, fast_mirror: Option<&str>) -> Route {
    match built && asked && fast_mirror.is_some_and(|m| !m.is_empty()) {
        true => Route::Fast,
        false => Route::Mixnet,
    }
}

/// Chunks in flight at once: four on either route. On the mixnet each is
/// a named stream of the install's own on a circuit of its own (the project
/// allows four); the fast route's sockets are the capsule's.
pub fn width(route: Route) -> usize {
    match route {
        Route::Mixnet | Route::Fast => super::sched::WIDTH_MAX,
    }
}
