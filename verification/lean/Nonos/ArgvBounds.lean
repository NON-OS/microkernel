/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The argument vector, and the stack it has to fit on.

`execve` copies the caller's arguments onto the new process's stack before the
program starts, so every bound on the arguments is really a bound on that stack.
Two of them were wrong in opposite directions.

Each argument was read under the path limit, two hundred and fifty-five bytes. A
path is at most that; an argument is not a path, and a long command line is
ordinary rather than exotic. So legal invocations were refused, and the refusal
came back as a failure to execute rather than as a length complaint.
`the_path_bound_refuses_a_legal_argument` is that, at the first length it
happens.

And the total was chosen separately from the stack size. The arguments, their
pointers and the terminators all land on the new stack, so the total and the frame
overhead together have to fit under it. `sound_total_fits_the_stack` is the
condition and `the_total_is_coupled_to_the_stack` is the part that matters for
maintenance: raising one without the other is a change from a refusal to an
overrun, and nothing about the two constants says they are related.

The count bound at the end is free and worth having: an argument costs at least
its terminator, so a total in bytes is also a bound on how many arguments there
can be, and the pointer array cannot be made to grow without the bytes growing
too.
-/

namespace Nonos.ArgvBounds

/-! ### The constants -/

/-- The longest path the kernel will resolve. -/
def maxPath : Nat := 255

/-- The longest single argument, which is not a path. -/
def maxArg : Nat := 4096

/-- The stack a new process gets, from the guest layout. -/
def stackSize : Nat := 0x100000

/-- What the entry frame needs below the arguments: the pointer arrays, the
    environment, the auxiliary vector and the initial alignment. -/
def frameOverhead : Nat := 4096

/-- The total the loader will copy. -/
def maxTotal : Nat := 0x20000

/-- Every argument carries a terminator, so the smallest possible argument still
    costs a byte. -/
def minArgCost : Nat := 1

/-! ### The per-argument bound -/

/-- What the loader accepted when it read arguments under the path limit. -/
def acceptsUnderPathBound (len : Nat) : Bool := len ≤ maxPath

/-- What it accepts now. -/
def acceptsArg (len : Nat) : Bool := len ≤ maxArg

/-- A path-length argument is accepted either way, which is why nothing showed up
    until someone ran a real command line. -/
theorem short_arguments_are_unaffected (len : Nat) (h : len ≤ maxPath) :
    acceptsUnderPathBound len = true ∧ acceptsArg len = true := by
  unfold acceptsUnderPathBound acceptsArg maxPath maxArg
  unfold maxPath at h
  simp only [decide_eq_true_eq]
  omega

/-- An argument one byte past a path is legal and was refused. -/
theorem the_path_bound_refuses_a_legal_argument :
    acceptsArg 256 = true ∧ acceptsUnderPathBound 256 = false := by
  constructor
  · unfold acceptsArg maxArg; simp
  · unfold acceptsUnderPathBound maxPath; simp

/-- The two bounds differ on exactly the arguments between them, so the defect's
    reach is the interval rather than the tail. -/
theorem the_bounds_differ_on_an_interval (len : Nat)
    (h1 : maxPath < len) (h2 : len ≤ maxArg) :
    acceptsArg len = true ∧ acceptsUnderPathBound len = false := by
  unfold acceptsArg acceptsUnderPathBound
  unfold maxPath at h1
  unfold maxArg at h2
  constructor
  · unfold maxArg; simp; omega
  · unfold maxPath; simp; omega

/-- And an argument past the real limit is still refused, so widening the bound is
    not removing it. -/
theorem oversized_arguments_are_still_refused (len : Nat) (h : maxArg < len) :
    acceptsArg len = false := by
  unfold acceptsArg
  unfold maxArg at h
  unfold maxArg
  simp
  omega

/-! ### The total, and the stack -/

/-- The total is sound when everything the loader will copy, plus the frame it
    copies it into, fits on the stack. -/
def SoundTotal (total overhead stack : Nat) : Prop := total + overhead ≤ stack

instance (total overhead stack : Nat) : Decidable (SoundTotal total overhead stack) := by
  unfold SoundTotal; infer_instance

/-- The shipped numbers are sound: a hundred and twenty-eight kilobytes of
    arguments and a page of frame inside a megabyte of stack. -/
theorem sound_total_fits_the_stack : SoundTotal maxTotal frameOverhead stackSize := by
  unfold SoundTotal maxTotal frameOverhead stackSize
  omega

/-- With room left, which is what makes the bound a bound rather than an exact
    fit: an extra page of frame does not turn it into an overrun. -/
theorem the_fit_has_margin :
    SoundTotal maxTotal (frameOverhead + frameOverhead) stackSize := by
  unfold SoundTotal maxTotal frameOverhead stackSize
  omega

/-- A total equal to the stack is unsound, whatever the overhead: there is no room
    for the frame the arguments are copied into. -/
theorem a_total_the_size_of_the_stack_is_unsound (overhead : Nat) (h : 0 < overhead) :
    ¬ SoundTotal stackSize overhead stackSize := by
  unfold SoundTotal
  omega

/-- The coupling, stated as the thing that breaks: raising the total without
    raising the stack turns a sound bound into an unsound one. This is the only
    theorem in the file that is about maintenance rather than about a value, and
    it is the one the two constants needed. -/
theorem the_total_is_coupled_to_the_stack (total overhead stack raise : Nat)
    (h : SoundTotal total overhead stack) (hraise : stack < total + overhead + raise) :
    ¬ SoundTotal (total + raise) overhead stack := by
  unfold SoundTotal at *
  omega

/-- Raising both together keeps it sound, which is what the coupling asks for. -/
theorem raising_both_stays_sound (total overhead stack raise : Nat)
    (h : SoundTotal total overhead stack) :
    SoundTotal (total + raise) overhead (stack + raise) := by
  unfold SoundTotal at *
  omega

/-- The largest total the stack admits, characterised rather than chosen. A
    constant derived from the stack cannot drift away from it. -/
theorem greatest_sound_total (overhead stack total : Nat) (h : overhead ≤ stack) :
    SoundTotal total overhead stack ↔ total ≤ stack - overhead := by
  unfold SoundTotal
  omega

/-! ### How many arguments -/

/-- What an argument list costs: every argument plus its terminator. -/
def totalCost : List Nat → Nat
  | [] => 0
  | l :: rest => (l + minArgCost) + totalCost rest

/-- An argument list fits when its bytes fit. -/
def Fits (lengths : List Nat) : Prop := totalCost lengths ≤ maxTotal

/-- Every argument costs at least its terminator, so the cost bounds the count. -/
theorem length_le_totalCost (lengths : List Nat) : lengths.length ≤ totalCost lengths := by
  induction lengths with
  | nil => simp [totalCost]
  | cons l rest ih =>
    show rest.length + 1 ≤ (l + minArgCost) + totalCost rest
    unfold minArgCost
    omega

/-- So a list that fits is a list with a bounded number of entries. The pointer
    array therefore cannot be grown without growing the bytes, and a caller cannot
    pass a million empty arguments to make the loader allocate a million
    pointers. -/
theorem count_is_bounded_by_the_total (lengths : List Nat) (h : Fits lengths) :
    lengths.length ≤ maxTotal := by
  unfold Fits at h
  have := length_le_totalCost lengths
  omega

/-- An empty list fits, so the bound admits the ordinary case rather than only
    refusing the extreme one. -/
theorem no_arguments_fit : Fits [] := by
  show totalCost [] ≤ maxTotal
  show (0 : Nat) ≤ maxTotal
  omega

/-- And a single argument at the per-argument limit fits the total, so the two
    bounds are consistent: nothing the argument check admits is refused by the
    total check on its own. -/
theorem one_maximal_argument_fits : Fits [maxArg] := by
  show totalCost [maxArg] ≤ maxTotal
  show (maxArg + minArgCost) + totalCost [] ≤ maxTotal
  unfold maxArg minArgCost maxTotal totalCost
  omega

end Nonos.ArgvBounds
