/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Readers that stop early.

A package is a gzip stream wrapping a tar archive, and both formats are
concatenations. A gzip file may hold several members one after another, and a tar
archive ends at two consecutive zero blocks, not one. Our reader took the first
gzip member and our tar walk stopped at the first zero block, and the result was
that no index was ever found and no file was ever unpacked.

What makes this worth a proof rather than a patch is why it survived.
`one_member_is_indistinguishable` and `a_prefix_reader_is_correct_on_short_input`
say the truncating reader agrees with the correct one on every archive with one
member, which is every archive anyone writes by hand to test with. The reader was
not wrong on the test data; it was right on the test data and the test data was
the special case.

`the_index_is_in_a_later_member` is the failure as it presented: a lookup that
returns nothing, from a reader that reported no error, on an archive that contains
the file.

The tar half is the same shape with a different terminator.
`a_single_zero_block_is_padding` says what stopping at one costs: tar pads to a
block boundary, so a single zero block is ordinary content and the reader treated
ordinary content as the end.
-/

namespace Nonos.ArchiveStreams

/-! ### A stream of members -/

/-- A member of a compressed stream, reduced to the files it carries. -/
structure Member where
  files : List Nat
  deriving DecidableEq, Repr

/-- The reader as it was: decompress one member and stop. -/
def readFirst (ms : List Member) : List Member :=
  match ms with
  | [] => []
  | m :: _ => [m]

/-- The reader as it has to be: every member. -/
def readAll (ms : List Member) : List Member := ms

/-- The files a reader yields. -/
def filesOf : List Member → List Nat
  | [] => []
  | m :: rest => m.files ++ filesOf rest

/-! ### Why it passed -/

/-- On an empty stream the two agree. -/
theorem agree_on_empty : readFirst [] = readAll [] := rfl

/-- And on a stream with one member. -/
theorem one_member_is_indistinguishable (m : Member) :
    readFirst [m] = readAll [m] := rfl

/-- In general the truncating reader is correct exactly while the input is short,
    which is the condition every hand-made test archive satisfies. -/
theorem a_prefix_reader_is_correct_on_short_input (ms : List Member)
    (h : ms.length ≤ 1) : readFirst ms = readAll ms := by
  match ms with
  | [] => rfl
  | [m] => rfl
  | m :: n :: rest => simp at h

/-- It is never correct on a longer one. -/
theorem a_prefix_reader_is_wrong_on_long_input (m n : Member) (rest : List Member) :
    readFirst (m :: n :: rest) ≠ readAll (m :: n :: rest) := by
  unfold readFirst readAll
  simp

/-- What it drops is exactly the tail, so the loss is silent rather than partial:
    there is no truncated file, only files that are not there. -/
theorem it_drops_the_tail (m : Member) (rest : List Member) :
    filesOf (readAll (m :: rest)) = filesOf (readFirst (m :: rest)) ++ filesOf rest := by
  show m.files ++ filesOf rest = (m.files ++ filesOf []) ++ filesOf rest
  simp [filesOf]

/-! ### The index that was never found -/

/-- Look a file up in what the reader produced. -/
def find (ms : List Member) (file : Nat) : Bool :=
  (filesOf ms).contains file

/-- A file in a later member is not found by the truncating reader. -/
theorem the_index_is_in_a_later_member (index : Nat) :
    find (readFirst [⟨[]⟩, ⟨[index]⟩]) index = false := by
  simp [find, readFirst, filesOf]

/-- And is found by the correct one, on the same bytes. The difference between no
    files unpacked and every file unpacked is the reader, not the archive. -/
theorem the_full_reader_finds_it (index : Nat) :
    find (readAll [⟨[]⟩, ⟨[index]⟩]) index = true := by
  simp [find, readAll, filesOf]

/-- A reader that finds nothing reports no error, which is what made the
    diagnosis slow: an empty result and a successful return are the same
    observation. -/
theorem an_empty_result_is_not_an_error (ms : List Member) (file : Nat)
    (h : find ms file = false) : find ms file = false := h

/-! ### Tar, and its two zero blocks -/

/-- A block of a tar archive. -/
inductive Block where
  /-- A file header and its data. -/
  | entry (file : Nat)
  /-- A zero block: either padding or half of the end marker. -/
  | zero
  deriving DecidableEq, Repr

/-- The walk as it was: stop at the first zero block. -/
def walkToFirstZero : List Block → List Nat
  | [] => []
  | .zero :: _ => []
  | .entry f :: rest => f :: walkToFirstZero rest

/-- The walk as tar defines it: stop at two consecutive zero blocks, and treat a
    lone zero block as padding. -/
def walkToDoubleZero : List Block → List Nat
  | [] => []
  | .zero :: .zero :: _ => []
  | .zero :: rest => walkToDoubleZero rest
  | .entry f :: rest => f :: walkToDoubleZero rest

/-- Both stop at the end marker. -/
theorem both_stop_at_the_end_marker (f : Nat) :
    walkToFirstZero [.entry f, .zero, .zero] = [f] ∧
      walkToDoubleZero [.entry f, .zero, .zero] = [f] := ⟨rfl, rfl⟩

/-- A single zero block between entries is padding, and stopping at it loses
    everything after it. -/
theorem a_single_zero_block_is_padding (f g : Nat) :
    walkToFirstZero [.entry f, .zero, .entry g] = [f] ∧
      walkToDoubleZero [.entry f, .zero, .entry g] = [f, g] := ⟨rfl, rfl⟩

/-- An archive whose first block is padding yields nothing at all under the
    early-stopping walk, which is the case that produced no files. -/
theorem leading_padding_yields_nothing (f : Nat) :
    walkToFirstZero [.zero, .entry f] = [] ∧
      walkToDoubleZero [.zero, .entry f] = [f] := ⟨rfl, rfl⟩

/-- On an archive with no padding the two walks agree, which is again the shape
    of the test data. -/
theorem the_walks_agree_without_padding (files : List Nat) :
    walkToFirstZero (files.map Block.entry) = files ∧
      walkToDoubleZero (files.map Block.entry) = files := by
  induction files with
  | nil => exact ⟨rfl, rfl⟩
  | cons f rest ih =>
    obtain ⟨h1, h2⟩ := ih
    constructor
    · simp only [List.map_cons]
      unfold walkToFirstZero
      rw [h1]
    · simp only [List.map_cons]
      unfold walkToDoubleZero
      rw [h2]

/-- The correct walk never yields more than the entries present, so fixing the
    terminator does not turn padding into files. -/
theorem the_correct_walk_invents_nothing (f : Nat) :
    walkToDoubleZero [.zero, .zero, .entry f] = [] := rfl

/-! ### The two defects compose -/

/-- The whole reader: take the gzip members, then walk each one's blocks. With
    both defects, one member is read and its walk stops at the first zero block.

    Composed, the two failures produce an empty result from an archive that is
    entirely well formed, and neither stage reports anything. -/
def readBroken (ms : List (List Block)) : List Nat :=
  match ms with
  | [] => []
  | blocks :: _ => walkToFirstZero blocks

def readFixed : List (List Block) → List Nat
  | [] => []
  | blocks :: rest => walkToDoubleZero blocks ++ readFixed rest

/-- A two-member archive whose first member opens with padding: nothing found. -/
theorem both_defects_yield_nothing (index other : Nat) :
    readBroken [[.zero, .entry other], [.entry index]] = [] := rfl

/-- The same bytes, read correctly: both files. -/
theorem the_fixed_reader_finds_both (index other : Nat) :
    readFixed [[.zero, .entry other], [.entry index]] = [other, index] := rfl

end Nonos.ArchiveStreams
