# GAL Performance Benchmarking System

A comprehensive, production-grade benchmarking framework for the GAL language that rivals industry standards like JMH (Java Microbenchmark Harness) and Criterion (Rust). This system provides statistical analysis, performance regression detection, hardware performance counter integration, and comprehensive reporting capabilities.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Benchmark Categories](#benchmark-categories)
- [Statistical Analysis](#statistical-analysis)
- [Performance Regression Detection](#performance-regression-detection)
- [Hardware Performance Counters](#hardware-performance-counters)
- [Memory Profiling](#memory-profiling)
- [Export Formats](#export-formats)
- [CI/CD Integration](#cicd-integration)
- [Configuration](#configuration)
- [Advanced Usage](#advanced-usage)
- [Comparison with Other Languages](#comparison-with-other-languages)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)

## Overview

The GAL benchmarking system is designed to provide comprehensive performance measurement and optimization capabilities across all aspects of the language:

- **Actor System Performance**: Creation, messaging, lifecycle management, and fault tolerance
- **Message Passing Efficiency**: Serialization, queue throughput, latency, and memory patterns
- **Compiler Optimization**: Compilation speed, optimization effectiveness, and cross-platform performance
- **Runtime System**: Memory allocation, garbage collection, and system resource utilization

## Features

### Core Features

- **Production-Grade Framework**: Statistical rigor with confidence intervals and significance testing
- **Comprehensive Coverage**: Actor systems, message passing, compiler, and runtime benchmarks
- **Performance Regression Detection**: Automated detection with configurable thresholds
- **Hardware Integration**: CPU performance counters, cache analysis, and memory profiling
- **Multi-Platform Support**: Native, WebAssembly, embedded, and GPU targets
- **Export Capabilities**: JSON, CSV, HTML, and Prometheus metrics
- **CI/CD Ready**: Automated testing with failure thresholds and baseline management

### Statistical Analysis

- Mean, median, standard deviation, min/max values
- Confidence intervals with configurable levels (90%, 95%, 99%)
- Percentile analysis (P10, P25, P50, P75, P90, P95, P99)
- Outlier detection using IQR (Interquartile Range) method
- Coefficient of variation for stability assessment
- Statistical significance testing for baseline comparisons

### Visualization and Reporting

- Interactive HTML reports with performance trends
- CSV exports for spreadsheet analysis
- JSON output for programmatic processing
- Prometheus metrics for monitoring system integration
- Performance score calculation and ranking
- Historical trend analysis with predictions

## Installation

The benchmarking system is included with GAL. Build the benchmark tool:

```bash
# Build the GAL compiler with benchmarking support
cargo build --release --bin gal-bench

# Or build everything
cargo build --release
```

## Quick Start

### Running Basic Benchmarks

```bash
# Run all benchmark suites
gal-bench run

# Run specific suites
gal-bench run --suites actor message compiler

# Quick benchmarks (reduced iterations)
gal-bench run --quick

# Verbose output
gal-bench run --verbose
```

### Viewing Results

```bash
# List available benchmarks
gal-bench list --detailed

# Export results to different formats
gal-bench run --export html
gal-bench run --export csv
```

### Baseline Management

```bash
# Save current results as baseline
gal-bench run --save-baseline

# Compare against baseline
gal-bench run --compare-baseline

# Fail on regression
gal-bench run --compare-baseline --fail-on-regression
```

## Benchmark Categories

### Actor System Benchmarks

#### Actor Creation and Lifecycle
- **Actor Creation**: Measures overhead of spawning new actors
- **Actor Destruction**: Cleanup and resource deallocation timing
- **Lifecycle Management**: Complete create-run-destroy cycles
- **Memory Usage**: Actor state and metadata overhead

```rust
// Example: Custom actor creation benchmark
use gal::benchmarks::actor_benchmarks::*;

let config = ActorCreationConfig {
    actors_per_iteration: 1000,
    actor_complexity: ActorComplexity::Simple,
    cleanup_between_iterations: true,
};

let benchmark = ActorCreationBenchmark::new(config);
```

#### Message Passing Performance
- **Point-to-Point**: Direct actor-to-actor messaging
- **Broadcast**: One-to-many message distribution
- **Round-Robin**: Load distribution patterns
- **Random**: Stress testing with unpredictable patterns

#### Supervisor System Performance
- **Fault Injection**: Simulated failure scenarios
- **Recovery Time**: Restart and escalation timing
- **Supervision Trees**: Hierarchical fault tolerance
- **Resource Management**: Memory and CPU during faults

### Message Passing Benchmarks

#### Serialization Performance
- **Format Comparison**: JSON vs Bincode vs MessagePack vs Protobuf
- **Message Size Impact**: Small (< 1KB) to Huge (> 100KB) messages
- **Complex Data Structures**: Nested objects, graphs, and binary data
- **Compression Efficiency**: Size reduction and speed trade-offs

```rust
// Example: Message serialization benchmark
use gal::benchmarks::message_benchmarks::*;

let config = SerializationConfig {
    message_count: 10000,
    message_types: vec![MessageType::Small, MessageType::Large],
    serialization_format: SerializationFormat::Bincode,
};

let benchmark = MessageSerializationBenchmark::new(config);
```

#### Queue Throughput and Latency
- **Producer-Consumer Patterns**: Various ratios and loads
- **Queue Types**: In-memory, persistent, distributed, priority
- **Latency Measurement**: End-to-end message timing
- **Backpressure Handling**: Queue overflow scenarios

#### Memory Allocation Patterns
- **Allocation Strategies**: Pool vs heap allocation
- **Fragmentation Analysis**: Memory efficiency over time
- **Garbage Collection**: Impact on message processing
- **Memory Reuse**: Object pooling effectiveness

### Compiler Benchmarks

#### Compilation Speed
- **Code Complexity**: From minimal to huge codebases
- **Optimization Levels**: None, basic, standard, aggressive
- **Incremental Compilation**: Change impact analysis
- **Cross-Platform Targets**: Native, WASM, embedded, GPU

```rust
// Example: Compilation speed benchmark
use gal::benchmarks::compiler_benchmarks::*;

let config = CompilerBenchmarkConfig {
    source_complexity: SourceComplexity::Large,
    optimization_level: OptimizationLevel::Aggressive,
    target_platform: TargetPlatform::Native,
    enable_incremental: true,
    memory_profiling: true,
};

let benchmark = CompilationSpeedBenchmark::new(config);
```

#### Optimization Effectiveness
- **Code Size Reduction**: Optimization impact on binary size
- **Runtime Performance**: Speed improvements from optimizations
- **Compilation Time**: Overhead of optimization passes
- **Memory Usage**: Compiler memory consumption during optimization

## Statistical Analysis

### Measurement Collection

The framework collects multiple measurements per benchmark iteration:

- **Timing Data**: High-resolution duration measurements
- **Hardware Counters**: CPU cycles, instructions, cache misses
- **Memory Metrics**: Allocations, deallocations, peak usage
- **Custom Metrics**: Benchmark-specific measurements

### Statistical Calculations

```rust
pub struct StatisticalAnalysis {
    pub mean: Duration,                           // Arithmetic mean
    pub median: Duration,                         // 50th percentile
    pub std_dev: Duration,                        // Standard deviation
    pub min: Duration,                            // Minimum value
    pub max: Duration,                            // Maximum value
    pub percentiles: HashMap<u8, Duration>,      // P10, P25, P75, P90, P95, P99
    pub confidence_interval: (Duration, Duration), // 95% confidence interval
    pub outliers: Vec<Duration>,                  // Detected outliers
    pub coefficient_of_variation: f64,            // Std dev / mean
}
```

### Confidence Intervals

Confidence intervals are calculated using the t-distribution for small samples and normal distribution for larger samples:

- **90% Confidence**: z-score ≈ 1.645
- **95% Confidence**: z-score ≈ 1.96
- **99% Confidence**: z-score ≈ 2.576

### Outlier Detection

Outliers are identified using the IQR (Interquartile Range) method:

```
Lower Bound = Q1 - 1.5 × IQR
Upper Bound = Q3 + 1.5 × IQR
```

Where IQR = Q3 - Q1 (75th percentile - 25th percentile)

## Performance Regression Detection

### Regression Analysis

The system automatically detects performance regressions by comparing current measurements against historical baselines:

```rust
pub struct BaselineComparison {
    pub baseline_mean: Duration,        // Historical baseline
    pub current_mean: Duration,         // Current measurement
    pub change_percentage: f64,         // Percentage change
    pub is_regression: bool,            // Exceeds regression threshold
    pub is_improvement: bool,           // Significant improvement
    pub statistical_significance: f64,  // Confidence in the change
}
```

### Regression Severity Levels

- **Critical**: > 20% performance degradation
- **High**: 10-20% degradation
- **Medium**: 5-10% degradation
- **Low**: 2-5% degradation
- **Minor**: < 2% degradation

### Automated Recommendations

The system provides actionable recommendations based on detected regressions:

```rust
// Example regression alert
RegressionAlert {
    benchmark_name: "actor_creation",
    severity: RegressionSeverity::High,
    change_percentage: 15.7,
    statistical_confidence: 0.95,
    description: "Actor creation is 15.7% slower than baseline",
    suggested_actions: vec![
        "Profile actor creation hot paths",
        "Review recent memory allocator changes",
        "Check for increased initialization overhead"
    ],
}
```

## Hardware Performance Counters

### Supported Metrics

When hardware performance counters are enabled, the system collects:

- **CPU Cycles**: Total processor cycles consumed
- **Instructions**: Number of instructions executed
- **Cache References**: L1/L2/L3 cache access patterns
- **Cache Misses**: Cache miss rates and penalties
- **Branch Instructions**: Branch prediction effectiveness
- **Page Faults**: Memory management overhead
- **Context Switches**: Process scheduling impact

### Derived Metrics

- **IPC (Instructions Per Cycle)**: CPU utilization efficiency
- **Cache Miss Rate**: Memory access patterns
- **Branch Miss Rate**: Branch prediction accuracy

### Platform Support

- **Linux**: perf_event interface
- **macOS**: Limited support via system APIs
- **Windows**: Performance counters API
- **Embedded**: Platform-specific implementations

## Memory Profiling

### Memory Metrics

The framework tracks comprehensive memory usage:

```rust
pub struct MemoryAnalysis {
    pub peak_memory: usize,           // Maximum memory usage
    pub average_memory: usize,        // Average memory consumption
    pub total_allocations: usize,     // Number of allocations
    pub allocation_rate: f64,         // Allocations per second
    pub fragmentation_ratio: f64,     // Memory fragmentation
    pub gc_pressure: f64,             // Garbage collection frequency
    pub memory_efficiency: f64,       // Operations per byte allocated
}
```

### Allocation Tracking

- **Allocation Patterns**: Size distribution and frequency
- **Deallocation Timing**: Memory lifetime analysis
- **Fragmentation Detection**: Heap fragmentation measurement
- **Pool Efficiency**: Memory pool utilization

### Garbage Collection Analysis

- **Collection Frequency**: GC trigger patterns
- **Collection Duration**: Stop-the-world time impact
- **Memory Recovery**: Freed memory per collection
- **Pressure Indicators**: Memory allocation rate vs GC rate

## Export Formats

### JSON Export

Comprehensive machine-readable format with all measurement data:

```json
{
  "name": "actor_creation",
  "group": "actor_system",
  "statistics": {
    "mean": "2.5ms",
    "median": "2.3ms",
    "std_dev": "0.4ms",
    "percentiles": {
      "95": "3.2ms",
      "99": "4.1ms"
    }
  },
  "baseline_comparison": {
    "change_percentage": -5.2,
    "is_improvement": true
  },
  "hardware_counters": {
    "cpu_cycles": 125000,
    "instructions": 95000,
    "ipc": 0.76,
    "cache_miss_rate": 0.024
  }
}
```

### CSV Export

Tabular format suitable for spreadsheet analysis:

```csv
name,group,mean_ns,median_ns,std_dev_ns,ops_per_second,change_percent
actor_creation,actor_system,2500000,2300000,400000,400.0,-5.2
message_throughput,message_passing,150000,145000,25000,6666.7,2.1
```

### HTML Export

Interactive reports with visualizations:

- Performance summary dashboard
- Detailed benchmark results table
- Regression/improvement highlighting
- Historical trend charts
- System metadata and configuration

### Prometheus Metrics

Integration with monitoring systems:

```prometheus
# HELP gal_benchmark_duration_seconds Benchmark execution time
# TYPE gal_benchmark_duration_seconds histogram
gal_benchmark_duration_seconds_bucket{benchmark="actor_creation",le="0.001"} 15
gal_benchmark_duration_seconds_bucket{benchmark="actor_creation",le="0.005"} 125
gal_benchmark_duration_seconds_bucket{benchmark="actor_creation",le="+Inf"} 200

# HELP gal_benchmark_operations_per_second Benchmark throughput
# TYPE gal_benchmark_operations_per_second gauge
gal_benchmark_operations_per_second{benchmark="actor_creation"} 400.0
```

## CI/CD Integration

### Continuous Performance Testing

```yaml
# .github/workflows/performance.yml
name: Performance Benchmarks

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Run benchmarks
        run: |
          cargo build --release --bin gal-bench
          ./target/release/gal-bench run \
            --compare-baseline \
            --fail-on-regression \
            --regression-threshold 5.0 \
            --export json

      - name: Upload results
        uses: actions/upload-artifact@v3
        with:
          name: benchmark-results
          path: benchmark-results/
```

### Performance Gates

Configure automatic failure on performance regression:

```bash
# Fail CI if any benchmark regresses by more than 10%
gal-bench run \
  --compare-baseline \
  --fail-on-regression \
  --regression-threshold 10.0

# CI-optimized quick benchmarks
gal-bench run \
  --quick \
  --compare-baseline \
  --max-time 300  # 5 minutes max
```

### Baseline Management

```bash
# Update baseline after confirmed performance improvements
gal-bench run --save-baseline

# Load baseline from file
gal-bench run --compare-baseline --baseline-file production-baseline.json

# Generate performance report for PR
gal-bench compare \
  --baseline baseline.json \
  --current pr-results.json \
  --format table \
  --min-change 2.0
```

## Configuration

### Configuration File

Create a `benchmark-config.json` file for consistent settings:

```json
{
  "warmup_iterations": 100,
  "measurement_iterations": 1000,
  "measurement_time_secs": 60,
  "confidence_level": 0.95,
  "regression_threshold": 0.05,
  "enable_perf_counters": true,
  "enable_memory_profiling": true,
  "environment": {
    "RUST_LOG": "info",
    "GAL_OPTIMIZE": "aggressive"
  }
}
```

### Environment Variables

- `GAL_BENCH_CONFIG`: Path to configuration file
- `GAL_BENCH_OUTPUT`: Output directory for results
- `GAL_BENCH_BASELINE`: Baseline file path
- `GAL_BENCH_VERBOSE`: Enable verbose output
- `GAL_BENCH_PERF_COUNTERS`: Enable hardware counters

### Command Line Options

```bash
gal-bench run [OPTIONS]

OPTIONS:
    --suites <SUITES>              Benchmark suites to run [actor, message, compiler, all]
    --benchmarks <BENCHMARKS>      Specific benchmark names to run
    --quick                        Run quick benchmarks (reduced iterations)
    --platform <PLATFORM>         Target platform [native, wasm, embedded, gpu]
    --export <FORMAT>              Export format [json, csv, html]
    --save-baseline               Save results as baseline
    --compare-baseline            Compare against baseline
    --fail-on-regression          Fail if regression detected
    --regression-threshold <PCT>   Regression threshold percentage
    --warmup <COUNT>              Number of warmup iterations
    --iterations <COUNT>          Number of measurement iterations
    --max-time <SECONDS>          Maximum time per benchmark
    --memory-profiling            Enable memory profiling
    --perf-counters              Enable hardware performance counters
    --output <DIR>               Output directory for results
    --config <FILE>              Configuration file path
    --verbose                    Enable verbose output
```

## Advanced Usage

### Custom Benchmarks

Implement custom benchmarks using the framework:

```rust
use gal::benchmarks::framework::{BenchmarkFunction, BenchmarkError};
use std::collections::HashMap;

struct CustomBenchmark {
    iterations: usize,
}

impl BenchmarkFunction for CustomBenchmark {
    fn setup(&mut self) -> Result<(), BenchmarkError> {
        // Initialize resources
        Ok(())
    }

    fn execute(&mut self) -> Result<(), BenchmarkError> {
        // Run the benchmark operation
        for _ in 0..self.iterations {
            // Your benchmark code here
            std::thread::sleep(std::time::Duration::from_micros(100));
        }
        Ok(())
    }

    fn teardown(&mut self) -> Result<(), BenchmarkError> {
        // Clean up resources
        Ok(())
    }

    fn get_name(&self) -> &str {
        "custom_benchmark"
    }

    fn get_group(&self) -> &str {
        "custom"
    }

    fn get_parameters(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("iterations".to_string(), self.iterations.to_string());
        params
    }
}

// Run the custom benchmark
let framework = BenchmarkFramework::new();
let benchmark = Box::new(CustomBenchmark { iterations: 1000 });
let result = framework.run_benchmark(benchmark)?;
```

### Performance Profiling Integration

```rust
use gal::benchmarks::framework::PerformanceProfiler;

struct CustomProfiler {
    // Custom profiling implementation
}

impl PerformanceProfiler for CustomProfiler {
    fn start_profiling(&mut self) -> Result<(), BenchmarkError> {
        // Start collecting performance data
        Ok(())
    }

    fn stop_profiling(&mut self) -> Result<HardwareCounters, BenchmarkError> {
        // Return collected performance counters
        Ok(HardwareCounters::default())
    }

    fn get_memory_usage(&self) -> Result<MemoryUsage, BenchmarkError> {
        // Return current memory usage
        Ok(MemoryUsage::default())
    }
}

let profiler = Box::new(CustomProfiler {});
let framework = BenchmarkFramework::new().with_profiler(profiler);
```

### Batch Processing

```rust
// Run multiple benchmark suites with different configurations
let configurations = vec![
    BenchmarkConfig { warmup_iterations: 50, ..Default::default() },
    BenchmarkConfig { warmup_iterations: 100, ..Default::default() },
    BenchmarkConfig { warmup_iterations: 200, ..Default::default() },
];

for config in configurations {
    let framework = BenchmarkFramework::with_config(config);
    let suite = create_actor_benchmark_suite();
    let results = framework.run_suite(suite)?;
    
    // Process results
    for result in results {
        println!("Benchmark: {}, Mean: {:?}", result.name, result.statistics.mean);
    }
}
```

## Comparison with Other Languages

### Benchmark Equivalents

| GAL Component | Rust (Criterion) | Java (JMH) | Go (testing) | Erlang/Elixir |
|---------------|------------------|------------|--------------|---------------|
| Actor Creation | tokio::spawn | CompletableFuture | goroutine | spawn |
| Message Passing | mpsc::channel | ArrayBlockingQueue | channel | ! (send) |
| Serialization | serde | Jackson | encoding/json | term_to_binary |
| Compilation | rustc | javac | go build | erlc |

### Performance Targets

Based on industry benchmarks, GAL aims for:

- **Actor Creation**: < 10μs per actor (comparable to Rust async tasks)
- **Message Throughput**: > 1M messages/second (comparable to Erlang)
- **Compilation Speed**: < 100ms per 1000 LOC (faster than rustc)
- **Memory Efficiency**: < 1KB overhead per actor (better than JVM)

### Cross-Language Benchmarks

```bash
# Compare GAL against other languages
gal-bench run --export csv > gal-results.csv

# Generate comparison report
gal-bench compare \
  --baseline rust-criterion-results.csv \
  --current gal-results.csv \
  --format table
```

## Best Practices

### Benchmark Design

1. **Isolation**: Each benchmark should test one specific aspect
2. **Repeatability**: Results should be consistent across runs
3. **Relevance**: Benchmarks should reflect real-world usage patterns
4. **Scalability**: Test performance across different scales

### Measurement Accuracy

1. **Warmup**: Always include sufficient warmup iterations
2. **Duration**: Run benchmarks long enough for statistical significance
3. **Environment**: Control for external factors (CPU frequency, background tasks)
4. **Precision**: Use high-resolution timers and multiple measurements

### Regression Prevention

1. **Continuous Monitoring**: Run benchmarks on every commit
2. **Baseline Management**: Regularly update baselines after confirmed improvements
3. **Threshold Tuning**: Set appropriate regression thresholds for different components
4. **Root Cause Analysis**: Investigate all regressions promptly

### Performance Optimization Workflow

1. **Profile First**: Identify actual bottlenecks before optimizing
2. **Measure Impact**: Benchmark before and after optimizations
3. **Validate Changes**: Ensure optimizations don't break functionality
4. **Document Results**: Record optimization techniques and their impact

## Troubleshooting

### Common Issues

#### High Variability in Results

**Symptoms**: Large standard deviation, high coefficient of variation

**Causes**:
- Background system activity
- CPU frequency scaling
- Thermal throttling
- Insufficient warmup

**Solutions**:
```bash
# Increase warmup iterations
gal-bench run --warmup 200

# Run longer measurements
gal-bench run --max-time 120

# Isolate system
sudo sysctl -w kernel.perf_event_paranoid=1  # Linux
```

#### Performance Counter Access Issues

**Symptoms**: Missing hardware counter data

**Causes**:
- Insufficient permissions
- Virtualized environment
- Unsupported hardware

**Solutions**:
```bash
# Linux: Grant perf access
echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid

# Run without performance counters
gal-bench run --no-perf-counters

# Check system capabilities
cat /proc/cpuinfo | grep flags
```

#### Memory Profiling Failures

**Symptoms**: No memory usage data collected

**Causes**:
- Platform limitations
- Debug symbols missing
- Profiler conflicts

**Solutions**:
```bash
# Build with debug info
cargo build --release --bin gal-bench

# Disable memory profiling
gal-bench run --no-memory-profiling

# Use alternative profilers
MALLOC_CONF="prof:true" gal-bench run
```

### Debug Information

Enable debug output for troubleshooting:

```bash
# Maximum verbosity
RUST_LOG=debug gal-bench run --verbose

# Benchmark-specific logging
RUST_LOG=gal::benchmarks=trace gal-bench run

# Save debug information
gal-bench run --verbose 2>&1 | tee benchmark-debug.log
```

### Performance Analysis

When benchmarks show unexpected results:

1. **Check System State**:
   ```bash
   # CPU usage
   top -bn1 | grep "Cpu(s)"
   
   # Memory pressure
   free -h
   
   # Disk I/O
   iostat -x 1 5
   ```

2. **Verify Configuration**:
   ```bash
   # Check benchmark parameters
   gal-bench list --detailed --parameters
   
   # Validate configuration
   gal-bench run --dry-run --verbose
   ```

3. **Compare Baselines**:
   ```bash
   # Historical comparison
   gal-bench compare \
     --baseline last-week-baseline.json \
     --current current-results.json
   ```

## Contributing

### Adding New Benchmarks

1. **Design**: Plan the benchmark scope and methodology
2. **Implement**: Create benchmark following the framework patterns
3. **Test**: Validate accuracy and repeatability
4. **Document**: Add documentation and usage examples
5. **Integrate**: Add to appropriate benchmark suite

### Improving Framework

1. **Statistical Methods**: Enhance analysis algorithms
2. **Platform Support**: Add support for new platforms
3. **Export Formats**: Implement additional export formats
4. **Performance**: Optimize benchmark framework overhead

### Reporting Issues

Include in bug reports:
- GAL version and build configuration
- Operating system and hardware details
- Benchmark configuration and command line
- Complete error output and logs
- Reproducible test case if possible

---

*This document covers the comprehensive GAL benchmarking system. For additional information, see the API documentation and source code examples.*