# rustlang_journey

This repository documents my journey into learning and mastering rustlang.

It includes:

* Variables,
* Data Types
* Functions
* Structs and Enum
* ownerships and other programming paradigm in rust

## Variables and Immutability in rust
Variables are immutable by default in rust. Adding *mut* in front of your variable declaration makes it muttable.

#### Constants
They are values bound to a name and are not alllowed to change. They are immutable and *mut* is not allowed. The type & value must be initialize when declaring a variable.

#### Shadowing
Involves using the value of an initially declared variable to update the same variable. Shadowing is different from using *mut* becuase in it let's yoou change the variable type while maintaining the initial variable name

#### Data Types
Scalar types in rust includes:
* Integers: i8, i16, i32, i64, isize, u8, u16, u32, u64 & usize
* Float: f32 & f64
* Boolean: true & false
* Character

Compound types in rust
* Turples : used to group together number of different types
* Array: is another wy for storing colection of multiple values. All element in array in rust must have the same type, fixed length once declared.

#### Functions
block of reusable code
#### Control Flow
checking for conditions before executing a code
