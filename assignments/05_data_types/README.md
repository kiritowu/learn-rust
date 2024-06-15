# Data Types

- Rust is statistically-typed language (i.e. type MUST be specified during compile time)
- Type must be specified in event of type ambiguity (e.g. `let guess: u32 = "42".parse().expect("not a number");`)

## Static Types

### Integer Types

- In rust, there are two main attributes related to an integer type:
    - Signed (i) vs Unsigned (u)
        - Whether an integer can be negative (i) or always positive (u)
    - Datasize
        - 8-bit (8), 16-bit (16), 32-bit (32), 64-bit (64), 128-bit (128), arch (size)
        - Each signed variant can store integer from -(2^{n-1}) to 2^{n-1}-1
        - Each unsigned variante can store integer from 0 to 2^n-1
        - `size` variant decide datasize based on system architechture (i.e. 64-bit or 32-bit)
- Integer literal allow type suffix such as 57u8 (i.e. 57 as unsigned 8-bit integer)
- Integer literal allow `_` as visual seperator for easy reading such as 1_000 (i.e. 1000)
- Integer literal can be of different number bases:
    - Decimal (98_22), Hex (0xff), Octal (0o77)A, Binary (0b1111_0000), Byte, u8 only (b'A')

### Floating-Point Types

- Only two-variants: `f32` and `f64`. Both signed.

### Boolean Types

- `true` or `false`, one byte in size.

### Character Type

- Primitive alphabetic type.

## Compound Type

### Tuple Type

- Group a number of values of different type together
- Fixed length: Cannot grow or shrink in size after declaration.
- Written with comma-seperated list of values inside parantheses. (e.g. `(1,2,3)`)
- Support deconstruction of tuple by index. (e.g. `let (x,y,z) = (1,2,3);`)
- Support access of element using `.<index>` expression. (e.g. `let a0 = tup.0;`)
- Empty tuple (`()`) is called unit, and its usually returned by expression without any other value.

### Array type

- Fixed length, fixed type.
- Different from `vector` that grows or shrink in size.
- Data in continuous stack rather than heap. (Elaborated in Chapter 4)
- Declared using `[type; size]` type declaration. (e.g. `let a: [i32; 5] = [1,2,3,4,5];`)
- Initalized with same values using following declaration `[123; 5] == [123, 123, ...];`
- Accessed using square bracket notation `let first = a[0];`
- Rust's memory safety principles throw error if index is more than length. This prevent invalid memory access.
