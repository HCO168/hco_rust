# The trait structure for Numberoid

# Compare

## NaN

**IsNaN** 
(include a function identifying NaN)
* auto impl for T: NeverNaN (return false)

**MaybeNaN** 
(marker for types that can actually be NaN)

**NeverNaN** 
(marker for types that can't be NaN)
* in practice not overlapping with MaybeNaN

## Min/Max

**Min/Max: PartialOrd**
(min/max value of values)

# Constants

## Type Expressiveness

### Finite boundaries

**MinValue/MaxValue** 
(min/max finite value for this type)

**IsMinValue/IsMaxValue** 
(checks whether the value is the minimum or maximum value)

### Infinities

**HasPosInf/HasNegInf** 
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

### MulId

Signed:AddId