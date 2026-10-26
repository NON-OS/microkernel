
`mutants.py [first]` breaks each rule once, runs the whole proof crate that
must catch it (`cargo test --release`), and puts the file back. Run from
because no test gave a listed device id under another vendor's id; that
`a_controller_whose_error_a_reset_clears_is_brought_back` failure under D7
was main's revive race (hw/xhci-reply-race), not the mutant.

| mutant | file | crate | result | killed by |
|---|---|---|---|---|
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
