/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/

/-!
The polynomial gadgets evaluate by Horner's method, folding the coefficients with one
multiply and add per degree. Soundness is that the folded form is the polynomial it names:
the quadratic and cubic Horner expressions equal their expanded sums of monomials, and the
linear interpolation hits its endpoints. The identities are integer ring identities, proven
over the core library with the distributive law, so they hold in the field the compiler
targets.
-/

namespace Zkolang.Poly

def line (m b x : Int) : Int := m * x + b
def quad (a b c x : Int) : Int := (a * x + b) * x + c
def cubic (a b c d x : Int) : Int := ((a * x + b) * x + c) * x + d
def lerp (a b t : Int) : Int := a + t * (b - a)

/-- The line gadget is its own expansion. -/
theorem line_horner (m b x : Int) : line m b x = m * x + b := rfl

/-- The quadratic Horner form is the expanded quadratic. -/
theorem quad_horner (a b c x : Int) : quad a b c x = a * x * x + b * x + c := by
  unfold quad; rw [Int.add_mul]

/-- The cubic Horner form is the expanded cubic. -/
theorem cubic_horner (a b c d x : Int) :
    cubic a b c d x = a * x * x * x + b * x * x + c * x + d := by
  unfold cubic; rw [Int.add_mul, Int.add_mul, Int.add_mul]

/-- Interpolation returns the first endpoint at zero. -/
theorem lerp_zero (a b : Int) : lerp a b 0 = a := by
  unfold lerp; omega

/-- Interpolation returns the second endpoint at one. -/
theorem lerp_one (a b : Int) : lerp a b 1 = b := by
  unfold lerp; omega

end Zkolang.Poly
