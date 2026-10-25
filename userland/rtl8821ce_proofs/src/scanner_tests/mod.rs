// NONOS Operating System (AGPL-3.0-or-later)
//! No scan transmits. Frames a station might answer (probe requests, to us
//! and wildcard, authentication, deauthentication, an SA Query, an EAPOL
//! key frame) are drained by the scanner, associated or not, and nothing is
//! queued: the TX ring and buffers stay zero and the queue is never kicked.

mod drain;
mod rig;
mod tune;
