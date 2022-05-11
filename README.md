## Variables, Immutability and Constants

- Variables. By default, variables are immutable. <br>
`let x = 5;`

- Variables mutable. <br>
`let mut y = 5;`
`y = 6;`

- Shadowing <br>
`let x = 6;`

- Constants ( required data type, can't be changed, can't be shadowed) whatever scope <br>
`const AGE: i32 = 23;`

## Type of data
### Integers
- Signed
`i8, i16, i32, i64, i128, isize`

- Unsigned
`u8, u16, u32, u64, u128, usize`

### Floating
- `f32, f64`

### Boolean
- `bool`

### Character
- `char`
- Single quotes `''`

## Compound Types
### Tuple
- `let tuple = ('a', 32, true);`
- `let (x, y, z) = tuple;` assign each value of tuple
- `let value1 = tuple.0` get first value of tuple

### Array
- `[i125:5]` <type, size>
- `let array = [1, 2, 5]` fixed size

## String
- `let name = "kevin"`
- `let address: &'static str = "C 123"`
- `let lastName: String = "cuadros"`