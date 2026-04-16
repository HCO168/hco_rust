# The trait structure for Numberoid

# Compare
(mod: cmp)

## NonCmp

### NonCmpPair

**IsNonCmpPair: PartialOrd**
(includes a function identifying non-comparable values)
* auto impl for T: NeverNonCmpPair
* auto impl for T: MaybeNonCmpPair

**MaybeNonCmpPair: PartialEq**
(marker for types that can actually have non-comparable values)

**NeverNonCmpPair: Eq**
(marker for types that do not have non-comparable values)
* in practice not overlapping with MaybeNonCmpPair

### NonCmpValue

**IsNonCmpValue: PartialEq**
(includes a function identifying single non-comparable value)
* auto impl for T: NeverNonCmpValue
* auto impl for T: MaybeNonCmpValue

**MaybeNonCmpValue: PartialEq**
(marker for types that can actually be non-comparable)

**NeverNonCmpValue: Eq**
(marker for types that can't be non-comparable)
* in practice not overlapping with MaybeNonCmpValue

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
(mod: constants)

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
(mod: additive)

**AddId**
(identity for addition, aka zero)

### MulId
(mod: multiplicative)

**MulId**
(identity for multiplication, aka one)

# Signs
(mod sign)

## Check Sign

**Struct:Sign**
(enum representing the sign of a number)

**HasPartialSign: PartialOrd**
(provides the sign of a number, return None if non cmp to zero)

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
(mod categories)

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

# NaN
(mod: nan)

## CheckNaN

**IsNaN**
(include a function identifying NaN)
* auto impl for T: NeverNaN
* auto impl for T: MaybeNaN

## Marker traits

**MaybeNaN**
(trait for types that can actually be NaN)

**NeverNaN**
(marker for types that can't be NaN)
* in practice not overlapping with MaybeNaN

## NaN Constants
 
**NaN**
(constant for NaN)
