/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The axiom profile of the specification layer, checked by Lean itself rather
than asserted. `#print axioms` prints the exact axiom closure of a theorem:
for every flagship theorem below the expected output names at most Lean's
three standard axioms (propext, Classical.choice, Quot.sound) and never
`sorryAx`, which is what a `sorry` would introduce. The CI lean job runs this
file and publishes the output as evidence, so a green run leaves a record of
what was proven and on what foundations, not just an exit code.
-/

import Nonos

#print axioms Nonos.AntiRollback.no_rollback_after_boot
#print axioms Nonos.AntiRollbackState.refines_abstract
#print axioms Nonos.AntiRollbackState.no_rollback_after_update
#print axioms Nonos.AntiRollbackState.update_floor_monotone
#print axioms Nonos.Authorization.empty_token_denied
#print axioms Nonos.BlockIO.accepted_request_stays_on_disk
#print axioms Nonos.BootImage.accepted_region_stays_in_bounds
#print axioms Nonos.Capability.attenuate_confines
#print axioms Nonos.CapabilityBits.word_chain_never_widens
#print axioms Nonos.Crypto.wrong_tag_rejected

-- Wallet userland and signing crypto: live NOX/staking reads, EIP-1559 fee
-- sizing, EIP-2 low-s canonicalisation, address derivation, hex parsing, and
-- the private-key import wipe, each proven on Lean's standard axioms alone.
#print axioms Nonos.WalletNoxApr.empty_pool_no_rate
#print axioms Nonos.WalletNoxApr.apr_monotone_in_emission
#print axioms Nonos.WalletNoxCalldata.padding_is_zero
#print axioms Nonos.WalletNoxCalldata.address_placed
#print axioms Nonos.WalletQuantity.high_bytes_refused
#print axioms Nonos.WalletQuantity.decoded_lt_two_pow_128
#print axioms Nonos.WalletEip1559.tip_at_least_one_gwei
#print axioms Nonos.WalletEip1559.cap_covers_tip
#print axioms Nonos.WalletHex.nibble_lt_16
#print axioms Nonos.WalletFormatNox.whole_reconstructs
#print axioms Nonos.WalletFormatApr.reconstructs
#print axioms Nonos.WalletParseWord.words_disjoint
#print axioms Nonos.WalletImportWipe.wiped_all_zero
#print axioms Nonos.CryptoLowS.normalized_is_low
#print axioms Nonos.CryptoLowS.normalize_idempotent
#print axioms Nonos.CryptoKeccakAddr.in_hash_range
#print axioms Nonos.CryptoSecretValid.zero_rejected
#print axioms Nonos.CryptoSecretValid.overflow_rejected
#print axioms Nonos.CryptoSecretValid.valid_iff
#print axioms Nonos.CryptoRfc6979.distinct_nonce_distinct_msg
#print axioms Nonos.WalletRlp.single_low_byte_bare
#print axioms Nonos.WalletRlp.short_prefix_in_range
#print axioms Nonos.WalletGwei.never_overstates
#print axioms Nonos.WalletGwei.monotone
#print axioms Nonos.WalletShortAddr.shown_in_range
#print axioms Nonos.KeyringCustody.access_implies_owner
#print axioms Nonos.KeyringCustody.non_owner_denied
#print axioms Nonos.KeyringCustody.non_owner_indistinguishable
#print axioms Nonos.WalletTxEnvelope.type_prefix_unambiguous
#print axioms Nonos.WalletTxEnvelope.signing_adds_signature
#print axioms Nonos.WalletNonceReplay.replay_refused
#print axioms Nonos.WalletNonceReplay.no_two_nonces
#print axioms Nonos.CryptoGf256.add_self
#print axioms Nonos.CryptoGf256.add_cancel
#print axioms Nonos.CryptoGf256.add_assoc
#print axioms Nonos.CryptoKeccakPad.multiple_of_rate
#print axioms Nonos.CryptoKeccakPad.always_pads

#print axioms Nonos.Ipc.zero_length_rejected
#print axioms Nonos.Isolation.no_wx_page
#print axioms Nonos.Loader.accepted_entry_inside_file
#print axioms Nonos.NetParse.a_pointer_ends_the_walk
#print axioms Nonos.Paging.confined_preserves_no_wx
#print axioms Nonos.Path.leading_dotdot_neutralized
#print axioms Nonos.Secure.every_trace_is_secure
#print axioms Nonos.Secure.dma_owned_by_caller
#print axioms Nonos.Secure.dma_within_class_limit
#print axioms Nonos.Secure.elf_table_in_bounds
#print axioms Nonos.Secure.irq_vector_count_bounded
#print axioms Nonos.Secure.irq_not_already_bound
#print axioms Nonos.Secure.quota_within_cap
#print axioms Nonos.NonInterference.step_preserves_token
#print axioms Nonos.NonInterference.token_locality
#print axioms Nonos.NonInterference.token_noninterference
#print axioms Nonos.NonInterference.no_authority_leak
#print axioms Nonos.NonInterference.step_no_gain
#print axioms Nonos.NonInterference.no_authority_amplification
#print axioms Nonos.NonInterference.step_preserves_mapping
#print axioms Nonos.NonInterference.mapping_locality
#print axioms Nonos.NonInterference.mapping_noninterference
#print axioms Nonos.NonInterference.Unwinding.locality
#print axioms Nonos.NonInterference.Unwinding.noninterference
#print axioms Nonos.NonInterference.admitted_noninterference
#print axioms Nonos.NonInterference.dma_noninterference
#print axioms Nonos.NonInterference.copies_noninterference
#print axioms Nonos.NonInterference.floor_noninterference
#print axioms Nonos.NonInterference.elf_noninterference
#print axioms Nonos.NonInterference.irq_noninterference
#print axioms Nonos.NonInterference.step_preserves_domain_view
#print axioms Nonos.NonInterference.domain_noninterference
#print axioms Nonos.NonInterference.touches_disjoint
#print axioms Nonos.NonInterference.domain_isolation
#print axioms Nonos.NonInterference.disjoint_domains_noninterfere
#print axioms Nonos.AttestBinding.starkAttest_true_iff
#print axioms Nonos.AttestBinding.admitted_is_enrolled
#print axioms Nonos.AttestBinding.admitted_enrolled_after_trace
#print axioms Nonos.AttestBinding.admitted_accepted_after_trace
#print axioms Nonos.AttestBinding.no_cross_policy_replay
#print axioms Nonos.Spawn.only_attested_capsules_run
#print axioms Nonos.Spawn.enforcing_run_admits_only_verified
#print axioms Nonos.Spawn.enforcing_refuses_a_missing_trailer
#print axioms Nonos.Spawn.production_is_always_enforcing
#print axioms Nonos.SpawnCaps.installed_within_ceiling
#print axioms Nonos.SpawnCaps.installed_within_manifest
#print axioms Nonos.SpawnCaps.authority_only_narrows
#print axioms Nonos.Delegation.never_outlives_parent
#print axioms Nonos.Delegation.never_outlasts_request
#print axioms Nonos.Delegation.live_child_implies_live_parent
#print axioms Nonos.Stark.AssociationSet.an_excluded_deposit_cannot_pass
#print axioms Nonos.Stark.AssociationSet.the_registry_only_grows
#print axioms Nonos.Stark.Attest.a_proof_for_one_capsule_is_rejected_for_another
#print axioms Nonos.Stark.Constraint.constraint_holds_iff_quotient_exists
#print axioms Nonos.Stark.Constraint.the_quotient_is_pinned
#print axioms Nonos.Stark.CopyConstraint.wiring_forces_equality
#print axioms Nonos.Stark.Field.mul_distributes_over_add
#print axioms Nonos.Stark.Extension.low_degree_extension_is_unique
#print axioms Nonos.Stark.Commitment.commitment_binds_value
#print axioms Nonos.Stark.FeeRouter.route_conserves
#print axioms Nonos.Stark.FeeRouter.an_accepted_fee_is_within_cap
#print axioms Nonos.Stark.Fold.eval_split
#print axioms Nonos.Stark.Instance.honest_run_has_a_quotient
#print axioms Nonos.Stark.Instance.tampered_run_has_no_quotient
#print axioms Nonos.Stark.Fold.fold_length
#print axioms Nonos.Stark.Fold.the_honest_fold_reaches_a_constant
#print axioms Nonos.Stark.Fri.final_layer_accepts_iff_matches
#print axioms Nonos.Stark.Lookup.non_table_value_has_zero_multiplicity
#print axioms Nonos.Stark.Merkle.distinct_leaves_give_distinct_roots
#print axioms Nonos.Stark.NullifierSet.a_recorded_nullifier_cannot_be_respent
#print axioms Nonos.Stark.NullifierSet.spend_only_grows
#print axioms Nonos.Stark.Permutation.perm_preserves_count
#print axioms Nonos.Stark.Pool.every_reachable_pool_is_solvent
#print axioms Nonos.Stark.RootWindow.a_pushed_root_is_accepted
#print axioms Nonos.Stark.RootWindow.the_window_never_exceeds_the_cap
#print axioms Nonos.Stark.Staking.rewards_never_exceed_fees
#print axioms Nonos.Stark.RunningSum.final_is_total
#print axioms Nonos.Stark.RunningSum.conservation
#print axioms Nonos.Stark.RunningSum.follows_pins_the_trace
#print axioms Nonos.Stark.Polynomial.eval_mul
#print axioms Nonos.Stark.Polynomial.zerofier_nonzero_off_the_points
#print axioms Nonos.Stark.Polynomial.factor
#print axioms Nonos.Stark.Polynomial.roots_divide
#print axioms Nonos.Stark.Polynomial.agreement_divides_by_zerofier
#print axioms Nonos.Stark.Polynomial.root_bound
#print axioms Nonos.Stark.Transcript.order_changes_the_state
#print axioms Nonos.Syscall.decode_agrees_with_the_registry
#print axioms Nonos.UsbHid.bindings_never_exceed_the_cap

-- Assurance capstone: the guarantees composed, post-quantum hybrid authority,
-- and the admission theorem that holds over the exact gated function the loader
-- runs (attested AND rollback-fresh AND post-quantum-authorized).
#print axioms Nonos.Assurance.authority
#print axioms Nonos.Assurance.freshness
#print axioms Nonos.Assurance.integrity
#print axioms Nonos.Assurance.classical_break_insufficient
#print axioms Nonos.Assurance.no_pq_no_authority
#print axioms Nonos.Assurance.only_ok_capsules_run
#print axioms Nonos.Assurance.run_capsule_pq_authorized
#print axioms Nonos.Assurance.unattested_never_runs
#print axioms Nonos.Assurance.stale_never_runs
#print axioms Nonos.Assurance.unsigned_pq_never_runs

-- Kernel-mechanism modules added alongside the capstone. One flagship theorem
-- from each is profiled so the CI evidence records its axiom closure and the
-- sorryAx gate covers it.
#print axioms Nonos.MemGrant.run_conserves
#print axioms Nonos.MemGrant.run_granted_le_capacity
#print axioms Nonos.Scheduler.rotate_mem
#print axioms Nonos.Scheduler.at_most_one_cpu_wins
#print axioms Nonos.Scheduler.a_ready_process_is_claimed_once
#print axioms Nonos.Scheduler.only_ready_is_claimed
#print axioms Nonos.PageDescriptor.leaf_present_iff_requested
#print axioms Nonos.PageDescriptor.kernel_leaf_never_reaches_el0
#print axioms Nonos.PageDescriptor.user_leaf_reaches_el0
#print axioms Nonos.PageDescriptor.read_only_leaf_never_writable
#print axioms Nonos.PageDescriptor.user_leaf_never_executes_at_el1
#print axioms Nonos.PageDescriptor.kernel_leaf_never_executes_at_el0
#print axioms Nonos.PageDescriptor.table_is_present_and_not_a_block
#print axioms Nonos.PageTable.mapAllChecked_safe
#print axioms Nonos.Iommu.empty_grant_denies
#print axioms Nonos.Dispatch.serviced_requires_cap
#print axioms Nonos.Dispatch.denied_below
#print axioms Nonos.Quota.acquireAll_used_le_cap
#print axioms Nonos.Interval.disjoint_not_mem
#print axioms Nonos.Interval.mem_of_subset
#print axioms Nonos.Refcount.dec_from_one_dead
#print axioms Nonos.Rflags.mask_is_exactly_the_privileged_bits
#print axioms Nonos.Rflags.iopl_is_masked
#print axioms Nonos.Rflags.interrupt_flag_is_not_masked
#print axioms Nonos.Timer.tickAll_monotone
#print axioms Nonos.Endpoint.recv_was_sent
#print axioms Nonos.Heap.double_free_safe
#print axioms Nonos.Heap.free_not_allocated
#print axioms Nonos.Fd.close_not_open
#print axioms Nonos.Ring.run_count_le_cap
#print axioms Nonos.Bounds.index_in_buffer
#print axioms Nonos.Nonce.issue2_distinct
#print axioms Nonos.Priority.preempts_total
#print axioms Nonos.Zeroize.wiped_is_zero
#print axioms Nonos.Mmio.empty_grant_denies
#print axioms Nonos.CapTable.revoke_not_holds
#print axioms Nonos.CapTable.grant_then_revoke

-- Capability tokens: a valid token clears all three gates, and the boolean and
-- result-path entry points agree.
#print axioms Nonos.CapToken.valid_not_revoked
#print axioms Nonos.CapToken.revoked_invalid
#print axioms Nonos.CapToken.full_ok_iff_valid

-- Capability masks: a subset (delegated) mask never carries a capability its
-- parent lacks, and granting is monotone and non-aliasing.
#print axioms Nonos.CapMask.subset_no_extra
#print axioms Nonos.CapMask.has_add_other
#print axioms Nonos.CapMask.subset_trans
#print axioms Nonos.Vfs.resolve_dotdots_root
#print axioms Nonos.Rng.drawN_advances
#print axioms Nonos.Tlb.invalidate_evicts

-- Concurrency, reclamation and rate-limiting mechanisms. One flagship theorem
-- from each is profiled so its axiom closure is recorded and the sorryAx gate
-- covers it.
#print axioms Nonos.Semaphore.acquire_valid
#print axioms Nonos.Semaphore.acquire_release_roundtrip
#print axioms Nonos.Mutex.owner_unique
#print axioms Nonos.Ticket.serving_unique
#print axioms Nonos.Ticket.take_monotone
#print axioms Nonos.Seqlock.changed_rejected
#print axioms Nonos.TokenBucket.refill_never_exceeds_burst
#print axioms Nonos.Signal.blocked_signal_still_pending
#print axioms Nonos.Signal.unblock_delivers
#print axioms Nonos.Reaper.reaped_not_zombie
#print axioms Nonos.Epoch.drainN_old_le
#print axioms Nonos.Barrier.not_released_before_all
#print axioms Nonos.Buddy.alloc_conserves
#print axioms Nonos.Buddy.split_conserves
#print axioms Nonos.Cow.write_drops_original
#print axioms Nonos.Bitmap.set_then_clear_frees

-- Locking and address-space mechanisms, each backed by a real kernel primitive.
#print axioms Nonos.Spinlock.try_fails_when_held
#print axioms Nonos.Rwlock.writer_excludes_readers
#print axioms Nonos.Futex.fifo_first_out
#print axioms Nonos.Futex.waiter_enqueued
#print axioms Nonos.Vma.disjoint_no_shared_addr

-- WiFi trusted path: the WPA2 supplicant's handshake discipline and the CCMP
-- packet-number replay window the data plane enforces under each key.
#print axioms Nonos.Wpa2Handshake.install_requires_valid_msg3
#print axioms Nonos.Wpa2Handshake.replay_never_advances
#print axioms Nonos.Wpa2Handshake.mic_input_within_eapol
#print axioms Nonos.Wpa2Handshake.connected_ptk_from_fixed_nonces
#print axioms Nonos.CcmpReplay.no_nonce_reuse
#print axioms Nonos.CcmpReplay.replay_dropped
#print axioms Nonos.CcmpReplay.accepted_pn_dead_forever

-- User/kernel boundary: the range policy every usercopy clears before a byte
-- moves keeps an accepted range wholly inside user space.
#print axioms Nonos.UserCopy.accepted_within_user
#print axioms Nonos.UserCopy.accepted_nonzero_addr

-- The page walk behind that policy: every table above a returned leaf granted
-- user, and a leaf a transfer accepts is user and, for writes, writable.
#print axioms Nonos.UserWalk.tables_above_grant_user
#print axioms Nonos.UserWalk.read_path_is_user_accessible
#print axioms Nonos.UserWalk.write_path_is_user_writable
#print axioms Nonos.UserWalk.write_implies_read

-- Demand paging: the fault router and the per-process page budget, and the
-- fact that a served page is never executable.
#print axioms Nonos.DemandPaging.kernel_half_never_mapped
#print axioms Nonos.DemandPaging.refused_forever
#print axioms Nonos.DemandPaging.saturated_refused
#print axioms Nonos.DemandPaging.saturated_admission_bounded
#print axioms Nonos.DemandPaging.demand_not_wx

-- ELF load protection: no writable-and-executable segment is admitted, and the
-- RELRO span ends the load read-only.
#print axioms Nonos.LoadProtect.accepted_wx_safe
#print axioms Nonos.LoadProtect.sealed_not_writable

-- Service registry: registration preserves name and port uniqueness and never
-- grows the table past its cap.
#print axioms Nonos.ServiceRegistry.register_preserves_names
#print axioms Nonos.ServiceRegistry.register_preserves_ports
#print axioms Nonos.ServiceRegistry.register_within_cap

-- PCI command-write allowlist: an admitted write changes only writable bits,
-- the merge branch is confined, and a raw protected-bit write is refused.
#print axioms Nonos.PciCmdWrite.admitted_changes_only_writable
#print axioms Nonos.PciCmdWrite.merge_branch_confined
#print axioms Nonos.PciCmdWrite.raw_protected_write_refused

-- PID allocation: an allocated PID is never the reserved 0, is not already
-- live, and the stored counter never wraps to 0.
#print axioms Nonos.PidAlloc.chosen_pid_ne_zero
#print axioms Nonos.PidAlloc.chosen_pid_inactive
#print axioms Nonos.PidAlloc.chosen_next_ne_zero

-- Network state machines: DHCP binds a lease only on a matching ACK, and TCP
-- reaches Established only through the handshake.
#print axioms Nonos.Dhcp.bound_only_via_matching_ack
#print axioms Nonos.Tcp.established_only_via_handshake

-- Syscall routing: an unclaimed number is refused, and the first handler that
-- claims a number decides it and shadows all later handlers.
#print axioms Nonos.SyscallRoute.route_unclaimed_is_enosys
#print axioms Nonos.SyscallRoute.route_earlier_shadows
#print axioms Nonos.SyscallRoute.route_append_stable

-- File-descriptor allocation: a returned descriptor is the lowest free one at
-- or above the floor, and the allocator declines only when the window is full.
#print axioms Nonos.FdAlloc.alloc_free
#print axioms Nonos.FdAlloc.alloc_lowest
#print axioms Nonos.FdAlloc.alloc_none_full

-- Multisig k-of-n: an accepted config is a well-formed threshold, a valid add
-- keeps signers distinct and authorized, and a met threshold is backed by them.
#print axioms Nonos.MultiSig.valid_config
#print axioms Nonos.MultiSig.add_preserves_nodup
#print axioms Nonos.MultiSig.threshold_backed_by_distinct_authorized

-- MSI-X exclusion: no address a clamped BAR mapping covers falls inside the
-- protected MSI-X table or PBA region.
#print axioms Nonos.MsixExclusion.no_protected_byte_mapped
#print axioms Nonos.MsixExclusion.safeLen_le_length

-- MSI-X interrupt bind: an admitted bind is bounded to the pool and device
-- table, addressable, a device IRQ, and not a double-bind.
#print axioms Nonos.IrqBind.accepted_vector_count_bounded
#print axioms Nonos.IrqBind.accepted_msix_addressable
#print axioms Nonos.IrqBind.accepted_not_already_bound

-- DMA map admission: an accepted mapping is owned by the caller, on a fresh
-- claim epoch, and bounded to the device class page limit.
#print axioms Nonos.DmaMap.accepted_owned_by_caller
#print axioms Nonos.DmaMap.accepted_fresh_epoch
#print axioms Nonos.DmaMap.accepted_within_class_limit

-- ELF relocation write size: any supported relocation writes at most 8 bytes,
-- an unknown type is refused, and an admitted relocation writes only in-segment.
#print axioms Nonos.ElfReloc.writeSize_le_8
#print axioms Nonos.ElfReloc.other_unsupported
#print axioms Nonos.ElfReloc.admitted_no_oob

-- ELF program-header bounds: an accepted table lies wholly inside the image, so
-- every header the loader reads is in bounds.
#print axioms Nonos.ElfPhdr.accepted_table_in_bounds
#print axioms Nonos.ElfPhdr.accepted_no_overflow
#print axioms Nonos.ElfPhdr.wrong_size_rejected


-- Scheduling fairness / liveness: the ready set is a scheduling invariant, no
-- admitted task is ever dropped or starved out of the queue.
#print axioms Nonos.Fairness.rotateN_mem
#print axioms Nonos.Fairness.rotateN_length
#print axioms Nonos.Fairness.no_starvation_by_loss
#print axioms Nonos.Fairness.never_stalls
#print axioms Nonos.Fairness.rotateN_to_head
#print axioms Nonos.Fairness.reaches_head

-- Trusted-path audit fixes (PR #405). Each theorem pins the property the code
-- change establishes, on Lean's standard axioms alone.
#print axioms Nonos.FrameNoAlias.alloc_no_alias
#print axioms Nonos.FrameNoAlias.bump_aliases
#print axioms Nonos.FrameNoAlias.fixed_none_on_empty
#print axioms Nonos.InputConsumer.drain_implies_post
#print axioms Nonos.InputConsumer.irq_only_cannot_drain
-- And who can hold a token to stamp with in the first place: only the process
-- the call was addressed to, spent once.
#print axioms Nonos.ReplyAuthorization.a_reply_token_comes_from_a_call_to_the_replier
#print axioms Nonos.ReplyAuthorization.a_redirect_token_comes_from_a_call_to_the_sender
#print axioms Nonos.ReplyAuthorization.remove_consumes_one
#print axioms Nonos.ReplyAuthorization.pop_consumes_one

#print axioms Nonos.ReplyCorrelation.forged_never_delivered
#print axioms Nonos.ReplyCorrelation.firstMatch_matches
#print axioms Nonos.ReplyCorrelation.all_forged_none
#print axioms Nonos.ServiceRegisterAuth.new_no_bypass
#print axioms Nonos.ServiceRegisterAuth.old_low_pid_bypass
#print axioms Nonos.FramebufferSwap.swap_involutive
#print axioms Nonos.FramebufferSwap.present_rgb_converts
#print axioms Nonos.ZeroState.off_was_wiped
#print axioms Nonos.ZeroState.wipe_covers_every_region
#print axioms Nonos.ZeroState.wiped_residue_empty
#print axioms Nonos.ZeroState.off_holds_nothing
#print axioms Nonos.ZeroState.into_off_only_from_wiped
#print axioms Nonos.ZeroState.no_shortcut_to_off
#print axioms Nonos.ZeroState.quiescing_is_not_wiping
#print axioms Nonos.ZeroState.secrets_imply_still_here
#print axioms Nonos.ZeroState.off_is_terminal
#print axioms Nonos.StationAddress.never_group
#print axioms Nonos.StationAddress.always_local
#print axioms Nonos.StationAddress.never_a_factory_address
#print axioms Nonos.StationAddress.never_broadcast
#print axioms Nonos.StationAddress.never_zero

-- The answer path: a refusal that does not reach the caller is not a refusal,
-- and a call nobody answered must deliver nothing. Both were unstated until a
-- resume from the saved trap frame reported every refused syscall as success.
#print axioms Nonos.Answer.refusal_is_an_error
#print axioms Nonos.Answer.served_is_not_an_error
#print axioms Nonos.Answer.error_test_is_faithful
#print axioms Nonos.Answer.parked_yields_nothing
#print axioms Nonos.Answer.observation_implies_decided
#print axioms Nonos.Answer.answer_ignores_the_request
#print axioms Nonos.Answer.trap_frame_resume_fakes_success
#print axioms Nonos.Answer.trap_frame_resume_is_unsound
#print axioms Nonos.Answer.small_request_numbers_look_like_success
#print axioms Nonos.Answer.gate_refusals_reach_the_caller
#print axioms Nonos.Answer.gate_is_void_when_answered_from_the_frame

-- DnsAutomap
#print axioms Nonos.DnsAutomap.range_is_cgnat
#print axioms Nonos.DnsAutomap.size_matches_the_prefix
#print axioms Nonos.DnsAutomap.address_is_in_range
#print axioms Nonos.DnsAutomap.address_is_injective
#print axioms Nonos.DnsAutomap.position_of_address
#print axioms Nonos.DnsAutomap.outside_the_range_has_no_position
#print axioms Nonos.DnsAutomap.find_lt
#print axioms Nonos.DnsAutomap.find_correct
#print axioms Nonos.DnsAutomap.repeated_resolution_is_stable
#print axioms Nonos.DnsAutomap.new_name_is_appended
#print axioms Nonos.DnsAutomap.resolved_addresses_are_synthetic
#print axioms Nonos.DnsAutomap.automap_round_trips
#print axioms Nonos.DnsAutomap.existing_address_maps_back
#print axioms Nonos.DnsAutomap.resolution_emits_nothing
#print axioms Nonos.DnsAutomap.no_name_reaches_a_resolver
#print axioms Nonos.DnsAutomap.exhaustion_refuses_rather_than_asking
#print axioms Nonos.DnsAutomap.refusal_carries_no_address

-- ForeignBoundary
#print axioms Nonos.ForeignBoundary.accepted_bytes_are_user_bytes
#print axioms Nonos.ForeignBoundary.boundary_is_exclusive
#print axioms Nonos.ForeignBoundary.wrapAdd_faithful_below_the_word
#print axioms Nonos.ForeignBoundary.wrapping_sum_looks_contained
#print axioms Nonos.ForeignBoundary.wrapping_span_is_refused
#print axioms Nonos.ForeignBoundary.pidArg_is_exact
#print axioms Nonos.ForeignBoundary.truncation_renames_the_target
#print axioms Nonos.ForeignBoundary.never_reaches_an_unsupervised_guest
#print axioms Nonos.ForeignBoundary.out_of_range_pid_resolves_nothing
#print axioms Nonos.ForeignBoundary.own_guest_resolves
#print axioms Nonos.ForeignBoundary.chunk_is_bounded
#print axioms Nonos.ForeignBoundary.chunk_stays_inside
#print axioms Nonos.ForeignBoundary.chunk_of_a_contained_range_is_contained
#print axioms Nonos.ForeignBoundary.nonempty_takes_a_call
#print axioms Nonos.ForeignBoundary.empty_takes_no_calls
#print axioms Nonos.ForeignBoundary.last_chunk_ends_at_len

-- FpuState
#print axioms Nonos.FpuState.every_is_complete
#print axioms Nonos.FpuState.zero_unmasks_everything
#print axioms Nonos.FpuState.zero_traps_on_arithmetic
#print axioms Nonos.FpuState.default_masks_everything
#print axioms Nonos.FpuState.default_never_traps
#print axioms Nonos.FpuState.default_sets_nothing_else
#print axioms Nonos.FpuState.mask_field_is_exactly_the_default
#print axioms Nonos.FpuState.default_is_the_sum_of_the_masks
#print axioms Nonos.FpuState.blank_is_not_fresh
#print axioms Nonos.FpuState.fresh_restore_is_usable
#print axioms Nonos.FpuState.blank_restore_is_unusable

-- GuestLayout
#print axioms Nonos.GuestLayout.layout_is_ordered
#print axioms Nonos.GuestLayout.limits_are_page_aligned
#print axioms Nonos.GuestLayout.pageUp_ge
#print axioms Nonos.GuestLayout.pageUp_aligned
#print axioms Nonos.GuestLayout.pageUp_lt_next_page
#print axioms Nonos.GuestLayout.pageUp_fixes_aligned
#print axioms Nonos.GuestLayout.pageUp_respects_an_aligned_bound
#print axioms Nonos.GuestLayout.break_stays_below_mappings
#print axioms Nonos.GuestLayout.break_never_goes_below_base
#print axioms Nonos.GuestLayout.outside_the_window_is_refused
#print axioms Nonos.GuestLayout.heap_apart_from_mappings
#print axioms Nonos.GuestLayout.mappings_apart_from_image
#print axioms Nonos.GuestLayout.mappings_apart_from_stack
#print axioms Nonos.GuestLayout.heap_apart_from_stack
#print axioms Nonos.GuestLayout.guest_reach_is_below_the_kernel_half
#print axioms Nonos.GuestLayout.guest_reach_is_bounded
#print axioms Nonos.GuestLayout.zero_length_unmap_is_refused

-- ImageCeiling
#print axioms Nonos.ImageCeiling.table_indices_are_the_range
#print axioms Nonos.ImageCeiling.shipped_is_thirty_one_bits
#print axioms Nonos.ImageCeiling.shipped_admits_the_low_table
#print axioms Nonos.ImageCeiling.shipped_refuses_attest_read
#print axioms Nonos.ImageCeiling.shipped_refuses_foreign_exec
#print axioms Nonos.ImageCeiling.shipped_refuses_local_sign
#print axioms Nonos.ImageCeiling.the_shipped_ceiling_is_not_sound
#print axioms Nonos.ImageCeiling.full_mask_is_sound
#print axioms Nonos.ImageCeiling.full_mask_covers_the_highest
#print axioms Nonos.ImageCeiling.sound_ceiling_admits_every_capability
#print axioms Nonos.ImageCeiling.sound_ceiling_is_at_least_full
#print axioms Nonos.ImageCeiling.an_unenforced_ceiling_refuses_nothing
#print axioms Nonos.ImageCeiling.shipped_ceiling_refused_nothing_in_practice
#print axioms Nonos.ImageCeiling.enforcement_changes_behaviour_where_unsound
#print axioms Nonos.ImageCeiling.enforcing_a_sound_ceiling_is_invisible

-- LocalSign
#print axioms Nonos.LocalSign.unheld_authority_is_unreachable
#print axioms Nonos.LocalSign.dedicated_bit_is_reachable
#print axioms Nonos.LocalSign.dedicated_bit_still_refuses
#print axioms Nonos.LocalSign.asking_is_not_signing
#print axioms Nonos.LocalSign.refusal_became_an_untrailered_install
#print axioms Nonos.LocalSign.propagating_install_refuses
#print axioms Nonos.LocalSign.propagating_install_never_writes_untrailered
#print axioms Nonos.LocalSign.one_dissenting_site_closes_the_path
#print axioms Nonos.LocalSign.agreement_opens_the_path
#print axioms Nonos.LocalSign.derived_sites_agree
#print axioms Nonos.LocalSign.derived_path_is_open_iff_held
#print axioms Nonos.LocalSign.the_seven_sites_agree

-- PerCpuAsid
#print axioms Nonos.PerCpuAsid.no_single_value_tracks_two_spaces
#print axioms Nonos.PerCpuAsid.per_cpu_read_is_exact
#print axioms Nonos.PerCpuAsid.global_read_is_exact_only_by_luck
#print axioms Nonos.PerCpuAsid.flushed_space_is_gone
#print axioms Nonos.PerCpuAsid.flush_keeps_other_spaces
#print axioms Nonos.PerCpuAsid.wrong_key_leaves_the_stale_entry
#print axioms Nonos.PerCpuAsid.right_key_clears_the_space
#print axioms Nonos.PerCpuAsid.single_field_flush_is_unsound
#print axioms Nonos.PerCpuAsid.reload_leaves_nothing
#print axioms Nonos.PerCpuAsid.reload_is_sound_regardless
#print axioms Nonos.PerCpuAsid.pcid_reload_is_the_selective_flush

-- Provenance
#print axioms Nonos.Provenance.le_refl
#print axioms Nonos.Provenance.le_trans
#print axioms Nonos.Provenance.rank_injective
#print axioms Nonos.Provenance.unauthenticated_is_bottom
#print axioms Nonos.Provenance.verified_is_top
#print axioms Nonos.Provenance.meet_le_left
#print axioms Nonos.Provenance.meet_le_right
#print axioms Nonos.Provenance.unauthenticated_is_absorbing
#print axioms Nonos.Provenance.meet_never_raises
#print axioms Nonos.Provenance.no_signature_concludes_nothing
#print axioms Nonos.Provenance.verified_requires_both
#print axioms Nonos.Provenance.signature_alone_stops_at_signed
#print axioms Nonos.Provenance.stage_never_raises
#print axioms Nonos.Provenance.no_laundering
#print axioms Nonos.Provenance.unauthenticated_stays
#print axioms Nonos.Provenance.unauthenticated_never_executes
#print axioms Nonos.Provenance.signed_but_unmatched_never_executes
#print axioms Nonos.Provenance.fetched_without_authentication_never_runs
#print axioms Nonos.Provenance.never_attests_unauthenticated

-- StoreRoot
#print axioms Nonos.StoreRoot.root_is_a_prefix
#print axioms Nonos.StoreRoot.up_at_the_root_is_refused
#print axioms Nonos.StoreRoot.escape_is_refused
#print axioms Nonos.StoreRoot.resolve_refuses_an_escape
#print axioms Nonos.StoreRoot.up_below_the_root_pops
#print axioms Nonos.StoreRoot.here_is_a_no_op
#print axioms Nonos.StoreRoot.depth_is_bounded
#print axioms Nonos.StoreRoot.resolved_length_is_bounded
#print axioms Nonos.StoreRoot.every_key_is_confined
#print axioms Nonos.StoreRoot.key_remembers_its_root
#print axioms Nonos.StoreRoot.escape_mints_no_key
#print axioms Nonos.StoreRoot.every_open_is_under_the_root

-- TrapEntry
#print axioms Nonos.TrapEntry.zero_means_no_switch
#print axioms Nonos.TrapEntry.encode_decode
#print axioms Nonos.TrapEntry.encode_injective
#print axioms Nonos.TrapEntry.slot_is_never_zero
#print axioms Nonos.TrapEntry.encoded_slots_are_in_range
#print axioms Nonos.TrapEntry.off_by_one_selects_the_stack_below
#print axioms Nonos.TrapEntry.off_by_one_disables_the_first_slot
#print axioms Nonos.TrapEntry.wrong_encoding_is_not_injective
#print axioms Nonos.TrapEntry.swap_involutive
#print axioms Nonos.TrapEntry.even_swaps_are_identity
#print axioms Nonos.TrapEntry.odd_swaps_are_a_swap
#print axioms Nonos.TrapEntry.paired_path_restores_the_user_base
#print axioms Nonos.TrapEntry.unpaired_path_leaks_the_kernel_base
#print axioms Nonos.TrapEntry.unconditional_swap_breaks_ring_zero
#print axioms Nonos.TrapEntry.handler_always_runs_on_the_kernel_base
#print axioms Nonos.TrapEntry.return_restores_what_entry_changed

-- ApBringup
#print axioms Nonos.ApBringup.arrived_survives
#print axioms Nonos.ApBringup.interrupts_need_the_tables
#print axioms Nonos.ApBringup.interrupts_need_the_per_cpu_base
#print axioms Nonos.ApBringup.interrupts_need_the_lapic
#print axioms Nonos.ApBringup.interrupts_after_everything_survive
#print axioms Nonos.ApBringup.halting_with_interrupts_off_never_wakes
#print axioms Nonos.ApBringup.halting_with_interrupts_on_wakes
#print axioms Nonos.ApBringup.the_window_is_exactly_one_step
#print axioms Nonos.ApBringup.one_step_earlier_does_not_survive
#print axioms Nonos.ApBringup.the_bad_order_looks_correct_at_the_end
#print axioms Nonos.ApBringup.removing_it_does_not_wake
#print axioms Nonos.ApBringup.started_is_not_ready
#print axioms Nonos.ApBringup.waiting_for_ready_is_what_makes_it_safe
#print axioms Nonos.ApBringup.counting_before_the_acknowledgement_is_unsafe

-- ArchiveStreams
#print axioms Nonos.ArchiveStreams.agree_on_empty
#print axioms Nonos.ArchiveStreams.one_member_is_indistinguishable
#print axioms Nonos.ArchiveStreams.a_prefix_reader_is_correct_on_short_input
#print axioms Nonos.ArchiveStreams.a_prefix_reader_is_wrong_on_long_input
#print axioms Nonos.ArchiveStreams.it_drops_the_tail
#print axioms Nonos.ArchiveStreams.the_index_is_in_a_later_member
#print axioms Nonos.ArchiveStreams.the_full_reader_finds_it
#print axioms Nonos.ArchiveStreams.an_empty_result_is_not_an_error
#print axioms Nonos.ArchiveStreams.both_stop_at_the_end_marker
#print axioms Nonos.ArchiveStreams.a_single_zero_block_is_padding
#print axioms Nonos.ArchiveStreams.leading_padding_yields_nothing
#print axioms Nonos.ArchiveStreams.the_walks_agree_without_padding
#print axioms Nonos.ArchiveStreams.the_correct_walk_invents_nothing
#print axioms Nonos.ArchiveStreams.both_defects_yield_nothing
#print axioms Nonos.ArchiveStreams.the_fixed_reader_finds_both

-- ArgvBounds
#print axioms Nonos.ArgvBounds.short_arguments_are_unaffected
#print axioms Nonos.ArgvBounds.the_path_bound_refuses_a_legal_argument
#print axioms Nonos.ArgvBounds.the_bounds_differ_on_an_interval
#print axioms Nonos.ArgvBounds.oversized_arguments_are_still_refused
#print axioms Nonos.ArgvBounds.sound_total_fits_the_stack
#print axioms Nonos.ArgvBounds.the_fit_has_margin
#print axioms Nonos.ArgvBounds.a_total_the_size_of_the_stack_is_unsound
#print axioms Nonos.ArgvBounds.the_total_is_coupled_to_the_stack
#print axioms Nonos.ArgvBounds.raising_both_stays_sound
#print axioms Nonos.ArgvBounds.greatest_sound_total
#print axioms Nonos.ArgvBounds.count_is_bounded_by_the_total
#print axioms Nonos.ArgvBounds.no_arguments_fit
#print axioms Nonos.ArgvBounds.one_maximal_argument_fits

-- GateLayers
#print axioms Nonos.GateLayers.entry_alone_is_not_authority
#print axioms Nonos.GateLayers.authority_alone_is_not_the_whole_gate
#print axioms Nonos.GateLayers.absent_from_the_table_is_unreachable
#print axioms Nonos.GateLayers.denied_by_authority_is_unreachable
#print axioms Nonos.GateLayers.admission_requires_both
#print axioms Nonos.GateLayers.both_passing_admits
#print axioms Nonos.GateLayers.widening_the_table_grants_no_authority
#print axioms Nonos.GateLayers.widening_the_table_widens_reach
#print axioms Nonos.GateLayers.the_audit_is_complete
#print axioms Nonos.GateLayers.nothing_is_unaudited
#print axioms Nonos.GateLayers.every_syscall_has_both_layers
#print axioms Nonos.GateLayers.disclosure_and_independence_conflict
#print axioms Nonos.GateLayers.a_hardcoded_pid_names_whoever_holds_it

-- GsiRouting
#print axioms Nonos.GsiRouting.identity_without_an_override
#print axioms Nonos.GsiRouting.override_wins
#print axioms Nonos.GsiRouting.other_overrides_are_skipped
#print axioms Nonos.GsiRouting.translation_is_total
#print axioms Nonos.GsiRouting.a_collapsing_translation_merges_two_devices
#print axioms Nonos.GsiRouting.identity_collapses_nothing
#print axioms Nonos.GsiRouting.vector_avoids_the_exception_range
#print axioms Nonos.GsiRouting.vector_fits
#print axioms Nonos.GsiRouting.vectors_are_distinct
#print axioms Nonos.GsiRouting.the_boot_processor_receives_everything
#print axioms Nonos.GsiRouting.a_fixed_destination_starves_the_others
#print axioms Nonos.GsiRouting.a_chosen_destination_reaches_any_processor
#print axioms Nonos.GsiRouting.masked_delivers_to_nobody
#print axioms Nonos.GsiRouting.unmasked_rewrite_has_a_bad_window
#print axioms Nonos.GsiRouting.masked_rewrite_has_no_window
#print axioms Nonos.GsiRouting.masked_rewrite_ends_correct
#print axioms Nonos.GsiRouting.a_routed_interrupt_arrives
#print axioms Nonos.GsiRouting.uncollapsed_interrupts_get_different_vectors

-- ImageStaging
#print axioms Nonos.ImageStaging.stale_package_boots_the_earlier_build
#print axioms Nonos.ImageStaging.the_tree_holds_the_later_build
#print axioms Nonos.ImageStaging.tree_and_image_disagree
#print axioms Nonos.ImageStaging.package_copies_the_tree
#print axioms Nonos.ImageStaging.building_leaves_the_image
#print axioms Nonos.ImageStaging.booting_changes_nothing
#print axioms Nonos.ImageStaging.no_package_boots_nothing
#print axioms Nonos.ImageStaging.resequenced_boots_the_current_build
#print axioms Nonos.ImageStaging.sequenced_tree_and_image_agree
#print axioms Nonos.ImageStaging.package_then_boot_is_current
#print axioms Nonos.ImageStaging.measurement_follows_the_image
#print axioms Nonos.ImageStaging.stale_measurement_is_true_about_the_wrong_kernel
#print axioms Nonos.ImageStaging.measuring_before_packaging_measures_nothing
#print axioms Nonos.ImageStaging.measure_after_package_matches_the_boot
#print axioms Nonos.ImageStaging.all_three_agree_when_sequenced

-- PodBound
#print axioms Nonos.PodBound.bool_has_no_padding
#print axioms Nonos.PodBound.bool_is_not_inhabited
#print axioms Nonos.PodBound.invalid_pattern_is_reachable
#print axioms Nonos.PodBound.plain_read_is_always_sound
#print axioms Nonos.PodBound.non_plain_read_needs_a_check
#print axioms Nonos.PodBound.plain_layout_writes_only_the_value
#print axioms Nonos.PodBound.padding_is_not_determined_by_the_value
#print axioms Nonos.PodBound.padding_fails_the_bound
#print axioms Nonos.PodBound.padded_layout_leaks
#print axioms Nonos.PodBound.padded_layout_reads_soundly
#print axioms Nonos.PodBound.padded_layout_is_not_plain
#print axioms Nonos.PodBound.the_bound_discharges_every_site
#print axioms Nonos.PodBound.one_bad_site_is_enough

-- Reachability
#print axioms Nonos.Reachability.unreached_is_not_live
#print axioms Nonos.Reachability.removing_an_unreached_control_changes_nothing
#print axioms Nonos.Reachability.a_reached_control_can_refuse
#print axioms Nonos.Reachability.dead_stack_is_empty
#print axioms Nonos.Reachability.dead_controls_grant_everything
#print axioms Nonos.Reachability.any_number_of_dead_controls_grants_everything
#print axioms Nonos.Reachability.a_duplicate_with_a_dead_copy_is_one_check
#print axioms Nonos.Reachability.the_dead_copy_does_not_tighten
#print axioms Nonos.Reachability.ratchet_is_reflexive
#print axioms Nonos.Reachability.ratchet_is_transitive
#print axioms Nonos.Reachability.a_new_dead_control_breaks_the_ratchet
#print axioms Nonos.Reachability.wiring_one_up_satisfies_the_ratchet
#print axioms Nonos.Reachability.zero_is_the_floor

-- TimerLiveness
#print axioms Nonos.TimerLiveness.one_source_is_a_single_point_of_failure
#print axioms Nonos.TimerLiveness.a_second_live_source_keeps_time
#print axioms Nonos.TimerLiveness.any_live_source_suffices
#print axioms Nonos.TimerLiveness.time_stops_only_if_all_are_dead
#print axioms Nonos.TimerLiveness.no_sources_is_no_time
#print axioms Nonos.TimerLiveness.a_derived_source_adds_nothing
#print axioms Nonos.TimerLiveness.a_derived_heartbeat_does_not_save_the_hp
#print axioms Nonos.TimerLiveness.an_independent_source_covers_the_failure
#print axioms Nonos.TimerLiveness.the_clock_is_independent_of_the_timer
#print axioms Nonos.TimerLiveness.derived_is_never_independent
#print axioms Nonos.TimerLiveness.the_heartbeat_bounds_the_stall
#print axioms Nonos.TimerLiveness.the_bound_is_under_ten_milliseconds
#print axioms Nonos.TimerLiveness.a_dead_source_bounds_nothing
#print axioms Nonos.TimerLiveness.the_heartbeat_bounds_preemption
#print axioms Nonos.TimerLiveness.more_sources_never_loosen_the_bound

-- Stark.Blinding
#print axioms Nonos.Stark.Blinding.revealing_more_than_the_coefficients_determines
#print axioms Nonos.Stark.Blinding.masking_past_the_surplus_hides
#print axioms Nonos.Stark.Blinding.masking_within_the_surplus_hides_nothing
#print axioms Nonos.Stark.Blinding.least_sufficient_mask
#print axioms Nonos.Stark.Blinding.shipped_surplus_is_one
#print axioms Nonos.Stark.Blinding.the_shipped_numbers_leave_nothing_hidden
#print axioms Nonos.Stark.Blinding.a_mask_of_one_is_not_enough
#print axioms Nonos.Stark.Blinding.a_mask_of_two_hides
#print axioms Nonos.Stark.Blinding.two_is_the_least_sufficient_mask
#print axioms Nonos.Stark.Blinding.part_masks_add
#print axioms Nonos.Stark.Blinding.the_composition_needs_its_own_margin
#print axioms Nonos.Stark.Blinding.composition_hides_iff
#print axioms Nonos.Stark.Blinding.the_relayer_learns_the_column

-- Stark.ChallengeDrawing
#print axioms Nonos.Stark.ChallengeDrawing.fixed_challenge_is_unsound
#print axioms Nonos.Stark.ChallengeDrawing.the_witness_fails_at_a_different_challenge
#print axioms Nonos.Stark.ChallengeDrawing.shipped_beta_is_unsound
#print axioms Nonos.Stark.ChallengeDrawing.constantDraw_is_constant
#print axioms Nonos.Stark.ChallengeDrawing.a_constant_draw_is_known_in_advance
#print axioms Nonos.Stark.ChallengeDrawing.a_constant_draw_admits_a_forgery
#print axioms Nonos.Stark.ChallengeDrawing.a_separating_draw_pins_one_commitment
#print axioms Nonos.Stark.ChallengeDrawing.separating_is_not_constant
#print axioms Nonos.Stark.ChallengeDrawing.changing_the_commitment_changes_the_challenge
#print axioms Nonos.Stark.ChallengeDrawing.a_constant_draw_makes_the_transcript_empty
