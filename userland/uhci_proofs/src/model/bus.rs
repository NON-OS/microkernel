// NONOS Operating System (AGPL-3.0-or-later)
//! Which device a transaction to an address reaches: the device on the
//! root port or one below hubs on enabled ports, with its speed and the
//! TT a slower device below a high-speed hub is reached through (the
//! nearest such hub's address and the port toward the device).

use super::device::Device;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Path {
    pub speed: u8,
    pub tt: Option<(u8, u8)>,
    /// Below a hub, not on the root port.
    pub routed: bool,
}

/// The way to the device at `addr` below `d` (at `speed`), `d` included:
/// the hub port indices from `d` down, and its path.
fn locate(d: &Device, speed: u8, addr: u8, tt: Option<(u8, u8)>) -> Option<(Vec<usize>, Path)> {
    if d.addr == addr {
        return Some((vec![], Path { speed, tt, routed: false }));
    }
    let hub = d.hub.as_ref()?;
    for (i, p) in hub.ports.iter().enumerate() {
        let Some(child) = p.dev.as_ref().filter(|_| p.enabled) else { continue };
        let tt = if speed == 3 && p.speed != 3 { Some((d.addr, i as u8 + 1)) } else { tt };
        if let Some((mut way, mut path)) = locate(child, p.speed, addr, tt) {
            way.insert(0, i);
            path.routed = true;
            return Some((way, path));
        }
    }
    None
}

/// The path to the device at `addr` below `root` (at `speed`), if any.
pub fn path(root: &Device, speed: u8, addr: u8) -> Option<Path> {
    locate(root, speed, addr, None).map(|(_, p)| p)
}

/// The device at `addr` below the root device `root` (at `speed`).
pub fn find(root: &mut Device, speed: u8, addr: u8) -> Option<(&mut Device, Path)> {
    let (way, path) = locate(root, speed, addr, None)?;
    let mut d = root;
    for i in way {
        d = d.hub.as_mut()?.ports[i].dev.as_mut()?;
    }
    Some((d, path))
}
