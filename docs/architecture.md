# GAL Architecture Guide

This document provides a comprehensive overview of the GAL compiler and runtime architecture, designed for compiler developers, language researchers, and advanced users who need to understand the internal workings of GAL.

## Table of Contents

1. [Overview](#overview)
2. [Compiler Pipeline](#compiler-pipeline)
3. [Actor Runtime System](#actor-runtime-system)
4. [Chaos Engineering Infrastructure](#chaos-engineering-infrastructure)
5. [Gödelian Self-Modification Engine](#gödelian-self-modification-engine)
6. [Memory Management](#memory-management)
7. [Type System](#type-system)
8. [Code Generation](#code-generation)
9. [Package System](#package-system)
10. [IDE Integration](#ide-integration)

## Overview

GAL follows a multi-stage compilation pipeline that transforms source code through several intermediate representations before generating optimized native code. The architecture is designed around three core principles:

1. **Correctness First**: Every transformation preserves semantics and enables verification
2. **Performance by Design**: Zero-cost abstractions with aggressive optimization
3. **Resilience Native**: Chaos engineering and fault tolerance built into the language runtime

### High-Level Architecture

```
Source Code (.gal)
       ↓
    Lexer → Tokens
       ↓
    Parser → AST (Abstract Syntax Tree)
       ↓
 Semantic Analysis → Typed AST + Type Environment
       ↓
  IR Generation → GAL IR (Graph-based)
       ↓
  Optimization → Optimized IR
       ↓
  Code Generation → LLVM IR / Cranelift / C
       ↓
   Link + JIT → Native Code + Runtime
```

## Compiler Pipeline

### 1. Lexical Analysis

**Location**: `src/lexer.rs`

The lexer transforms source text into a stream of tokens using a finite state machine approach. GAL's lexer is implemented using the `logos` crate for performance and handles:

- **Keywords**: `actor`, `on`, `spawn`, `send`, `reply`, etc.
- **Operators**: Arithmetic, logical, comparison, and message operators
- **Literals**: Numbers, strings, booleans, and reflection literals
- **Identifiers**: Variable names, actor types, and message patterns
- **Comments**: Line and block comments with metadata preservation

```rust
// Token definition (simplified)
#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("actor")]
    Actor,
    
    #[token("on")]
    On,
    
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),
    
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Integer(i64),
    
    // Chaos engineering annotations
    #[token("@chaos_test")]
    ChaosTest,
    
    // Verification annotations
    #[token("invariant")]
    Invariant,
}
```

**Key Features**:
- **Position Tracking**: Maintains line/column information for diagnostics
- **Error Recovery**: Continues lexing after encountering invalid characters
- **Unicode Support**: Full UTF-8 support for identifiers and string literals
- **Annotation Processing**: Special handling for chaos and verification annotations

### 2. Parsing

**Location**: `src/parser.rs`

GAL uses a recursive descent parser with operator precedence parsing for expressions. The parser generates an Abstract Syntax Tree (AST) that preserves source location information for diagnostics.

```rust
// Core AST nodes
pub enum Item {
    Actor(ActorDecl),
    Function(FunctionDecl),
    Import(ImportDecl),
    TypeAlias(TypeAliasDecl),
}

pub struct ActorDecl {
    pub name: Identifier,
    pub annotations: Vec<Annotation>,
    pub state_fields: Vec<StateField>,
    pub handlers: Vec<MessageHandler>,
    pub constructor: Option<Constructor>,
    pub verification: Option<VerificationAnnotations>,
    pub span: Span,
}
```

**Parsing Strategy**:
- **Top-Down**: Recursive descent for statements and declarations
- **Bottom-Up**: Precedence climbing for expressions
- **Error Recovery**: Synchronization tokens for robust error handling
- **Incremental**: Designed for IDE integration with partial parsing

### 3. Semantic Analysis

**Location**: `src/semantic.rs`, `src/types.rs`

The semantic analyzer performs type checking, ownership analysis, and generates a typed AST. This phase ensures program correctness and enables advanced optimizations.

#### Type Checking

GAL implements a sophisticated type system with:

```gal
// Basic types
Int, Float, String, Bool

// Composite types
Array<T>, Tuple<T1, T2, ..., Tn>, Option<T>

// Actor types
ActorRef<MessageType>, ActorState<StateType>

// Function types
Fn(T1, T2) -> T3

// Verification types
Verified<T, Invariant>, Chaos<T, FaultModel>
```

#### Ownership Analysis

GAL uses a Rust-inspired ownership system adapted for the actor model:

- **Move Semantics**: Values are moved between actors by default
- **Borrowing**: Temporary references for reading state
- **Message Ownership**: Clear ownership transfer in message passing
- **Actor Isolation**: Each actor owns its state exclusively

```gal
actor DataProcessor {
    state data: Array<Int>
    
    on ProcessData(input: Array<Int>) =>  // input is moved
        data = transform(input)           // input consumed here
        
    on GetData => reply(&data)           // borrow for reading
}
```

### 4. IR Generation

**Location**: `src/ir.rs`

GAL IR is a graph-based intermediate representation optimized for actor systems and chaos engineering. It's designed to:

- **Model Concurrency**: Explicit representation of message flows and actor boundaries
- **Enable Optimization**: Control flow graphs for standard optimizations
- **Support Verification**: Formal semantics for proof generation
- **Chaos Integration**: Fault injection points identified in IR

```rust
pub struct Function {
    pub name: String,
    pub signature: FunctionSignature,
    pub basic_blocks: HashMap<BlockId, BasicBlock>,
    pub entry_block: BlockId,
    pub actor_context: Option<ActorContext>,
}

pub struct BasicBlock {
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
    pub chaos_points: Vec<ChaosPoint>,
}

pub enum Instruction {
    // Standard operations
    Assign { target: Register, value: Value },
    Call { result: Register, function: String, args: Vec<Value> },
    
    // Actor operations
    Spawn { result: Register, actor_type: String, args: Vec<Value> },
    Send { target: Register, message: Value },
    Receive { patterns: Vec<ReceivePattern> },
    
    // Chaos operations
    InjectFault { fault_type: FaultType, condition: Option<Value> },
    
    // Verification
    Assert { condition: Value, message: String },
    Assume { condition: Value },
}
```

## Actor Runtime System

**Location**: `src/runtime.rs`, `src/supervisor.rs`

The GAL runtime implements a high-performance actor system with built-in fault tolerance and chaos engineering capabilities.

### Actor Lifecycle

```
Created → Initialized → Running ⇌ Suspended → Terminated
            ↓              ↑         ↓
         Failed ←----------+    Restarting
```

### Core Components

#### 1. Actor Scheduler

The scheduler uses a work-stealing approach with priority queues:

```rust
pub struct ActorScheduler {
    /// Work-stealing queues for each OS thread
    worker_queues: Vec<WorkStealingQueue<ActorMessage>>,
    
    /// Global priority queue for high-priority messages
    priority_queue: PriorityQueue<ActorMessage>,
    
    /// Actor registry
    actors: DashMap<ActorId, ActorHandle>,
    
    /// Chaos engine integration
    chaos_engine: Option<Arc<ChaosEngine>>,
}
```

**Features**:
- **Work Stealing**: Automatic load balancing across threads
- **Priority Scheduling**: System messages get higher priority
- **Actor Affinity**: Actors can be pinned to specific threads
- **Backpressure**: Automatic flow control when queues are full

#### 2. Message Passing

GAL implements zero-copy message passing where possible:

```rust
pub enum MessageDelivery {
    /// Direct delivery (same thread)
    Direct,
    
    /// Cross-thread via channel
    Channel(crossbeam::channel::Sender<ActorMessage>),
    
    /// Remote delivery (distributed actors)
    Remote(RemoteActorRef),
    
    /// Chaos-modified delivery
    Chaos(Box<dyn ChaosInterceptor>),
}
```

#### 3. Supervision Trees

Supervisors monitor actor health and implement restart strategies:

```gal
supervisor DatabaseSupervisor {
    strategy = OneForOne {
        max_restarts: 10,
        within: 1_minute,
        backoff: ExponentialBackoff(100ms, 30s, 2.0)
    }
    
    // Chaos-aware supervision
    on ActorFailed(actor_id, error) =>
        if error.is_chaos_induced() {
            schedule_restart(actor_id, strategy.chaos_delay())
        } else {
            strategy.handle_failure(actor_id, error)
        }
}
```

## Chaos Engineering Infrastructure

**Location**: `src/chaos.rs`, `src/chaos_contracts/`

GAL's chaos engineering system is integrated directly into the language runtime, enabling systematic reliability testing.

### Chaos Engine Architecture

```rust
pub struct ChaosEngine {
    config: ChaosConfig,
    fault_injectors: HashMap<FaultType, Box<dyn FaultInjector>>,
    active_experiments: Vec<ChaosExperiment>,
    statistics: ChaosStatistics,
}
```

### Fault Types

1. **Message Faults**
   - Drop: Messages disappear in transit
   - Delay: Messages arrive late
   - Duplicate: Messages are delivered multiple times
   - Corruption: Message contents are modified

2. **Actor Faults**
   - Crash: Actors terminate unexpectedly
   - Slow: Actors process messages slowly
   - Memory: Actors consume excessive memory

3. **System Faults**
   - Network partitions
   - Resource exhaustion
   - Thread starvation

### Chaos Contracts

Chaos contracts specify system behavior under failure conditions:

```gal
@chaos_contract
property message_ordering {
    given actors: Vec<ActorRef> = arbitrary(2..10)
    given messages: Vec<Message> = ordered_messages(100)
    
    when {
        chaos.apply([MessageDelay(0ms..100ms)])
        
        for (i, msg) in messages.enumerate() {
            send(actors[i % actors.len()], msg)
        }
        
        chaos.wait_for_quiescence(5s)
    }
    
    then {
        // Despite delays, messages should be processed in order
        // within each actor
        forall actor in actors {
            actor.processed_messages().is_ordered()
        }
    }
}
```

## Gödelian Self-Modification Engine

**Location**: `src/godel.rs`, `src/meta.rs`

The self-modification engine allows GAL programs to examine and modify their own structure at runtime, inspired by Gödel's work on self-reference.

### Reflection API

```gal
// Examine actor structure
let my_code = reflect(self)
let handler_count = my_code.handlers.len()

// Modify behavior at runtime
let new_handler = ast {
    on NewMessage(data) => println("Handling: " + data)
}
self.add_handler(new_handler)

// Generate optimized code
let optimized = self.compile_hot_path(usage_stats)
self.replace_implementation(optimized)
```

### Meta-Compilation

The system maintains multiple representations of running code:

1. **Source AST**: For human-readable modifications
2. **Typed AST**: For type-safe transformations
3. **IR**: For low-level optimizations
4. **Native Code**: For execution

### Self-Optimization

Actors can optimize themselves based on runtime behavior:

```rust
pub struct SelfOptimizer {
    /// Runtime statistics collection
    profiler: ActorProfiler,
    
    /// Hot path detection
    hot_path_detector: HotPathDetector,
    
    /// Code generation cache
    code_cache: CompiledCodeCache,
    
    /// Optimization history
    optimization_log: OptimizationLog,
}
```

## Memory Management

GAL uses a hybrid approach combining automatic memory management with manual control where needed.

### Per-Actor Heaps

Each actor has its own heap to ensure isolation:

```rust
pub struct ActorHeap {
    /// Fast bump allocator for temporary objects
    bump_allocator: BumpAllocator,
    
    /// Generational GC for long-lived objects
    gc_heap: GenerationalGC,
    
    /// Large object heap for big allocations
    large_object_heap: LargeObjectHeap,
}
```

### Garbage Collection

- **Generational**: Young objects are collected frequently
- **Concurrent**: GC runs alongside actor execution
- **Precise**: Exact reference tracking prevents leaks
- **Actor-Local**: Each actor's GC is independent

### Message Memory

Messages use reference counting for zero-copy sharing:

```rust
pub struct Message {
    /// Message payload
    data: Arc<dyn MessageData>,
    
    /// Sender information
    sender: ActorId,
    
    /// Message metadata
    metadata: MessageMetadata,
}
```

## Type System

GAL's type system is designed for safety, expressiveness, and performance.

### Core Type System

```gal
// Primitive types
Int, Float, String, Bool

// Algebraic data types
type Option<T> = Some(T) | None
type Result<T, E> = Ok(T) | Err(E)

// Actor types
ActorRef<MessageType>
ActorState<StateFields>

// Verification types
Verified<T, Contract>
Chaos<T, FaultModel>
```

### Type Inference

GAL uses Hindley-Milner type inference with extensions for actors:

```gal
// Type is inferred as ActorRef<CounterMessage>
let counter = spawn Counter(0)

// Type is inferred as Result<Int, Error>
let result = try_increment(counter)
```

### Contract Types

The type system integrates formal contracts:

```gal
// Precondition and postcondition types
fn divide(a: Int, b: Int) -> Int
    requires b != 0
    ensures result * b == a
{
    a / b
}

// Invariant types
type PositiveInt = Int 
    where value >= 0

actor BankAccount {
    state balance: PositiveInt  // Type enforces invariant
}
```

## Code Generation

GAL supports multiple compilation backends for different use cases.

### LLVM Backend

**Location**: `src/codegen/llvm_backend.rs`

The LLVM backend generates optimized native code:

```rust
pub struct LLVMCodegen {
    context: inkwell::context::Context,
    module: inkwell::module::Module,
    builder: inkwell::builder::Builder,
    
    /// Actor runtime integration
    runtime_functions: RuntimeFunctions,
    
    /// Chaos injection points
    chaos_points: Vec<ChaosInjectionPoint>,
}
```

**Features**:
- **Zero-cost abstractions**: High-level constructs compile to efficient code
- **Vectorization**: Automatic SIMD for data-parallel operations
- **Inlining**: Aggressive inlining for message handlers
- **PGO**: Profile-guided optimization support

### Cranelift Backend

**Location**: `src/codegen/cranelift_backend.rs`

Cranelift provides faster compilation for development:

```rust
pub struct CraneliftCodegen {
    builder_context: cranelift::codegen::Context,
    module: cranelift_module::Module<cranelift_jit::JITModule>,
    
    /// Function signatures
    signatures: HashMap<String, cranelift::codegen::ir::Signature>,
}
```

### JIT Compilation

**Location**: `src/jit.rs`

Hot code paths are recompiled at runtime:

```rust
pub struct JITCompiler {
    /// Runtime profiling data
    profiler: RuntimeProfiler,
    
    /// Hot path detection
    hot_detector: HotPathDetector,
    
    /// Code cache
    code_cache: CodeCache,
    
    /// Compilation queue
    compile_queue: CompilationQueue,
}
```

## Package System

**Location**: `src/package/`, `src/registry/`

GAL includes a comprehensive package management system inspired by Cargo.

### Package Structure

```
my-package/
├── gal.toml          # Package manifest
├── src/
│   ├── main.gal      # Main actor
│   └── lib.gal       # Library code
├── tests/
│   └── test.gal      # Test actors
├── examples/
│   └── demo.gal      # Example code
└── gal.lock          # Dependency lock file
```

### Dependency Resolution

The resolver uses a SAT solver for complex dependency graphs:

```rust
pub struct DependencyResolver {
    /// Package registry
    registry: Registry,
    
    /// Constraint solver
    solver: SATSolver,
    
    /// Cache for resolved dependencies
    cache: DependencyCache,
}
```

### Build System

**Location**: `src/build/`

The build system supports:

- **Incremental compilation**: Only rebuild changed modules
- **Parallel builds**: Compile multiple modules simultaneously
- **Cross-compilation**: Target different architectures
- **Chaos profiles**: Different fault injection configurations

## IDE Integration

**Location**: `src/lsp/`

GAL provides rich IDE support through the Language Server Protocol.

### Language Server

```rust
pub struct GalLanguageServer {
    /// Compiler integration
    compiler: IncrementalCompiler,
    
    /// Symbol index
    symbols: SymbolIndex,
    
    /// Diagnostics engine
    diagnostics: DiagnosticsEngine,
    
    /// Chaos testing integration
    chaos_runner: ChaosTestRunner,
}
```

### Features

1. **Syntax Highlighting**: Semantic highlighting with scope information
2. **Error Reporting**: Real-time error detection and suggestions
3. **Code Completion**: Context-aware completions with type information
4. **Go-to-Definition**: Navigate to symbol definitions across modules
5. **Find References**: Find all uses of symbols
6. **Refactoring**: Automated code transformations
7. **Debugging**: Integration with debugger and time-travel debugging
8. **Chaos Testing**: Run chaos tests from the IDE

### Incremental Compilation

The LSP uses incremental compilation for fast feedback:

```rust
pub struct IncrementalCompiler {
    /// Dependency graph
    module_graph: ModuleGraph,
    
    /// Compilation cache
    cache: CompilationCache,
    
    /// Change detection
    file_watcher: FileWatcher,
}
```

## Performance Characteristics

### Compilation Performance

| Phase | Time (1000 LOC) | Memory Usage |
|-------|------------------|--------------|
| Lexing | 2ms | 1MB |
| Parsing | 8ms | 5MB |
| Semantic Analysis | 15ms | 10MB |
| IR Generation | 5ms | 8MB |
| Optimization | 25ms | 15MB |
| Code Generation | 40ms | 20MB |
| **Total** | **95ms** | **59MB** |

### Runtime Performance

- **Message Throughput**: 2M+ messages/second
- **Actor Creation**: 100K+ actors/second  
- **Memory Overhead**: 512 bytes per actor
- **GC Pause**: < 1ms (99th percentile)

## Future Enhancements

### Planned Improvements

1. **Distributed Runtime**: Transparent actor distribution across nodes
2. **Advanced Optimizations**: Whole-program optimization and devirtualization
3. **Formal Verification**: Integration with proof assistants (Coq, Lean)
4. **GPU Acceleration**: CUDA/OpenCL backend for data-parallel actors
5. **WebAssembly**: Compile GAL to WASM for web deployment

### Research Areas

1. **Session Types**: Compile-time protocol verification
2. **Linear Types**: Resource usage guarantees
3. **Effect Systems**: Tracking side effects in the type system
4. **Probabilistic Programming**: Built-in support for probabilistic models

---

This architecture enables GAL to achieve its goals of combining safety, performance, and resilience in a unified programming model. The design ensures that chaos engineering and formal verification are not afterthoughts but integral parts of the language's design.