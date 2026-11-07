/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The axiom closure of every headline theorem, printed by Lean. CI runs this
file through formal/scripts/check_axioms.sh, which fails on `sorryAx` or on
any axiom outside Lean's standard three (propext, Classical.choice,
Quot.sound). The model adds no axiom of its own: the cryptographic primitives
are parameters of `Formal.Platform`.
-/

import Formal

#print axioms Formal.capability_monotonicity
#print axioms Formal.capability_bound_any_rule
#print axioms Formal.capability_monotonicity_without_admin
#print axioms Formal.AdminGrant.old_rule_grant_exceeds_admitted
#print axioms Formal.AdminGrant.fixed_rule_refuses_the_old_witness
#print axioms Formal.admission_soundness
#print axioms Formal.syscall_gate_completeness
#print axioms Formal.syscall_outside_allowlist_denied
#print axioms Formal.syscall_within_manifest
#print axioms Formal.only_attest_doc_negates
#print axioms Formal.wx_preservation
#print axioms Formal.isolation_preservation
#print axioms Formal.caps_change_only_by_target
#print axioms Formal.empty_word_reaches_only_tty_query_or_random
#print axioms Formal.launcher_matters_only_for_app_launch
#print axioms Formal.every_abi_number_is_claimed
#print axioms Formal.handler_requires_capability
#print axioms Formal.revoke_takes_effect
#print axioms Formal.quarantine_takes_effect
#print axioms Formal.dead_holds_nothing
#print axioms Formal.spawn_starts_empty
#print axioms Formal.ipc_touches_only_receiver
#print axioms Formal.revoke_bound
#print axioms Formal.protected_service_answers_to_init
