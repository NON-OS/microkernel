# H-06(a): one mutant per rule

Each row is one change to the shipping source, made alone and reverted after,
with the proof crate that ran against it (`mutants.py` here, from the
repository root; host-run, as the boot cannot reach most of these cases).
KILLED means at least one test failed; the test that caught it is named.
21 mutants, 21 killed. Run on hw/gop-refusal after 3dd09336f, with the
glue pin for the loader's `refusal()` added (K15, K16).

| Mutant | File | Crate | Result | Caught by |
|---|---|---|---|---|
| K1 a stray bit trusted | `src/boot/handoff/types/refusal.rs` | kernel_proofs | KILLED | a_stray_bit_above_the_depth_is_unknown_whatever_the_code |
| K2 both flags accepted | `src/boot/handoff/types/refusal.rs` | kernel_proofs | KILLED | both_flags_are_malformed_neither_is_no_reason_and_a_framebuffer_is_none |
| K3 no reason read as no GOP | `src/boot/handoff/types/refusal.rs` | kernel_proofs | KILLED | both_flags_are_malformed_neither_is_no_reason_and_a_framebuffer_is_none |
| K4 a depth with a code that has none | `src/boot/handoff/types/refusal.rs` | kernel_proofs | KILLED | every_code_byte_with_every_depth_decodes_to_its_reason_or_unknown |
| K5 an unknown word handed to userland raw | `src/boot/handoff/types/refusal_says.rs` | kernel_proofs | KILLED | userland_is_told_only_checked_words |
| K6 the loader's flag a different bit | `nonos-bootloader/src/handoff/types/constants.rs` | kernel_proofs | KILLED | every_word_the_loader_makes_decodes_to_what_it_meant, the_flag_is_bit_16_on_both_sides_and_every_other_flag_still_agrees |
| K7 the loader keeps a depth for any code | `nonos-bootloader/src/display/gop/refusal_word.rs` | kernel_proofs | KILLED | every_word_the_loader_makes_decodes_to_what_it_meant |
| K8 the both-flags check removed | `src/boot/handoff/api/security/framebuffer.rs` | kernel_proofs | KILLED | a_handoff_claiming_both_is_refused_before_anything_reads_the_framebuffer |
| K9 ENOTSUP with no word written | `src/syscall/dispatch/router/graphics_backend.rs` | kernel_proofs | KILLED | gdim_tells_why_with_enotsup_and_zero_with_a_framebuffer |
| K10 the loader never sets the flag | `nonos-bootloader/src/handoff/prepare/flags.rs` | kernel_proofs | KILLED | the_loader_sets_the_flag_with_no_framebuffer_and_hands_over_its_word |
| K11 flag_names stops at 16 | `src/boot/handoff/types/constants.rs` | kernel_proofs | KILLED | the_flag_is_bit_16_on_both_sides_and_every_other_flag_still_agrees |
| C1 said twice | `userland/compositor/src/refusal_note.rs` | compositor_proofs | KILLED | nothing_is_logged_before_the_bound_and_nothing_twice, said_at_once_then_once_more_at_the_bound_and_never_again |
| C2 the bound dropped | `userland/compositor/src/refusal_note.rs` | compositor_proofs | KILLED | nothing_is_logged_before_the_bound_and_nothing_twice, said_at_once_then_once_more_at_the_bound_and_never_again |
| C3 logged before the bound | `userland/compositor/src/refusal_note.rs` | compositor_proofs | KILLED | nothing_is_logged_before_the_bound_and_nothing_twice, said_at_once_then_once_more_at_the_bound_and_never_again |
| K4b BltOnly takes a depth | `src/boot/handoff/types/refusal.rs` | kernel_proofs | KILLED | every_code_byte_with_every_depth_decodes_to_its_reason_or_unknown |
| K4c BadGeometry takes a depth | `src/boot/handoff/types/refusal.rs` | kernel_proofs | KILLED | every_code_byte_with_every_depth_decodes_to_its_reason_or_unknown |
| K12 no 0 written with a framebuffer | `src/syscall/dispatch/router/graphics_backend.rs` | kernel_proofs | KILLED | gdim_tells_why_with_enotsup_and_zero_with_a_framebuffer |
| K13 a pointer left 0 written | `src/syscall/dispatch/router/graphics_backend.rs` | kernel_proofs | KILLED | gdim_tells_why_with_enotsup_and_zero_with_a_framebuffer |
| K14 out_why 0 written on the refusal path | `src/syscall/dispatch/router/graphics_backend.rs` | kernel_proofs | KILLED | gdim_tells_why_with_enotsup_and_zero_with_a_framebuffer |
| K15 refusal() drops the depth | `nonos-bootloader/src/display/gop/state.rs` | kernel_proofs | KILLED | the_loader_sets_the_flag_with_no_framebuffer_and_hands_over_its_word |
| K16 refusal() reads no failure | `nonos-bootloader/src/display/gop/state.rs` | kernel_proofs | KILLED | the_loader_sets_the_flag_with_no_framebuffer_and_hands_over_its_word |
