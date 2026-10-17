// NONOS Operating System (AGPL-3.0-or-later)

//! Each join's address: drawn once a link, one per network (a name under
//! one security), written to the MAC-ID registers, and never changed under an
//! association; a hidden network's hunt runs under a one-off address.

mod hunt;
mod network;
mod rig;
