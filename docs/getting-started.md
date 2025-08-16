# Getting Started with GAL

Welcome to GAL (Gödelian Actor Language)! This guide will help you get up and running with GAL in just a few minutes.

## Prerequisites

Before you begin, ensure you have the following installed:
- Rust 1.70 or later
- Git
- A text editor (VS Code recommended for IDE support)

## Installation

### Building from Source

```bash
# Clone the GAL repository
git clone https://github.com/geeknik/gal
cd gal

# Build the compiler
cargo build --release

# Add to PATH (for current session)
export PATH="$PWD/target/release:$PATH"

# Or install globally
cargo install --path .
```

## Your First GAL Program

### Hello World

Create a file named `hello.gal`:

```gal
actor Main {
    new create() =>
        println("Hello, GAL World!")
}
```

Compile and run:

```bash
# Compile the program
galc hello.gal -o hello

# Run it
./hello
```

You should see:
```
Hello, GAL World!
```

## Understanding Actors

GAL is built on the actor model. Actors are isolated units of computation that communicate via message passing.

### Basic Actor Example

Create `counter.gal`:

```gal
actor Counter {
    state count: Int = 0
    
    on Increment =>
        count += 1
        println("Count is now: " + count.to_string())
    
    on GetCount =>
        reply(count)
}

actor Main {
    new create() =>
        let counter = spawn Counter
        
        // Send messages to the counter
        send(counter, Increment)
        send(counter, Increment)
        send(counter, Increment)
        
        // Get the current count
        let final_count = ask(counter, GetCount)
        println("Final count: " + final_count.to_string())
}
```

Compile and run:
```bash
galc counter.gal -o counter
./counter
```

Output:
```
Count is now: 1
Count is now: 2
Count is now: 3
Final count: 3
```

## Chaos Engineering

One of GAL's unique features is built-in chaos engineering support. You can test your actors under failure conditions.

### Chaos Example

Create `resilient_counter.gal`:

```gal
@chaos_test(faults: [MessageDrop(0.1)])
actor ResilientCounter {
    state count: Int = 0
    
    // Define an invariant that must always hold
    invariant count >= 0
    
    on Increment =>
        count += 1
        reply(count)
    
    on Decrement =>
        requires count > 0  // Precondition
        count -= 1
        reply(count)
}

actor Main {
    new create() =>
        let counter = spawn ResilientCounter
        
        // Enable chaos mode - 10% of messages will be dropped
        chaos.enable([MessageDrop(0.1)])
        
        // Send 10 increment messages
        for i in 1..10 {
            send(counter, Increment)
        }
        
        // Some messages may be dropped due to chaos
        println("Testing completed under chaos conditions")
}
```

## Formal Verification

GAL supports contracts and formal verification to ensure correctness.

### Contract Example

```gal
actor BankAccount {
    state balance: Money = 0
    
    // Invariants that must always be true
    invariant balance >= 0
    
    on Deposit(amount: Money) =>
        requires amount > 0        // Precondition
        ensures balance == old(balance) + amount  // Postcondition
        balance += amount
        reply(balance)
    
    on Withdraw(amount: Money) =>
        requires amount > 0
        requires amount <= balance
        ensures balance == old(balance) - amount
        balance -= amount
        reply(balance)
}
```

## Pattern Matching

GAL supports powerful pattern matching on messages:

```gal
actor Calculator {
    on Calculate(op) => match op {
        Add(a, b) => reply(a + b)
        Subtract(a, b) => reply(a - b)
        Multiply(a, b) => reply(a * b)
        Divide(a, b) when b != 0 => reply(a / b)
        Divide(_, 0) => reply_error("Division by zero!")
    }
}
```

## Supervision Trees

Build fault-tolerant systems with supervision:

```gal
supervisor WorkerSupervisor {
    strategy OneForOne(max_restarts: 3, within: 60s)
    
    children [
        Worker(id: 1),
        Worker(id: 2),
        Worker(id: 3)
    ]
    
    on WorkerCrashed(id) =>
        println("Worker " + id.to_string() + " crashed, restarting...")
        restart_child(id)
}

actor Worker {
    state id: Int
    state tasks_completed: Int = 0
    
    new create(id: Int) =>
        self.id = id
    
    on DoWork(task) =>
        // Simulate work
        tasks_completed += 1
        reply(TaskComplete(id, tasks_completed))
}
```

## Next Steps

### Learn More

1. **Language Reference**: Read the [complete language reference](language-reference.md) for detailed syntax and semantics
2. **Actor Patterns**: Explore [common actor patterns](patterns.md) and best practices
3. **Chaos Testing**: Deep dive into [chaos engineering](chaos-engineering.md) features
4. **Formal Methods**: Learn about [formal verification](formal-verification.md) in GAL

### Example Projects

Check out these example projects to see GAL in action:

- `examples/chat_server.gal` - A distributed chat server
- `examples/key_value_store.gal` - A fault-tolerant key-value store
- `examples/distributed_counter.gal` - A distributed, replicated counter
- `examples/chaos_contracts_demo.gal` - Advanced chaos testing with contracts

### IDE Support

Install the GAL extension for your editor:

- **VS Code**: Search for "GAL Language" in the extensions marketplace
- **Vim/Neovim**: Install `coc-gal` via your plugin manager
- **Emacs**: Add `gal-mode` to your configuration

### Community

- Join our [Discord server](https://discord.gg/gal-lang)
- Follow development on [GitHub](https://github.com/gal-lang/gal)
- Read the [GAL blog](https://gal-lang.org/blog) for updates and tutorials

## Troubleshooting

### Common Issues

**Q: Compilation fails with "actor not found"**
A: Ensure your Main actor is defined. Every GAL program needs a Main actor as the entry point.

**Q: Messages are not being received**
A: Check that you're using `send` for async messages or `ask` for synchronous requests. Also verify the message handler exists.

**Q: Chaos tests always pass even with faults**
A: Ensure you have invariants defined. Chaos testing validates that invariants hold despite failures.

### Getting Help

If you encounter issues:
1. Check the [FAQ](faq.md)
2. Search [GitHub issues](https://github.com/geeknik/gal/issues)
3. Report bugs on [GitHub](https://github.com/geeknik/gal/issues/new)

## Summary

You've learned the basics of GAL:
- Creating and compiling programs
- Working with actors and messages
- Using chaos engineering features
- Writing contracts for verification
- Building supervision trees

GAL combines the safety of Rust, the concurrency of Erlang, and unique chaos engineering capabilities to help you build resilient, distributed systems. Happy coding!
