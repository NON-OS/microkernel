/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/

/-!
The small arithmetic gadgets, powers by repeated squaring and a few linear shapes. Soundness
is that each names the value it claims: the repeated-squaring powers equal the plain product
of that many factors, and the linear shapes are their multiples. The power identities hold up
to associativity and commutativity of multiplication, proven over the core library, so they
carry to the field the compiler targets.
-/

namespace Zkolang.Math

def sq (x : Int) : Int := x * x
def cube (x : Int) : Int := x * x * x
def pow4 (x : Int) : Int := sq (sq x)
def pow6 (x : Int) : Int := sq (cube x)
def pow8 (x : Int) : Int := sq (pow4 x)
def double (x : Int) : Int := x + x
def triple (x : Int) : Int := x + x + x

/-- Squaring the square is the fourth power. -/
theorem pow4_eq (x : Int) : pow4 x = x * x * x * x := by
  unfold pow4 sq; ac_rfl

/-- Squaring the cube is the sixth power. -/
theorem pow6_eq (x : Int) : pow6 x = x * x * x * x * x * x := by
  unfold pow6 sq cube; ac_rfl

/-- Squaring the fourth power is the eighth power. -/
theorem pow8_eq (x : Int) : pow8 x = x * x * x * x * x * x * x * x := by
  unfold pow8 pow4 sq; ac_rfl

/-- Doubling is multiplication by two. -/
theorem double_eq (x : Int) : double x = 2 * x := by
  unfold double; omega

/-- Tripling is multiplication by three. -/
theorem triple_eq (x : Int) : triple x = 3 * x := by
  unfold triple; omega

end Zkolang.Math
