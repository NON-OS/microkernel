// NONOS Operating System (AGPL-3.0-or-later)
//! The model's mutex rules (`model.rs` states them): the mutexes held in
//! the order taken, with their levels, and each one's depth; 3 is \_GL.

use crate::exec::FaultKind;

pub type Held = Vec<(u8, u8)>;

pub fn take(n: u8, level: u8, held: &mut Held, depth: &mut [u32; 4]) -> Result<(), FaultKind> {
    if held.last().is_some_and(|(_, l)| *l > level) {
        return Err(FaultKind::Mutex);
    }
    if depth[n as usize] == 0 {
        held.push((n, level));
    }
    depth[n as usize] += 1;
    Ok(())
}

pub fn give(n: u8, held: &mut Held, depth: &mut [u32; 4]) -> Result<(), FaultKind> {
    if depth[n as usize] == 0 {
        return Err(FaultKind::Mutex);
    }
    depth[n as usize] -= 1;
    if depth[n as usize] == 0 {
        held.retain(|(m, _)| *m != n);
    }
    Ok(())
}
