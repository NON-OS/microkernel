/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Where a Linux guest's memory lives, and why it cannot leave.

A guest holds no capabilities. That was read for a while as meaning it was
contained, and it is not the same statement: a guest with no authority at all
still calls `brk` and `mmap`, and a heap that grows without a ceiling takes the
machine down without ever needing a capability to do it. Containment is the
layout's job, not the capability table's.

The constants are the ones in `userland/capsule_linux/src/linux/guest/layout.rs`
and the admission window is the one `call/mem/memory.rs::brk` applies. The
theorems say what the layout buys: the heap cannot reach the mapping area, the
mapping area cannot reach the loaded images, the stack sits above everything a
guest maps for itself, and the three together are bounded by a gigabyte, so the
worst a guest can ask for is a number the machine already has.

`break_stays_below_mappings` is the one that needed proving rather than
asserting. `brk` does not use the requested break directly, it rounds up to a
page first, and a rounding step applied after a bound check is how a bound check
gets bypassed. It holds here, and it holds because `MMAP_BASE` is page aligned:
`page_up_respects_an_aligned_bound` is where that is actually used, and it is
the reason changing `MMAP_BASE` to an unaligned address would be a security
change rather than a layout change.
-/

namespace Nonos.GuestLayout

/-! ### The layout -/

def pageSize : Nat := 4096

/-- The heap, growing up from here as `brk` moves. -/
def brkBase : Nat := 0x10000000

/-- Anonymous and file mappings, growing up from here. -/
def mmapBase : Nat := 0x20000000

/-- Where the loader biases a position-independent executable. -/
def execBase : Nat := 0x40000000

/-- Where its interpreter goes. -/
def interpBase : Nat := 0x50000000

/-- The stack top a guest wakes on. -/
def stackTop : Nat := 0x7FFFF000

/-- The stack a guest gets. -/
def stackSize : Nat := 0x100000

/-- The break may not reach the mapping area. -/
def brkLimit : Nat := mmapBase

/-- A mapping may not reach the images above it. -/
def mmapLimit : Nat := execBase

/-- The lowest stack address, derived rather than written down, so it cannot
    disagree with the top and the size. -/
def stackBase : Nat := stackTop - stackSize

/-- The first address of the kernel half, from `process/foreign/peer_guard.rs`.
    Repeated here only to prove the guest never comes near it. -/
def userVaEnd : Nat := 0x800000000000

/-- Every boundary is strictly ordered, so each region has somewhere to grow
    and no two of them start at the same address. -/
theorem layout_is_ordered :
    brkBase < mmapBase ∧ mmapBase < execBase ∧ execBase < interpBase ∧
      interpBase < stackBase ∧ stackBase < stackTop := by
  unfold brkBase mmapBase execBase interpBase stackBase stackTop stackSize
  omega

/-- Both growth ceilings are page aligned. Page rounding is safe against an
    aligned ceiling and unsafe against an unaligned one, so this is a
    precondition of `break_stays_below_mappings` and not a detail. -/
theorem limits_are_page_aligned :
    brkLimit % pageSize = 0 ∧ mmapLimit % pageSize = 0 := by
  unfold brkLimit mmapLimit mmapBase execBase pageSize
  omega

/-! ### Page rounding

    `page_up` in `guest/mod.rs`, as the ceiling division it is. -/

def pageUp (a : Nat) : Nat := ((a + pageSize - 1) / pageSize) * pageSize

/-- Rounding up never moves an address down. -/
theorem pageUp_ge (a : Nat) : a ≤ pageUp a := by
  unfold pageUp pageSize
  omega

/-- The result is a whole number of pages. -/
theorem pageUp_aligned (a : Nat) : pageUp a % pageSize = 0 := by
  unfold pageUp pageSize
  omega

/-- Rounding moves an address by less than a page, so it cannot skip a page
    boundary and land past a region it was inside. -/
theorem pageUp_lt_next_page (a : Nat) : pageUp a < a + pageSize := by
  unfold pageUp pageSize
  omega

/-- An address already on a page boundary is left alone. -/
theorem pageUp_fixes_aligned (a : Nat) (h : a % pageSize = 0) : pageUp a = a := by
  unfold pageUp pageSize
  unfold pageSize at h
  omega

/-- Rounding up respects any page-aligned upper bound.

    This is the load-bearing lemma. Against an unaligned bound it is false:
    rounding a value just under the bound produces a value just over it. -/
theorem pageUp_respects_an_aligned_bound (a b : Nat)
    (hb : b % pageSize = 0) (h : a ≤ b) : pageUp a ≤ b := by
  unfold pageUp pageSize
  unfold pageSize at hb
  omega

/-! ### The admission window `brk` applies -/

/-- What `brk` accepts as a move. A request of zero is a query, and a request
    outside the heap area is reported as no move at all, which is what a Linux
    program reads as a refusal. -/
def Admits (want : Nat) : Prop := want ≠ 0 ∧ brkBase ≤ want ∧ want ≤ brkLimit

instance : DecidablePred Admits := fun w => by
  unfold Admits; infer_instance

/-- An admitted break, after the page rounding the mapping call actually uses,
    is still below the mapping area. The check and the rounding agree. -/
theorem break_stays_below_mappings (want : Nat) (h : Admits want) :
    pageUp want ≤ mmapBase := by
  obtain ⟨_, _, hlim⟩ := h
  have halign : brkLimit % pageSize = 0 := limits_are_page_aligned.1
  have := pageUp_respects_an_aligned_bound want brkLimit halign hlim
  unfold brkLimit at this
  exact this

/-- Nothing below the heap base is ever admitted, so a guest cannot walk its
    break down into the pages below its own image. -/
theorem break_never_goes_below_base (want : Nat) (h : Admits want) :
    brkBase ≤ want := h.2.1

/-- A break request outside the window is refused whatever else is true of it,
    which is why the refusal path needs no further reasoning about the value. -/
theorem outside_the_window_is_refused (want : Nat)
    (h : want < brkBase ∨ brkLimit < want) : ¬ Admits want := by
  intro ⟨_, hlo, hhi⟩
  cases h with
  | inl hl => omega
  | inr hr => omega

/-! ### Regions, and that they do not meet

    Ranges are carried as a base and a size rather than as a structure, so every
    statement below is linear arithmetic that `omega` can see through without a
    projection in the way. -/

/-- `a` lies in `[base, base + size)`. -/
def CoversRange (base size a : Nat) : Prop := base ≤ a ∧ a < base + size

/-- Two ranges never cover the same address. -/
def Apart (p q : Nat → Prop) : Prop := ∀ a, ¬ (p a ∧ q a)

/-- A range that ends at or below where another starts is apart from it. Every
    disjointness result in this file is this lemma plus one arithmetic fact
    about the layout constants. -/
theorem apart_of_ordered (b1 s1 b2 s2 : Nat) (h : b1 + s1 ≤ b2) :
    Apart (CoversRange b1 s1) (CoversRange b2 s2) := by
  intro a ⟨⟨_, h1⟩, ⟨h2, _⟩⟩
  omega

/-- The heap a guest has grown, at a given break. -/
def heapSize (brk : Nat) : Nat := brk - brkBase
def heapCovers (brk : Nat) : Nat → Prop := CoversRange brkBase (heapSize brk)

/-- The mappings a guest has taken, up to a given top. -/
def mappingsSize (top : Nat) : Nat := top - mmapBase
def mappingsCovers (top : Nat) : Nat → Prop := CoversRange mmapBase (mappingsSize top)

/-- The stack, whose size does not move. -/
def stackCovers : Nat → Prop := CoversRange stackBase stackSize

/-- The executable and its interpreter: here to be kept clear of, not grown. -/
def imageCovers : Nat → Prop := CoversRange execBase (interpBase - execBase)

/-- However far the break has moved inside its window, the heap does not reach
    the mapping area. -/
theorem heap_apart_from_mappings (brk top : Nat) (hb : brk ≤ brkLimit) :
    Apart (heapCovers brk) (mappingsCovers top) := by
  unfold heapCovers mappingsCovers
  refine apart_of_ordered _ _ _ _ ?_
  unfold heapSize brkBase mmapBase
  unfold brkLimit mmapBase at hb
  omega

/-- However far the mappings have grown inside their window, they do not reach
    the loaded images. -/
theorem mappings_apart_from_image (top : Nat) (ht : top ≤ mmapLimit) :
    Apart (mappingsCovers top) imageCovers := by
  unfold mappingsCovers imageCovers
  refine apart_of_ordered _ _ _ _ ?_
  unfold mappingsSize mmapBase execBase
  unfold mmapLimit execBase at ht
  omega

/-- And neither reaches the stack, which is the region whose corruption is
    directly a control-flow transfer. -/
theorem mappings_apart_from_stack (top : Nat) (ht : top ≤ mmapLimit) :
    Apart (mappingsCovers top) stackCovers := by
  unfold mappingsCovers stackCovers
  refine apart_of_ordered _ _ _ _ ?_
  unfold mappingsSize mmapBase stackBase stackTop stackSize
  unfold mmapLimit execBase at ht
  omega

/-- The heap is apart from the stack as well. -/
theorem heap_apart_from_stack (brk : Nat) (hb : brk ≤ brkLimit) :
    Apart (heapCovers brk) stackCovers := by
  unfold heapCovers stackCovers
  refine apart_of_ordered _ _ _ _ ?_
  unfold heapSize brkBase stackBase stackTop stackSize
  unfold brkLimit mmapBase at hb
  omega

/-! ### What a guest can cost the machine -/

/-- Every byte a guest can name is below the kernel half, with most of the
    address space to spare. A guest mapping cannot alias kernel memory because
    it cannot reach an address that high, independently of the peer-call bound
    check that also refuses it. -/
theorem guest_reach_is_below_the_kernel_half (brk top a : Nat)
    (hb : brk ≤ brkLimit) (ht : top ≤ mmapLimit)
    (h : heapCovers brk a ∨ mappingsCovers top a ∨ stackCovers a) :
    a < userVaEnd := by
  unfold heapCovers mappingsCovers stackCovers CoversRange heapSize mappingsSize
    brkBase mmapBase stackBase stackTop stackSize at h
  unfold userVaEnd
  unfold brkLimit mmapBase at hb
  unfold mmapLimit execBase at ht
  rcases h with ⟨_, h⟩ | ⟨_, h⟩ | ⟨_, h⟩ <;> omega

/-- The total memory a guest can hold is bounded by a gigabyte, by the layout
    alone and with no capability involved.

    This is the containment statement. A guest that asks for everything the
    layout allows asks for a number the machine already has, so `brk` and
    `mmap` are not a denial-of-service channel. -/
theorem guest_reach_is_bounded (brk top : Nat)
    (hb : brk ≤ brkLimit) (ht : top ≤ mmapLimit) :
    heapSize brk + mappingsSize top + stackSize ≤ 0x40000000 := by
  unfold heapSize mappingsSize stackSize brkBase mmapBase
  unfold brkLimit mmapBase at hb
  unfold mmapLimit execBase at ht
  omega

/-- A zero-length unmap is refused rather than treated as a whole-space unmap,
    which is what a length of zero means to an implementation that computes an
    end address and iterates to it. -/
def unmapAccepts (len : Nat) : Bool := len ≠ 0

theorem zero_length_unmap_is_refused : unmapAccepts 0 = false := by
  unfold unmapAccepts; simp

end Nonos.GuestLayout
