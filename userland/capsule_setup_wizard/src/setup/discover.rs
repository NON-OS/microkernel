use nonos_libc::{mk_service_lookup, mk_yield, Deadline};

const COMPOSITOR_SERVICE: &[u8] = b"compositor";
const INPUT_ROUTER_SERVICE: &[u8] = b"input_router";
const POLICY_SERVICE: &[u8] = b"policy";

fn lookup_port(name: &[u8]) -> Result<u32, &'static str> {
    let mut pid: u32 = 0;
    let mut port: u32 = 0;
    let rc =
        mk_service_lookup(name.as_ptr(), name.len(), &mut port as *mut u32, &mut pid as *mut u32);
    if rc < 0 || pid == 0 || port == 0 {
        return Err("service lookup failed");
    }
    Ok(port)
}

fn lookup_optional(name: &[u8]) -> u32 {
    lookup_port(name).unwrap_or(0)
}

/// Setup is spawned beside the compositor and the input router, and they may
/// not have registered yet. This bounds only a desktop that never arrives.
const DESKTOP_WAIT_MS: u64 = 60_000;

/// The compositor and input router ports, waited for rather than asked once.
/// Asked once, a lookup that raced their registration ended setup before it
/// drew anything, and setup ending is what starts the rest of the desktop, so
/// the machine came up without first-boot setup and said nothing about it.
pub fn wait_for_desktop() -> Result<(u32, u32), &'static str> {
    let until = Deadline::after_ms(DESKTOP_WAIT_MS);
    loop {
        match (lookup_compositor_port(), lookup_router_port()) {
            (Ok(compositor), Ok(router)) => return Ok((compositor, router)),
            (Err(why), _) | (_, Err(why)) if until.expired() => return Err(why),
            _ => {
                let _ = mk_yield();
            }
        }
    }
}

pub fn lookup_compositor_port() -> Result<u32, &'static str> {
    lookup_port(COMPOSITOR_SERVICE).map_err(|_| "lookup compositor")
}

pub fn lookup_router_port() -> Result<u32, &'static str> {
    lookup_port(INPUT_ROUTER_SERVICE).map_err(|_| "lookup input_router")
}

pub fn lookup_policy_port() -> u32 {
    lookup_optional(POLICY_SERVICE)
}
