# GAL Language Reference

This document provides a complete specification of the GAL (Gödelian Actor Language) syntax, semantics, and standard library. GAL is designed for building resilient, distributed systems with built-in chaos engineering and formal verification capabilities.

## Table of Contents

1. [Language Overview](#language-overview)
2. [Lexical Structure](#lexical-structure)
3. [Type System](#type-system)
4. [Actor Model](#actor-model)
5. [Message Passing](#message-passing)
6. [Expressions](#expressions)
7. [Statements](#statements)
8. [Chaos Engineering](#chaos-engineering)
9. [Formal Verification](#formal-verification)
10. [Gödelian Features](#gödelian-features)
11. [Standard Library](#standard-library)
12. [Package System](#package-system)

## Language Overview

GAL is a compiled, statically-typed language that combines:

- **Actor-based concurrency** for scalable parallel computing
- **Memory safety** through ownership and borrowing
- **Chaos engineering** for systematic reliability testing
- **Formal verification** for proving program correctness
- **Self-modification** for adaptive and self-optimizing systems

### Design Principles

1. **Safety First**: Memory safety and type safety are non-negotiable
2. **Performance by Design**: Zero-cost abstractions with aggressive optimization
3. **Resilience Native**: Fault tolerance is built into the language, not bolted on
4. **Verifiable Correctness**: Programs can carry proofs of their correctness

## Lexical Structure

### Identifiers

Identifiers follow Unicode standards and may contain letters, digits, and underscores:

```gal
// Valid identifiers
actor_name
UserSession
_private_var
counter1
validateInput
π  // Unicode identifiers are supported
```

### Keywords

GAL reserves the following keywords:

```gal
// Core language
actor     on        new       spawn     send      reply
let       mut       if        else      match     while
for       loop      break     continue  return    
true      false     null      
Int       Float     String    Bool      Array     Tuple

// Types and verification
type      trait     impl      where     requires  ensures
invariant assume    assert    verify    proof     contract

// Chaos engineering
chaos     fault     inject    @chaos_test    @chaos_contract
timeout   receive   select    supervise      restart

// Meta-programming
reflect   compile   optimize  transform      @godel
ast       meta      quote     unquote        eval

// Imports and modules
import    export    module    package        use
```

### Literals

#### Integer Literals
```gal
42          // Decimal
0x2A        // Hexadecimal
0o52        // Octal
0b101010    // Binary
1_000_000   // Underscores for readability
```

#### Float Literals
```gal
3.14
2.5e10
1.23e-4
6.022e23
```

#### String Literals
```gal
"Hello, World!"
"Unicode: αβγ 你好 🚀"
"Escape sequences: \n\t\r\\"
r"Raw string with \n literal backslashes"

// Multi-line strings
"""
This is a multi-line string
that preserves formatting
and line breaks.
"""
```

#### Boolean Literals
```gal
true
false
```

### Comments

```gal
// Line comment

/* Block comment */

/*
 * Multi-line block comment
 * with documentation
 */

/// Documentation comment for the following item
actor DocumentedActor {
    // ...
}
```

### Operators

#### Arithmetic Operators
```gal
+  -  *  /  %    // Add, subtract, multiply, divide, modulo
** // Power
```

#### Comparison Operators
```gal
==  !=  <  >  <=  >=
```

#### Logical Operators
```gal
&&  ||  !        // And, or, not
```

#### Bitwise Operators
```gal
&  |  ^  <<  >>  ~
```

#### Assignment Operators
```gal
=  +=  -=  *=  /=  %=
&=  |=  ^=  <<= >>=
```

#### Message Operators
```gal
->     // Send message (async)
<-     // Receive message
=>     // Message handler arrow
@      // Actor reference
```

## Type System

GAL uses a sophisticated type system that combines static typing with runtime contracts and chaos annotations.

### Primitive Types

```gal
Int       // 64-bit signed integer
Float     // 64-bit floating point
String    // UTF-8 string
Bool      // Boolean value
```

### Composite Types

#### Arrays
```gal
Array<Int>        // Dynamic array of integers
Array<String>     // Array of strings
[Int; 10]         // Fixed-size array of 10 integers

// Array literals
let numbers = [1, 2, 3, 4, 5]
let matrix = [[1, 2], [3, 4]]
```

#### Tuples
```gal
(Int, String)           // Tuple with integer and string
(Bool, Float, String)   // Three-element tuple
()                      // Unit type (empty tuple)

// Tuple literals
let point = (10, 20)
let person = ("Alice", 25, true)
```

#### Options
```gal
Option<Int>       // May contain an integer or be empty

let some_value = Some(42)
let no_value = None
```

#### Results
```gal
Result<Int, String>    // Success with Int or failure with String

let success = Ok(100)
let failure = Err("Something went wrong")
```

### Function Types

```gal
Fn(Int, Int) -> Int           // Function taking two Ints, returning Int
Fn() -> String                // Function with no parameters
Fn(String) -> ()              // Function returning unit

// Function with chaos annotation
ChaosFn<[MessageDrop]>(Int) -> Result<Int, Error>
```

### Actor Types

```gal
ActorRef<MessageType>         // Reference to an actor
ActorState<StateFields>       // Actor's internal state

// Actor with verification contracts
VerifiedActor<Messages, Invariants>

// Actor with chaos testing
ChaosActor<Messages, FaultModel>
```

### Type Aliases

```gal
type UserId = Int
type UserName = String
type Point = (Float, Float)

type Counter = actor {
    state count: Int
    
    on Increment -> ()
    on GetValue -> Int
}
```

### Algebraic Data Types

```gal
// Enumeration
type Status = Active | Inactive | Pending

// Sum types with data
type Message = 
    | Text(String)
    | Image(String, Int, Int)  // path, width, height
    | System(SystemMessage)

// Pattern matching
match message {
    Text(content) => println("Text: " + content)
    Image(path, w, h) => println("Image: " + path + " (" + w + "x" + h + ")")
    System(sys_msg) => handle_system_message(sys_msg)
}
```

### Generic Types

```gal
type Stack<T> = actor {
    state items: Array<T>
    
    on Push(item: T) -> ()
    on Pop -> Option<T>
}

// Generic function
fn map<T, U>(array: Array<T>, f: Fn(T) -> U) -> Array<U> {
    let result = Array<U>::new()
    for item in array {
        result.push(f(item))
    }
    result
}
```

### Constraints and Bounds

```gal
// Trait bounds
fn sort<T>(array: Array<T>) -> Array<T> 
    where T: Comparable
{
    // Sort implementation
}

// Multiple bounds
fn process<T>(data: T) -> String
    where T: Serializable + Debuggable
{
    data.serialize()
}
```

## Actor Model

Actors are the fundamental unit of computation in GAL. They encapsulate state and behavior, communicating only through message passing.

### Actor Declaration

```gal
actor Counter {
    // State declaration
    state count: Int = 0
    state max_value: Int
    
    // Constructor
    new create(initial: Int, max: Int) =>
        count = initial
        max_value = max
    
    // Message handlers
    on Increment =>
        if count < max_value {
            count += 1
            reply(count)
        } else {
            reply(Error("Maximum value reached"))
        }
    
    on Decrement =>
        if count > 0 {
            count -= 1
            reply(count)
        } else {
            reply(Error("Cannot decrement below zero"))
        }
    
    on GetValue => reply(count)
    
    on Reset => 
        count = 0
        reply(())
}
```

### Actor State

Actor state is private and can only be accessed by the actor itself:

```gal
actor BankAccount {
    // Private state
    state balance: Float = 0.0
    state account_id: String
    state transaction_history: Array<Transaction>
    
    // Computed properties
    property last_transaction: Option<Transaction> =>
        transaction_history.last()
    
    // State invariants
    invariant balance >= 0.0
    invariant account_id.length() > 0
}
```

### Message Handlers

Message handlers define how actors respond to different message types:

```gal
actor DatabaseManager {
    state connections: Array<Connection>
    
    // Simple message handler
    on GetConnection => reply(connections.next_available())
    
    // Handler with pattern matching
    on Query(sql: String, params: Array<Value>) =>
        match validate_query(sql) {
            Ok(validated) => {
                let result = execute_query(validated, params)
                reply(Ok(result))
            }
            Err(error) => reply(Err(error))
        }
    
    // Handler with guards
    on Shutdown when connections.all_idle() =>
        for conn in connections {
            conn.close()
        }
        reply(ShutdownComplete)
    
    // Timeout handler
    on Heartbeat timeout 30s =>
        check_connection_health()
}
```

### Actor Lifecycle

```gal
actor ServiceActor {
    // Called when actor is created
    on Initialize(config: Config) =>
        setup_resources(config)
        send(self, Ready)
    
    // Normal operation
    on Ready =>
        println("Service is ready")
        become_state(Running)
    
    // State transition
    on Shutdown =>
        cleanup_resources()
        become_state(Terminated)
    
    // Error handling
    on Error(error: ActorError) =>
        log_error(error)
        if error.is_recoverable() {
            restart_with_backoff()
        } else {
            escalate(error)
        }
}
```

## Message Passing

GAL uses asynchronous message passing as the primary communication mechanism between actors.

### Sending Messages

```gal
// Basic message sending
send(actor_ref, Message)

// Send with reply expectation
let response = ask(actor_ref, Query("SELECT * FROM users"))

// Send to multiple actors
broadcast([actor1, actor2, actor3], Notification("System update"))

// Send with timeout
try {
    let result = ask(remote_actor, ComplexQuery) timeout 5s
    handle_result(result)
} catch TimeoutError {
    handle_timeout()
}
```

### Receiving Messages

```gal
actor MessageProcessor {
    on ProcessMessage(msg: String) =>
        let processed = process(msg)
        reply(processed)
    
    // Pattern matching on message content
    on HandleRequest(request) =>
        match request {
            UserRequest(user_id, action) => handle_user_request(user_id, action)
            SystemRequest(priority, data) => handle_system_request(priority, data)
            _ => reply(Error("Unknown request type"))
        }
    
    // Selective receive
    on ComplexOperation =>
        select {
            SubResult1(data) => {
                // Handle first sub-result
                continue_operation(data)
            }
            SubResult2(data) => {
                // Handle second sub-result
                finalize_operation(data)
            }
            Error(err) => {
                // Handle error
                rollback_operation(err)
            }
        } timeout 10s => {
            // Handle timeout
            timeout_recovery()
        }
}
```

### Message Types

```gal
// Simple message types
type Increment = ()
type GetValue = ()

// Messages with data
type SetValue = (Int)
type UpdateUser = (UserId, UserData)

// Complex message types
type DatabaseQuery = {
    sql: String,
    parameters: Array<Value>,
    timeout: Duration,
    callback: ActorRef<QueryResult>
}

// System messages
type SystemMessage = 
    | Shutdown
    | Restart(Reason)
    | Supervisor(SupervisorMessage)
```

## Expressions

### Basic Expressions

```gal
// Literals
42
3.14
"Hello"
true

// Variables
my_variable
counter_value

// Binary operations
a + b
x * y + z
left && right

// Function calls
calculate(10, 20)
process_data(input, config)

// Method calls
array.length()
string.substring(0, 5)
```

### Control Flow Expressions

#### If Expressions
```gal
let result = if condition {
    "true branch"
} else {
    "false branch"
}

// If-let for pattern matching
if let Some(value) = optional_value {
    process(value)
} else {
    handle_none()
}
```

#### Match Expressions
```gal
let description = match status {
    Active => "Currently active"
    Inactive => "Not active"
    Pending => "Waiting for activation"
}

// Match with guards
match number {
    n when n > 0 => "Positive"
    n when n < 0 => "Negative" 
    _ => "Zero"
}

// Complex pattern matching
match request {
    UserRequest { user_id, action: Login } => handle_login(user_id)
    UserRequest { user_id, action: Logout } => handle_logout(user_id)
    SystemRequest { priority: High, .. } => handle_high_priority()
    _ => handle_other()
}
```

### Loop Expressions

```gal
// For loop
for item in collection {
    process(item)
}

// For loop with index
for (index, item) in collection.enumerate() {
    println(index + ": " + item)
}

// While loop
while condition {
    perform_action()
    update_condition()
}

// Infinite loop with break
loop {
    let input = read_input()
    if input == "quit" {
        break
    }
    process(input)
}
```

### Array and Tuple Operations

```gal
// Array indexing
let first = array[0]
let last = array[array.length() - 1]

// Array slicing
let slice = array[1..5]      // Elements 1 through 4
let suffix = array[3..]      // From element 3 to end
let prefix = array[..3]      // First 3 elements

// Tuple access
let point = (10, 20)
let x = point.0
let y = point.1

// Tuple destructuring
let (x, y) = point
let (first, rest...) = tuple
```

## Statements

### Variable Declarations

```gal
// Immutable binding
let name = "Alice"
let age = 30

// Mutable binding
let mut counter = 0
counter += 1

// Type annotations
let value: Int = 42
let message: String = "Hello"

// Destructuring
let (x, y) = get_coordinates()
let [first, second, ...rest] = array
```

### Assignment

```gal
// Simple assignment
variable = new_value

// Compound assignment
counter += 1
balance *= 1.05
flags |= new_flag

// Tuple assignment
(x, y) = (y, x)  // Swap values
```

### Function Definitions

```gal
// Simple function
fn add(a: Int, b: Int) -> Int {
    a + b
}

// Function with contracts
fn divide(a: Int, b: Int) -> Int
    requires b != 0
    ensures result * b == a
{
    a / b
}

// Generic function
fn identity<T>(value: T) -> T {
    value
}

// Function with chaos annotation
@chaos_test(faults: [Delay(100ms)])
fn network_call(url: String) -> Result<String, Error> {
    http_get(url)
}
```

### Actor Spawning

```gal
// Spawn actor with constructor arguments
let counter = spawn Counter(0, 100)

// Spawn with supervisor
let db_actor = spawn_supervised(db_supervisor, DatabaseActor(config))

// Spawn remote actor
let remote_actor = spawn_remote("worker-node-1", WorkerActor)

// Spawn with chaos configuration
let chaos_actor = spawn_with_chaos(
    TestActor,
    ChaosConfig {
        message_drop_rate: 0.1,
        crash_probability: 0.05
    }
)
```

## Chaos Engineering

GAL provides first-class support for chaos engineering, allowing developers to systematically test system resilience.

### Chaos Annotations

```gal
// Basic chaos test
@chaos_test(faults: [MessageDrop(0.1)])
actor TestActor {
    on ProcessMessage(msg) => 
        // This message handling will randomly drop 10% of messages
        handle_message(msg)
}

// Multiple fault types
@chaos_test(faults: [
    MessageDrop(0.05),
    MessageDelay(100ms..500ms),
    ActorCrash(0.01)
])
actor ResilientService {
    // Implementation
}

// Chaos with conditions
@chaos_test(
    faults: [NetworkPartition(5s)],
    condition: "system_load > 0.8"
)
actor DistributedCache {
    // Implementation
}
```

### Fault Injection

```gal
// Programmatic fault injection
chaos.inject_fault(MessageDrop(0.2))
chaos.inject_fault(ActorCrash, target: specific_actor)

// Conditional fault injection
if system_stress_test_mode {
    chaos.enable([
        MessageDelay(50ms..200ms),
        MemoryPressure(80_percent),
        ThreadStarvation(2)
    ])
}

// Scheduled faults
chaos.schedule_fault(
    NetworkPartition(duration: 10s),
    at: now() + 2_minutes
)
```

### Chaos Contracts

```gal
@chaos_contract
property eventual_consistency {
    given replicas: Vec<ActorRef> = arbitrary(3..10)
    given updates: Vec<Update> = arbitrary_updates(50)
    
    when {
        // Apply chaos conditions
        chaos.apply([
            MessageDelay(0ms..1000ms),
            NetworkPartition(probability: 0.1, duration: 2s)
        ])
        
        // Apply updates to random replicas
        for update in updates {
            let replica = random_choice(replicas)
            send(replica, update)
        }
        
        // Wait for system to stabilize
        chaos.wait_for_quiescence(timeout: 30s)
    }
    
    then {
        // All replicas should converge to the same state
        let states = replicas.map(|r| ask(r, GetState))
        all_equal(states)
    }
}
```

### Fault Models

```gal
// Define custom fault models
fault_model DatabaseFailures {
    ConnectionTimeout(duration: Duration) => {
        delay: duration,
        affects: DatabaseOperations
    }
    
    QueryError(error_rate: Float) => {
        probability: error_rate,
        recovery: RetryWithBackoff(3, 1s, 2.0)
    }
    
    ConnectionLoss => {
        duration: Exponential(mean: 5s),
        recovery: Reconnect
    }
}

// Apply fault model to actor
@chaos_test(fault_model: DatabaseFailures)
actor DatabaseClient {
    // Implementation that will be tested with database failures
}
```

## Formal Verification

GAL integrates formal verification tools to enable mathematical proofs of program correctness.

### Contracts and Specifications

```gal
// Function contracts
fn factorial(n: Int) -> Int
    requires n >= 0
    ensures result >= 1
    ensures n == 0 implies result == 1
    ensures n > 0 implies result == n * factorial(n - 1)
{
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// Actor contracts
actor BankAccount {
    state balance: Float
    
    // State invariants
    invariant balance >= 0.0
    
    on Deposit(amount: Float)
        requires amount > 0.0
        ensures self.balance == old(self.balance) + amount
    =>
        balance += amount
    
    on Withdraw(amount: Float) -> Result<Float, Error>
        requires amount > 0.0
        ensures success implies self.balance == old(self.balance) - amount
        ensures failure implies self.balance == old(self.balance)
    =>
        if amount <= balance {
            balance -= amount
            Ok(balance)
        } else {
            Err("Insufficient funds")
        }
}
```

### Proof Annotations

```gal
fn binary_search<T>(array: Array<T>, target: T) -> Option<Int>
    where T: Comparable
    requires array.is_sorted()
    ensures result.is_some() implies array[result.unwrap()] == target
{
    let mut left = 0
    let mut right = array.length()
    
    while left < right {
        // Loop invariant
        assert!(left <= right)
        assert!(right <= array.length())
        
        let mid = left + (right - left) / 2
        
        match array[mid].compare(target) {
            Less => left = mid + 1,
            Greater => right = mid,
            Equal => return Some(mid)
        }
    }
    
    None
}
```

### Verification Directives

```gal
// Assume facts for verification
assume(database.is_connected())

// Assert conditions that must hold
assert!(balance >= 0.0)

// Verify a property holds
verify!(all_accounts_have_positive_balance())

// Proof hints for the theorem prover
lemma balance_preservation {
    forall account: BankAccount,
    forall operation: Operation =>
        operation.preserves_balance() implies
        account.balance_after(operation) == 
        account.balance_before(operation) + operation.delta()
}
```

## Gödelian Features

GAL supports self-reference and self-modification inspired by Gödel's work on incompleteness and self-reference.

### Reflection

```gal
actor SelfAwareActor {
    on AnalyzeSelf =>
        // Get the actor's own AST
        let my_ast = reflect(self)
        
        // Analyze structure
        let handler_count = my_ast.handlers.length()
        let state_vars = my_ast.state_fields.length()
        
        println("I have " + handler_count + " handlers")
        println("I have " + state_vars + " state variables")
        
        // Examine specific handlers
        for handler in my_ast.handlers {
            println("Handler: " + handler.pattern)
            println("Complexity: " + analyze_complexity(handler.body))
        }
}
```

### Code Generation

```gal
actor MetaProgrammer {
    on GenerateHandler(pattern: String, behavior: String) =>
        // Generate new handler AST
        let new_handler = ast {
            on #{pattern}(data) => #{behavior}(data)
        }
        
        // Add to self
        self.add_handler(new_handler)
        
        reply("Handler added successfully")
    
    on OptimizeHotPath(handler_name: String) =>
        // Get runtime statistics
        let stats = get_performance_stats(handler_name)
        
        if stats.is_hot_path() {
            // Generate optimized version
            let optimized = optimize_handler(
                self.get_handler(handler_name),
                stats
            )
            
            // Replace implementation
            self.replace_handler(handler_name, optimized)
        }
}
```

### Self-Modification

```gal
actor AdaptiveCache {
    state cache: HashMap<String, Value>
    state access_pattern: AccessTracker
    
    on Get(key: String) =>
        // Record access for self-optimization
        access_pattern.record_access(key)
        
        // Check if we should adapt
        if access_pattern.should_optimize() {
            self.optimize_based_on_pattern()
        }
        
        reply(cache.get(key))
    
    fn optimize_based_on_pattern(self) {
        let pattern = access_pattern.analyze()
        
        match pattern {
            Sequential => {
                // Add prefetching handler
                let prefetch_handler = ast {
                    on Get(key) => {
                        let result = cache.get(key)
                        // Prefetch next likely keys
                        prefetch_next_keys(key)
                        reply(result)
                    }
                }
                self.replace_handler("Get", prefetch_handler)
            }
            
            Random => {
                // Optimize for random access
                self.reorganize_cache_structure(RandomAccessOptimal)
            }
            
            Clustered => {
                // Group related items
                self.reorganize_cache_structure(ClusteredOptimal)
            }
        }
    }
}
```

### Gödel Numbers

```gal
// Encode programs as numbers for manipulation
actor GodelEncoder {
    on EncodeProgram(program: AST) -> Int =>
        let godel_number = encode_as_godel_number(program)
        reply(godel_number)
    
    on DecodeProgram(number: Int) -> AST =>
        let program = decode_godel_number(number)
        reply(program)
    
    on SelfEncode =>
        // Encode this very actor
        let my_code = reflect(self)
        let my_number = encode_as_godel_number(my_code)
        
        // Create a program that prints its own Gödel number
        let self_referential = ast {
            actor Quine {
                on Start => println(#{my_number})
            }
        }
        
        reply(self_referential)
}
```

## Standard Library

GAL provides a comprehensive standard library for common programming tasks.

### Core Types and Operations

```gal
// Array operations
let numbers = [1, 2, 3, 4, 5]
numbers.push(6)
numbers.pop()
numbers.map(|x| x * 2)
numbers.filter(|x| x > 3)
numbers.fold(0, |acc, x| acc + x)

// String operations
let text = "Hello, World!"
text.length()
text.substring(0, 5)
text.split(", ")
text.replace("World", "GAL")
text.to_uppercase()

// Option and Result types
let maybe_value = Some(42)
maybe_value.unwrap_or(0)
maybe_value.map(|x| x * 2)

let result = Ok(100)
result.unwrap_or_else(|err| handle_error(err))
```

### Collections

```gal
// HashMap
let mut map = HashMap<String, Int>::new()
map.insert("key", 42)
let value = map.get("key")

// Set
let mut set = HashSet<String>::new()
set.insert("item")
let contains = set.contains("item")

// Vector (dynamic array)
let mut vec = Vec<Int>::new()
vec.push(1)
vec.push(2)
vec.extend([3, 4, 5])
```

### Actors and Concurrency

```gal
// Actor utilities
let actor_ref = spawn(MyActor)
let response = ask(actor_ref, GetValue) timeout 5s

// Supervision
let supervisor = spawn(Supervisor::new(OneForOne))
let child = spawn_supervised(supervisor, ChildActor)

// Actor pools
let pool = ActorPool::new(WorkerActor, size: 10)
pool.route(WorkMessage("data"))

// Futures and async operations
let future = async {
    let result1 = ask(actor1, Query1).await
    let result2 = ask(actor2, Query2).await
    combine(result1, result2)
}
```

### I/O and Networking

```gal
// File I/O
let content = fs::read_to_string("file.txt")
fs::write("output.txt", "Hello, file!")

// HTTP client
let response = http::get("https://api.example.com/data")
let json = response.json()

// TCP server
let server = TcpListener::bind("127.0.0.1:8080")
for connection in server.incoming() {
    spawn(ConnectionHandler(connection))
}
```

### Serialization

```gal
// JSON serialization
let data = Person { name: "Alice", age: 30 }
let json = data.to_json()
let parsed = Person::from_json(json)

// Binary serialization
let bytes = data.to_bytes()
let restored = Person::from_bytes(bytes)
```

### Time and Duration

```gal
// Time handling
let now = Time::now()
let duration = Duration::from_seconds(30)
let future_time = now + duration

// Timeouts
timeout(5s) {
    expensive_operation()
}
```

### Logging and Diagnostics

```gal
// Logging
log::info("Application started")
log::error("An error occurred: {}", error)
log::debug("Debug information: {}", debug_data)

// Metrics
metrics::counter("requests_total").increment()
metrics::gauge("memory_usage").set(current_memory())
metrics::histogram("request_duration").observe(duration)
```

## Package System

GAL includes a built-in package manager for dependency management and code distribution.

### Package Manifest

```toml
# gal.toml
[package]
name = "my-package"
version = "1.0.0"
description = "A sample GAL package"
authors = ["Your Name <your.email@example.com>"]
license = "MIT"
repository = "https://github.com/user/my-package"

[dependencies]
actor-utils = "1.2.0"
chaos-testing = { version = "0.5.0", features = ["contracts"] }
http-client = "2.0.0"

[dev-dependencies]
test-framework = "1.0.0"

[chaos]
default-profile = "development"

[chaos.profiles.development]
message-drop-rate = 0.01
actor-crash-rate = 0.001

[chaos.profiles.testing] 
message-drop-rate = 0.1
actor-crash-rate = 0.05
network-partition-rate = 0.02

[build]
targets = ["x86_64-unknown-linux-gnu", "aarch64-apple-darwin"]
optimization-level = 2
```

### Import System

```gal
// Import from standard library
import std::collections::HashMap
import std::actors::{ActorPool, Supervisor}

// Import from external package
import actor_utils::MessageRouter
import chaos_testing::{ChaosConfig, FaultInjector}

// Import with alias
import very_long_package_name::SomeType as ShortType

// Re-export
export utils::helper_function
export {Type1, Type2} from internal_module
```

### Package Structure

```
my-package/
├── gal.toml              # Package manifest
├── src/
│   ├── lib.gal          # Library root
│   ├── actors/
│   │   ├── mod.gal      # Module declaration
│   │   ├── worker.gal   # Worker actor
│   │   └── manager.gal  # Manager actor
│   └── utils/
│       ├── mod.gal
│       └── helpers.gal
├── tests/
│   ├── integration.gal  # Integration tests
│   └── chaos.gal        # Chaos tests
├── examples/
│   └── demo.gal         # Example usage
├── docs/
│   └── README.md        # Documentation
└── benches/
    └── performance.gal  # Benchmarks
```

---

This completes the GAL Language Reference. The language is designed to make building resilient, distributed systems both safe and expressive, with unique features like built-in chaos engineering and formal verification that set it apart from other programming languages.