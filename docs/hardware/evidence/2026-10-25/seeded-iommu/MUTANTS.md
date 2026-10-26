# Item 7 mutants

`mutants.py` (kernel_proofs) and `dsm_mutants.py` (acpi_aml_proofs) break
each rule once, run the whole proof crate and put the file back. 14 kernel
and parser rules: 13 killed, 1 equivalent.

| mutant | file | result | killed by |
|---|---|---|---|
| S1 an inverted IVHD range kept | `src/arch/x86_64/acpi/parser/other/ivhd_kind.rs` | KILLED | inverted_ranges_take_no_span_and_every_real_one_after_them_is_kept, ivrs_blocks_and_entries_stay_inside_the_table_and_hold_their_caps |
| S2 DMAR padding read as a cut | `src/arch/x86_64/acpi/parser/other/dmar_walk.rs` | KILLED | a_cut_after_the_rmrrs_hides_none_of_them_and_one_before_may |
| S3 DMAR hands on a cut structure | `src/arch/x86_64/acpi/parser/other/dmar_walk.rs` | KILLED | hostile_dmars_are_walked_whole_or_cut_and_nothing_reads_past_them, dmar_structures_stay_inside_the_table_and_each_parser_keeps_its_rules |
| S4 RMRR inverted region kept | `src/arch/x86_64/acpi/parser/other/dmar_rmrr.rs` | KILLED | an_rmrr_that_is_not_exactly_sound_is_refused |
| S5 ANDD empty name kept | `src/arch/x86_64/acpi/parser/other/dmar_andd.rs` | KILLED | a_malformed_andd_is_refused, dmar_structures_stay_inside_the_table_and_each_parser_keeps_its_rules |
| S6 IVRS base counted twice | `src/arch/x86_64/acpi/parser/other/ivrs_walk.rs` | KILLED | one_iommu_described_twice_is_one_base, a_whole_table_is_read_whole |
| S9 inverted alias kept | `src/arch/x86_64/acpi/parser/other/ivhd_alias.rs` | KILLED | ivrs_blocks_and_entries_stay_inside_the_table_and_hold_their_caps |
| S10 special counted twice | `src/arch/x86_64/acpi/parser/other/ivhd_special.rs` | KILLED | an_ioapic_and_an_hpet_are_named_once_and_nothing_else |
| S7 IVRS short IVHD read for a base | `src/arch/x86_64/acpi/parser/other/ivrs_walk.rs` | KILLED | ivrs_blocks_and_entries_stay_inside_the_table_and_hold_their_caps |
| S8 IVMD empty range kept | `src/arch/x86_64/acpi/parser/other/ivmd_block.rs` | EQUIVALENT | no input reaches it: past `len == 0` and both `checked_add`s, `end` (rounded up from start + len) always exceeds `start` (rounded down) |
| A1 a register of 0 taken | `src/arch/x86_64/acpi/devices/i2c/dsm.rs` | KILLED | the_dsm_register_is_its_function_one_constant_or_nothing, an_intel_reference_placeholder_hid_is_found_by_its_cid_and_its_scope_names_the_bus |
| A2 a value past 16 bits taken | `src/arch/x86_64/acpi/devices/i2c/dsm.rs` | KILLED | the_dsm_register_is_its_function_one_constant_or_nothing |
| A3 another GUID's branch trusted | `src/arch/x86_64/acpi/devices/i2c/dsm.rs` | KILLED | the_dsm_register_is_its_function_one_constant_or_nothing |
| A4 One == Arg2 not read | `src/arch/x86_64/acpi/devices/i2c/dsm.rs` | KILLED | the_dsm_register_is_its_function_one_constant_or_nothing |

S1 is also killed by `ivrs_cap_tests.rs`, which pins the rule directly: with
1, 7 or 64 inverted ranges first, the 64 real spans after them are all kept.
S7 survived the first run: no generated IVHD block was honestly 16 to 23
bytes. `ivrs_gen.rs` now makes some, and S7 is killed. S2, S4, S6 and S10
are killed by the existing DMAR and IVHD tests as well as, or instead of,
the sweeps.
