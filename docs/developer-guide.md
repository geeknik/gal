# GAL Developer Guide

This guide provides comprehensive information for developers who want to contribute to the GAL compiler, understand its internals, or build tools for the GAL ecosystem.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Project Structure](#project-structure)
3. [Building from Source](#building-from-source)
4. [Testing](#testing)
5. [Compiler Architecture](#compiler-architecture)
6. [Contributing](#contributing)
7. [Debugging](#debugging)
8. [Performance Profiling](#performance-profiling)
9. [Release Process](#release-process)
10. [Tool Development](#tool-development)

## Getting Started

### Prerequisites

Before you can build GAL from source, ensure you have:

- **Rust 1.70 or later** with `cargo`
- **LLVM 17.0** development libraries (for LLVM backend)
- **Git** for version control
- **Python 3.8+** for build scripts
- **Node.js 16+** for documentation tools

### Quick Setup

```bash
# Clone the repository
git clone https://github.com/gal-lang/gal.git
cd gal

# Install development dependencies
./scripts/setup-dev.sh

# Build in development mode
cargo build

# Run basic tests
cargo test

# Install locally
cargo install --path .
```

### Development Environment

We recommend using one of these setups:

#### VS Code Setup
```bash
# Install recommended extensions
code --install-extension rust-lang.rust-analyzer
code --install-extension ms-vscode.lldb
code --install-extension GitHub.copilot

# Open project
code .
```

#### Vim/Neovim Setup
```lua
-- Add to your init.lua
require('lspconfig').rust_analyzer.setup({
  settings = {
    ["rust-analyzer"] = {
      cargo = {
        features = "all"
      }
    }
  }
})
```

## Project Structure

```
gal/
├── src/                    # Compiler source code
│   ├── bin/               # Binary entry points
│   │   ├── galc.rs       # Main compiler binary
│   │   ├── repl.rs       # Interactive REPL
│   │   └── chaos.rs      # Chaos testing tool
│   ├── lexer.rs          # Lexical analysis
│   ├── parser.rs         # Syntax analysis
│   ├── ast.rs            # AST definitions
│   ├── semantic.rs       # Semantic analysis
│   ├── types.rs          # Type system
│   ├── ir.rs             # Intermediate representation
│   ├── codegen/          # Code generation backends
│   │   ├── mod.rs
│   │   ├── llvm_backend.rs
│   │   └── cranelift_backend.rs
│   ├── runtime/          # Runtime system
│   ├── chaos/            # Chaos engineering
│   ├── godel/            # Self-modification features
│   ├── package/          # Package management
│   └── lsp/              # Language server
├── tests/                 # Test suite
│   ├── unit/             # Unit tests
│   ├── integration/      # Integration tests
│   └── chaos/            # Chaos engineering tests
├── examples/             # Example GAL programs
├── docs/                 # Documentation
├── scripts/              # Build and development scripts
├── tools/                # Development tools
├── stdlib/               # Standard library (GAL code)
└── benches/              # Performance benchmarks
```

### Key Source Files

| File | Purpose |
|------|---------|
| `src/lib.rs` | Main library entry point and module exports |
| `src/lexer.rs` | Tokenization and lexical analysis |
| `src/parser.rs` | Recursive descent parser |
| `src/semantic.rs` | Type checking and semantic analysis |
| `src/ir.rs` | Intermediate representation and transformations |
| `src/codegen/mod.rs` | Code generation interface |
| `src/runtime/mod.rs` | Actor runtime system |
| `src/chaos.rs` | Chaos engineering infrastructure |

## Building from Source

### Development Build

```bash
# Standard development build
cargo build

# Build with all features enabled
cargo build --all-features

# Build with specific backend
cargo build --features llvm-backend
cargo build --features cranelift-backend

# Build for release (optimized)
cargo build --release
```

### Build Profiles

GAL uses several build profiles:

```toml
# Cargo.toml profiles
[profile.dev]
debug = true
opt-level = 0
overflow-checks = true

[profile.release]
debug = false
opt-level = 3
lto = "thin"
codegen-units = 1
panic = "abort"

[profile.bench]
debug = false
opt-level = 3
lto = true
codegen-units = 1

[profile.test]
debug = true
opt-level = 1
```

### Feature Flags

```bash
# Available features
--features llvm-backend       # Enable LLVM code generation
--features cranelift-backend  # Enable Cranelift code generation
--features chaos-mode         # Enable chaos engineering features
--features formal-verification # Enable formal verification
--features serde              # Enable serialization support
```

### Cross-Compilation

```bash
# Install target
rustup target add aarch64-apple-darwin

# Cross-compile
cargo build --target aarch64-apple-darwin --release

# Available targets
rustc --print target-list | grep -E "(linux|darwin|windows)"
```

### Custom LLVM

If you need a custom LLVM build:

```bash
# Set environment variables
export LLVM_SYS_170_PREFIX=/path/to/llvm
export LLVM_CONFIG_PATH=/path/to/llvm/bin/llvm-config

# Build with custom LLVM
cargo build --features llvm-backend
```

## Testing

GAL has a comprehensive test suite covering all aspects of the compiler and runtime.

### Running Tests

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests
cargo test --test integration_test

# Specific test module
cargo test lexer

# Test with features
cargo test --features chaos-mode

# Test in release mode
cargo test --release
```

### Test Categories

#### Unit Tests
Located alongside source code:

```rust
// src/lexer.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier_tokenization() {
        let mut lexer = Lexer::new("my_variable");
        let tokens = lexer.tokenize(&mut DiagnosticsEngine::new()).unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Identifier("my_variable".to_string()));
    }
}
```

#### Integration Tests
Located in `tests/` directory:

```rust
// tests/integration_test.rs
#[test]
fn test_hello_world_compilation() {
    let source = r#"
        actor Main {
            new create() =>
                println("Hello, World!")
        }
    "#;
    
    let result = compile_source(source);
    assert!(result.is_ok());
}
```

#### Chaos Tests
Test system behavior under failure conditions:

```rust
// tests/chaos_test.rs
#[test]
fn test_message_drop_resilience() {
    let mut chaos = ChaosEngine::new(ChaosConfig {
        drop_probability: 0.1,
        ..Default::default()
    });
    
    // Test that system remains functional with 10% message drops
    let result = run_actor_system_with_chaos(&chaos);
    assert!(result.system_stable);
    assert!(result.messages_processed > expected_minimum);
}
```

### Property-Based Testing

GAL uses property-based testing with `proptest`:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_lexer_roundtrip(source in "\\PC*") {
        // Property: lexing then pretty-printing should preserve semantics
        if let Ok(tokens) = tokenize(&source) {
            let reconstructed = pretty_print_tokens(&tokens);
            let retokenized = tokenize(&reconstructed)?;
            prop_assert_eq!(tokens, retokenized);
        }
    }
}
```

### Snapshot Testing

For AST and IR testing, we use `insta`:

```rust
use insta::assert_debug_snapshot;

#[test]
fn test_function_parsing() {
    let source = "fn add(a: Int, b: Int) -> Int { a + b }";
    let ast = parse_source(source).unwrap();
    assert_debug_snapshot!(ast);
}
```

### Performance Testing

Benchmark critical paths:

```rust
// benches/parser_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_parsing(c: &mut Criterion) {
    let source = include_str!("../examples/large_program.gal");
    
    c.bench_function("parse_large_program", |b| {
        b.iter(|| {
            let mut parser = Parser::new(tokenize(black_box(source)).unwrap());
            parser.parse(&mut DiagnosticsEngine::new())
        })
    });
}

criterion_group!(benches, benchmark_parsing);
criterion_main!(benches);
```

## Compiler Architecture

Understanding the compiler's internal architecture is crucial for effective development.

### Compilation Pipeline

```rust
// Simplified compilation flow
pub fn compile(source: &str) -> Result<CompiledProgram> {
    // 1. Lexical analysis
    let tokens = Lexer::new(source).tokenize()?;
    
    // 2. Parsing
    let ast = Parser::new(tokens).parse()?;
    
    // 3. Semantic analysis
    let typed_ast = SemanticAnalyzer::new().analyze(ast)?;
    
    // 4. IR generation
    let ir = IRGenerator::new().generate(typed_ast)?;
    
    // 5. Optimization
    let optimized_ir = Optimizer::new().optimize(ir)?;
    
    // 6. Code generation
    let native_code = CodeGenerator::new().generate(optimized_ir)?;
    
    Ok(CompiledProgram::new(native_code))
}
```

### Error Handling

GAL uses a structured error handling system:

```rust
// src/error.rs
#[derive(Debug, thiserror::Error)]
pub enum GalError {
    #[error("Lexical error at {position}: {message}")]
    LexError { position: Position, message: String },
    
    #[error("Parse error: {message}")]
    ParseError { span: Span, message: String },
    
    #[error("Type error: {message}")]
    TypeError { span: Span, message: String },
    
    #[error("Runtime error: {message}")]
    RuntimeError { message: String },
}

pub type Result<T> = std::result::Result<T, GalError>;
```

### Diagnostics System

Rich error reporting with source locations:

```rust
// src/diagnostics.rs
pub struct DiagnosticsEngine {
    errors: Vec<Diagnostic>,
    warnings: Vec<Diagnostic>,
    source_map: SourceMap,
}

impl DiagnosticsEngine {
    pub fn error(&mut self, span: Span, message: impl Into<String>) {
        self.errors.push(Diagnostic {
            level: DiagnosticLevel::Error,
            span,
            message: message.into(),
            suggestions: Vec::new(),
        });
    }
    
    pub fn emit(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        for diagnostic in &self.errors {
            self.emit_diagnostic(diagnostic, writer)?;
        }
        Ok(())
    }
}
```

### Memory Management

The compiler uses several strategies for memory efficiency:

```rust
// String interning for identifiers
pub struct StringInterner {
    strings: HashMap<String, StringId>,
    ids: Vec<String>,
}

// Arena allocation for AST nodes
pub struct AstArena {
    arena: typed_arena::Arena<AstNode>,
}

// Reference counting for shared data
pub type SharedAst = Arc<AstNode>;
```

## Contributing

We welcome contributions to GAL! Here's how to get involved.

### Contribution Process

1. **Fork** the repository on GitHub
2. **Create** a feature branch: `git checkout -b feature/amazing-feature`
3. **Commit** your changes: `git commit -m 'Add amazing feature'`
4. **Push** to the branch: `git push origin feature/amazing-feature`
5. **Open** a Pull Request

### Code Style

GAL follows Rust community conventions:

```bash
# Format code
cargo fmt

# Run clippy for lints
cargo clippy -- -D warnings

# Check formatting
cargo fmt -- --check
```

### Commit Messages

Use conventional commit format:

```
feat: add support for pattern matching in message handlers
fix: resolve memory leak in actor cleanup
docs: improve getting started guide
test: add integration tests for chaos engineering
refactor: simplify IR generation for function calls
```

### Pull Request Guidelines

- Keep PRs focused and atomic
- Include tests for new functionality
- Update documentation as needed
- Ensure CI passes
- Write clear commit messages
- Add entry to `CHANGELOG.md`

### Review Process

1. **Automated checks** must pass (CI, formatting, tests)
2. **Code review** by at least one maintainer
3. **Documentation review** for user-facing changes
4. **Performance review** for compiler changes
5. **Security review** for runtime changes

### Areas for Contribution

#### Beginner-Friendly
- Documentation improvements
- Example programs
- Test coverage
- Error message improvements
- Standard library functions

#### Intermediate
- Parser enhancements
- Type system features
- Optimization passes
- IDE support features
- Package manager features

#### Advanced
- Code generation backends
- Chaos engineering features
- Formal verification integration
- Distributed runtime
- Self-modification engine

## Debugging

### Compiler Debugging

#### Debug Builds
```bash
# Build with debug info
cargo build --features debug-mode

# Enable verbose logging
RUST_LOG=gal=debug galc program.gal

# Dump intermediate representations
galc --emit ast program.gal
galc --emit ir program.gal
galc --emit llvm-ir program.gal
```

#### GDB/LLDB Debugging
```bash
# Debug the compiler
gdb target/debug/galc
(gdb) run program.gal

# Debug generated programs
galc -g program.gal -o program
gdb ./program
```

#### Debugging Tools
```rust
// Add debug prints in compiler
debug!("Parsing function: {}", function_name);
trace!("Token: {:?}", token);

// Visualize AST
let ast_json = serde_json::to_string_pretty(&ast)?;
println!("AST: {}", ast_json);

// Profile compilation phases
let _timer = Timer::new("semantic_analysis");
```

### Runtime Debugging

#### Actor System Debugging
```bash
# Enable actor tracing
GAL_TRACE_ACTORS=1 ./program

# Chaos debugging
GAL_CHAOS_LOG=debug ./program

# Memory debugging with valgrind
valgrind --tool=memcheck ./program
```

#### Time-Travel Debugging
GAL includes built-in deterministic replay:

```gal
// Enable replay recording
@record_replay
actor DebuggableActor {
    // All message handling will be recorded
}

// Replay from recorded events
let replay = ReplaySystem::from_file("recording.gal-replay");
replay.debug_from_point(checkpoint_id);
```

## Performance Profiling

### Compiler Performance

```bash
# Profile compilation time
time galc large_program.gal

# Detailed profiling with perf
perf record -g galc large_program.gal
perf report

# Memory profiling with heaptrack
heaptrack galc large_program.gal
heaptrack_gui heaptrack.program.PID.gz
```

### Generated Code Performance

```bash
# Profile generated programs
perf record -g ./compiled_program
perf report

# Flamegraph generation
perf script | flamegraph > profile.svg

# Cache analysis
perf stat -e cache-misses,cache-references ./program
```

### Benchmarking

```bash
# Run benchmarks
cargo bench

# Compare with baseline
cargo bench -- --save-baseline baseline
# Make changes...
cargo bench -- --baseline baseline

# Continuous benchmarking
./scripts/bench-compare.sh main feature-branch
```

## Release Process

### Version Management

GAL follows semantic versioning:

- **Major**: Breaking changes to language or APIs
- **Minor**: New features, backward compatible
- **Patch**: Bug fixes, no new features

### Release Checklist

1. **Update version** in `Cargo.toml`
2. **Update CHANGELOG.md** with release notes
3. **Run full test suite** including chaos tests
4. **Update documentation** for new features
5. **Create release tag**: `git tag v1.2.3`
6. **Build release artifacts** for all platforms
7. **Publish to registry** (when available)
8. **Update website** and announcement

### Automation

```bash
# Release script
./scripts/release.sh 1.2.3

# This script:
# - Updates versions
# - Runs tests
# - Builds releases
# - Creates tags
# - Generates release notes
```

## Tool Development

### Language Server Development

```rust
// Extend the language server
impl LanguageServer for GalLanguageServer {
    fn completion(&self, params: CompletionParams) -> Result<CompletionList> {
        let position = params.text_document_position.position;
        let completions = self.compute_completions(position);
        Ok(CompletionList::new(false, completions))
    }
}
```

### IDE Extensions

```typescript
// VS Code extension (TypeScript)
import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {
    // Register chaos testing command
    const chaosTest = vscode.commands.registerCommand(
        'gal.runChaosTest',
        () => {
            // Run chaos tests for current file
        }
    );
    
    context.subscriptions.push(chaosTest);
}
```

### Build Tools

```rust
// Custom build tool
fn main() -> Result<()> {
    let manifest = PackageManifest::load("gal.toml")?;
    let builder = Builder::new(&manifest);
    
    builder
        .with_chaos_profile("testing")
        .with_optimization_level(2)
        .build()?;
    
    Ok(())
}
```

### Testing Tools

```rust
// Chaos testing framework
pub struct ChaosTestRunner {
    scenarios: Vec<ChaosScenario>,
    config: ChaosConfig,
}

impl ChaosTestRunner {
    pub fn run_scenario(&self, scenario: &ChaosScenario) -> TestResult {
        // Set up chaos conditions
        // Run test
        // Collect results
        // Analyze system behavior
    }
}
```

## Development Workflow

### Daily Development

```bash
# Start development session
git pull origin main
cargo build
cargo test

# Make changes
vim src/parser.rs

# Test changes
cargo test parser
cargo test --features chaos-mode

# Commit changes
git add .
git commit -m "feat: improve error recovery in parser"

# Push for review
git push origin feature/parser-improvement
```

### Release Workflow

```bash
# Prepare release
./scripts/prepare-release.sh 1.3.0

# This runs:
# - Full test suite
# - Documentation build
# - Benchmark comparison
# - Security audit
# - Performance regression tests

# If all passes:
./scripts/create-release.sh 1.3.0
```

---

This developer guide provides the foundation for contributing to GAL effectively. The project's architecture prioritizes correctness, performance, and maintainability, with comprehensive testing and tooling to support development at scale.