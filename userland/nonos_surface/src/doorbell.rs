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

/*
 * Waking the consumer only when it sleeps.
 *
 * A consumer with nothing to read arms the doorbell; a producer that
 * publishes finds the arm and rings. The ring itself is the wake source's
 * business (`Bell`): between two processes it is a message to the
 * consumer's inbox, which the consumer already blocks on. A producer that
 * publishes a hundred records while the consumer is awake rings no bell.
 *
 * The arm is bit 31 of the head word, so arming and publishing are both
 * read-modify-writes of one location and are ordered by that location's
 * modification order alone. The consumer's `fetch_or` either comes before
 * the producer's swap, which then returns the arm and rings, or after it,
 * and then returns the new head, so the consumer sees the records and does
 * not sleep. No wake is lost and no fence is needed. (An arm in a separate
 * word, ordered by SeqCst fences, was the first design; loom found a lost
 * wake in it.)
 */

use crate::ring::{Consumer, Producer, ARMED};
use crate::ring_error::RingError;
use crate::ring_sync::Ordering;

/// The way a producer wakes a consumer that armed the doorbell.
pub trait Bell {
    fn ring(&self);
}

/// What the consumer should do after arming.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Armed {
    /// Records arrived meanwhile: the arm is withdrawn, read them.
    Ready,
    /// Nothing to read: block on the bell; a publish will ring it.
    Sleep,
}

impl<const W: usize> Producer<'_, '_, W> {
    /// `publish`, then ring `bell` if the consumer is asleep. Returns
    /// whether it rang.
    pub fn publish_and_ring(&mut self, bell: &impl Bell) -> bool {
        if self.publish() {
            bell.ring();
            return true;
        }
        false
    }
}

impl<const W: usize> Consumer<'_, '_, W> {
    pub fn arm(&mut self) -> Result<Armed, RingError> {
        let head = self.ring.header.head.fetch_or(ARMED, Ordering::AcqRel);
        if self.count(head)? > 0 {
            self.ring.header.head.fetch_and(!ARMED, Ordering::Relaxed);
            return Ok(Armed::Ready);
        }
        Ok(Armed::Sleep)
    }
}
