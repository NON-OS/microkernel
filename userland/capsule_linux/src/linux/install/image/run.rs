//! `install nonos-image-<package>`: the list held to the market's pin, the
//! missing chunks fetched and kept, then each member read out of its
//! chunks, held to its digest and placed. The listed package is placed
//! under the install's own name, as a tool's content is (tools_install.rs),
//! so the market binds its policy to it; the rest under their own names,
//! as the packages an index install would have placed. Any refusal stops
//! the install; what was fetched stays in the cache for the next one.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_image::{next, ChunkList, Head, Role};
use nonos_libc::mk_debug;

use super::super::auth::Verified;
use super::super::http::reach::reachable;
use super::super::mirror::parse;
use super::super::place::all::unpack;
use super::super::{timing, Why};
use super::fetch::{self, Mirror};
use super::named::{image_path, list_path};
use super::route_pick::{asked, route, width, Route};
use super::sched::Sched;
use super::{assemble, cache, evict, open, reuse};
use crate::linux::file::{key, store_read};

/// Asks for one chunk, after the first, when a stream breaks.
const AGAIN: usize = 3;

pub fn install(pkg: &str, pin: &[u8; 32]) -> Result<(), Why> {
    timing::timed(|| install_all(pkg, pin))
}

fn install_all(pkg: &str, pin: &[u8; 32]) -> Result<(), Why> {
    // Installed before, and every pack still the one placed: nothing to do.
    if let Some(names) = reuse::already(pkg, pin) {
        say(&alloc::format!("already in place: {} packs, nothing fetched", names.len()));
        super::super::record::closure::keep(pkg, &names)?;
        evict::touch(pin, pkg);
        return Ok(());
    }
    let Some((at, port, route)) = chosen() else {
        say("this image was built without a NONOS package mirror");
        return Err(Why::NO_MIRROR);
    };
    let m = Mirror { at, port, host: at, route };
    if route == Route::Fast {
        say("fetching off the mixnet: the fast route is on");
    }
    let raw = match cache::list(pin) {
        Some(raw) => raw,
        None => {
            reach(&m)?;
            let raw = fetch::list(&m, &list_path(pin)).ok_or_else(|| refused("its chunk list did not download"))?;
            cache::keep_list(pin, &raw);
            raw
        }
    };
    let list = ChunkList::parse(&raw, pin).map_err(|_| refused("its chunk list is not the one the market listed"))?;
    say("provenance Verified: the chunk list is the market's pin");
    timing::packages();
    fetch_missing(&m, pin, &list)?;
    let mut get = |i: usize| cache::chunk(pin, &list, i).or_else(|| fetched(&m, pin, &list, i));
    let mut get = assemble::Chunks::new(&mut get);
    let head = assemble::head(&list, &mut get).ok_or_else(|| refused("its head could not be read"))?;
    let head = Head::parse(&head).map_err(|_| refused("its head is not an image"))?;
    let want = super::super::market::hold::wanted();
    if want.is_some() {
        super::super::market::hold::bind_anywhere();
    }
    let (mut names, mut bound) = (Vec::new(), false);
    for member in &head.members {
        fits(member, list.chunk)?;
        let stored = assemble::bytes(&list, &member.at, &mut get).ok_or_else(|| refused("a member could not be read"))?;
        let tar = timing::verifying(|| open::tar(member, stored)).ok_or_else(|| refused("a member is not its digest"))?;
        // The member placed under the listing's name: the one carrying the
        // program a market policy names, or with no policy the listed one.
        let chosen = match want {
            Some(w) => !bound && carries_program(&tar, &w),
            None => member.role == Role::Listed,
        };
        bound |= chosen;
        let name = match chosen {
            true => String::from(pkg),
            false => String::from_utf8_lossy(member.name).into_owned(),
        };
        let files = Verified::checked(tar);
        timing::placing(|| unpack(&files, &name, chosen, false))?;
        names.push(name);
    }
    super::super::record::closure::keep(pkg, &names)?;
    reuse::keep(pin, &names);
    evict::touch(pin, pkg);
    evict::prune(pin);
    Ok(())
}

/// The mirror and route this build and setting give, or None without one.
fn chosen() -> Option<(&'static str, u16, Route)> {
    let fast_at = fast_mirror();
    let setting = store_read(&key(b"/etc/nonos-install-fast-route"), 16).ok();
    let r = route(cfg!(feature = "install-fast-route"), asked(setting.as_deref()), fast_at);
    let at = match r {
        Route::Fast => fast_at,
        Route::Mixnet => option_env!("NONOS_PACKAGE_MIRROR"),
    };
    let (name, port, _) = parse(at.filter(|m| !m.is_empty())?);
    Some((name, port, r))
}

/// The fast route's mirror, an address, named when the build was made.
fn fast_mirror() -> Option<&'static str> {
    match cfg!(feature = "install-fast-route") {
        true => option_env!("NONOS_FAST_MIRROR"),
        false => None,
    }
}

/// The route is up: Anyone has a circuit, or the fast route needs none.
fn reach(m: &Mirror) -> Result<(), Why> {
    match m.route {
        Route::Mixnet => reachable(m.at),
        Route::Fast => Ok(()),
    }
}

/// Every chunk not kept, fetched in order, `width` at a time, each held to
/// its digest and kept: on the mixnet by the window of named streams
/// (window.rs), on the fast route in batches. One that does not come after
/// its asks ends the install; those that came stay for the next one.
fn fetch_missing(m: &Mirror, pin: &[u8; 32], list: &ChunkList) -> Result<(), Why> {
    let mut held: Vec<bool> = (0..list.count()).map(|i| cache::present(pin, list, i)).collect();
    let kept = held.iter().filter(|h| **h).count();
    if kept > 0 {
        say(&alloc::format!("{kept} of {} chunks already here", list.count()));
    }
    if kept < list.count() {
        reach(m)?;
    }
    if m.route == Route::Mixnet {
        let mut sched = Sched::new(held, width(m.route));
        return match super::window::fetch(m, pin, list, &image_path(pin), &mut sched) {
            true => Ok(()),
            false => Err(refused("its chunks stopped coming; what came is kept for the next try")),
        };
    }
    let mut misses = 0;
    loop {
        let mut batch = Vec::new();
        while let Some(i) = next(&held, &batch, width(m.route)) {
            batch.push(i);
        }
        if batch.is_empty() {
            return Ok(());
        }
        let ranges: Vec<_> = batch.iter().filter_map(|i| list.range(*i)).collect();
        let got = fetch::many(m, &image_path(pin), &ranges, list.total);
        let mut came = false;
        for (i, bytes) in batch.iter().zip(got) {
            if let Some(b) = bytes.filter(|b| list.holds(*i, b)) {
                // One the store would not take is fetched again when it is read.
                let _ = cache::keep_chunk(pin, *i, &b);
                (held[*i], came) = (true, true);
            }
        }
        misses = if came { 0 } else { misses + 1 };
        if misses > AGAIN {
            return Err(refused("its chunks stopped coming; what came is kept for the next try"));
        }
    }
}

/// Whether a member's tar carries the program `want` names (bound.rs).
fn carries_program(tar: &[u8], want: &[u8; 32]) -> bool {
    let all = super::super::tar::entries(tar);
    let pairs: Vec<(&[u8], &[u8])> = all.iter().map(|e| (e.name.as_slice(), e.body)).collect();
    timing::verifying(|| super::bound::carries(&pairs, want, |b| *blake3::hash(b).as_bytes()))
}

/// Whether `member` can be placed in the heap left; when not, the install
/// ends as UNPACK_MEMORY, naming it, before any of it is read.
fn fits(member: &nonos_app_image::Member, chunk: u32) -> Result<(), Why> {
    let need = super::room::needs(member.raw, chunk);
    let h = nonos_libc::heap_stats();
    if need <= h.size.saturating_sub(h.used) as u64 {
        return Ok(());
    }
    let name = String::from_utf8_lossy(member.name);
    say(&alloc::format!("not enough memory to place {name} ({need} bytes)"));
    Err(Why::UNPACK_MEMORY)
}

/// Chunk `i` fetched now, when it was not kept or is no longer its digest.
fn fetched(m: &Mirror, pin: &[u8; 32], list: &ChunkList, i: usize) -> Option<Vec<u8>> {
    let r = list.range(i)?;
    let b = (0..=AGAIN).find_map(|_| fetch::chunk(m, &image_path(pin), &r, list.total).filter(|b| list.holds(i, b)))?;
    cache::keep_chunk(pin, i, &b);
    Some(b)
}

fn refused(why: &str) -> Why {
    say(why);
    super::super::refuse::tell("image", "", why);
    Why::PACKAGE
}

fn say(line: &str) {
    let line = alloc::format!("[LINUX] image: {line}\n");
    let _ = mk_debug(line.as_ptr(), line.len());
}
