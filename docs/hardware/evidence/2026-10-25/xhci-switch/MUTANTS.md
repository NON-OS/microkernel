# H-10 mutants

`mutants.py [first]` breaks each rule once, runs the whole proof crate that
must catch it (`cargo test --release`), and puts the file back. Run from
the repository root. Twenty rules, twenty killed. K4 survived the first run,
because no test gave a listed device id under another vendor's id; that
case was added (8f5c1e6f2) and K4 was then killed. The extra
`a_controller_whose_error_a_reset_clears_is_brought_back` failure under D7
was main's revive race (hw/xhci-reply-race), not the mutant.

| mutant | file | crate | result | killed by |
|---|---|---|---|---|
| K1 USB2 mask ignored | `src/hardware/broker/pci/quirk_bits/xhci_route.rs` | kernel_proofs | KILLED | each_cited_id_may_move_exactly_the_bits_firmware_masks |
| K2 USB3 mask ignored | `src/hardware/broker/pci/quirk_bits/xhci_route.rs` | kernel_proofs | KILLED | each_cited_id_may_move_exactly_the_bits_firmware_masks |
| K3 device list dropped | `src/hardware/broker/pci/quirk_bits/xhci_route.rs` | kernel_proofs | KILLED | an_id_not_cited_another_vendors_and_every_other_function_stay_refused |
| K4 vendor check dropped | `src/hardware/broker/pci/quirk_bits/xhci_route.rs` | kernel_proofs | KILLED | an_id_not_cited_another_vendors_and_every_other_function_stay_refused |
| K5 EHCI-claimed guard dropped | `src/hardware/broker/pci/quirk_bits/xhci_route.rs` | kernel_proofs | KILLED | while_an_ehci_on_its_bus_is_held_nothing_routes |
| K6 blocks: any bus | `src/hardware/broker/pci/quirk_bits/xhci_route.rs` | kernel_proofs | KILLED | only_a_claimed_ehci_on_the_same_segment_and_bus_blocks |
| K7 blocks: any class | `src/hardware/broker/pci/quirk_bits/xhci_route.rs` | kernel_proofs | KILLED | only_a_claimed_ehci_on_the_same_segment_and_bus_blocks |
| K8 blocks: unclaimed EHCI blocks too | `src/hardware/broker/pci/quirk_bits/xhci_route.rs` | kernel_proofs | KILLED | only_a_claimed_ehci_on_the_same_segment_and_bus_blocks |
| K9 masks not read from the device | `src/hardware/broker/pci/ident_read.rs` | kernel_proofs | KILLED | the_masks_and_the_companion_are_read_at_the_write_for_an_intel_xhci_only |
| K10 companion ignores claims | `src/hardware/broker/pci/companion.rs` | kernel_proofs | KILLED | any_claim_on_an_ehci_on_the_bus_counts_whatever_its_stage |
| D1 order swapped | `userland/capsule_driver_xhci/src/setup/intel_ports_plan.rs` | xhci_proofs | KILLED | a_broker_that_refuses_both_writes_leaves_nothing_to_hand_back, a_device_that_ignores_the_write_is_still_handed_back_what_was_admitted |
| D2 no restore on teardown | `userland/capsule_driver_xhci/src/setup/teardown.rs` | xhci_proofs | KILLED | a_bring_up_that_fails_after_the_switch_hands_the_ports_back_halted, the_ports_move_after_the_reset_and_before_the_controller_runs |
| D3 restore leaves bits set | `userland/capsule_driver_xhci/src/setup/intel_ports_plan.rs` | xhci_proofs | KILLED | the_ports_move_after_the_reset_and_before_the_controller_runs, a_bring_up_that_fails_after_the_switch_hands_the_ports_back_halted |
| D4 restore before the halt | `userland/capsule_driver_xhci/src/setup/teardown.rs` | xhci_proofs | KILLED | a_bring_up_that_fails_after_the_switch_hands_the_ports_back_halted |
| D5 switch after the controller runs | `userland/capsule_driver_xhci/src/setup/sequence.rs` | xhci_proofs | KILLED | a_bring_up_that_fails_after_the_switch_hands_the_ports_back_halted, the_ports_move_after_the_reset_and_before_the_controller_runs |
| D6 Sony VAIO switched | `userland/capsule_driver_xhci/src/setup/intel_ports_plan.rs` | xhci_proofs | KILLED | the_vaio_that_cannot_switch_and_a_controller_not_intels_are_left_alone |
| D7 vendor check dropped | `userland/capsule_driver_xhci/src/setup/intel_ports_plan.rs` | xhci_proofs | KILLED | the_vaio_that_cannot_switch_and_a_controller_not_intels_are_left_alone, a_controller_whose_error_a_reset_clears_is_brought_back |
| D8 zero mask written | `userland/capsule_driver_xhci/src/setup/intel_ports.rs` | xhci_proofs | KILLED | a_mask_of_zero_is_never_written_as_the_unmasked_form |
| D9 refused switch kept | `userland/capsule_driver_xhci/src/setup/intel_ports.rs` | xhci_proofs | KILLED | a_broker_that_refuses_both_writes_leaves_nothing_to_hand_back |
| D10 read-back unchecked | `userland/capsule_driver_xhci/src/setup/intel_ports.rs` | xhci_proofs | KILLED | a_device_that_ignores_the_write_is_still_handed_back_what_was_admitted |
