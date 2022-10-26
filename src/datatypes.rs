pub mod datatypes {
    pub fn datatypes_section() {}
    // Scalar Types
    // A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters
    // Integers
    // Signed integers are represented with the i prefix and unsigned integers are represented with the u prefix
    // The number following the prefix indicates the number of bits the integer takes up
    // The default integer type is i32
    // The isize and usize types depend on the kind of computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture
    // Integer Overflow
    // Integer overflow is a common error that occurs when a value goes beyond the range that the type can hold
    // Rust has some interesting defaults regarding integer overflow
    // By default, Rust performs two’s complement wrapping
    // Two’s complement wrapping means that when a value exceeds the maximum value that the type can hold, it “wraps around” to the minimum value of the type
    // For example, 255 + 1 = 0
    // Rust also has checked and unchecked integer arithmetic
    // Checked integer arithmetic causes an error if an overflow occurs
    // Unchecked integer arithmetic does not cause an error if an overflow occurs
    // Rust has the following integer types:
    // i8, u8
    // i16, u16
    // i32, u32
    // i64, u64
    // i128, u128
    // isize, usize
    // Floating-Point Types
    // Rust has two primitive types for floating-point numbers: f32 and f64
    // The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision
    // Numeric Operations
    // Rust supports all of the basic mathematical operations you’d expect for all of the number types
    // Rust also has a few extra operators specific to numbers
    // The remainder operator (%) divides left-hand operand by right-hand operand and returns the remainder
    // The boolean type is a single value with two possible values: true and false
    // The boolean type is specified using the bool keyword
    // The character type is specified using the char keyword
    // The char type represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII
    // Rust’s char type is four bytes in size and represents a Unicode Scalar Value
    // Compound Types
    // Compound types can group multiple values into one type
    // Rust has two primitive compound types: tuples and arrays
    // Tuples
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size
    // Tuples are useful when you want to return multiple values from a function
    // Tuples are also useful when you want to pass a bunch of values into a function that takes a single parameter
    // Tuples are created using parentheses
    // Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same
    // Tuples can be destructured to create bindings to their individual pieces
    // Arrays
    // Arrays are similar to tuples, but every element of an array must have the same type
    // Arrays in Rust are different from arrays in some other languages because arrays in Rust have a fixed length, like tuples
    // Arrays are useful when you want your data allocated on the stack rather than the heap
    // Arrays are created using square brackets
    // Arrays are useful when you want to ensure you always have a fixed number of elements
}
