/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Interrupt vector classification, on the extracted code.

`Vectors.lean` is generated from `src/interrupts/idt/vectors.rs` by Charon and
Aeneas, so the definitions below are the real predicates lowered from the MIR
rustc compiles. They divide the vector space into exceptions, hardware
interrupts and a user-allocatable range, and three things about that division
are worth having written down rather than read off the constants.

The ranges leave a hole. Exceptions end at 31, hardware interrupts run 32 to 47,
and the user range starts at 48. That is sixteen hardware lines, the legacy
count. An IOAPIC has at least twenty-four, so a global interrupt above 15 gets a
vector that `is_irq` rejects and `is_user_allocatable` accepts:
`the_sixteenth_line_lands_in_the_user_range` is that, and it is the reason
`vector_to_irq` cannot invert `irq_to_vector` past 15.

The syscall vector is inside the user range. `VECTOR_SYSCALL` is 0x80, and 0x80
is between 48 and 239, so `is_user_allocatable` says the syscall entry point is
free to hand out. Nothing calls that predicate today, so this is latent rather
than live, which is exactly the state a control is in just before something
starts calling it.

And `irq_to_vector` adds 32 to a `u8` without a bound. Aeneas models the add as
checked, so the extracted function fails where the Rust would panic in a debug
build and wrap in a release one. `irq_to_vector_can_fail` is the witness. Every
classification predicate is total by contrast, so the one arithmetic step is the
only place in this module that can go wrong.
-/

import NonosExtraction.Vectors

open Aeneas Aeneas.Std Result
open nonos_vectors

set_option linter.hashCommand false

namespace NonosExtraction

/-! ### Totality -/

/-- None of the classification predicates can fail: they are comparisons, and a
    comparison has no failure mode. Stated for each so a later change that
    introduces arithmetic shows up here. -/
theorem is_exception_is_total (v : Std.U8) :
    ∃ b, interrupts.vectors.is_exception v = ok b := ⟨_, rfl⟩

theorem is_irq_is_total (v : Std.U8) :
    ∃ b, interrupts.vectors.is_irq v = ok b := by
  unfold interrupts.vectors.is_irq
  split <;> exact ⟨_, rfl⟩

theorem is_user_allocatable_is_total (v : Std.U8) :
    ∃ b, interrupts.vectors.is_user_allocatable v = ok b := by
  unfold interrupts.vectors.is_user_allocatable
  split <;> exact ⟨_, rfl⟩

theorem exception_has_error_code_is_total (v : Std.U8) :
    ∃ b, interrupts.vectors.exception_has_error_code v = ok b := by
  unfold interrupts.vectors.exception_has_error_code
  split <;> exact ⟨_, rfl⟩

theorem exception_is_fatal_is_total (v : Std.U8) :
    ∃ b, interrupts.vectors.exception_is_fatal v = ok b := by
  unfold interrupts.vectors.exception_is_fatal
  split <;> exact ⟨_, rfl⟩

/-! ### The boundaries -/

/-- The last exception vector is an exception and the first hardware vector is
    not, so the boundary is where the constants say it is. -/
theorem exception_range_ends_at_31 :
    interrupts.vectors.is_exception 31#u8 = ok true ∧
      interrupts.vectors.is_exception 32#u8 = ok false := by
  unfold interrupts.vectors.is_exception interrupts.vectors.EXCEPTION_VECTOR_END
  exact ⟨by rfl, by rfl⟩

/-- The hardware range is 32 through 47 inclusive, and nothing outside it is a
    hardware interrupt. -/
theorem irq_range_is_32_to_47 :
    interrupts.vectors.is_irq 31#u8 = ok false ∧
      interrupts.vectors.is_irq 32#u8 = ok true ∧
      interrupts.vectors.is_irq 47#u8 = ok true ∧
      interrupts.vectors.is_irq 48#u8 = ok false := by
  unfold interrupts.vectors.is_irq interrupts.vectors.IRQ_VECTOR_START
    interrupts.vectors.IRQ_VECTOR_END
  exact ⟨by rfl, by rfl, by rfl, by rfl⟩

/-- A vector is never both an exception and a hardware interrupt, at the
    boundary where a one-off error would put it. -/
theorem exception_and_irq_are_disjoint_at_the_boundary :
    (interrupts.vectors.is_exception 31#u8 = ok true ∧
      interrupts.vectors.is_irq 31#u8 = ok false) ∧
    (interrupts.vectors.is_exception 32#u8 = ok false ∧
      interrupts.vectors.is_irq 32#u8 = ok true) := by
  unfold interrupts.vectors.is_exception interrupts.vectors.is_irq
    interrupts.vectors.EXCEPTION_VECTOR_END interrupts.vectors.IRQ_VECTOR_START
    interrupts.vectors.IRQ_VECTOR_END
  exact ⟨⟨by rfl, by rfl⟩, ⟨by rfl, by rfl⟩⟩

/-- And never both a hardware interrupt and user-allocatable. -/
theorem irq_and_user_are_disjoint_at_the_boundary :
    (interrupts.vectors.is_irq 47#u8 = ok true ∧
      interrupts.vectors.is_user_allocatable 47#u8 = ok false) ∧
    (interrupts.vectors.is_irq 48#u8 = ok false ∧
      interrupts.vectors.is_user_allocatable 48#u8 = ok true) := by
  unfold interrupts.vectors.is_irq interrupts.vectors.is_user_allocatable
    interrupts.vectors.IRQ_VECTOR_START interrupts.vectors.IRQ_VECTOR_END
    interrupts.vectors.USER_VECTOR_START interrupts.vectors.USER_VECTOR_END
  exact ⟨⟨by rfl, by rfl⟩, ⟨by rfl, by rfl⟩⟩

/-! ### The hole at sixteen lines -/

/-- Fifteen is the last interrupt the mapping handles: it lands on 47, the last
    vector `is_irq` accepts, and inverts. -/
theorem line_fifteen_round_trips :
    interrupts.vectors.irq_to_vector 15#u8 = ok 47#u8 ∧
      interrupts.vectors.vector_to_irq 47#u8 = ok (some 15#u8) := by
  unfold interrupts.vectors.irq_to_vector interrupts.vectors.vector_to_irq
    interrupts.vectors.is_irq interrupts.vectors.IRQ_VECTOR_START
    interrupts.vectors.IRQ_VECTOR_END
  exact ⟨by rfl, by rfl⟩

/-- Sixteen does not. The vector is computed, it is not a hardware vector, and it
    is in the range a user allocator would treat as free.

    An IOAPIC publishes at least twenty-four lines, so this is not a hypothetical
    index; it is the seventeenth pin of the first controller. -/
theorem the_sixteenth_line_lands_in_the_user_range :
    interrupts.vectors.irq_to_vector 16#u8 = ok 48#u8 ∧
      interrupts.vectors.is_irq 48#u8 = ok false ∧
      interrupts.vectors.is_user_allocatable 48#u8 = ok true ∧
      interrupts.vectors.vector_to_irq 48#u8 = ok none := by
  unfold interrupts.vectors.irq_to_vector interrupts.vectors.vector_to_irq
    interrupts.vectors.is_irq interrupts.vectors.is_user_allocatable
    interrupts.vectors.IRQ_VECTOR_START interrupts.vectors.IRQ_VECTOR_END
    interrupts.vectors.USER_VECTOR_START interrupts.vectors.USER_VECTOR_END
  exact ⟨by rfl, by rfl, by rfl, by rfl⟩

/-- So the mapping is not invertible past fifteen: a vector the kernel computed
    from a line number cannot be turned back into that line number. A dispatcher
    that recovers the line from the vector loses the identity of every line above
    fifteen. -/
theorem the_mapping_is_not_invertible_past_fifteen :
    interrupts.vectors.irq_to_vector 16#u8 = ok 48#u8 ∧
      interrupts.vectors.vector_to_irq 48#u8 ≠ ok (some 16#u8) := by
  refine ⟨?_, ?_⟩
  · unfold interrupts.vectors.irq_to_vector interrupts.vectors.IRQ_VECTOR_START
    rfl
  · unfold interrupts.vectors.vector_to_irq interrupts.vectors.is_irq
      interrupts.vectors.IRQ_VECTOR_START interrupts.vectors.IRQ_VECTOR_END
    simp

/-! ### The syscall vector -/

/-- `VECTOR_SYSCALL` is 0x80, which the user-allocatable predicate accepts.

    An allocator that trusted this predicate could hand out the syscall entry
    point. Nothing calls it today, which makes this latent: the predicate is
    wrong and no caller has found out yet. -/
theorem the_syscall_vector_is_user_allocatable :
    interrupts.vectors.is_user_allocatable 128#u8 = ok true := by
  unfold interrupts.vectors.is_user_allocatable
    interrupts.vectors.USER_VECTOR_START interrupts.vectors.USER_VECTOR_END
  rfl

/-- It is not an exception and not a hardware interrupt either, so no other
    predicate in the module excludes it. -/
theorem nothing_else_excludes_the_syscall_vector :
    interrupts.vectors.is_exception 128#u8 = ok false ∧
      interrupts.vectors.is_irq 128#u8 = ok false := by
  unfold interrupts.vectors.is_exception interrupts.vectors.is_irq
    interrupts.vectors.EXCEPTION_VECTOR_END interrupts.vectors.IRQ_VECTOR_START
    interrupts.vectors.IRQ_VECTOR_END
  exact ⟨by rfl, by rfl⟩

/-! ### The unchecked add -/

/-- `irq_to_vector` adds 32 to a byte and returns a byte. At 224 that does not
    fit, and the extracted function has no result at all: in Rust the same
    expression panics in a debug build and wraps in a release one, and the kernel
    is not allowed to do either. -/
theorem irq_to_vector_can_fail :
    ∀ r, interrupts.vectors.irq_to_vector 224#u8 ≠ ok r := by
  intro r h
  unfold interrupts.vectors.irq_to_vector interrupts.vectors.IRQ_VECTOR_START at h
  have heq := Std.UScalar.add_equiv 32#u8 224#u8
  rw [h] at heq
  simp only at heq
  obtain ⟨hlt, _⟩ := heq
  simp at hlt

/-- It succeeds below that, so the failure is a bound rather than a break. -/
theorem irq_to_vector_succeeds_below_224 :
    interrupts.vectors.irq_to_vector 223#u8 = ok 255#u8 := by
  unfold interrupts.vectors.irq_to_vector interrupts.vectors.IRQ_VECTOR_START
  rfl

/-- The inverse is total at the boundaries of its own condition: below the base,
    at both ends of the hardware range, just above it, and at the top of the byte.

    It subtracts only on the branch `is_irq` accepted, so the subtraction has
    nothing to underflow. That is shown here at the values where an off-by-one
    would put it rather than for every byte: the general statement needs the
    scalar solver to see through the checked subtraction, and the two directions
    are asymmetric either way, which is the point worth recording. -/
theorem vector_to_irq_is_total_at_the_boundaries :
    (∃ r, interrupts.vectors.vector_to_irq 0#u8 = ok r) ∧
      (∃ r, interrupts.vectors.vector_to_irq 31#u8 = ok r) ∧
      (∃ r, interrupts.vectors.vector_to_irq 32#u8 = ok r) ∧
      (∃ r, interrupts.vectors.vector_to_irq 47#u8 = ok r) ∧
      (∃ r, interrupts.vectors.vector_to_irq 48#u8 = ok r) ∧
      (∃ r, interrupts.vectors.vector_to_irq 255#u8 = ok r) := by
  unfold interrupts.vectors.vector_to_irq interrupts.vectors.is_irq
    interrupts.vectors.IRQ_VECTOR_START interrupts.vectors.IRQ_VECTOR_END
  exact ⟨⟨_, rfl⟩, ⟨_, rfl⟩, ⟨_, rfl⟩, ⟨_, rfl⟩, ⟨_, rfl⟩, ⟨_, rfl⟩⟩

/-! ### The exception tables -/

/-- Double fault both carries an error code and is fatal, which is the pair of
    facts the entry stub needs to get its frame layout and its handling right. -/
theorem double_fault_is_classified :
    interrupts.vectors.exception_has_error_code 8#u8 = ok true ∧
      interrupts.vectors.exception_is_fatal 8#u8 = ok true := ⟨rfl, rfl⟩

/-- A page fault carries an error code and is not fatal by itself, which is what
    makes demand paging possible. -/
theorem page_fault_is_recoverable :
    interrupts.vectors.exception_has_error_code 14#u8 = ok true ∧
      interrupts.vectors.exception_is_fatal 14#u8 = ok false := ⟨rfl, rfl⟩

/-- A breakpoint carries no error code, so a stub that popped one would corrupt
    its own return frame. -/
theorem breakpoint_has_no_error_code :
    interrupts.vectors.exception_has_error_code 3#u8 = ok false := rfl

/-- And nothing outside the exception range carries an error code, so the table
    does not claim a frame layout for a hardware interrupt. -/
theorem hardware_vectors_carry_no_error_code :
    interrupts.vectors.exception_has_error_code 32#u8 = ok false ∧
      interrupts.vectors.exception_has_error_code 48#u8 = ok false ∧
      interrupts.vectors.exception_has_error_code 128#u8 = ok false :=
  ⟨rfl, rfl, rfl⟩

/-! ### Axiom profile -/

#print axioms NonosExtraction.is_exception_is_total
#print axioms NonosExtraction.is_irq_is_total
#print axioms NonosExtraction.is_user_allocatable_is_total
#print axioms NonosExtraction.exception_range_ends_at_31
#print axioms NonosExtraction.irq_range_is_32_to_47
#print axioms NonosExtraction.exception_and_irq_are_disjoint_at_the_boundary
#print axioms NonosExtraction.irq_and_user_are_disjoint_at_the_boundary
#print axioms NonosExtraction.line_fifteen_round_trips
#print axioms NonosExtraction.the_sixteenth_line_lands_in_the_user_range
#print axioms NonosExtraction.the_mapping_is_not_invertible_past_fifteen
#print axioms NonosExtraction.the_syscall_vector_is_user_allocatable
#print axioms NonosExtraction.irq_to_vector_can_fail
#print axioms NonosExtraction.vector_to_irq_is_total_at_the_boundaries
#print axioms NonosExtraction.double_fault_is_classified
#print axioms NonosExtraction.page_fault_is_recoverable

end NonosExtraction
