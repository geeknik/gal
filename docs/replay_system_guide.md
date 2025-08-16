# GAL Deterministic Replay System

The GAL Deterministic Replay System is the world's first programming language with built-in deterministic replay and time-travel debugging capabilities. It enables perfect reproduction of distributed system behaviors, including chaos scenarios, for debugging and analysis.

## Overview

The replay system consists of two main components:

1. **Recording Engine** (`replay.rs`) - Captures all relevant system events during execution
2. **Replay Runtime** (`replay_runtime.rs`) - Reproduces execution with exact timing and enables time-travel debugging

## Key Features

### Event Recording Infrastructure
- **Comprehensive Event Capture**: Records all actor messages, state changes, I/O operations, and chaos injections
- **Deterministic Message Ordering**: Uses logical clocks for consistent event ordering
- **State Snapshots**: Configurable intervals for efficient time-travel navigation
- **Compact Binary Format**: Compressed event logs with minimal storage overhead
- **Privacy-Aware Recording**: Redacts sensitive data while preserving debugging information

### Time-Travel Debugging
- **Bidirectional Navigation**: Step forward/backward through execution
- **Jump to Specific Points**: Navigate to events, timestamps, or conditions
- **Breakpoint Support**: Event-based, time-based, and conditional breakpoints
- **State Inspection**: Examine actor state at any point in execution
- **Watch Expressions**: Monitor values that change during replay

### Production Features
- **Continuous Recording**: Ring buffer for ongoing recording with bounded memory
- **Selective Recording**: Filter by actors, event types, or custom criteria
- **Performance Monitoring**: <5% overhead with adaptive configuration
- **Divergence Detection**: Alerts when replay differs from original execution

## Quick Start

### Basic Recording

```gal
// Configure recording
let config = RecordingConfig {
    max_events: Some(1_000_000),
    snapshot_interval: 1s,
    record_chaos: true,
    record_timing: true
}

// Start recording
let system = DeterministicReplaySystem.new(config)
let session = system.start_recording("my_recording.chaos")

// Your application code runs here
// All events are automatically recorded

// Stop recording
let stats = session.stop()
println("Recorded {} events", stats.total_events_recorded)
```

### Basic Replay

```gal
// Load recording
let recording = replay.load_recording("my_recording.chaos")

// Configure replay
let config = ReplayConfig {
    deterministic_mode: true,
    speed_multiplier: 1.0,
    enable_breakpoints: true
}

// Create replay runtime
let runtime = ReplayRuntime.new(recording, config)

// Set breakpoints
runtime.set_event_breakpoint(event_id: 12345)
runtime.set_time_breakpoint(timestamp: LogicalTimestamp(5000))

// Start replay
runtime.start()

// Control replay
runtime.pause()
runtime.step_forward()
runtime.jump_to_time(LogicalTimestamp(10000))
runtime.resume()
```

## Recording Configuration

### Basic Configuration

```gal
let config = RecordingConfig {
    // Event limits
    max_events: Some(10_000_000),          // Maximum events to record
    event_buffer_size: 100_000,            // Buffer size for batching
    
    // Snapshot settings
    snapshot_interval: Duration.from_secs(30), // Snapshot every 30 seconds
    
    // Performance settings
    enable_compression: true,               // Compress recordings
    recording_overhead_target: 0.05,       // Target 5% overhead
    
    // What to record
    record_io: false,                      // Skip I/O in production
    record_chaos: true,                    // Always record chaos events
    record_timing: true,                   // Record precise timing
    
    // Privacy settings
    actor_filter: Some(["PaymentProcessor", "OrderHandler"]), // Only critical actors
}
```

### Privacy and Security

```gal
let config = RecordingConfig {
    // Redaction rules for sensitive data
    redaction_rules: [
        RedactionRule {
            pattern: r"credit_card:\s*\d{4}-\d{4}-\d{4}-\d{4}",
            replacement: "credit_card: [REDACTED]",
            applies_to: [EventType.MessageSent, EventType.StateChange]
        },
        RedactionRule {
            pattern: r"ssn:\s*\d{3}-\d{2}-\d{4}",
            replacement: "ssn: [REDACTED]",
            applies_to: [EventType.StateChange]
        }
    ],
    
    // Actor-specific privacy
    actor_privacy: HashMap.from([
        ("UserActor", PrivacySettings {
            record_state: false,        // Don't record user state
            record_messages: true,      // But do record message flow
            redact_payloads: true       // Redact message content
        })
    ])
}
```

## Replay Configuration

### Debugging Configuration

```gal
let config = ReplayConfig {
    // Replay behavior
    deterministic_mode: true,              // Exact timing reproduction
    speed_multiplier: 0.1,                 // Slow motion for debugging
    stop_on_divergence: true,              // Stop if replay differs
    
    // Debugging features
    enable_breakpoints: true,              // Enable all breakpoint types
    max_replay_duration: Some(Duration.from_hours(1)), // Safety timeout
}
```

### Analysis Configuration

```gal
let config = ReplayConfig {
    deterministic_mode: true,
    speed_multiplier: 10.0,                // Fast replay for analysis
    enable_breakpoints: false,             // Skip breakpoints
    stop_on_divergence: false,             // Continue through divergences
}
```

## Breakpoints and Debugging

### Event Breakpoints

```gal
// Break when specific event occurs
runtime.set_event_breakpoint(EventId(12345), None)

// Break with condition
runtime.set_event_breakpoint(
    EventId(67890), 
    Some("message.amount > 1000")
)
```

### Time Breakpoints

```gal
// Break at specific time
runtime.set_time_breakpoint(LogicalTimestamp(5000), None)

// Break with condition
runtime.set_time_breakpoint(
    LogicalTimestamp(10000),
    Some("system.error_count > 5")
)
```

### Actor Breakpoints

```gal
// Break when actor receives message
runtime.set_actor_breakpoint(
    actor_id: ActorId(123),
    trigger: ActorTrigger.MessageReceived { message_type: Some("ProcessPayment") },
    condition: Some("message.amount > 10000")
)

// Break when actor state changes
runtime.set_actor_breakpoint(
    actor_id: ActorId(456),
    trigger: ActorTrigger.StateChanged { field_path: Some("error_count") },
    condition: Some("new_value > old_value")
)

// Break when chaos is injected
runtime.set_actor_breakpoint(
    actor_id: ActorId(789),
    trigger: ActorTrigger.FaultInjected,
    condition: None
)
```

### Watch Expressions

```gal
// Monitor values during replay
runtime.add_watch_expression(
    expression: "processor.total_processed",
    frequency: EvaluationFrequency.EveryEvent
)

runtime.add_watch_expression(
    expression: "system.memory_usage_mb",
    frequency: EvaluationFrequency.EveryNEvents(100)
)

runtime.add_watch_expression(
    expression: "chaos.active_faults.len()",
    frequency: EvaluationFrequency.OnStateChange
)
```

## State Inspection

### Actor State Inspection

```gal
// Inspect current state of an actor
let inspection = runtime.inspect_actor_state(ActorId(123))
println("Actor state: {}", inspection.formatted_state)
println("Health: {:?}", inspection.health_status)

// Get all fields as key-value pairs
for (field, value) in inspection.fields {
    println("{}: {}", field, value)
}
```

### State Change Tracking

```gal
// Get all state changes for an actor
let changes = runtime.get_state_changes(ActorId(123))

for change in changes {
    println("Time {}: {} changed from {:?} to {}",
           change.timestamp.0,
           change.field_path,
           change.old_value,
           change.new_value)
}
```

## Time Control

### Navigation

```gal
// Basic navigation
runtime.step_forward()                     // Step one event forward
runtime.step_backward()                    // Step one event backward (time travel)

// Jump to specific points
runtime.jump_to_time(LogicalTimestamp(5000))
runtime.jump_to_event(EventId(12345))

// Speed control
runtime.set_speed(0.1)                     // Slow motion
runtime.set_speed(10.0)                    // Fast forward

// Bulk navigation
runtime.fast_forward(LogicalTimestamp(10000))
runtime.rewind(LogicalTimestamp(1000))
```

### Execution Control

```gal
// Control replay execution
runtime.start()                           // Start replay
runtime.pause()                           // Pause at current point
runtime.resume()                          // Resume from pause
runtime.stop()                            // Stop replay

// Check current state
match runtime.get_execution_state() {
    ExecutionState.Running { speed } => {
        println("Running at {}x speed", speed)
    }
    ExecutionState.Paused { position, reason } => {
        println("Paused at {} due to {:?}", position.0, reason)
    }
    ExecutionState.Completed => {
        println("Replay completed successfully")
    }
    ExecutionState.Diverged { divergence } => {
        println("Divergence: {}", divergence.description)
    }
}
```

## Causal Analysis

### Causal Dependencies

```gal
// Get events that led to a specific event
let dependencies = runtime.get_causal_dependencies(EventId(12345))
for dep_id in dependencies {
    println("Event {} depends on event {}", 12345, dep_id.0)
}
```

### Causal Graph Visualization

```gal
// Generate causal graph
let options = CausalRenderingOptions {
    show_timing: true,                     // Show timing information
    show_actor_boundaries: true,           // Group by actor
    highlight_critical_path: true,         // Highlight critical path
    color_by_actor: true,                  // Color code by actor
    filter_event_types: ["MessageSent", "ChaosInjection"] // Only show these event types
}

let graph_dot = runtime.generate_causal_graph(options)
write_file("causal_graph.dot", graph_dot)

// Convert to image using graphviz
system("dot -Tpng causal_graph.dot -o causal_graph.png")
```

## Message Flow Analysis

### Flow Metrics

```gal
// Get message flow between actors
let metrics = runtime.get_message_flow(
    sender: ActorId(123),
    recipient: ActorId(456)
)

println("Messages: {}", metrics.message_count)
println("Total bytes: {}", metrics.total_bytes)
println("Average latency: {:?}", metrics.average_latency)
println("Error rate: {}", metrics.error_count as f64 / metrics.message_count as f64)
```

### Flow Visualization

```gal
// Generate message flow diagram
let options = FlowRenderingOptions {
    show_latencies: true,                  // Show message latencies
    show_volumes: true,                    // Show message volumes
    show_errors: true,                     // Highlight errors
    animate_flows: false,                  // Static diagram
    filter_actors: Some([ActorId(123), ActorId(456)]), // Only these actors
    time_window: Some((LogicalTimestamp(1000), LogicalTimestamp(5000))) // Time range
}

let flow_svg = runtime.generate_flow_diagram(options)
write_file("message_flow.svg", flow_svg)
```

## Chaos Event Analysis

### Chaos Event Tracking

```gal
// Find all chaos events in a time range
let chaos_events = runtime.get_chaos_events_in_range(
    start: LogicalTimestamp(1000),
    end: LogicalTimestamp(5000)
)

for event in chaos_events {
    println("Time {}: {} injected into actor {}",
           event.timestamp.0,
           event.fault_type,
           event.target_actor.0)
           
    for (param, value) in event.parameters {
        println("  {}: {}", param, value)
    }
}
```

### Chaos Impact Analysis

```gal
// Analyze impact of chaos injections
let analysis = runtime.analyze_chaos_impact()

println("Total chaos events: {}", analysis.total_chaos_events)
println("Actors affected: {}", analysis.affected_actors.len())
println("Average recovery time: {:?}", analysis.average_recovery_time)

for (fault_type, impact) in analysis.fault_impact {
    println("{}: {} occurrences, {} failures caused",
           fault_type, impact.occurrence_count, impact.failure_count)
}
```

## Production Usage

### Continuous Recording

```gal
// Set up continuous recording with ring buffer
let config = RecordingConfig {
    max_events: None,                      // Unlimited events
    ring_buffer_size: 50_000_000,          // 50M event ring buffer
    ring_buffer_duration: Duration.from_hours(24), // 24 hour window
    enable_compression: true,
    record_timing: false,                  // Skip timing in production
    privacy_mode: PrivacyMode.ProductionSafe
}

let session = replay.start_continuous_recording("/var/log/gal/recordings", config)

// Recording runs continuously in background
// Save recording when issues are detected
if detect_anomaly() {
    let saved_recording = session.save_current_window("incident_{}", incident_id)
    analyze_incident(saved_recording.path)
}
```

### Performance Monitoring

```gal
// Monitor recording performance
let stats = session.get_current_stats()

println("Recording overhead: {:.2}%", stats.overhead_percentage)
println("Events per second: {}", stats.events_per_second)
println("Buffer utilization: {:.1}%", stats.buffer_utilization * 100.0)
println("Compression ratio: {:.2}:1", stats.compression_ratio)

// Adaptive configuration based on performance
if stats.overhead_percentage > 5.0 {
    session.reduce_recording_detail()
}
```

### Incident Analysis

```gal
fn analyze_incident(recording_path: str) {
    let recording = replay.load_recording(recording_path)
    let runtime = ReplayRuntime.new(recording, ReplayConfig {
        speed_multiplier: 50.0,            // Fast analysis
        deterministic_mode: true,
        enable_breakpoints: false          // No manual debugging
    })
    
    // Automated analysis
    let analysis = runtime.run_automated_analysis([
        "find_error_patterns",
        "trace_error_causes",
        "identify_performance_bottlenecks",
        "detect_chaos_correlations"
    ])
    
    // Generate incident report
    let report = IncidentReport {
        recording_path,
        analysis_results: analysis,
        causal_graph: runtime.generate_causal_graph(CausalRenderingOptions.default()),
        message_flow: runtime.generate_flow_diagram(FlowRenderingOptions.default()),
        recommendations: generate_recommendations(analysis)
    }
    
    save_incident_report(report)
}
```

## Best Practices

### Recording Best Practices

1. **Production Configuration**
   - Use selective recording to minimize overhead
   - Enable compression to reduce storage
   - Configure privacy rules for sensitive data
   - Monitor performance metrics continuously

2. **Development Configuration**
   - Record everything for comprehensive debugging
   - Use shorter snapshot intervals
   - Enable detailed timing information
   - Include all event types

3. **Testing Configuration**
   - Use deterministic seeds for reproducible chaos
   - Record all chaos events and their effects
   - Take frequent snapshots for time travel
   - Monitor state changes closely

### Debugging Best Practices

1. **Efficient Navigation**
   - Use snapshots to jump to approximate locations
   - Set breakpoints before detailed inspection
   - Use watch expressions to monitor key values
   - Start with fast replay, then slow down for details

2. **State Inspection**
   - Focus on state changes rather than static state
   - Use field-specific breakpoints for efficiency
   - Compare state before and after events
   - Track causal relationships for complex bugs

3. **Performance Analysis**
   - Use message flow diagrams for bottlenecks
   - Analyze chaos event correlation with performance
   - Monitor resource usage during replay
   - Generate automated reports for patterns

### Integration Best Practices

1. **CI/CD Integration**
   ```gal
   // In test pipelines
   let test_recording = run_test_with_recording("critical_test")
   if test_recording.has_performance_regression() {
       fail_build("Performance regression detected")
   }
   ```

2. **Monitoring Integration**
   ```gal
   // Alert on recording issues
   if recording_session.get_overhead() > 10.0 {
       send_alert("Recording overhead too high: {}%", overhead)
   }
   ```

3. **Incident Response**
   ```gal
   // Automated incident analysis
   on_incident_detected(incident) => {
       let recording = save_incident_recording(incident)
       let analysis = analyze_recording_async(recording)
       notify_oncall_team(incident, analysis)
   }
   ```

## API Reference

For complete API documentation, see:
- [`DeterministicReplaySystem`](../src/runtime/replay.rs) - Main recording system
- [`ReplayRuntime`](../src/runtime/replay_runtime.rs) - Replay execution environment
- [Event Types](../src/replay/events.rs) - All recordable event types
- [Configuration](../src/replay/mod.rs) - Configuration options

## Examples

See the [examples directory](../examples/) for complete working examples:
- [`deterministic_replay_example.gal`](../examples/deterministic_replay_example.gal) - Comprehensive usage example
- [`chaos_replay_demo.gal`](../examples/chaos_replay_demo.gal) - Chaos engineering with replay
- [`production_monitoring.gal`](../examples/production_monitoring.gal) - Production deployment patterns

## Troubleshooting

### Common Issues

1. **High Recording Overhead**
   - Reduce event buffer size
   - Enable selective recording
   - Increase snapshot intervals
   - Disable detailed timing

2. **Large Recording Files**
   - Enable compression
   - Use shorter recording windows
   - Filter unnecessary events
   - Implement privacy redaction

3. **Replay Divergences**
   - Check for non-deterministic code
   - Verify chaos seeds are consistent
   - Review timing-dependent logic
   - Analyze external dependencies

4. **Time Travel Performance**
   - Increase snapshot frequency
   - Use incremental snapshots
   - Optimize state serialization
   - Cache frequently accessed positions

For more help, see the [troubleshooting guide](troubleshooting.md) or file an issue on GitHub.