# The trait structure for Numberoid

# Compare

## NaN

**IsNaN** 
(include a function identifying NaN, the value where x != x)
* auto impl for T: NeverNaN
* auto impl for T: MaybeNaN

**MaybeNaN** 
(marker for types that can actually be NaN)
* fn is_nan(&self) -> bool, which will be forwarded to IsNaN
* auto impl is_nan for T: PartialEq

**NeverNaN** 
(marker for types that can't be NaN)
* in practice not overlapping with MaybeNaN
* fn is_nan(&self) -> bool, which will always return false

## Min/Max

**MinMax: PartialOrd**
(min and max value of values, with extra functions)
* default impl for T: PartialOrd
* new impl for T: Ord (simple override)

## MinValue/MaxValue

**MinValue/MaxValue**
(min/max value for this type, always 
less and equal/greater and equal than numbers of the same type)

**IsMinValue/IsMaxValue**
(checks whether the value is the minimum or maximum value)

# Constants

## Type Expressiveness

### Finite boundaries

**MinFiniteValue/MaxFiniteValue** 
(min/max finite value for this type)

**IsMinFiniteValue/IsMaxFiniteValue** 
(checks whether the value is the minimum or maximum value)

### Infinities

**PosInf/NegInf** 
(infinity constants for this type)

**IsPosInf/IsNegInf** 
(checks whether the value is positive or negative infinity)

### Value closest to zero
**MinPositiveValue/MaxNegativeValue** 
(constants closest to zero)

**IsMinPositiveValue/IsMaxNegativeValue** 
(checks whether the value is closest to zero)

## Arithmetic Constants

### AddId

**AddId**
(identity for addition, aka zero)

### MulId

**MulId**
(identity for multiplication, aka one)

# Signs

## Check Sign

**Struct:Sign**
(enum representing the sign of a number)

**HasPartialSign: PartialOrd**
(provides the sign of a number, return None if NaN)

**HasSign: HasPartialSign+Ord**
(provides the sign of a number)

**Signum: PartialOrd**
(provides the sign of a number in its own type)

## Functions

**Abs**
(absolute value of a number)

## Marker traits

**Signed: Neg**
(marker trait for signed numbers)
* in practice not overlapping with *Unsigned*

**Unsigned**
(marker trait for unsigned numbers)
* in practice not overlapping with *Signed*

# Integer/Float

## Marker traits
**Integer**
(marker trait for integer types)
* in practice not overlapping with *IsFloat*

**NonInteger**
(marker trait for non-integer types)
* in practice not overlapping with *IsInteger*

## Functions

**IntDiv**
(integer division)

**Inc**
(increment, where there does not exist a value **b** 
to make a++>**b**>a happening)

**Dec**
(decrement, similar to **Inc** but in reverse)


