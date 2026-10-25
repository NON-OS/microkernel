# H-10 mutants (partial: the run was stopped when the unit was parked)

`mutants.py` breaks each rule once and runs the whole proof crate that must
catch it. Three of twenty were run before the stop; the other seventeen
are listed in `mutants.py` and **not run**.

| mutant | file | crate | result | killed by |
|---|---|---|---|---|
| K1 USB2 mask ignored | `src/hardware/broker/pci/quirk_bits/xhci_route.rs` | kernel_proofs | KILLED | each_cited_id_may_move_exactly_the_bits_firmware_masks |
| K2 USB3 mask ignored | `src/hardware/broker/pci/quirk_bits/xhci_route.rs` | kernel_proofs | KILLED | each_cited_id_may_move_exactly_the_bits_firmware_masks |
| K3 device list dropped | `src/hardware/broker/pci/quirk_bits/xhci_route.rs` | kernel_proofs | KILLED | an_id_not_cited_and_every_other_function_stay_refused |
| K4 to K10, D1 to D10 | see `mutants.py` | | NOT RUN | |
