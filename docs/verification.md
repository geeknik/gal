# GAL Formal Verification System

The GAL formal verification system provides comprehensive SMT-based verification capabilities for actor systems, enabling mathematical proofs of correctness, safety, and liveness properties.

## Overview

The verification system consists of four main components:

1. **SMT Solver Integration** (`smt_solver.rs`) - Interface to multiple SMT solvers (Z3, CVC5, Yices)
2. **Verification Engine** (`verification_engine.rs`) - Orchestrates verification process
3. **Proof Generator** (`proof_generator.rs`) - Generates formal proofs and certificates
4. **GAL to SMT Translator** (`gal_to_smt.rs`) - Translates GAL programs to SMT formulas

## Features

### Core Verification Capabilities

- **Actor State Invariants** - Verify properties that must always hold for actor state
- **Message Handler Correctness** - Verify preconditions and postconditions for message handlers
- **Function Verification** - Hoare logic verification for pure functions
- **Global System Properties** - Verify system-wide invariants across all actors
- **Deadlock Freedom** - Verify absence of deadlocks in actor communication
- **Liveness Properties** - Verify that desired events eventually occur
- **Safety Properties** - Verify that undesired events never occur

### Advanced Features

- **Temporal Logic** - Support for Linear Temporal Logic (LTL) and Computation Tree Logic (CTL)
- **Chaos Engineering Verification** - Model fault injection and verify resilience properties
- **Distributed System Verification** - Model consensus protocols, consistency guarantees, and partition tolerance
- **Proof Generation** - Generate formal proofs with certificates in multiple formats
- **Counterexample Generation** - Generate and minimize counterexamples for failed verifications

## SMT Solver Backends

The system supports multiple SMT solver backends:

- **Z3** - Microsoft's Z3 theorem prover (recommended)
- **CVC5** - Stanford's CVC5 solver
- **Yices** - SRI's Yices solver
- **Internal** - Simple built-in solver for basic cases

## Verification Annotations

### Actor Verification

```gal
@verify
actor BankAccount {
    state balance: Int = 0
    
    // State invariant
    proof balance_non_negative: balance >= 0
    
    on Deposit(amount: Int)
        requires amount > 0
        ensures balance == old(balance) + amount
    =>
        balance = balance + amount
}
```

### Function Verification

```gal
function factorial(n: Int) -> Int
    requires n >= 0
    ensures result == factorial_spec(n) && result > 0
{
    if n <= 1 {
        return 1
    } else {
        return n * factorial(n - 1)
    }
}
```

### Global Properties

```gal
@prove_property
property money_conservation {
    always(sum(all_accounts.balance) == total_money)
}

@prove_property
property eventual_consistency {
    eventually(all_replicas_consistent())
}
```

### Chaos Engineering Verification

```gal
@prove_resilient(
    faults = [MessageDrop(0.1), ActorCrash(0.05)],
    properties = [SafetyProperty(invariant), LivenessProperty(progress)]
)
actor ResilientActor {
    // Actor implementation with fault tolerance
}
```

## Command Line Usage

The `gal-verify` tool provides command-line access to verification capabilities:

```bash
# Basic verification
gal-verify program.gal

# Use specific SMT solver
gal-verify --smt-backend z3 program.gal

# Generate proofs
gal-verify --generate-proofs --export-format coq --proof-output proofs.v program.gal

# Verify specific properties
gal-verify --verify-safety --verify-liveness --verify-deadlock program.gal

# Enable chaos verification
gal-verify --chaos-verification --fault-injection program.gal

# Show statistics
gal-verify --stats --verbose program.gal

# Generate counterexamples
gal-verify --counterexample --minimize-counterexample program.gal
```

### Command Line Options

- `--smt-backend BACKEND` - Choose SMT solver (z3, cvc5, yices, internal)
- `--timeout SECONDS` - Set SMT solver timeout
- `--verify-safety` - Verify safety properties
- `--verify-liveness` - Verify liveness properties
- `--verify-deadlock` - Verify deadlock freedom
- `--generate-proofs` - Generate formal proofs
- `--export-format FORMAT` - Export proofs (gal, coq, lean, isabelle, smt, tptp)
- `--proof-output FILE` - Output file for proofs
- `--chaos-verification` - Enable chaos engineering verification
- `--fault-injection` - Enable fault injection modeling
- `--distributed-verification` - Enable distributed system verification
- `--temporal-bound N` - Set bound for temporal logic verification
- `--max-actors N` - Maximum number of actors to model
- `--counterexample` - Generate counterexamples for failed verifications
- `--minimize-counterexample` - Minimize generated counterexamples
- `--stats` - Show verification statistics
- `--verbose` - Enable verbose output

## Verification Workflow

1. **Parse GAL Program** - Parse and analyze the GAL source code
2. **Extract Annotations** - Extract verification annotations and properties
3. **Translate to SMT** - Convert GAL constructs to SMT formulas
4. **Encode Properties** - Encode invariants, contracts, and temporal properties
5. **Invoke SMT Solver** - Use SMT solver to check satisfiability
6. **Generate Results** - Create verification results and proofs
7. **Export Artifacts** - Export proofs, counterexamples, and certificates

## Example: Bank Account Verification

Here's a complete example of verifying a bank account actor:

```gal
@verify
actor BankAccount {
    state balance: Int = 0
    state account_id: String
    state is_frozen: Bool = false
    
    // Invariants
    proof balance_non_negative: balance >= 0
    proof account_id_not_empty: account_id.len() > 0
    
    constructor(id: String) 
        requires id.len() > 0
        ensures balance == 0 && account_id == id && !is_frozen
    {
        account_id = id
    }
    
    on Deposit(amount: Int)
        requires amount > 0 && !is_frozen
        ensures balance == old(balance) + amount
    =>
        balance = balance + amount
    
    on Withdraw(amount: Int) 
        requires amount > 0 && amount <= balance && !is_frozen
        ensures balance == old(balance) - amount
    =>
        balance = balance - amount
}
```

To verify this program:

```bash
gal-verify --generate-proofs --stats bank_account.gal
```

## Proof Formats

The system can export proofs in multiple formats:

### GAL Native Format (JSON)
```json
{
  "id": "proof_123",
  "property": {
    "StateInvariant": {
      "actor": "BankAccount",
      "invariant": "balance >= 0"
    }
  },
  "steps": [
    {
      "step_id": "1",
      "rule": "SMTDerivation",
      "conclusion": "balance >= 0",
      "justification": "SMT solver proved invariant holds"
    }
  ]
}
```

### Coq Format
```coq
Theorem balance_non_negative : forall (balance : Z), balance >= 0.
Proof.
  intros balance.
  (* SMT derivation *)
  admit.
Qed.
```

### Lean Format
```lean
theorem balance_non_negative (balance : ℤ) : balance ≥ 0 := by
  sorry -- SMT derivation
```

## Configuration

The verification system can be configured through various parameters:

### SMT Configuration
```rust
SmtConfig {
    logic: "QF_LIA".to_string(),          // SMT logic to use
    model_generation: true,                // Generate models for SAT results
    proof_generation: true,                // Generate proofs for UNSAT results
    unsat_core: true,                     // Generate unsat cores
    incremental: true,                    // Enable incremental solving
    random_seed: Some(42),                // Seed for reproducibility
}
```

### Translation Configuration
```rust
TranslationConfig {
    max_actors: 10,                       // Maximum actors to model
    max_queue_size: 100,                  // Maximum message queue size
    temporal_bound: 50,                   // Bound for temporal verification
    chaos_modeling: true,                 // Enable chaos injection modeling
    failure_modeling: true,               // Model communication failures
    message_ordering: true,               // Enable message ordering constraints
}
```

## Performance Considerations

### Scalability
- The system uses bounded model checking for temporal properties
- Actor and message queue sizes are bounded to ensure decidability
- Incremental solving and caching optimize repeated queries

### Optimization Strategies
- Use appropriate SMT logic (e.g., QF_LIA for linear arithmetic)
- Enable proof caching to avoid redundant computations
- Set reasonable bounds for temporal verification
- Use internal solver for simple properties

### Memory Usage
- SMT solvers can consume significant memory for complex formulas
- Consider using streaming verification for large programs
- Monitor memory usage and adjust bounds accordingly

## Troubleshooting

### Common Issues

1. **SMT Solver Not Found**
   - Ensure Z3, CVC5, or Yices is installed and in PATH
   - Use `--smt-backend internal` as fallback

2. **Verification Timeout**
   - Increase timeout with `--timeout`
   - Reduce model bounds with `--max-actors` and `--temporal-bound`
   - Simplify complex properties

3. **Memory Issues**
   - Reduce the number of actors being verified
   - Use smaller temporal bounds
   - Verify properties incrementally

4. **Unsupported Features**
   - Some GAL features may not be fully supported in SMT translation
   - Use simpler abstractions for complex language constructs

### Debug Mode

Enable verbose output to see detailed verification steps:

```bash
gal-verify --verbose --stats program.gal
```

This will show:
- SMT formula generation
- Solver invocations and results
- Proof generation steps
- Performance statistics

## Implementation Details

### Architecture

The verification system follows a modular architecture:

```
GAL Program → Parser → AST → Verification Engine
                              ↓
SMT Translator ← SMT Solver ← Verification Context
    ↓              ↓
Proof Generator → Certificate Generator
```

### Key Algorithms

1. **Bounded Model Checking** - For temporal properties
2. **Weakest Precondition Calculus** - For Hoare logic verification
3. **Separation Logic** - For concurrent actor verification
4. **Delta Debugging** - For counterexample minimization

### Future Enhancements

- **Interactive Verification** - Integration with proof assistants
- **Compositional Verification** - Verify large systems modularly
- **Automatic Invariant Discovery** - Learn invariants from code
- **Parallel Verification** - Distribute verification across cores
- **Cloud Integration** - Use cloud SMT solvers for scalability

## References

- [SMT-LIB Standard](http://smtlib.cs.uiowa.edu/)
- [Z3 Theorem Prover](https://github.com/Z3Prover/z3)
- [CVC5 SMT Solver](https://cvc5.github.io/)
- [Temporal Logic Model Checking](https://en.wikipedia.org/wiki/Model_checking)
- [Separation Logic](https://en.wikipedia.org/wiki/Separation_logic)