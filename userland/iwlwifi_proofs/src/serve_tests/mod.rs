// NONOS Operating System (AGPL-3.0-or-later)
//! The serving loop's side of joining: which requests go where (net_core's
//! link protocol to the link, the join ops to the radio only when it can
//! join, the rest as before), the connect body read with every length
//! checked, every way a join ends mapped to the code the panels name, the
//! connect and link replies read back by the client's own parsers, and the
//! hunt for the network's access points against the modeled firmware (a
//! whole sweep heard, candidates ranked by security then signal, not heard,
//! a background sweep stopped first).

mod choice;
mod codes;
mod hunt;
mod quiet;
mod regdom;
mod replies;
mod route;
mod scripted;
mod silent;
