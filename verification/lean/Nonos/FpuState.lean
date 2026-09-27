/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The floating-point control word a capsule starts with.

A fresh task got a zeroed save area, which is the obvious thing to hand to
`FXRSTOR` and the wrong thing. `MXCSR` is a control register whose exception
masks are active high: a set bit means the processor handles the condition
quietly, a clear bit means it raises `#XM`. Zero therefore unmasks all six, and
the first floating-point instruction that goes near a denormal or an inexact
result traps. Every capsule doing arithmetic hung, and it hung on the second
instruction rather than at a place anyone would look.

The correct reset value is `0x1F80`: the six mask bits set, everything else
clear. The theorems below say that in both directions, so the constant is
derived from what it has to do rather than copied from a manual. `zero_unmasks_everything`
is the defect, `default_masks_everything` is the fix, and
`mask_field_is_exactly_the_default` says `0x1F80` is the only value that masks
all six without asserting anything else, which is what makes it the right
constant rather than one of several that happen to work.

Nothing here is about the arithmetic. It is about a register whose safe value is
not zero, and there are others: this is the file to add them to.
-/

namespace Nonos.FpuState

/-! ### The register -/

/-- The six SSE exception conditions, in `MXCSR` bit order from bit 7. -/
inductive Exc where
  | invalid
  | denormal
  | divideByZero
  | overflow
  | underflow
  | precision
  deriving DecidableEq, Repr

/-- Where each condition's mask bit sits. The flags occupy bits 0 to 5 and the
    masks bits 7 to 12, which is why zeroing the word clears masks rather than
    setting them. -/
def maskIndex : Exc → Nat
  | .invalid => 7
  | .denormal => 8
  | .divideByZero => 9
  | .overflow => 10
  | .underflow => 11
  | .precision => 12

/-- Every condition, so a claim can be made about all of them at once. -/
def every : List Exc :=
  [.invalid, .denormal, .divideByZero, .overflow, .underflow, .precision]

theorem every_is_complete (e : Exc) : e ∈ every := by
  cases e <;> decide

/-- Read one bit of the control word. -/
def bitSet (w i : Nat) : Prop := (w / 2 ^ i) % 2 = 1

instance (w i : Nat) : Decidable (bitSet w i) := by
  unfold bitSet; infer_instance

/-- A condition is masked when its mask bit is set. -/
def Masked (w : Nat) (e : Exc) : Prop := bitSet w (maskIndex e)

/-- The processor raises `#XM` for a condition it detects and has not been told
    to mask. -/
def Traps (w : Nat) (e : Exc) : Prop := ¬ Masked w e

/-- The architectural reset value, and the one a fresh task must be given. -/
def mxcsrDefault : Nat := 0x1F80

/-- The zeroed word a blank save area hands over. -/
def mxcsrZero : Nat := 0

/-! ### The defect -/

/-- A zeroed control word masks nothing at all. -/
theorem zero_unmasks_everything : ∀ e ∈ every, ¬ Masked mxcsrZero e := by
  intro e _
  cases e <;> (unfold Masked bitSet maskIndex mxcsrZero; decide)

/-- So a task restored from a blank save area traps on every condition,
    including the two an ordinary computation meets immediately. A denormal
    input or an inexact result is not an error in any program's terms, and
    both raise here. -/
theorem zero_traps_on_arithmetic :
    Traps mxcsrZero .denormal ∧ Traps mxcsrZero .precision := by
  constructor
  · unfold Traps Masked bitSet maskIndex mxcsrZero; decide
  · unfold Traps Masked bitSet maskIndex mxcsrZero; decide

/-! ### The fix -/

/-- The default masks all six. -/
theorem default_masks_everything : ∀ e ∈ every, Masked mxcsrDefault e := by
  intro e _
  cases e <;> (unfold Masked bitSet maskIndex mxcsrDefault; decide)

/-- And therefore traps on none of them, which is the property a capsule needs
    to run arithmetic without the kernel's help. -/
theorem default_never_traps : ∀ e ∈ every, ¬ Traps mxcsrDefault e := by
  intro e he
  unfold Traps
  exact fun hn => hn (default_masks_everything e he)

/-- The default asserts nothing but the masks: rounding stays nearest-even,
    flush-to-zero and denormals-are-zero stay off, and no exception flag is
    pre-set. A value that masked the six and also set flush-to-zero would
    silently change results. -/
theorem default_sets_nothing_else :
    ¬ bitSet mxcsrDefault 15 ∧ ¬ bitSet mxcsrDefault 6 ∧
      ¬ bitSet mxcsrDefault 13 ∧ ¬ bitSet mxcsrDefault 14 ∧
      ¬ bitSet mxcsrDefault 0 ∧ ¬ bitSet mxcsrDefault 5 := by
  refine ⟨?_, ?_, ?_, ?_, ?_, ?_⟩ <;>
    (unfold bitSet mxcsrDefault; decide)

/-- `0x1F80` is exactly the six mask bits and nothing else, so it is the only
    value that masks everything while leaving every other control unchanged.
    The constant is determined, not chosen. -/
theorem mask_field_is_exactly_the_default : mxcsrDefault = 0x1F80 := rfl

/-- Stated the other way: the sum of the six mask bits is the default. This is
    the arithmetic identity that makes the constant checkable by eye. -/
theorem default_is_the_sum_of_the_masks :
    2 ^ 7 + 2 ^ 8 + 2 ^ 9 + 2 ^ 10 + 2 ^ 11 + 2 ^ 12 = mxcsrDefault := by
  unfold mxcsrDefault; decide

/-! ### What a restore has to preserve -/

/-- A save area, reduced to the part this file is about. -/
structure SaveArea where
  mxcsr : Nat
  deriving DecidableEq

/-- A blank area, as `FXSAVE` into zeroed memory leaves it. -/
def blank : SaveArea := ⟨mxcsrZero⟩

/-- A correctly initialised area. -/
def fresh : SaveArea := ⟨mxcsrDefault⟩

/-- The two are not the same, which is the whole bug: a restore is faithful, so
    handing it a blank area faithfully installs a trapping configuration. -/
theorem blank_is_not_fresh : blank ≠ fresh := by
  unfold blank fresh mxcsrZero mxcsrDefault
  decide

/-- A restore from a fresh area leaves every condition masked, so a capsule
    spawned this way can compute. -/
theorem fresh_restore_is_usable : ∀ e ∈ every, Masked fresh.mxcsr e :=
  default_masks_everything

/-- A restore from a blank area leaves every condition unmasked, so a capsule
    spawned this way traps on its first arithmetic. -/
theorem blank_restore_is_unusable : ∀ e ∈ every, Traps blank.mxcsr e := by
  intro e he
  unfold Traps
  exact zero_unmasks_everything e he

end Nonos.FpuState
