# The trait structure for Numberoid

# Compare
(mod: cmp)

## NonCmp

### NonCmpPair

**IsCmpPair: PartialOrd**
(provide function for identifying comparable or non-comparable values)
* fn
  * fn is_non_cmp_pair(&self, &other) -> bool
  * fn is_cmp_pair(&self, &other) -> bool
* default blanket impl for :
  * T: MaybeCmpPair
  * T: AlwaysCmpPair (Always return true for is_cmp_pair)

**MaybeCmpPair: PartialOrd**
(marker for types that can actually have comparable or non-comparable value)

**AlwaysCmpPair: Ord**
(marker for types that do not have non-comparable values)
* in practice not overlapping with MaybeNonCmpPair

### NonCmpValue

**IsNonCmpValue: PartialEq**
(provide function for identifying single non-comparable values)
* fn
  * fn is_non_cmp_value(&self) -> bool
  * fn is_cmp_value(&self) -> bool
* default blanket impl for:
  * T: PartialEq

**MaybeNonCmpValue: PartialEq**
(marker for types that can actually be non-comparable values)
* in practice not overlapping with NeverNonCmpValue

**NeverNonCmpValue: Eq**
(marker for types that cannot be non-comparable values)
* default blanket impl for:
  * T: Eq
* in practice not overlapping with MaybeNonCmpValue

## Min/Max

**MinMax: PartialOrd + IsNonCmpValue**
(provide min/max/clamp with non-comparable-value handling)
* fn
  * // ignore NonCmpValues
  * fn min_num(self, other: Self) -> Self
  * fn max_num(self, other: Self) -> Self
  * fn clamp_num(self, min: Self, max: Self) -> Self
  * // pass NonCmpValues
  * fn minimum(self, other: Self) -> Self
  * fn maximum(self, other: Self) -> Self
  * fn clamp(self, min: Self, max: Self) -> Self
* impl for:
  * Option<T: MinMax>
  * all primitive types that implement PartialOrd
  * references to primitive values

# Constants
(mod: consts)

## Finite boundaries

### MinValue/MaxValue

**MinValue**
(minimum value for this type)
* const
  * const MIN: Self

**MaxValue**
(maximum value for this type)
* const
  * const MAX: Self

**IsMinValue: MinValue + PartialEq**
(provide function for identifying the minimum value)
* fn
  * fn is_min_value(&self) -> bool
  * fn not_min_value(&self) -> bool
* default blanket impl for:
  * T: MinValue + PartialEq

**IsMaxValue: MaxValue + PartialEq**
(provide function for identifying the maximum value)
* fn
  * fn is_max_value(&self) -> bool
* default blanket impl for:
  * T: MaxValue + PartialEq

### MinFiniteValue/MaxFiniteValue

**MinFiniteValue**
(minimum finite value for this type)
* const
  * const MIN_FINITE_VALUE: Self
* default blanket impl for:
  * T: MinValue

**MaxFiniteValue**
(maximum finite value for this type)
* const
  * const MAX_FINITE_VALUE: Self
* default blanket impl for:
  * T: MaxValue

**IsMinFiniteValue: MinFiniteValue + PartialEq**
(provide function for identifying the minimum finite value)
* fn
  * fn is_min_finite_value(&self) -> bool
* default blanket impl for:
  * T: MinFiniteValue + PartialEq

**IsMaxFiniteValue: MaxFiniteValue + PartialEq**
(provide function for identifying the maximum finite value)
* fn
  * fn is_max_finite_value(&self) -> bool
* default blanket impl for:
  * T: MaxFiniteValue + PartialEq

## Infinities

**PosInf**
(positive infinity constant for this type)
* const
  * const POS_INF: Self

**NegInf**
(negative infinity constant for this type)
* const
  * const NEG_INF: Self

**IsPosInf**
(provide function for identifying positive infinity)
* fn
  * fn is_pos_inf(&self) -> bool
  * fn not_pos_inf(&self) -> bool
* default blanket impl for:
  * T: PartialEq + PosInf

**IsNegInf**
(provide function for identifying negative infinity)
* fn
  * fn is_neg_inf(&self) -> bool
  * fn not_neg_inf(&self) -> bool
* default blanket impl for:
  * T: PartialEq + NegInf

**IsInf: IsPosInf + IsNegInf**
(provide function for identifying either positive or negative infinity)
* fn
  * fn is_inf(&self) -> bool
  * fn not_inf(&self) -> bool
* default blanket impl for:
  * T: IsPosInf + IsNegInf

<!-- 
## Value closest to zero

**MinPositiveValue**
(smallest positive value for this type)
* const
  * const MIN_POSITIVE: Self

**MaxNegativeValue**
(largest negative value for this type)
* const
  * const MAX_NEGATIVE: Self

**IsMinPositiveValue**
(provide function for identifying the smallest positive value)
* fn
  * fn is_min_positive_value(&self) -> bool
* default blanket impl for:
  * T: PartialEq + MinPositiveValue

**IsMaxNegativeValue**
(provide function for identifying the largest negative value)
* fn
  * fn is_max_negative_value(&self) -> bool
* default blanket impl for:
  * T: PartialEq + MaxNegativeValue
-->

# Arithmetic Constants

## AddId
(mod: add)

**AddId**
(constant for additive identity)
* const
  * const ZERO: Self

**IsAddId: AddId + PartialEq**
(provide function for identifying additive identity)
* fn
  * fn is_zero(&self) -> bool
  * fn not_zero(&self) -> bool
* explicit impl for:
  * primitive numeric types
  * custom numeric types that want default or optimized zero checks

## MulId
(mod: mul)

**MulId**
(multiplicative identity, aka one)
* const
  * const ONE: Self

**NegMulId**
(negative multiplicative identity, aka negative one)
* const
  * const NEG_ONE: Self

**Recip**
(reciprocal operation)
* fn
  * fn recip(self) -> Self
* impl for:
  * float primitives

# Signs
(mod: sign)

## Sign value

**Struct: Sign**
(enum representing the sign of a number)
* values
  * Negative
  * Zero
  * Positive
* conversion impl:
  * From<Ordering> for Sign
  * From<Sign> for Ordering
  * From<Sign> for i8
  * From<i8> for Sign

## Check Sign

**HasPartialSign: PartialOrd + IsAddId**
(provide sign information, returning None if the value is non-comparable to zero)
* fn
  * fn partial_sign(&self) -> Option<Sign>
  * fn is_positive(&self) -> bool
  * fn not_positive(&self) -> bool
  * fn is_negative(&self) -> bool
  * fn not_negative(&self) -> bool
  * fn is_positive_or_zero(&self) -> bool
  * fn not_positive_or_zero(&self) -> bool
  * fn is_negative_or_zero(&self) -> bool
  * fn not_negative_or_zero(&self) -> bool
  * fn is_positive_or_negative(&self) -> bool
  * fn not_positive_or_negative(&self) -> bool
* default blanket impl for:
  * T: PartialOrd + AddId + IsAddId

**HasSign: HasPartialSign + Ord**
(provide total sign information)
* fn
  * fn sign(&self) -> Sign
* default blanket impl for:
  * T: HasPartialSign + Ord + AddId

**Signum: PartialOrd**
(provide the sign as a value of the same type)
* fn
  * fn signum(&self) -> Self
* impl for:
  * signed primitive numeric types
  * unsigned primitive numeric types

## Functions

**Abs**
(absolute value operation)
* associated type
  * type Output
* fn
  * fn abs(self) -> Self::Output

## Marker traits

**Signed: Neg**
(marker for signed numbers)
* in practice not overlapping with Unsigned

**Unsigned**
(marker for unsigned numbers)
* in practice not overlapping with Signed

# Integer/Float
(mod: int)

## Marker traits

**Integer**
(marker for integer types)
* in practice not overlapping with NonInteger

**NonInteger**
(marker for non-integer numeric types)
* in practice not overlapping with Integer

## Integer operations

**IntDiv: Integer**
(integer division)
* fn
  * fn int_div(self, rhs: Self) -> Self
* default blanket impl for:
  * T: Integer + Div<Output = T>

**Inc: Integer**
(increment by one)
* fn
  * fn inc(self) -> Self
* default blanket impl for:
  * T: Integer + MulId + Add<Output = T>

**Dec: Integer**
(decrement by one)
* fn
  * fn dec(self) -> Self
* default blanket impl for:
  * T: Integer + MulId + Sub<Output = T>

**TrailingZeros**
(count trailing zero bits)
* fn
  * fn trailing_zeros(self) -> usize
* impl for:
  * integer primitives

**LeadingZeros**
(count leading zero bits)
* fn
  * fn leading_zeros(self) -> usize
* impl for:
  * integer primitives

**GCD**
(greatest common divisor operation)
* fn
  * fn gcd(self, other: Self) -> Self
* impl for:
  * integer primitives
  * float primitives
* specialization impl:
  * default fn gcd(self, other: Self) -> Self for compatible numeric types

# NaN
(mod: nan)

## Check NaN

**IsNaN**
(provide function for identifying NaN)
* fn
  * fn is_nan(&self) -> bool
  * fn not_nan(&self) -> bool
* impl for:
  * f32
  * f64

## Marker traits

**MaybeNaN**
(marker for types that can actually be NaN)
* in practice not overlapping with NeverNaN

**NeverNaN**
(marker for types that cannot be NaN)
* impl for:
  * primitive integer types
* in practice not overlapping with MaybeNaN

## NaN constant

**NaN**
(NaN constant for this type)
* const
  * const NAN: Self