# Const Generics in GAL

This document describes the comprehensive const generics implementation for GAL, which provides compile-time guarantees and zero-cost abstractions for the actor model.

## Overview

Const generics in GAL allow you to parameterize types and functions with compile-time constant values. This enables:

1. **Compile-time guarantees** - Array bounds, buffer sizes, and resource limits are verified at compile time
2. **Zero-cost abstractions** - Generic code with const parameters has no runtime overhead
3. **Type safety** - Prevents buffer overflows, resource exhaustion, and other common errors
4. **Actor optimization** - Actor pools, message queues, and buffers can be sized optimally

## Syntax

### Const Parameters

Const parameters are declared using the `const` keyword followed by a name, type, and optional default value:

```gal
// Basic const parameter
actor Buffer<const SIZE: Int> { ... }

// With default value
actor Pool<const CAPACITY: Int = 10> { ... }

// With bounds
actor RateLimiter<const MAX_RATE: Int> 
where MAX_RATE > 0, MAX_RATE <= 1000
{ ... }

// Multiple const parameters
actor Matrix<const ROWS: Int, const COLS: Int>
where ROWS > 0, COLS > 0, ROWS <= 1000, COLS <= 1000
{ ... }
```

### Const Types

GAL supports the following const types:
- `Int` - Signed integers
- `Bool` - Boolean values
- `UInt` - Unsigned integers  
- `USize` - Size type (platform-dependent)

### Const Arrays

Arrays can be sized with const expressions:

```gal
// Fixed-size array
type Buffer<const N: Int> = [u8; N];

// Multi-dimensional arrays
type Matrix<const ROWS: Int, const COLS: Int> = [[Float; COLS]; ROWS];

// Nested const generics
type NestedBuffer<const OUTER: Int, const INNER: Int> = [[u8; INNER]; OUTER];
```

### Const Functions

Functions marked with `const fn` can be evaluated at compile time:

```gal
const fn factorial(n: Int) -> Int {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}

const fn max(a: Int, b: Int) -> Int {
    if a > b { a } else { b }
}

const fn is_power_of_2(n: Int) -> Bool {
    n > 0 && (n & (n - 1)) == 0
}
```

### Const Expressions

Const expressions are evaluated at compile time:

```gal
const BUFFER_SIZE: Int = factorial(5) + 20;  // 140
const POOL_CAPACITY: Int = max(10, min(100, 50));  // 50
const IS_POWER: Bool = is_power_of_2(256);  // true
```

## Where Clauses and Bounds

Const parameters can have bounds that are checked at compile time:

```gal
actor CircularBuffer<const SIZE: Int>
where SIZE > 0,                    // Must be positive
      SIZE & (SIZE - 1) == 0,      // Must be power of 2
      SIZE <= 4096                 // Upper bound
{
    state buffer: [Option<T>; SIZE]
    // ... implementation
}

fn process_batch<const N: Int>(items: [Item; N]) -> [ProcessedItem; N]
where N > 0,           // Must have at least one item
      N <= 1000        // Reasonable upper bound
{
    // ... implementation
}
```

## Actor Examples

### Rate Limiter

```gal
actor RateLimiter<const MAX_RATE: Int> 
where MAX_RATE > 0, MAX_RATE <= 1000
{
    state requests: RingBuffer<MAX_RATE>
    state current_count: Int = 0
    
    invariant current_count >= 0
    invariant current_count <= MAX_RATE
    
    on Request(r) => {
        if current_count < MAX_RATE {
            requests.push(r);
            current_count = current_count + 1;
            forward(r)
        } else {
            reply(ErrorResponse(429))
        }
    }
}
```

### Resource Pool

```gal
actor FixedPool<T, const SIZE: Int = 10> 
where SIZE > 0, SIZE <= 100
{
    state pool: [Option<T>; SIZE]
    state available: Int = SIZE
    
    invariant available >= 0
    invariant available <= SIZE
    
    on Acquire => {
        if available > 0 {
            for i in 0..SIZE {
                if pool[i].is_none() {
                    pool[i] = Some(create_resource());
                    available = available - 1;
                    reply(Ok(pool[i]))
                    return;
                }
            }
        }
        reply(Err(PoolExhausted))
    }
}
```

### Memory Pool

```gal
actor MemoryPool<const BLOCK_SIZE: Int, const BLOCK_COUNT: Int>
where BLOCK_SIZE >= 8,
      BLOCK_SIZE % 8 == 0,     // Alignment requirement
      BLOCK_COUNT > 0,
      BLOCK_COUNT <= 10000
{
    state memory: [u8; BLOCK_SIZE * BLOCK_COUNT]
    state free_blocks: [Bool; BLOCK_COUNT]
    state free_count: Int = BLOCK_COUNT
    
    on Allocate => {
        if free_count > 0 {
            for i in 0..BLOCK_COUNT {
                if free_blocks[i] {
                    free_blocks[i] = false;
                    free_count = free_count - 1;
                    let ptr = memory.as_ptr() + (i * BLOCK_SIZE);
                    reply(Ok(MemoryBlock { ptr, size: BLOCK_SIZE }))
                    return;
                }
            }
        }
        reply(Err(OutOfMemory))
    }
}
```

## Type System Integration

### Const Generic Types

The type system represents const generics as:

```rust
enum Type {
    // Const generic parameter
    ConstGeneric {
        name: String,
        ty: ConstType,
        value: Option<ConstValue>,
    },
    
    // Const-sized array
    ConstArray {
        element_type: Box<Type>,
        size: ConstValue,
    },
    
    // Generic type with const parameters
    ConstGenericType {
        base: Box<Type>,
        type_args: Vec<Type>,
        const_args: Vec<ConstValue>,
    },
}
```

### Const Values

Const values can be literals or expressions:

```rust
enum ConstValue {
    Int(i64),
    Bool(bool),
    UInt(u64),
    USize(usize),
    Symbol(String),
    BinaryOp { op: ConstBinaryOp, left: Box<ConstValue>, right: Box<ConstValue> },
    UnaryOp { op: ConstUnaryOp, operand: Box<ConstValue> },
    Call { function: String, args: Vec<ConstValue> },
    If { condition: Box<ConstValue>, then_val: Box<ConstValue>, else_val: Box<ConstValue> },
}
```

### Const Evaluator

The const evaluator performs compile-time evaluation:

```rust
impl ConstEvaluator {
    pub fn evaluate(&mut self, expr: &ConstExpression, env: &TypeEnvironment) -> Result<ConstValue, String>
    
    pub fn check_constraint(&mut self, constraint: &ConstConstraint, value: &ConstValue, env: &TypeEnvironment) -> Result<bool, String>
    
    pub fn evaluate_function_call(&mut self, function: &str, args: &[ConstValue], env: &TypeEnvironment) -> Result<ConstValue, String>
}
```

## Monomorphization

Const generic types are monomorphized at compile time, creating specialized versions for each unique combination of const arguments:

```gal
// This creates three distinct types:
let buffer1 = spawn CircularBuffer<String, 64>;   // CircularBuffer_String_64
let buffer2 = spawn CircularBuffer<String, 128>;  // CircularBuffer_String_128  
let buffer3 = spawn CircularBuffer<Int, 64>;      // CircularBuffer_Int_64
```

## Compile-time Optimizations

### Static Array Bounds Checking

Array accesses with const indices are bounds-checked at compile time:

```gal
fn safe_access<const SIZE: Int>(arr: [Int; SIZE], index: Int) -> Int
where index >= 0, index < SIZE  // Compile-time bounds check
{
    arr[index]  // No runtime bounds check needed
}
```

### Const Propagation

Const values are propagated through generic parameters:

```gal
const CAPACITY: Int = 256;
actor Buffer<const SIZE: Int = CAPACITY> { ... }  // SIZE = 256
```

### Memory Layout Optimization

Const generics enable optimal memory layout:

```gal
// Packed struct with known size
actor PackedData<const COUNT: Int> {
    state data: [u32; COUNT]  // Exactly COUNT * 4 bytes
    // No padding or dynamic allocation
}
```

## Error Handling

Const generic errors are caught at compile time:

```gal
// Compile-time error: SIZE must be positive
actor BadBuffer<const SIZE: Int = -1> { ... }  // ERROR

// Compile-time error: Array index out of bounds
const BAD_ACCESS: Int = [1, 2, 3][5];  // ERROR

// Compile-time error: Division by zero
const BAD_MATH: Int = 10 / 0;  // ERROR
```

## Performance Benefits

1. **Zero runtime overhead** - All const computations happen at compile time
2. **Optimal memory layout** - Fixed-size data structures with no dynamic allocation
3. **Inlined operations** - Array accesses and arithmetic become simple instructions
4. **Cache-friendly** - Known sizes enable better memory locality
5. **SIMD optimization** - Fixed-size arrays can use vectorized operations

## Comparison with Rust

GAL's const generics provide similar functionality to Rust's const generics but adapted for the actor model:

| Feature | Rust | GAL |
|---------|------|-----|
| Const parameters | `const N: usize` | `const N: Int` |
| Const arrays | `[T; N]` | `[T; N]` |
| Const functions | `const fn` | `const fn` |
| Where clauses | `where N: Const` | `where N > 0` |
| Actor integration | N/A | Full support |
| Compile-time evaluation | Limited | Full expression evaluation |

## Future Enhancements

1. **Const trait methods** - Trait methods that can be called in const contexts
2. **Const closures** - Closures that capture const values
3. **Const async** - Async functions with const parameters
4. **Template specialization** - Optimized implementations for specific const values
5. **Const reflection** - Compile-time introspection of const values

## Conclusion

Const generics in GAL provide powerful compile-time guarantees while maintaining zero runtime overhead. They enable safe, efficient actor systems with statically verified resource bounds, array sizes, and computational limits. This makes GAL ideal for systems programming where performance and safety are critical.