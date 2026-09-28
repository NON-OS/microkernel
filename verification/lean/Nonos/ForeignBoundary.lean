/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The one check every peer call makes before it touches a guest.

A Linux personality runs in userspace and supervises guests that live in their
own address spaces. Every call it makes to read, write or map a guest's memory
goes through the peer interface, and that interface is the whole boundary: the
personality is not trusted with a guest's address space, it is authorised for
one particular guest and bounded to one particular half of it.

Three separate things have to hold and each has its own failure mode.

The pid has to survive the trip. A syscall carries it as a 64-bit word and the
registry keys on 32 bits. Truncating means a number that names nothing becomes a
number that names something, and the ownership test below then passes, because
the caller really does supervise the guest the truncated value landed on.
`truncation_renames_the_target` is that, and `pid_arg` refuses instead.

The caller has to own the target. `supervised_asid` resolves an address space
only for a guest whose supervisor is the caller, so a personality cannot reach
sideways into a guest that belongs to another one.

And the span has to be checked without wrapping. `in_user_half` adds an address
to a length and compares against the kernel boundary, which on a 64-bit machine
is a sum that can wrap. `wrapping_sum_looks_contained` exhibits a range that
passes a wrapped comparison and is not contained, which is why the real code
uses `checked_add` and refuses on overflow rather than comparing first.

The chunking at the end is not a security property by itself. It is why the
bound check holds for a whole guest image: the image crosses in pieces of at
most `MAX_SPAN`, every piece is inside the half if the whole range was, and the
pieces end exactly at the end of the range rather than somewhere past it.
-/

namespace Nonos.ForeignBoundary

/-! ### The machine's numbers -/

/-- Two to the sixty-fourth, written out: the modulus a `u64` sum wraps at. -/
def wordSize : Nat := 18446744073709551616

/-- The first address of the kernel half. -/
def userVaEnd : Nat := 0x800000000000

/-- One call maps or copies at most this much, so a guest image crosses in
    bounded pieces and no single call holds the processor. -/
def maxSpan : Nat := 0x100000

def page : Nat := 4096

/-- The registry's pid width. -/
def pidSpace : Nat := 4294967296

/-! ### The span check -/

/-- `[addr, addr + len)` lies wholly in the guest's own half, computed in
    unbounded arithmetic: the sum does not overflow a machine word and the end
    is at or below the boundary. This is `in_user_half` with `checked_add`
    read as the overflow premise it is. -/
def InUserHalf (addr len : Nat) : Prop :=
  addr + len < wordSize ∧ addr + len ≤ userVaEnd

instance (addr len : Nat) : Decidable (InUserHalf addr len) := by
  unfold InUserHalf; infer_instance

/-- Every byte of an accepted span is below the kernel boundary. -/
theorem accepted_bytes_are_user_bytes (addr len i : Nat)
    (h : InUserHalf addr len) (hi : i < len) : addr + i < userVaEnd := by
  obtain ⟨_, hend⟩ := h
  omega

/-- An empty span at the boundary itself is accepted, and an empty span is the
    only thing accepted there: the boundary address is the first kernel byte,
    so a single byte at it is refused. -/
theorem boundary_is_exclusive : InUserHalf userVaEnd 0 ∧ ¬ InUserHalf userVaEnd 1 := by
  constructor
  · unfold InUserHalf userVaEnd wordSize; omega
  · intro ⟨_, h⟩; unfold userVaEnd at h; omega

/-- The sum as a machine word computes it. -/
def wrapAdd (a b : Nat) : Nat := (a + b) % wordSize

/-- A wrapped sum agrees with the real one exactly when there was no overflow,
    so the overflow premise is the whole difference between the two checks. -/
theorem wrapAdd_faithful_below_the_word (a b : Nat) (h : a + b < wordSize) :
    wrapAdd a b = a + b := by
  unfold wrapAdd wordSize
  unfold wordSize at h
  omega

/-- A span that wraps can pass a comparison made on the wrapped sum while
    reaching far outside the user half.

    The witness is one page below the top of the address space with two pages
    of length. The wrapped end is 4096, comfortably inside the user half, and
    the real range covers the top of kernel memory. A check that compares
    before it tests for overflow accepts it. -/
theorem wrapping_sum_looks_contained :
    ∃ a l, wordSize ≤ a + l ∧ wrapAdd a l ≤ userVaEnd ∧ ¬ InUserHalf a l := by
  refine ⟨18446744073709547520, 8192, ?_, ?_, ?_⟩
  · unfold wordSize; omega
  · unfold wrapAdd wordSize userVaEnd; omega
  · intro ⟨h, _⟩; unfold wordSize at h; omega

/-- And the check as written refuses every wrapping span, for any address and
    length at all, because the overflow premise is a conjunct rather than a
    consequence. -/
theorem wrapping_span_is_refused (a l : Nat) (h : wordSize ≤ a + l) :
    ¬ InUserHalf a l := by
  intro ⟨hw, _⟩
  omega

/-! ### The pid -/

/-- `pid_arg`: a pid the registry can key on, or nothing. -/
def pidArg (pid : Nat) : Option Nat :=
  if pid < pidSpace then some pid else none

/-- What truncating would have done instead. -/
def pidTruncate (pid : Nat) : Nat := pid % pidSpace

/-- An accepted pid is unchanged, so the refusal costs nothing on the path that
    matters. -/
theorem pidArg_is_exact (pid : Nat) (h : pid < pidSpace) : pidArg pid = some pid := by
  unfold pidArg; simp [h]

/-- Truncation turns a pid that names nothing into one that names something
    else. For every target a caller does supervise there is an out-of-range
    request that truncates onto it. -/
theorem truncation_renames_the_target (target : Nat) (h : target < pidSpace) :
    ∃ pid, pidArg pid = none ∧ pidTruncate pid = target := by
  refine ⟨pidSpace + target, ?_, ?_⟩
  · unfold pidArg
    have : ¬ (pidSpace + target < pidSpace) := by omega
    simp [this]
  · unfold pidTruncate pidSpace
    unfold pidSpace at h
    omega

/-! ### Ownership -/

/-- The part of the registry a peer call reads: who supervises a guest, and
    which address space it runs in. -/
structure Registry where
  supervisorOf : Nat → Option Nat
  asidOf : Nat → Option Nat

/-- `supervised_asid`: the guest's address space, but only for a caller that
    supervises it and a pid that fits. -/
def supervised (r : Registry) (caller pid : Nat) : Option Nat :=
  match pidArg pid with
  | none => none
  | some p => if r.supervisorOf p = some caller then r.asidOf p else none

/-- A peer call never resolves an address space it is not the supervisor of.
    Every read, write and map behind this interface inherits the property,
    because this is the only function that produces the address space they
    act on. -/
theorem never_reaches_an_unsupervised_guest (r : Registry) (caller pid asid : Nat)
    (h : supervised r caller pid = some asid) :
    ∃ p, pidArg pid = some p ∧ r.supervisorOf p = some caller := by
  unfold supervised at h
  cases hp : pidArg pid with
  | none => rw [hp] at h; exact absurd h (by simp)
  | some p =>
    rw [hp] at h
    by_cases hs : r.supervisorOf p = some caller
    · exact ⟨p, rfl, hs⟩
    · simp [hs] at h

/-- An out-of-range pid resolves nothing, whatever the registry says about the
    number it would have truncated to. The two guards compose in the right
    order: the pid is refused before it is ever used as a key. -/
theorem out_of_range_pid_resolves_nothing (r : Registry) (caller pid : Nat)
    (h : ¬ pid < pidSpace) : supervised r caller pid = none := by
  unfold supervised pidArg
  simp [h]

/-- A caller's own guest resolves, so the check is a gate and not a wall. -/
theorem own_guest_resolves (r : Registry) (caller pid asid : Nat)
    (hp : pid < pidSpace) (hs : r.supervisorOf pid = some caller)
    (ha : r.asidOf pid = some asid) : supervised r caller pid = some asid := by
  unfold supervised
  rw [pidArg_is_exact pid hp]
  simp [hs, ha]

/-! ### Crossing a whole image in bounded pieces -/

/-- How many calls a range of this length takes. -/
def chunkCount (len : Nat) : Nat := (len + maxSpan - 1) / maxSpan

/-- Where the `i`th call starts, relative to the range. -/
def chunkOffset (i : Nat) : Nat := i * maxSpan

/-- How much the `i`th call moves. -/
def chunkSize (len i : Nat) : Nat := min maxSpan (len - chunkOffset i)

/-- No call moves more than the limit, so no call holds the processor for an
    unbounded time and no call's own bound check is doing arithmetic on a
    guest-supplied length. -/
theorem chunk_is_bounded (len i : Nat) : chunkSize len i ≤ maxSpan := by
  unfold chunkSize
  omega

/-- Every piece lies inside the range it came from. -/
theorem chunk_stays_inside (len i : Nat) (h : i < chunkCount len) :
    chunkOffset i + chunkSize len i ≤ len := by
  simp only [chunkCount, chunkOffset, chunkSize, maxSpan] at h ⊢
  omega

/-- A piece of a contained range is contained. Each call's own `in_user_half`
    therefore passes, and the bound established once for the image holds for
    every call that carries part of it. -/
theorem chunk_of_a_contained_range_is_contained (addr len i : Nat)
    (hc : InUserHalf addr len) (h : i < chunkCount len) :
    InUserHalf (addr + chunkOffset i) (chunkSize len i) := by
  obtain ⟨hw, he⟩ := hc
  have hin := chunk_stays_inside len i h
  exact ⟨by omega, by omega⟩

/-- A non-empty range takes at least one call, so nothing is silently skipped. -/
theorem nonempty_takes_a_call (len : Nat) (h : 0 < len) : 0 < chunkCount len := by
  unfold chunkCount maxSpan
  omega

/-- An empty range takes none. -/
theorem empty_takes_no_calls : chunkCount 0 = 0 := by
  unfold chunkCount maxSpan
  omega

/-- The pieces end exactly at the end of the range: the last call stops at
    `len` rather than at a multiple of the span. A copy that ran to the rounded
    length would write past a guest's buffer by up to a megabyte. -/
theorem last_chunk_ends_at_len (len : Nat) (h : 0 < len) :
    chunkOffset (chunkCount len - 1) + chunkSize len (chunkCount len - 1) = len := by
  simp only [chunkCount, chunkOffset, chunkSize, maxSpan]
  omega

end Nonos.ForeignBoundary
