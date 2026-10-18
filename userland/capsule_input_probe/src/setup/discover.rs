use nonos_libc::mk_service_lookup;

const COMPOSITOR_SERVICE: &[u8] = b"compositor";
const INPUT_ROUTER_SERVICE: &[u8] = b"input_router";

/// The service's port and the pid that announced it.
fn lookup(name: &[u8]) -> Result<(u32, u32), &'static str> {
    let mut pid: u32 = 0;
    let mut port: u32 = 0;
    let rc =
        mk_service_lookup(name.as_ptr(), name.len(), &mut port as *mut u32, &mut pid as *mut u32);
    if rc < 0 || pid == 0 || port == 0 {
        return Err("service lookup failed");
    }
    Ok((port, pid))
}

fn lookup_port(name: &[u8]) -> Result<u32, &'static str> {
    lookup(name).map(|(port, _)| port)
}

/// The compositor's pid, the one process the overlay is shared with (K20).
pub fn lookup_compositor_pid() -> Result<u32, &'static str> {
    lookup(COMPOSITOR_SERVICE).map(|(_, pid)| pid).map_err(|_| "lookup compositor")
}

pub fn lookup_compositor_port() -> Result<u32, &'static str> {
    lookup_port(COMPOSITOR_SERVICE).map_err(|_| "lookup compositor")
}

pub fn lookup_router_port() -> Result<u32, &'static str> {
    lookup_port(INPUT_ROUTER_SERVICE).map_err(|_| "lookup input_router")
}
