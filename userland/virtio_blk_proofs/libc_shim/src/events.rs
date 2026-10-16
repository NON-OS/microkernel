// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! What the driver asked of the broker, in order, and a hook the test runs
//! at the moment each request is made, so it can look at the device then:
//! whether its queue was stopped when Bus Master went on, whether it was
//! reset when the claim was given back.

use std::cell::RefCell;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    Claim,
    /// A write of the PCI command register, with the bits written.
    Command(u16),
    MmioMap,
    PioGrant,
    IrqBind {
        msix: bool,
    },
    DmaMap(u64),
    Release,
}

type Hook = Box<dyn FnMut(Event)>;

thread_local! {
    static LOG: RefCell<Vec<Event>> = const { RefCell::new(Vec::new()) };
    static HOOK: RefCell<Option<Hook>> = const { RefCell::new(None) };
}

/// Run `hook` at every later request, before it is answered.
pub fn on_event(hook: impl FnMut(Event) + 'static) {
    HOOK.with(|h| *h.borrow_mut() = Some(Box::new(hook)));
}

/// Every request this thread's driver made, oldest first.
pub fn events() -> Vec<Event> {
    LOG.with(|l| l.borrow().clone())
}

pub(crate) fn record(event: Event) {
    HOOK.with(|h| {
        if let Some(hook) = h.borrow_mut().as_mut() {
            hook(event);
        }
    });
    LOG.with(|l| l.borrow_mut().push(event));
}
