/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Two things the trap entry path has to get exactly right.

The interrupt stack table is indexed from one in the descriptor and from zero
everywhere else. A gate descriptor's IST field holds `slot + 1`, and zero in that
field does not mean "the first stack", it means "no stack switch at all". So the
one arithmetic error available here has two different wrong answers: writing
`slot` for a non-zero slot selects the stack below the intended one, and writing
`slot` for slot zero turns the switch off entirely. The handler that most needs
its own stack is double fault, and double fault with no stack switch runs on the
stack that just faulted, which is a triple fault and a reset with no diagnostic.

`SWAPGS` is the other one. It exchanges the GS base with a shadow register, so it
is its own inverse, and the entry path has to run it exactly once on the way in
and once on the way out. Running it an odd number of times leaves the wrong base
installed: on the way out that hands a user thread the kernel's per-processor
pointer, and on the next entry the swap puts the user's value where the kernel
expects its own. It also must not run at all for a trap that arrived from ring
zero, because the base is already the kernel's, and swapping then installs the
user value while running kernel code.

The theorems are about the encoding and the parity. They do not need a model of
the fault at all: an odd number of swaps is wrong whatever the handler does, and
an off-by-one in the descriptor is wrong before the handler is reached.
-/

namespace Nonos.TrapEntry

/-! ### The interrupt stack table -/

/-- How many stacks the table has. -/
def istSlots : Nat := 7

/-- What the descriptor's IST field holds for a handler that wants a stack
    switch to `slot`, and what it holds for one that does not. -/
inductive IstField where
  | none
  | slot (n : Nat)
  deriving DecidableEq, Repr

/-- The field as the processor reads it: zero means no switch. -/
def encode : IstField → Nat
  | .none => 0
  | .slot n => n + 1

/-- And back, which is where the off-by-one shows up as an asymmetry rather
    than as a comment. -/
def decode (f : Nat) : IstField :=
  if f = 0 then .none else .slot (f - 1)

/-- Zero means no stack switch, not the first stack. -/
theorem zero_means_no_switch : decode 0 = .none := by
  unfold decode; simp

/-- The encoding round-trips, so a descriptor built from a slot reads back as
    that slot. -/
theorem encode_decode (f : IstField) : decode (encode f) = f := by
  cases f with
  | none => unfold encode decode; simp
  | slot n =>
    unfold encode decode
    simp

/-- Distinct intentions produce distinct descriptors: no slot encodes to the
    same field as another, and none encodes to the no-switch value. -/
theorem encode_injective (a b : IstField) (h : encode a = encode b) : a = b := by
  have ha := encode_decode a
  have hb := encode_decode b
  rw [h] at ha
  rw [← ha]
  exact hb

/-- A slot is never encoded as zero, which is the statement that a handler asking
    for a stack always gets one. -/
theorem slot_is_never_zero (n : Nat) : encode (.slot n) ≠ 0 := by
  show n + 1 ≠ 0
  omega

/-- Every real slot encodes into the field's range. -/
theorem encoded_slots_are_in_range (n : Nat) (h : n < istSlots) :
    1 ≤ encode (.slot n) ∧ encode (.slot n) ≤ istSlots := by
  show 1 ≤ n + 1 ∧ n + 1 ≤ istSlots
  unfold istSlots at h ⊢
  omega

/-- The off-by-one, written as the function it is: the slot number placed in the
    field directly, without the increment. -/
def encodeWrong : IstField → Nat
  | .none => 0
  | .slot n => n

/-- For any slot but the first, the wrong encoding selects the stack below the
    intended one. Two handlers that asked for separate stacks now share one, and
    a fault inside one of them overwrites the other's frame. -/
theorem off_by_one_selects_the_stack_below (n : Nat) (h : 0 < n) :
    decode (encodeWrong (.slot n)) = .slot (n - 1) := by
  unfold encodeWrong decode
  have : ¬ (n = 0) := by omega
  simp [this]

/-- And for the first slot it selects no stack at all.

    This is the case that matters. Slot zero is where double fault goes, and a
    double-fault gate with no IST switch runs the handler on the stack whose
    fault brought it there. -/
theorem off_by_one_disables_the_first_slot :
    decode (encodeWrong (.slot 0)) = .none := by
  unfold encodeWrong decode; simp

/-- So the wrong encoding is not merely shifted, it is not injective: slot zero
    and no-switch become the same descriptor. -/
theorem wrong_encoding_is_not_injective :
    encodeWrong (.slot 0) = encodeWrong .none ∧ (IstField.slot 0) ≠ .none := by
  constructor
  · unfold encodeWrong; rfl
  · intro h; exact IstField.noConfusion h

/-! ### SWAPGS -/

/-- Which GS base is installed. -/
inductive GsBase where
  | user
  | kernel
  deriving DecidableEq, Repr

/-- `SWAPGS` exchanges the active base with the shadow. -/
def swap : GsBase → GsBase
  | .user => .kernel
  | .kernel => .user

/-- It is its own inverse, which is why the parity of the count is the whole
    correctness condition. -/
theorem swap_involutive (g : GsBase) : swap (swap g) = g := by
  cases g <;> rfl

/-- Apply the instruction `n` times. -/
def swapN : Nat → GsBase → GsBase
  | 0, g => g
  | n + 1, g => swap (swapN n g)

/-- An even number of swaps is the identity. -/
theorem even_swaps_are_identity (k : Nat) (g : GsBase) : swapN (2 * k) g = g := by
  induction k with
  | zero => rfl
  | succ n ih =>
    have h : 2 * (n + 1) = (2 * n) + 1 + 1 := by omega
    rw [h]
    show swap (swap (swapN (2 * n) g)) = g
    rw [swap_involutive, ih]

/-- An odd number of swaps is a swap. -/
theorem odd_swaps_are_a_swap (k : Nat) (g : GsBase) :
    swapN (2 * k + 1) g = swap g := by
  show swap (swapN (2 * k) g) = swap g
  rw [even_swaps_are_identity]

/-- The paired path: swap in, swap out, the thread resumes with its own base. -/
theorem paired_path_restores_the_user_base :
    swapN 2 GsBase.user = GsBase.user := by
  have : (2 : Nat) = 2 * 1 := by omega
  rw [this]
  exact even_swaps_are_identity 1 GsBase.user

/-- The unpaired path hands a user thread the kernel's per-processor base. The
    thread can read it, and the next entry's swap then installs the user's value
    where the kernel expects its own. -/
theorem unpaired_path_leaks_the_kernel_base :
    swapN 1 GsBase.user = GsBase.kernel := rfl

/-- Swapping on a trap that arrived from ring zero installs the user base while
    kernel code is running, so the conditional is not an optimisation. -/
theorem unconditional_swap_breaks_ring_zero :
    swap GsBase.kernel = GsBase.user := rfl

/-- The condition, as the entry path evaluates it: swap only when the saved code
    selector says the trap came from user mode. -/
def entrySwap (fromUser : Bool) (g : GsBase) : GsBase :=
  if fromUser then swap g else g

/-- Whichever ring the trap came from, the handler runs on the kernel base. That
    is the property the conditional exists for, and it holds in both cases
    rather than in the common one. -/
theorem handler_always_runs_on_the_kernel_base (fromUser : Bool) (g : GsBase)
    (h : g = if fromUser then .user else .kernel) :
    entrySwap fromUser g = .kernel := by
  unfold entrySwap
  cases fromUser with
  | true => rw [h]; rfl
  | false => rw [h]; rfl

/-- And the return undoes exactly what the entry did, for both rings, because
    the return applies the same condition. -/
theorem return_restores_what_entry_changed (fromUser : Bool) (g : GsBase) :
    entrySwap fromUser (entrySwap fromUser g) = g := by
  unfold entrySwap
  cases fromUser with
  | true => exact swap_involutive g
  | false => rfl

end Nonos.TrapEntry
