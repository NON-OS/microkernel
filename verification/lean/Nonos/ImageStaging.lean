/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Which bytes the machine boots, and which bytes it measured.

The image is packaged from a linked kernel. Relink the kernel after packaging and
the image still holds the previous one, so the machine boots code that is not in
the tree, reports the version string of code that is not running, and attests a
measurement of bytes nobody is executing. It cost three boots in one afternoon
before anyone suspected the build rather than the code.

The failure is an ordering one and it has nothing to do with hashing. A package
step reads the linker's output at the moment it runs; a build step after it
changes that output and leaves the package alone. `stale_package_boots_the_earlier_build`
is that trace, and `booted_is_the_build_at_package_time` is the general statement:
what boots is whatever was on disk when the package ran, not whatever is on disk
now.

The fix is sequencing, and `resequenced_boots_the_current_build` says what
sequencing has to guarantee: no build step between the package and the boot. The
recipe enforces it by making the package a dependency of the run rather than a
step someone remembers.

`measurement_follows_the_image` is the attestation half. A measurement taken over
the linker's output rather than over the image is a measurement of something that
is not being booted, and it stays true while being useless, which is the worst
property a measurement can have.
-/

namespace Nonos.ImageStaging

/-! ### The build -/

/-- What a build step produces: a content identity for the linked kernel. -/
abbrev Digest := Nat

/-- The steps a recipe can take. -/
inductive Step where
  /-- Link the kernel; the tree now holds this. -/
  | build (d : Digest)
  /-- Copy whatever the tree holds into the image. -/
  | package
  /-- Measure whatever the image holds. -/
  | measure
  /-- Start the machine from the image. -/
  | boot
  deriving DecidableEq, Repr

/-- What the recipe has produced so far: the tree's current kernel, the image's
    kernel, and the measurement that was taken. -/
structure State where
  tree : Option Digest
  image : Option Digest
  measured : Option Digest
  deriving DecidableEq, Repr

def State.empty : State := ⟨none, none, none⟩

/-- One step. `package` reads the tree, `measure` reads the image, and `boot`
    changes nothing: it is the observation, not an action. -/
def step (s : State) : Step → State
  | .build d => { s with tree := some d }
  | .package => { s with image := s.tree }
  | .measure => { s with measured := s.image }
  | .boot => s

def run (s : State) : List Step → State
  | [] => s
  | st :: rest => run (step s st) rest

/-- What the machine runs: whatever the image holds. -/
def booted (trace : List Step) : Option Digest := (run State.empty trace).image

/-- What was measured. -/
def measured (trace : List Step) : Option Digest := (run State.empty trace).measured

/-- What the tree holds, which is what a developer looking at the source sees. -/
def inTree (trace : List Step) : Option Digest := (run State.empty trace).tree

/-! ### The trace that cost the afternoon -/

/-- Link, package, link again, boot. -/
def staleTrace (first second : Digest) : List Step :=
  [.build first, .package, .build second, .boot]

/-- The image still holds the first kernel. -/
theorem stale_package_boots_the_earlier_build (first second : Digest) :
    booted (staleTrace first second) = some first := rfl

/-- While the tree holds the second. Nothing in the tree is wrong, and nothing in
    the image is what the tree says. -/
theorem the_tree_holds_the_later_build (first second : Digest) :
    inTree (staleTrace first second) = some second := rfl

/-- So the two disagree whenever the relink changed anything, which is the silent
    part: both values are well formed, and only their difference is the defect. -/
theorem tree_and_image_disagree (first second : Digest) (h : first ≠ second) :
    booted (staleTrace first second) ≠ inTree (staleTrace first second) := by
  rw [stale_package_boots_the_earlier_build, the_tree_holds_the_later_build]
  intro hx
  injection hx with hx
  exact h hx

/-! ### What boots, in general -/

/-- Packaging copies the tree as it stands. -/
theorem package_copies_the_tree (s : State) :
    (step s .package).image = s.tree := rfl

/-- Building does not touch the image, which is the whole mechanism of the
    defect: the step that changes the kernel is not the step that changes what
    boots. -/
theorem building_leaves_the_image (s : State) (d : Digest) :
    (step s (.build d)).image = s.image := rfl

/-- Booting observes and changes nothing. -/
theorem booting_changes_nothing (s : State) : step s .boot = s := rfl

/-- A trace with no package step boots nothing: there is no image to boot, which
    is the failure that is easy rather than the one that is silent. -/
theorem builds_never_touch_the_image (s : State) (ds : List Digest) :
    (run s (ds.map Step.build)).image = s.image := by
  induction ds generalizing s with
  | nil => rfl
  | cons d rest ih =>
    show (run (step s (.build d)) (rest.map Step.build)).image = s.image
    rw [ih (step s (.build d))]
    rfl

theorem no_package_boots_nothing (ds : List Digest) :
    booted (ds.map Step.build) = none := by
  unfold booted
  rw [builds_never_touch_the_image State.empty ds]
  rfl

/-! ### Sequencing -/

/-- Package immediately before boot. -/
def sequencedTrace (first second : Digest) : List Step :=
  [.build first, .build second, .package, .boot]

/-- Then what boots is what the tree holds. -/
theorem resequenced_boots_the_current_build (first second : Digest) :
    booted (sequencedTrace first second) = some second := rfl

/-- And the tree and the image agree, which is the property the recipe is
    enforcing by making the package a dependency of the run. -/
theorem sequenced_tree_and_image_agree (first second : Digest) :
    booted (sequencedTrace first second) = inTree (sequencedTrace first second) := rfl

/-- The general statement: a package step with no build after it puts the tree's
    kernel in the image, whatever came before. -/
theorem package_then_boot_is_current (s : State) :
    (run s [.package, .boot]).image = s.tree := rfl

/-! ### Measurement -/

/-- Measuring reads the image. -/
theorem measurement_follows_the_image (s : State) :
    (step s .measure).measured = s.image := rfl

/-- On the stale trace, the measurement is of the kernel that boots. It is
    correct and it is about the wrong kernel, so an attestation over it is true
    and misleading at once. -/
theorem stale_measurement_is_true_about_the_wrong_kernel (first second : Digest) :
    measured (staleTrace first second ++ [.measure]) = some first ∧
      inTree (staleTrace first second ++ [.measure]) = some second := ⟨rfl, rfl⟩

/-- A measurement taken before the package is of nothing at all, so the order of
    measure and package is load-bearing in the other direction as well. -/
theorem measuring_before_packaging_measures_nothing (d : Digest) :
    measured [.build d, .measure] = none := rfl

/-- Measure after package and the measurement is of the booted bytes. This is the
    only order in which an attestation means what it says. -/
theorem measure_after_package_matches_the_boot (d : Digest) :
    measured [.build d, .package, .measure, .boot] =
      booted [.build d, .package, .measure, .boot] := rfl

/-- And it equals the tree, so all three agree. The three-way agreement is the
    invariant the recipe exists to maintain. -/
theorem all_three_agree_when_sequenced (d : Digest) :
    measured [.build d, .package, .measure, .boot] = some d ∧
      booted [.build d, .package, .measure, .boot] = some d ∧
      inTree [.build d, .package, .measure, .boot] = some d := ⟨rfl, rfl, rfl⟩

end Nonos.ImageStaging
