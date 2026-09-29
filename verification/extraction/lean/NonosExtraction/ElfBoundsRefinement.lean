/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The ELF program-header bounds check, on the extracted code.

`program_header_bounds` stands in front of every program-header read. It turns the
header's `e_phoff`, `e_phentsize` and `e_phnum` into an offset, a stride and a
count, and `parse_program_header_at` then reads `size_of::<ProgramHeader>()` bytes
at `offset + stride * index` with `ptr::read_unaligned`. Slicing `elf_data[off..]`
checks only where that read starts. That the rest of it is inside the image is this
function's promise and nobody else's, and the image is attacker-controlled bytes,
so a table that ran past the end would be an out-of-bounds read in unsafe code.

`accepted_table_is_inside_the_file` is that promise over the extracted function:
when it accepts a non-empty table, the stride is the size of a program header and
the table ends at or before the end of the image. `every_entry_read_is_inside_the_file`
is the form the caller reads it in: for every index below the count, the header
read at that index ends inside the image. Both take one assumption, that
`Option::ok_or` never returns a value it was not given.

`program_header_bounds_is_the_model` is the stronger statement. On every header
whose offset fits a `usize`, the extracted function returns exactly the verdict of
`Nonos.ElfPhdr.check`, the core corpus's hand-written model of this check: the same
offset, stride and count when it accepts, and the same error when it refuses. The
model and the code were written apart and have to agree, and this is where they are
shown to. `an_offset_beyond_the_word_is_refused` covers the headers that are left.
`a_table_whose_end_wraps_is_refused` is the case a naive `phoff + size * count`
gets wrong: an end that wraps past the top of the address space compares as a small
number and would pass the image-length test. On the way the proof also shows the
multiplication cannot fail once the stride is fifty-six, since 56 * 65535 fits in
thirty-two bits, so the `checked_mul` refusal is dead code on every target.

What the extracted code cannot say on its own. Aeneas has no model for four of the
standard-library calls the function makes, `usize::try_from(u64)`,
`Result::map_err`, `Option::ok_or` and `size_of::<ProgramHeader>()`, and emits each
as an opaque axiom, registered in `ASSUMPTIONS.md`. Nothing can be proven about what
an opaque function returns, so the model theorem takes the documented behaviour of
each as a named hypothesis: the conversion is the one Aeneas models for every other
width, `map_err` touches only the error, `ok_or` is the match it is documented to be
(the four lines `Policy/FunsExternal.lean` gives it), and a `#[repr(C)]` program
header of two `u32` and six `u64` fields is fifty-six bytes. Each hypothesis is an
equation fixing a different opaque constant, and each is satisfied by the function
it names, so together they cannot be contradictory, and none of them constrains
`program_header_bounds` itself.

The fifty-six is also a second copy of one fact. The header validator compares
`e_phentsize` with `ProgramHeader::SIZE`, a literal 56, while this function compares
it with `size_of::<ProgramHeader>()`. They agree because of the layout, not because
anything checks that they do, and `ProgramHeaderIsFiftySixBytes` is where that is
written down.

Word width is left open, as Aeneas leaves it. On a sixty-four bit target every
`e_phoff` fits and `an_offset_beyond_the_word_is_refused` is about no header; it is
what the code does on a thirty-two bit one.
-/

import NonosExtraction.Elf
import Nonos.ElfPhdr

open Aeneas Aeneas.Std Result
open nonos_elf

set_option linter.hashCommand false

namespace NonosExtraction.ElfBounds

open elf.loader.core.parse_header.bounds renaming program_header_bounds → phBounds
open elf.types.program.state (ProgramHeader)
open elf.errors.types.state (ElfError)

theorem bind_eq_ok {α β : Type} (x : Result α) (f : α → Result β) (v : β) :
    (x >>= f) = ok v ↔ ∃ a, x = ok a ∧ f a = ok v := by
  cases x <;> simp

/-! ### What the standard library is assumed to do -/

/-- `usize::try_from(u64)` is the conversion Aeneas models for every other pair of
    widths: the value when it fits, an error when it does not. -/
def TryFromIsTheConversion : Prop :=
  ∀ x : Std.U64,
    nonos_elf.Usize.Insts.CoreConvertTryFromU64TryFromIntError.try_from x =
      core.num.tryFromUScalar .Usize x

/-- `Result::map_err` passes a value through and applies its closure to an error. -/
def MapErrMapsTheError : Prop :=
  ∀ {T E F O : Type} (inst : core.ops.function.FnOnce O E F) (r : core.result.Result T E) (c : O),
    core.result.Result.map_err inst r c =
      match r with
      | .Ok v => ok (.Ok v)
      | .Err e => do
        let f ← inst.call_once c e
        ok (.Err f)

/-- `Option::ok_or`: `Some` becomes `Ok`, `None` becomes the given error. -/
def OkOrIsTheMatch : Prop :=
  ∀ {T E : Type} (o : Option T) (e : E),
    core.option.Option.ok_or o e =
      ok (match o with
        | some x => .Ok x
        | none => .Err e)

/-- The half of `ok_or` the bound needs: an `Ok` carries the value it was given. -/
def OkOrInventsNothing : Prop :=
  ∀ (o : Option Std.Usize) (e : ElfError) (x : Std.Usize),
    core.option.Option.ok_or o e = ok (.Ok x) → o = some x

theorem OkOrIsTheMatch.inventsNothing (hok : OkOrIsTheMatch) : OkOrInventsNothing := by
  intro o e x h
  rw [hok] at h
  cases o <;> simp_all

/-- Two `u32` and six `u64` fields under `#[repr(C)]`, with no padding. -/
def ProgramHeaderIsFiftySixBytes : Prop :=
  core.mem.size_of ProgramHeader = ok 56#usize

/-! ### Accepted means inside the file -/

/-- An accepted non-empty table has the stride of a program header and ends at or
    before the end of the image. The count and stride are the header's own fields.
    An empty table is accepted without looking at its stride, which is harmless
    because nothing is read from it. -/
theorem accepted_table_is_inside_the_file (hok : OkOrInventsNothing)
    (elf_data : Slice Std.U8) (header : elf.types.header.state.ElfHeader)
    (off size count : Std.Usize)
    (h : phBounds elf_data header = ok (.Ok (off, size, count))) :
    size.val = header.e_phentsize.val ∧ count.val = header.e_phnum.val ∧
      (count.val = 0 ∨
        ∃ i, core.mem.size_of ProgramHeader = ok i ∧ size = i ∧
          off.val + size.val * count.val ≤ elf_data.length) := by
  unfold phBounds at h
  simp only [bind_eq_ok] at h
  obtain ⟨r, -, r1, -, cf, hcf, h⟩ := h
  cases r1 with
  | Err e =>
    simp only [core.result.Result.Insts.CoreOpsTry.branch, ok.injEq] at hcf
    subst hcf
    simp [core.result.Result.Insts.CoreOpsTryTraitFromResidualResultInfallible.from_residual] at h
  | Ok val =>
    simp only [core.result.Result.Insts.CoreOpsTry.branch, ok.injEq] at hcf
    subst hcf
    simp only [lift, bind_tc_ok] at h
    have hsz := core.convert.num.FromUsizeU16.from_val_eq header.e_phentsize
    have hct := core.convert.num.FromUsizeU16.from_val_eq header.e_phnum
    split at h
    · rename_i hzero
      simp only [ok.injEq, core.result.Result.Ok.injEq, Prod.mk.injEq] at h
      obtain ⟨rfl, rfl, rfl⟩ := h
      refine ⟨hsz, hct, Or.inl ?_⟩
      rw [hzero]
      rfl
    · simp only [bind_eq_ok] at h
      obtain ⟨i, hi, h⟩ := h
      split at h
      · simp at h
      · rename_i hsame
        simp only [bind_eq_ok] at h
        obtain ⟨r2, hr2, cf1, hcf1, h⟩ := h
        cases r2 with
        | Err e =>
          simp only [core.result.Result.Insts.CoreOpsTry.branch, ok.injEq] at hcf1
          subst hcf1
          simp [core.result.Result.Insts.CoreOpsTryTraitFromResidualResultInfallible.from_residual] at h
        | Ok val1 =>
          have hmul := hok _ _ _ hr2
          simp only [core.result.Result.Insts.CoreOpsTry.branch, ok.injEq] at hcf1
          subst hcf1
          simp only [bind_eq_ok] at h
          obtain ⟨r3, hr3, cf2, hcf2, h⟩ := h
          cases r3 with
          | Err e =>
            simp only [core.result.Result.Insts.CoreOpsTry.branch, ok.injEq] at hcf2
            subst hcf2
            simp [core.result.Result.Insts.CoreOpsTryTraitFromResidualResultInfallible.from_residual] at h
          | Ok val2 =>
            have hadd := hok _ _ _ hr3
            simp only [core.result.Result.Insts.CoreOpsTry.branch, ok.injEq] at hcf2
            subst hcf2
            simp only at h
            split at h
            · simp at h
            · rename_i hle
              simp only [ok.injEq, core.result.Result.Ok.injEq, Prod.mk.injEq] at h
              obtain ⟨rfl, rfl, rfl⟩ := h
              have hm := Usize.checked_mul_bv_spec (core.convert.num.FromUsizeU16.from header.e_phentsize)
                (core.convert.num.FromUsizeU16.from header.e_phnum)
              rw [hmul] at hm
              have ha := Usize.checked_add_bv_spec val val1
              rw [hadd] at ha
              have hsame' : core.convert.num.FromUsizeU16.from header.e_phentsize = i := by
                simp only [bne_iff_ne, ne_eq, Decidable.not_not] at hsame
                exact hsame
              have hend : val2.val ≤ elf_data.length := by
                simp only [gt_iff_lt, not_lt] at hle
                scalar_tac
              refine ⟨hsz, hct, Or.inr ⟨i, hi, hsame', ?_⟩⟩
              omega

/-- What `parse_program_header_at` relies on: for every index below the count, the
    `size_of::<ProgramHeader>()` bytes it reads at `offset + stride * index` are
    inside the image. -/
theorem every_entry_read_is_inside_the_file (hok : OkOrInventsNothing)
    (elf_data : Slice Std.U8) (header : elf.types.header.state.ElfHeader)
    (off size count : Std.Usize)
    (h : phBounds elf_data header = ok (.Ok (off, size, count)))
    (k : Nat) (hk : k < count.val) :
    ∃ i, core.mem.size_of ProgramHeader = ok i ∧
      off.val + size.val * k + i.val ≤ elf_data.length := by
  obtain ⟨-, -, hc⟩ := accepted_table_is_inside_the_file hok elf_data header off size count h
  rcases hc with hz | ⟨i, hi, hsi, hle⟩
  · omega
  · refine ⟨i, hi, ?_⟩
    subst hsi
    have hstep : size.val * (k + 1) ≤ size.val * count.val := Nat.mul_le_mul_left _ hk
    rw [Nat.mul_succ] at hstep
    omega

/-! ### The code is the model -/

/-- A result of the extracted function and a verdict of the model say the same
    thing: the same triple on acceptance, the same error on refusal. -/
def Agrees : core.result.Result (Std.Usize × Std.Usize × Std.Usize) ElfError →
    Nonos.ElfPhdr.Outcome → Prop
  | .Ok (a, b, c), .ok off size count => a.val = off ∧ b.val = size ∧ c.val = count
  | .Err .InvalidProgramHeaderSize, .errSize => True
  | .Err .ProgramHeadersOutOfBounds, .errBounds => True
  | _, _ => False

theorem Agrees.errBounds {r : core.result.Result (Std.Usize × Std.Usize × Std.Usize) ElfError}
    (h : Agrees r .errBounds) : r = .Err .ProgramHeadersOutOfBounds := by
  rcases r with ⟨a, b, c⟩ | e
  · simp [Agrees] at h
  · cases e <;> simp_all [Agrees]

/-- On every header whose offset fits a `usize`, the extracted function returns the
    verdict of `Nonos.ElfPhdr.check` with the expected size fifty-six. -/
theorem program_header_bounds_is_the_model
    (htry : TryFromIsTheConversion) (hme : MapErrMapsTheError) (hok : OkOrIsTheMatch)
    (hsz : ProgramHeaderIsFiftySixBytes)
    (elf_data : Slice Std.U8) (header : elf.types.header.state.ElfHeader)
    (hfit : header.e_phoff.val ≤ Usize.max) :
    ∃ r, phBounds elf_data header = ok r ∧
      Agrees r (Nonos.ElfPhdr.check elf_data.length header.e_phoff.val
        header.e_phentsize.val header.e_phnum.val 56) := by
  unfold phBounds
  rw [htry]
  unfold core.num.tryFromUScalar
  have hfit' : header.e_phoff.val ≤ UScalar.max .Usize := by scalar_tac
  rw [if_pos hfit']
  simp only [bind_tc_ok, core.result.Result.Insts.CoreOpsTry.branch, lift]
  rw [hme]
  simp only [bind_tc_ok]
  have hmax : Usize.max + 1 ≤ 2 ^ 64 := by
    rw [Usize.max_succ_eq_pow]
    cases System.Platform.numBits_eq <;> simp [*]
  have hlen : elf_data.length ≤ Usize.max := by scalar_tac
  have hv : (UScalar.cast UScalarTy.Usize header.e_phoff).val = header.e_phoff.val := by
    rw [UScalar.cast_val_eq]
    apply Nat.mod_eq_of_lt
    have := Usize.max_succ_eq_pow
    simp only [UScalarTy.numBits] at *
    omega
  have hsz' := core.convert.num.FromUsizeU16.from_val_eq header.e_phentsize
  have hct' := core.convert.num.FromUsizeU16.from_val_eq header.e_phnum
  unfold Nonos.ElfPhdr.check
  by_cases hn : header.e_phnum.val = 0
  · have h0 : core.convert.num.FromUsizeU16.from header.e_phnum = 0#usize := by
      scalar_tac
    rw [if_pos h0, if_pos hn]
    exact ⟨_, rfl, hv, hsz', hct'⟩
  · have h0 : ¬ core.convert.num.FromUsizeU16.from header.e_phnum = 0#usize := by
      intro h
      apply hn
      rw [← hct', h]
      rfl
    rw [if_neg h0, if_neg hn, hsz, bind_tc_ok]
    by_cases hs : header.e_phentsize.val = 56
    · have hs' : ¬ (core.convert.num.FromUsizeU16.from header.e_phentsize != 56#usize) = true := by
        simp only [bne_iff_ne, ne_eq, Decidable.not_not]
        scalar_tac
      have hnum : header.e_phnum.val < 65536 := by scalar_tac
      have hprod : header.e_phentsize.val * header.e_phnum.val = 56 * header.e_phnum.val := by
        rw [hs]
      have hprod' : (core.convert.num.FromUsizeU16.from header.e_phentsize).val *
          (core.convert.num.FromUsizeU16.from header.e_phnum).val = 56 * header.e_phnum.val := by
        rw [hsz', hct', hs]
      rw [if_neg hs', if_neg (by omega)]
      have hm := Usize.checked_mul_bv_spec (core.convert.num.FromUsizeU16.from header.e_phentsize)
        (core.convert.num.FromUsizeU16.from header.e_phnum)
      cases hmul : Usize.checked_mul (core.convert.num.FromUsizeU16.from header.e_phentsize)
          (core.convert.num.FromUsizeU16.from header.e_phnum) with
      | none =>
        rw [hmul] at hm
        exfalso
        have := Usize.max_succ_eq_pow
        have h32 : 2 ^ 32 ≤ 2 ^ System.Platform.numBits := by
          cases System.Platform.numBits_eq <;> simp [*]
        scalar_tac
      | some t =>
        rw [hmul] at hm
        obtain ⟨-, ht, -⟩ := hm
        rw [hok]
        simp only [bind_tc_ok]
        rw [if_neg (by simp only [Nonos.ElfPhdr.usizeMax]; omega)]
        have ha := Usize.checked_add_bv_spec (UScalar.cast UScalarTy.Usize header.e_phoff) t
        cases hadd : Usize.checked_add (UScalar.cast UScalarTy.Usize header.e_phoff) t with
        | none =>
          rw [hadd] at ha
          rw [hok]
          simp only [bind_tc_ok,
            core.result.Result.Insts.CoreOpsTryTraitFromResidualResultInfallible.from_residual,
            core.convert.FromSame.from]
          refine ⟨_, rfl, ?_⟩
          have hover : Usize.max < header.e_phoff.val + header.e_phentsize.val * header.e_phnum.val := by
            omega
          split_ifs with h1 h2
          · trivial
          · trivial
          · exfalso
            omega
        | some e =>
          rw [hadd] at ha
          obtain ⟨-, he, -⟩ := ha
          rw [hok]
          simp only [bind_tc_ok]
          have hend : e.val = header.e_phoff.val + header.e_phentsize.val * header.e_phnum.val := by
            omega
          have heb : e.val ≤ Usize.max := by scalar_tac
          by_cases hpast : e.val > elf_data.length
          · have hpast' : e > elf_data.len := by scalar_tac
            rw [if_pos hpast', if_neg (by simp only [Nonos.ElfPhdr.usizeMax]; omega), if_pos (by omega)]
            exact ⟨_, rfl, trivial⟩
          · have hpast' : ¬ e > elf_data.len := by scalar_tac
            rw [if_neg hpast', if_neg (by simp only [Nonos.ElfPhdr.usizeMax]; omega), if_neg (by omega)]
            exact ⟨_, rfl, hv, hsz', hct'⟩
    · have hs' : (core.convert.num.FromUsizeU16.from header.e_phentsize != 56#usize) = true := by
        simp only [bne_iff_ne, ne_eq]
        intro h
        apply hs
        rw [← hsz', h]
        rfl
      rw [if_pos hs', if_pos hs]
      exact ⟨_, rfl, trivial⟩

/-- The headers the model theorem leaves out: an offset that does not fit a `usize`
    is refused as out of bounds, through the `map_err` closure. -/
theorem an_offset_beyond_the_word_is_refused
    (htry : TryFromIsTheConversion) (hme : MapErrMapsTheError)
    (elf_data : Slice Std.U8) (header : elf.types.header.state.ElfHeader)
    (hbig : Usize.max < header.e_phoff.val) :
    phBounds elf_data header = ok (.Err .ProgramHeadersOutOfBounds) := by
  unfold phBounds
  rw [htry]
  unfold core.num.tryFromUScalar
  have hover : ¬ header.e_phoff.val ≤ UScalar.max .Usize := by scalar_tac
  rw [if_neg hover]
  simp only [bind_tc_ok]
  rw [hme]
  simp [elf.loader.core.parse_header.bounds.program_header_bounds.closure.Insts.CoreOpsFunctionFnOnceTupleTryFromIntErrorElfError.call_once,
    core.result.Result.Insts.CoreOpsTry.branch,
    core.result.Result.Insts.CoreOpsTryTraitFromResidualResultInfallible.from_residual,
    core.convert.FromSame.from]

/-- A table whose end would wrap past the top of the address space is refused. A
    check that formed `phoff + size * count` without `checked_add` would compare a
    small wrapped number against the image length and accept it. The hypotheses
    hold at `e_phoff = usize::MAX` with one entry, whatever the word width. -/
theorem a_table_whose_end_wraps_is_refused
    (htry : TryFromIsTheConversion) (hme : MapErrMapsTheError) (hok : OkOrIsTheMatch)
    (hsz : ProgramHeaderIsFiftySixBytes)
    (elf_data : Slice Std.U8) (header : elf.types.header.state.ElfHeader)
    (hfit : header.e_phoff.val ≤ Usize.max) (hs : header.e_phentsize.val = 56)
    (hn : header.e_phnum.val ≠ 0)
    (hwrap : Usize.max < header.e_phoff.val + 56 * header.e_phnum.val) :
    phBounds elf_data header = ok (.Err .ProgramHeadersOutOfBounds) := by
  obtain ⟨r, hr, hag⟩ := program_header_bounds_is_the_model htry hme hok hsz elf_data header hfit
  have hlen : elf_data.length ≤ Usize.max := by scalar_tac
  have hmodel : Nonos.ElfPhdr.check elf_data.length header.e_phoff.val
      header.e_phentsize.val header.e_phnum.val 56 = .errBounds := by
    unfold Nonos.ElfPhdr.check
    rw [if_neg hn, hs, if_neg (by omega)]
    split_ifs <;> first | rfl | omega
  rw [hmodel] at hag
  rw [hr, hag.errBounds]

end NonosExtraction.ElfBounds

/-! ### Axiom profile

    Printed by the build. The four opaque standard-library calls appear in every
    line whose closure unfolds `program_header_bounds`, because they are in its
    definition; they are registered in `ASSUMPTIONS.md`. `sorryAx` in any line
    would mean a `sorry` reached a theorem about extracted code. -/

#print axioms NonosExtraction.ElfBounds.accepted_table_is_inside_the_file
#print axioms NonosExtraction.ElfBounds.every_entry_read_is_inside_the_file
#print axioms NonosExtraction.ElfBounds.program_header_bounds_is_the_model
#print axioms NonosExtraction.ElfBounds.an_offset_beyond_the_word_is_refused
#print axioms NonosExtraction.ElfBounds.a_table_whose_end_wraps_is_refused
