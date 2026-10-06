// NONOS Operating System (AGPL-3.0-or-later)
//! Stand-in for the capsule's TLS fetch: there is no network in a host test.

pub fn fetch_tls(
    _: u32,
    _: &str,
    _: &[[u8; 4]],
    _: &str,
    _: usize,
) -> Result<alloc::vec::Vec<u8>, u16> {
    Err(21)
}
