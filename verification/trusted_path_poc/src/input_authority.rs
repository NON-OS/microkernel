// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! #401 input authority (src/capabilities/token/types/authority_broker.rs, gated
//! in src/syscall/contract/cap_table/mk_surface.rs). Since K8 posting takes
//! InputSource and draining takes InputConsumer; neither Irq nor Admin stands in.

/// The input-relevant rights a capability token may hold.
#[derive(Clone, Copy)]
pub struct Rights {
    pub input_source: bool,
    pub input_consumer: bool,
    pub irq: bool,
    pub admin: bool,
}

/// can_input_source: may POST an input event. InputSource alone.
pub fn can_post(r: Rights) -> bool {
    r.input_source
}

/// can_input_consumer: may DRAIN or WAIT on the input ring. InputConsumer alone,
/// so a driver holding Irq, or a holder of Admin, cannot read keystrokes.
pub fn can_drain(r: Rights) -> bool {
    r.input_consumer
}
