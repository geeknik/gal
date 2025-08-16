# GAL Quickstart Guide

Welcome to GAL (Gödelian Actor Language) - the world's first programming language with native chaos engineering capabilities! This guide will get you up and running with GAL in minutes.

## What is GAL?

GAL is a memory-safe, actor-based programming language that combines:
- **Actor Model Concurrency** - Scalable, fault-tolerant systems
- **Native Chaos Engineering** - Built-in fault injection and resilience testing  
- **Gödelian Self-Reference** - Programs that can reason about themselves
- **Formal Verification** - Mathematical correctness proofs
- **Self-Optimization** - AI-powered runtime performance improvement

## Installation

### Prerequisites
- Rust 1.70+ (for building from source)
- LLVM 17+ (optional, for LLVM backend)
- Git

### Building GAL

```bash
# Clone the repository
git clone https://github.com/geeknik/gal
cd gal

# Build with default features
cargo build --release

# Build with all features
cargo build --release --all-features

# Install GAL tools
cargo install --path . --bins
```

### Available Tools

After building, you'll have access to these command-line tools:

- `galc` - GAL compiler
- `gal-repl` - Interactive REPL
- `gal-chaos` - Chaos engineering CLI
- `gal-language-server` - LSP server for editors
- `gal-verify` - Formal verification tool
- `gal-pkg` - Package manager
- `gal-bench` - Performance benchmarking

## Your First GAL Program

### Hello, World!

Create a file called `hello.gal`:

```gal
// hello.gal - Your first GAL program
import std.io;

fn main() {
    println("Hello, GAL World!");
}
```

Compile and run:

```bash
galc hello.gal -o hello
./hello
```

### Hello, Actors!

GAL's real power comes from its actor system. Create `hello_actors.gal`:

```gal
// hello_actors.gal - Actor-based hello world
import std.io;
import std.actors;

actor Greeter {
    fn receive(msg: String) {
        println("Hello, {}!", msg);
    }
}

fn main() {
    let greeter = spawn Greeter;
    greeter.send("Actor World");
    
    // Wait for message processing
    std.actors.wait_all();
}
```

## Core Language Features

### Actor Definitions

```gal
// Basic actor with state
actor Counter {
    state count: i32 = 0;
    
    fn receive(msg: CounterMsg) {
        match msg {
            Increment => {
                self.count += 1;
                println("Count: {}", self.count);
            }
            Get(reply_to) => {
                reply_to.send(self.count);
            }
        }
    }
}

enum CounterMsg {
    Increment,
    Get(ActorRef<i32>),
}
```

### Pattern Matching

```gal
fn process_message(msg: Message) {
    match msg {
        Text(content) => println("Text: {}", content),
        Number(n) if n > 100 => println("Large number: {}", n),
        Number(n) => println("Small number: {}", n),
        _ => println("Unknown message type"),
    }
}
```

### Memory Safety

GAL provides Rust-like ownership semantics for memory safety:

```gal
fn transfer_ownership() {
    let data = vec![1, 2, 3, 4, 5];
    let moved_data = data;  // data is moved, no longer accessible
    
    // println("{:?}", data);  // Error: use after move
    println("{:?}", moved_data);  // OK
}

fn borrowing_example() {
    let data = vec![1, 2, 3, 4, 5];
    let borrowed = &data;  // Immutable borrow
    
    println("{:?}", data);      // OK: original still accessible
    println("{:?}", borrowed);  // OK: borrow is valid
}
```

## Chaos Engineering Features 🌟

GAL's unique selling point is native chaos engineering. You can inject faults directly in your code:

### Basic Chaos Annotations

```gal
// Chaos-aware actor that can handle network failures
@chaos(network_failures: 0.1, message_drops: 0.05)
actor NetworkService {
    state retry_count: i32 = 0;
    
    @fault_tolerant(max_retries: 3, backoff: "exponential")
    fn process_request(req: NetworkRequest) {
        // This method will automatically retry on failures
        match self.make_network_call(req) {
            Ok(response) => self.send_response(response),
            Err(e) => {
                self.retry_count += 1;
                // Chaos engine will inject failures here
                if self.retry_count < 3 {
                    self.retry_after_delay(req);
                } else {
                    self.send_error("Service unavailable");
                }
            }
        }
    }
}
```

### Chaos Contracts

Define formal invariants that must hold even under chaos:

```gal
@chaos_contract {
    invariant: "count >= 0",
    fault_tolerance: ["message_loss", "actor_restart"],
    max_recovery_time: "5s"
}
actor ReliableCounter {
    state count: i32 = 0;
    
    @ensures("count == old(count) + 1")
    fn increment() {
        self.count += 1;
    }
    
    @ensures("result == count")
    fn get() -> i32 {
        self.count
    }
}
```

### Running Chaos Experiments

```bash
# Run your program with chaos engineering enabled
galc hello_actors.gal --features chaos-mode -o hello_chaos
gal-chaos experiment --name "network-failure" --duration 60s ./hello_chaos

# View chaos experiment results
gal-chaos report --experiment "network-failure"
```

## Gödelian Self-Reference Features 🤯

GAL supports programs that can reason about and modify themselves:

### Self-Inspecting Programs

```gal
import std.reflection;

actor SelfAware {
    fn analyze_self() {
        let my_source = reflection.get_source_code(self);
        let my_bytecode = reflection.get_bytecode(self);
        
        println("I am {} lines of source code", my_source.line_count());
        println("My bytecode size is {} bytes", my_bytecode.len());
        
        // Check if this actor mentions itself
        if my_source.contains("SelfAware") {
            println("I am self-referential!");
        }
    }
}
```

### Quine Generation

```gal
import std.quine;

fn generate_quine() -> String {
    // GAL can generate programs that print themselves
    quine.generate_for_function(generate_quine)
}

fn main() {
    let self_printing_program = generate_quine();
    println("Here's a program that prints itself:");
    println("{}", self_printing_program);
}
```

### Self-Modification (Advanced)

```gal
import std.meta;

@self_modifiable
actor EvolvingService {
    state performance_history: Vec<f64> = vec![];
    
    fn optimize_self() {
        let avg_performance = self.performance_history.average();
        
        if avg_performance < 0.8 {
            // Rewrite this actor's code to be more efficient
            let optimized_code = meta.optimize_actor_code(self);
            meta.replace_actor_implementation(self, optimized_code);
            
            println("I have evolved to be more efficient!");
        }
    }
}
```

## Interactive Development

### Using the REPL

```bash
# Start the GAL REPL
gal-repl

# Try some expressions
gal> let x = 42;
gal> let y = x * 2;
gal> println("Result: {}", y);
Result: 84

# Spawn actors interactively
gal> actor Test { fn receive(msg: i32) { println("Got: {}", msg); } }
gal> let test_actor = spawn Test;
gal> test_actor.send(123);
Got: 123
```

### IDE Support

GAL provides comprehensive Language Server Protocol (LSP) support:

```bash
# Start the language server for your editor
gal-language-server

# Features available:
# - Syntax highlighting
# - Code completion
# - Error diagnostics
# - Go to definition
# - Hover documentation
# - Refactoring support
# - Actor system visualization
```

## Formal Verification

GAL includes built-in formal verification capabilities:

### Specifying Contracts

```gal
// Function with formal specification
@requires("n >= 0")
@ensures("result >= n")
fn factorial(n: i32) -> i32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// Actor with protocol specification
@protocol {
    states: ["waiting", "processing", "done"],
    transitions: [
        ("waiting", "start", "processing"),
        ("processing", "finish", "done"),
        ("done", "reset", "waiting")
    ]
}
actor StateMachine {
    state current_state: String = "waiting";
    
    fn handle_event(event: Event) {
        // Implementation automatically verified against protocol
    }
}
```

### Running Verification

```bash
# Verify your program's correctness
gal-verify hello_actors.gal

# Generate mathematical proofs
gal-verify --generate-proofs hello_actors.gal

# Check specific properties
gal-verify --property "deadlock_free" --property "memory_safe" hello_actors.gal
```

## Package Management

### Creating a Package

```bash
# Initialize a new GAL package
gal-pkg init my_actor_system
cd my_actor_system

# Package structure
# my_actor_system/
# ├── gal.toml          # Package configuration
# ├── src/
# │   └── main.gal      # Main source file
# └── tests/
#     └── integration.gal
```

### Package Configuration (`gal.toml`)

```toml
[package]
name = "my_actor_system"
version = "0.1.0"
description = "My first GAL actor system"
authors = ["Your Name <you@example.com>"]
license = "MIT"

[dependencies]
gal_std = "1.0"
chaos_utils = "0.5"

[features]
default = ["chaos-engineering"]
chaos-engineering = []
formal-verification = []

[chaos]
default_profile = "development"

[chaos.profiles.development]
fault_injection_rate = 0.01
max_concurrent_faults = 2

[chaos.profiles.production]
fault_injection_rate = 0.001
max_concurrent_faults = 1
```

### Installing Dependencies

```bash
# Add a dependency
gal-pkg add distributed_actors --version "^2.0"

# Update dependencies
gal-pkg update

# Build with dependencies
gal-pkg build

# Run tests
gal-pkg test

# Publish to registry
gal-pkg publish
```

## Performance Optimization

### Zero-Cost Abstractions

```gal
// High-level actor operations compile to efficient code
fn process_pipeline(data: Vec<i32>) -> Vec<i32> {
    data.into_iter()
        .filter(|&x| x > 0)
        .map(|x| x * 2)
        .collect()
}

// Actors with zero-cost message passing
@zero_cost
actor HighPerformanceProcessor {
    fn process_batch(items: Vec<DataItem>) {
        // Compiler optimizes this to direct memory operations
        for item in items {
            self.process_item(item);
        }
    }
}
```

### JIT Compilation

```gal
// Hot paths are automatically JIT compiled
@hot_path
fn compute_intensive_operation(data: &[f64]) -> f64 {
    // This function will be JIT compiled after several calls
    data.iter().map(|&x| x.sqrt().sin().cos()).sum()
}
```

### Benchmarking

```bash
# Run performance benchmarks
gal-bench --all

# Benchmark specific functions
gal-bench --function compute_intensive_operation

# Chaos benchmarks (performance under fault injection)
gal-bench --chaos --fault-rate 0.1
```

## Distributed Systems

### Multi-Node Actor Systems

```gal
import std.distributed;

@distributed
actor DistributedWorker {
    fn process_work(work: WorkItem) {
        // This actor can run on any node in the cluster
        let result = self.compute(work);
        self.send_result_to_coordinator(result);
    }
}

fn main() {
    // Join or create a cluster
    let cluster = distributed.join_cluster("worker-cluster");
    
    // Spawn distributed actors
    let workers = (0..10).map(|_| cluster.spawn(DistributedWorker)).collect();
    
    // Distribute work across the cluster
    for (i, work) in work_items.iter().enumerate() {
        workers[i % workers.len()].send(work);
    }
}
```

### Running Distributed Applications

```bash
# Start a cluster coordinator
gal-chaos cluster start --name "my-cluster" --port 8080

# Join the cluster from different machines
galc my_distributed_app.gal --distributed --cluster "my-cluster" --coordinator "coordinator-ip:8080"

# Monitor cluster health
gal-chaos cluster status --name "my-cluster"
```

## Example Projects

### 1. Fault-Tolerant Web Service

```gal
// A web service that can handle chaos gracefully
@chaos_contract {
    invariant: "response_time < 5s",
    fault_tolerance: ["network_partition", "node_failure"],
    availability: 0.99
}
actor WebServer {
    state request_count: i64 = 0;
    state error_count: i64 = 0;
    
    @fault_tolerant(circuit_breaker: true)
    fn handle_request(req: HttpRequest) -> HttpResponse {
        self.request_count += 1;
        
        match self.process_request(req) {
            Ok(response) => response,
            Err(e) => {
                self.error_count += 1;
                HttpResponse.error(500, "Service temporarily unavailable")
            }
        }
    }
}
```

### 2. Self-Optimizing Data Processor

```gal
// A data processor that optimizes itself based on performance
@self_optimizing
actor DataProcessor {
    state processing_strategy: Strategy = Strategy.Default;
    state performance_metrics: PerformanceTracker = PerformanceTracker.new();
    
    fn process_batch(data: Vec<DataItem>) {
        let start_time = time.now();
        
        let results = match self.processing_strategy {
            Strategy.Default => self.process_sequential(data),
            Strategy.Parallel => self.process_parallel(data),
            Strategy.SIMD => self.process_simd(data),
        };
        
        let duration = time.now() - start_time;
        self.performance_metrics.record(duration);
        
        // Auto-optimize strategy based on performance
        if self.performance_metrics.should_optimize() {
            self.evolve_strategy();
        }
        
        results
    }
    
    @self_modify
    fn evolve_strategy() {
        // AI-powered strategy optimization
        let new_strategy = ml.optimize_strategy(
            self.performance_metrics.history(),
            self.processing_strategy
        );
        self.processing_strategy = new_strategy;
    }
}
```

### 3. Distributed Consensus System

```gal
// Raft consensus implementation with chaos testing
@consensus_protocol("raft")
@chaos_contract {
    invariant: "consistency",
    fault_tolerance: ["node_failure", "network_partition"],
    max_nodes_down: 2  // Can tolerate 2 out of 5 nodes failing
}
actor RaftNode {
    state term: u64 = 0;
    state voted_for: Option<NodeId> = None;
    state log: Vec<LogEntry> = vec![];
    state state: NodeState = NodeState.Follower;
    
    fn handle_message(msg: RaftMessage) {
        match (self.state, msg) {
            (NodeState.Follower, RaftMessage.RequestVote(req)) => {
                self.handle_vote_request(req);
            }
            (NodeState.Leader, RaftMessage.AppendEntries(entries)) => {
                self.replicate_entries(entries);
            }
            // ... other state transitions
        }
    }
}
```

## Advanced Topics

### Macro System

```gal
// Define custom syntax for actor patterns
macro_rules! supervisor_tree {
    ($($child:ident : $actor_type:ty),*) => {
        actor SupervisorTree {
            state children: HashMap<String, ActorRef> = HashMap::new();
            
            fn start() {
                $(
                    let $child = spawn $actor_type;
                    self.children.insert(stringify!($child), $child);
                    self.monitor($child);
                )*
            }
        }
    }
}

// Use the macro
supervisor_tree! {
    worker1: Worker,
    worker2: Worker,
    database: DatabaseActor
}
```

### Foreign Function Interface

```gal
// Call C functions from GAL
extern "C" {
    fn c_compute_hash(data: *const u8, len: usize) -> u64;
}

actor CInteropActor {
    fn compute_hash(data: Vec<u8>) -> u64 {
        unsafe {
            c_compute_hash(data.as_ptr(), data.len())
        }
    }
}
```

## Best Practices

### 1. Actor Design
- Keep actors small and focused
- Use immutable messages when possible
- Design for fault tolerance from the start
- Use supervision trees for error handling

### 2. Chaos Engineering
- Start with low fault injection rates
- Test individual components before integration
- Use chaos contracts to specify expected behavior
- Monitor and measure resilience continuously

### 3. Performance
- Profile before optimizing
- Use zero-cost abstractions
- Let the JIT compiler optimize hot paths
- Measure chaos impact on performance

### 4. Verification
- Write contracts for critical functions
- Use property-based testing
- Verify protocols and state machines
- Generate proofs for security-critical code

## Getting Help

- **GitHub Issues**: [https://github.com/geeknik/gal/issues](https://github.com/geeknik/gal/issues)

## What's Next?

Now that you've got GAL running, explore these areas:

1. **Build a distributed actor system** - Try the cluster features
2. **Experiment with chaos engineering** - Break things safely
3. **Try formal verification** - Prove your code is correct
4. **Create self-modifying programs** - Explore Gödelian features
5. **Contribute to the ecosystem** - Build packages and tools

Welcome to the future of resilient, self-aware programming! 🚀

---

**Happy GAL coding!** 

Remember: "In GAL, we don't just handle errors - we chaos engineer our way to antifragility!" 💪%                                                                                                                                                           
