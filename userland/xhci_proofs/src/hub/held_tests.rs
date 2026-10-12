// NONOS Operating System (AGPL-3.0-or-later)
//! The owner byte a virtual port reports once claimed: the parked slot's,
//! 1 while only addressed, 2 once a class driver configured an endpoint;
//! and which slots count as hubs for OP_PARK and the routed address.

use crate::dma::DmaPool;
use crate::slots::{SlotResources, SlotTable};

#[test]
fn an_addressed_slot_is_held_and_one_with_an_endpoint_claimed() {
    let pool = DmaPool::new(1, 1);
    let mut t = SlotTable::new();
    for slot in [3u8, 4] {
        assert!(t.mark_allocated(slot, 8));
        let res = SlotResources::allocate(&pool, 32, slot, 1, 3, false).expect("slot");
        assert!(t.attach_addressed(res, 8));
    }
    assert_eq!((t.held(3), t.held(4)), (1, 1));
    t.resources_mut(4, 8).unwrap().bulk =
        Some(crate::controller::BulkPipes::new(&pool, 4, 5).unwrap());
    assert_eq!(t.held(4), 2);
    assert!(!t.is_hub(3) && !t.hub_on(1));
    t.resources_mut(3, 8).unwrap().hub = true;
    assert!(t.is_hub(3) && t.hub_on(1) && !t.hub_on(2));
}
