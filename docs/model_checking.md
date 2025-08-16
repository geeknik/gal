# GAL Model Checking: Comprehensive Formal Verification

GAL's model checking system provides world-class formal verification capabilities for concurrent actor systems. This comprehensive guide covers all aspects of model checking in GAL, from basic deadlock detection to advanced temporal logic verification.

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Model Checking Backends](#model-checking-backends)
- [Property Specification](#property-specification)
- [Deadlock and Livelock Detection](#deadlock-and-livelock-detection)
- [Temporal Logic Verification](#temporal-logic-verification)
- [Communication Pattern Verification](#communication-pattern-verification)
- [State Space Exploration](#state-space-exploration)
- [Counterexample Analysis](#counterexample-analysis)
- [Performance Optimization](#performance-optimization)
- [Integration with SMT Solvers](#integration-with-smt-solvers)
- [CLI Reference](#cli-reference)
- [Configuration](#configuration)
- [Examples](#examples)
- [Advanced Topics](#advanced-topics)
- [Troubleshooting](#troubleshooting)

## Overview

GAL's model checking system integrates multiple formal verification tools and techniques:

- **TLA+ Integration**: Generate TLA+ specifications and verify with TLC
- **SPIN Integration**: Create Promela models for efficient verification
- **NuSMV Integration**: Symbolic model checking for CTL properties
- **Custom State Explorer**: Optimized state space exploration with reduction techniques
- **SMT Solver Integration**: Z3/CVC5 integration for decidable theories
- **Petri Net Modeling**: Actor systems as Petri nets for specific analyses

### Key Features

- **Automatic Model Extraction**: Generate formal models directly from GAL source code
- **Multiple Property Types**: Safety, liveness, fairness, and reachability properties
- **Deadlock/Livelock Detection**: Comprehensive concurrency bug detection
- **Communication Verification**: Verify actor communication patterns and protocols
- **Counterexample Generation**: Detailed traces for property violations
- **State Space Reduction**: Partial order reduction, symmetry reduction, abstraction
- **Interactive Verification**: Step-by-step verification with user guidance
- **Performance Analysis**: Benchmarking and optimization recommendations

## Quick Start

### Installation

First, ensure you have the required model checking tools installed:

```bash
# Install TLA+ (TLC)
wget https://github.com/tlaplus/tlaplus/releases/download/v1.8.0/tla2tools.jar
export TLC_PATH="/path/to/tla2tools.jar"

# Install SPIN
sudo apt-get install spin  # Ubuntu/Debian
brew install spin         # macOS

# Install NuSMV
sudo apt-get install nusmv  # Ubuntu/Debian
brew install nusmv         # macOS

# Install Z3 (optional)
sudo apt-get install z3    # Ubuntu/Debian
brew install z3           # macOS
```

### Basic Usage

```bash
# Check for deadlocks
gal-model-check deadlock examples/bank_account.gal

# Verify temporal properties
gal-model-check temporal examples/bank_account.gal --properties properties.json

# Interactive verification session
gal-model-check interactive examples/bank_account.gal

# Export models for external tools
gal-model-check export examples/bank_account.gal --output models/ --formats tla-plus spin nusmv
```

### Simple Example

Consider this GAL actor system:

```gal
actor BankAccount {
    state {
        balance: Int = 0,
        locked: Bool = false,
    }
    
    message Deposit(amount: Int)
    message Withdraw(amount: Int)
    message Lock()
    message Unlock()
    
    handler Deposit(amount) {
        requires !locked && amount > 0
        ensures balance == old(balance) + amount
        
        balance += amount;
    }
    
    handler Withdraw(amount) {
        requires !locked && amount > 0 && balance >= amount
        ensures balance == old(balance) - amount
        
        balance -= amount;
    }
    
    handler Lock() {
        requires !locked
        ensures locked
        
        locked = true;
    }
    
    handler Unlock() {
        requires locked
        ensures !locked
        
        locked = false;
    }
    
    invariant balance >= 0
    invariant !(locked && balance < 0)
}

actor Bank {
    state {
        accounts: Map<String, ActorRef<BankAccount>>,
        total_balance: Int = 0,
    }
    
    message CreateAccount(name: String)
    message Transfer(from: String, to: String, amount: Int)
    
    handler CreateAccount(name) {
        let account = spawn BankAccount();
        accounts[name] = account;
    }
    
    handler Transfer(from, to, amount) {
        requires from in accounts && to in accounts
        requires amount > 0
        
        let from_account = accounts[from];
        let to_account = accounts[to];
        
        send from_account.Withdraw(amount);
        send to_account.Deposit(amount);
    }
    
    invariant total_balance == sum(account.balance for account in accounts.values())
}
```

Check this system for deadlocks:

```bash
gal-model-check deadlock bank_system.gal --max-states 100000 --por
```

## Model Checking Backends

### TLA+ Integration

TLA+ (Temporal Logic of Actions) is excellent for specifying and verifying concurrent systems.

**Strengths:**
- Mature and well-tested model checker (TLC)
- Excellent for temporal logic properties
- Strong mathematical foundation
- Good support for fairness conditions

**When to use:**
- Complex temporal properties
- Systems with intricate timing requirements
- When you need mathematical rigor

**Example generated TLA+ specification:**

```tla
---- MODULE BankAccount ----
EXTENDS Naturals, Sequences, TLC

CONSTANTS AccountInstances, MaxMessages, MaxSteps

VARIABLES
  accounts,      \* Set of account states
  messages,      \* Message queue
  step,          \* Current step
  account_balance,
  account_locked

TypeInvariant ==
  /\ accounts \in SUBSET AccountState
  /\ messages \in Seq(Message)
  /\ step \in Nat
  /\ account_balance \in [AccountInstances -> Nat]
  /\ account_locked \in [AccountInstances -> BOOLEAN]

Init ==
  /\ accounts = {}
  /\ messages = <<>>
  /\ step = 0
  /\ account_balance = [i \in AccountInstances |-> 0]
  /\ account_locked = [i \in AccountInstances |-> FALSE]

Deposit(account, amount) ==
  /\ ~account_locked[account]
  /\ amount > 0
  /\ account_balance' = [account_balance EXCEPT ![account] = @ + amount]
  /\ UNCHANGED <<accounts, messages, step, account_locked>>

Withdraw(account, amount) ==
  /\ ~account_locked[account]
  /\ amount > 0
  /\ account_balance[account] >= amount
  /\ account_balance' = [account_balance EXCEPT ![account] = @ - amount]
  /\ UNCHANGED <<accounts, messages, step, account_locked>>

Next ==
  \/ \E account \in AccountInstances, amount \in 1..100 :
       Deposit(account, amount)
  \/ \E account \in AccountInstances, amount \in 1..100 :
       Withdraw(account, amount)
  \/ UNCHANGED <<accounts, messages, step, account_balance, account_locked>>

Fairness ==
  /\ WF_vars(\E account \in AccountInstances : ProcessMessage(account))
  /\ SF_vars(SendMessage)

Spec == Init /\ [][Next]_vars /\ Fairness

\* Safety property: Balance never goes negative
BalanceNonNegative == 
  [](\A account \in AccountInstances : account_balance[account] >= 0)

\* Liveness property: Eventually all messages are processed
AllMessagesProcessed == 
  <>(Len(messages) = 0)

THEOREM Spec => BalanceNonNegative
THEOREM Spec => AllMessagesProcessed
====
```

### SPIN Integration

SPIN with Promela models excels at finding bugs in concurrent systems.

**Strengths:**
- Efficient state space exploration
- Excellent deadlock detection
- Fast verification for safety properties
- Good memory optimization

**When to use:**
- Deadlock detection
- Reachability analysis
- Safety property verification
- Systems with many concurrent processes

**Example generated Promela model:**

```promela
/* Generated Promela model for GAL actor system */

mtype = { Deposit, Withdraw, Lock, Unlock, Transfer };

typedef Message {
  mtype type;
  int sender;
  int receiver;
  int data[4];
};

typedef AccountState {
  int balance;
  bool locked;
};

/* Global Variables */
int num_actors = 0;
int system_step = 0;
bool deadlock_detected = false;

AccountState account_state[5];

/* Communication Channels */
chan account_messages = [10] of { Message };
chan bank_messages = [10] of { Message };

/* Actor: BankAccount */
proctype BankAccount(int id) {
  Message msg;
  bool active = true;
  
  do
  :: active ->
    if
    :: account_messages ? msg ->
      if
      :: (msg.type == Deposit) ->
        atomic {
          if
          :: (!account_state[id].locked && msg.data[0] > 0) ->
            account_state[id].balance = account_state[id].balance + msg.data[0];
          :: else -> skip
          fi
        }
      :: (msg.type == Withdraw) ->
        atomic {
          if
          :: (!account_state[id].locked && msg.data[0] > 0 && 
              account_state[id].balance >= msg.data[0]) ->
            account_state[id].balance = account_state[id].balance - msg.data[0];
          :: else -> skip
          fi
        }
      :: (msg.type == Lock) ->
        atomic {
          if
          :: (!account_state[id].locked) ->
            account_state[id].locked = true;
          :: else -> skip
          fi
        }
      :: (msg.type == Unlock) ->
        atomic {
          if
          :: (account_state[id].locked) ->
            account_state[id].locked = false;
          :: else -> skip
          fi
        }
      fi
    :: atomic {
      /* Internal actions */
      system_step++;
    }
    :: !active -> break
  od
}

/* System Initialization */
init {
  /* Initialize account actors */
  int account_count = 0;
  do
  :: account_count < 3 ->
    run BankAccount(account_count);
    account_count++
  :: else -> break
  od;
  
  num_actors = 3;
}

/* LTL Properties */
ltl balance_non_negative { 
  []((account_state[0].balance >= 0) && 
     (account_state[1].balance >= 0) && 
     (account_state[2].balance >= 0)) 
}

ltl no_deadlock { []<>!deadlock_detected }
ltl progress { []<>(system_step > 0) }
```

### NuSMV Integration

NuSMV provides symbolic model checking for CTL properties.

**Strengths:**
- Symbolic verification (BDDs)
- Good for CTL temporal logic
- Handles large state spaces efficiently
- Excellent for hardware-like systems

**When to use:**
- CTL property verification
- Systems with large but regular state spaces
- Branching-time temporal properties

### Custom State Explorer

GAL's custom state space explorer provides optimized exploration with advanced reduction techniques.

**Features:**
- Partial order reduction
- Symmetry reduction
- Hash compaction
- Multiple search strategies (BFS, DFS, best-first)
- Custom abstraction techniques

**When to use:**
- When external tools are not available
- Custom verification algorithms
- Integration with GAL's type system
- Rapid prototyping of verification techniques

## Property Specification

### Temporal Logic Properties

GAL supports multiple temporal logic formalisms:

#### Linear Temporal Logic (LTL)

```json
{
  "properties": [
    {
      "name": "mutual_exclusion",
      "type": "safety",
      "formula": "G(!(critical1 && critical2))",
      "description": "At most one process in critical section",
      "critical": true
    },
    {
      "name": "starvation_freedom",
      "type": "liveness",
      "formula": "G(requesting -> F(in_critical))",
      "description": "Every request eventually gets served",
      "critical": true
    }
  ]
}
```

#### Computation Tree Logic (CTL)

```json
{
  "properties": [
    {
      "name": "deadlock_freedom",
      "type": "safety",
      "formula": "AG(EX(true))",
      "description": "System can always make progress",
      "critical": true
    },
    {
      "name": "reachability",
      "type": "reachability",
      "formula": "EF(goal_state)",
      "description": "Goal state is reachable",
      "critical": false
    }
  ]
}
```

### Safety Properties

Safety properties specify that "bad things never happen":

```gal
actor Mutex {
    state {
        owner: Option<ActorId> = None,
        waiting: Queue<ActorId> = Queue::new(),
    }
    
    invariant owner.is_some() => waiting.is_empty() || waiting.front() != owner.unwrap()
    invariant waiting.len() <= MAX_WAITING
    
    handler Acquire(requester: ActorId) {
        requires !waiting.contains(requester)
        ensures waiting.contains(requester)
        
        waiting.push(requester);
        if owner.is_none() {
            owner = Some(requester);
            waiting.pop();
        }
    }
    
    handler Release(releaser: ActorId) {
        requires owner == Some(releaser)
        ensures owner.is_none() || owner != Some(releaser)
        
        owner = None;
        if !waiting.is_empty() {
            owner = Some(waiting.pop());
        }
    }
    
    // Safety property: mutual exclusion
    safety mutual_exclusion {
        always(owner.is_some() => !exists(other: ActorId, other != owner.unwrap() && holds_lock(other)))
    }
}
```

### Liveness Properties

Liveness properties specify that "good things eventually happen":

```gal
actor RequestProcessor {
    state {
        pending_requests: Queue<Request> = Queue::new(),
        processing: Option<Request> = None,
    }
    
    handler SubmitRequest(req: Request) {
        pending_requests.push(req);
    }
    
    handler ProcessNext() {
        requires processing.is_none() && !pending_requests.is_empty()
        
        processing = Some(pending_requests.pop());
        // Process the request...
        processing = None;
    }
    
    // Liveness property: all requests eventually processed
    liveness request_progress {
        always(pending_requests.contains(req) => eventually(!pending_requests.contains(req)))
    }
    
    // Fairness assumption: processing eventually happens
    fairness processing_fairness {
        always(eventually(ProcessNext() || processing.is_some()))
    }
}
```

### Invariant Properties

Invariants are properties that must hold in all reachable states:

```gal
actor BankAccount {
    state {
        balance: Int = 0,
        transaction_log: List<Transaction> = List::new(),
    }
    
    // State invariants
    invariant balance >= 0
    invariant balance == transaction_log.iter().map(|t| t.amount).sum()
    invariant transaction_log.len() <= MAX_TRANSACTIONS
    
    // Protocol invariants
    invariant forall(t: Transaction in transaction_log, t.timestamp <= current_time())
    invariant forall(t1, t2: Transaction in transaction_log, 
                    t1.id == t2.id => t1 == t2)  // No duplicate transactions
}
```

## Deadlock and Livelock Detection

### Deadlock Detection

GAL's deadlock detection uses multiple algorithms:

1. **State Space Exploration**: Enumerate states to find those with no enabled transitions
2. **Resource Allocation Graph**: Analyze resource dependencies
3. **Communication Graph Analysis**: Detect circular dependencies in message passing

```bash
# Basic deadlock detection
gal-model-check deadlock dining_philosophers.gal

# Find all deadlocks with partial order reduction
gal-model-check deadlock dining_philosophers.gal --find-all --por

# Limit state space exploration
gal-model-check deadlock dining_philosophers.gal --max-states 1000000
```

#### Example: Dining Philosophers

```gal
actor Fork {
    state {
        holder: Option<ActorId> = None,
    }
    
    message Take(philosopher: ActorId)
    message Put(philosopher: ActorId)
    
    handler Take(philosopher) {
        requires holder.is_none()
        ensures holder == Some(philosopher)
        
        holder = Some(philosopher);
        send philosopher.ForkTaken(self);
    }
    
    handler Put(philosopher) {
        requires holder == Some(philosopher)
        ensures holder.is_none()
        
        holder = None;
        send philosopher.ForkReleased(self);
    }
}

actor Philosopher {
    state {
        left_fork: ActorRef<Fork>,
        right_fork: ActorRef<Fork>,
        state: PhilosopherState = Thinking,
        forks_held: Set<ActorRef<Fork>> = Set::new(),
    }
    
    enum PhilosopherState {
        Thinking,
        Hungry,
        Eating,
    }
    
    message StartEating()
    message ForkTaken(fork: ActorRef<Fork>)
    message ForkReleased(fork: ActorRef<Fork>)
    
    handler StartEating() {
        requires state == Thinking
        ensures state == Hungry
        
        state = Hungry;
        send left_fork.Take(self);  // Potential deadlock: all philosophers take left fork
        send right_fork.Take(self);
    }
    
    handler ForkTaken(fork) {
        forks_held.insert(fork);
        if forks_held.len() == 2 {
            state = Eating;
            // Eat for some time...
            send left_fork.Put(self);
            send right_fork.Put(self);
        }
    }
    
    handler ForkReleased(fork) {
        forks_held.remove(fork);
        if forks_held.is_empty() && state == Eating {
            state = Thinking;
        }
    }
}
```

**Deadlock Analysis Output:**

```
Deadlock Analysis Results
========================
❌ Deadlocks found: 1
Analysis time: 2.34s

Deadlock State:
- All philosophers holding their left fork
- All philosophers waiting for their right fork
- Circular wait: P1 → F2 → P2 → F3 → P3 → F4 → P4 → F5 → P5 → F1 → P1

Suggested Fixes:
1. Resource ordering: Always acquire forks in a fixed order
2. Timeout-based approach: Release forks if can't acquire all within timeout
3. Waiter solution: Use a centralized coordinator
4. Asymmetric solution: One philosopher acquires right fork first
```

### Livelock Detection

Livelocks occur when the system makes progress but no useful work is accomplished.

```bash
# Check for livelocks
gal-model-check livelock message_ping_pong.gal

# With fairness assumptions
gal-model-check livelock message_ping_pong.gal --fairness fairness.json
```

#### Example: Message Ping-Pong Livelock

```gal
actor Sender {
    state {
        receiver: ActorRef<Receiver>,
        message_count: Int = 0,
    }
    
    message Start()
    message Pong()
    
    handler Start() {
        send receiver.Ping();
        message_count += 1;
    }
    
    handler Pong() {
        // Immediately send another ping - potential livelock
        send receiver.Ping();
        message_count += 1;
    }
}

actor Receiver {
    state {
        sender: ActorRef<Sender>,
        message_count: Int = 0,
    }
    
    message Ping()
    
    handler Ping() {
        send sender.Pong();
        message_count += 1;
    }
}
```

**Livelock Analysis Output:**

```
Livelock Analysis Results
=========================
❌ Livelocks found: 1
Analysis time: 1.87s

Livelock Cycle:
- Sender.Start() → Receiver.Ping() → Sender.Pong() → Receiver.Ping() → ...
- System makes progress (message counters increase)
- No useful work accomplished (infinite ping-pong)

Fairness Violations:
- No fairness constraint prevents infinite ping-pong
- Missing termination condition

Suggested Fixes:
1. Add maximum message count limit
2. Introduce random delays or backoff
3. Add termination conditions
4. Implement proper protocol state machines
```

## Temporal Logic Verification

### LTL Property Verification

```bash
# Verify LTL properties using TLA+
gal-model-check temporal producer_consumer.gal --properties ltl_props.json --backend tla-plus

# Use SPIN for LTL verification
gal-model-check temporal producer_consumer.gal --properties ltl_props.json --backend spin

# Generate counterexamples
gal-model-check temporal producer_consumer.gal --properties ltl_props.json --counterexamples
```

#### Example: Producer-Consumer

```gal
actor Buffer {
    state {
        items: Queue<Item> = Queue::new(),
        capacity: Int = 10,
        producers_waiting: Int = 0,
        consumers_waiting: Int = 0,
    }
    
    message Produce(item: Item, producer: ActorRef<Producer>)
    message Consume(consumer: ActorRef<Consumer>)
    
    handler Produce(item, producer) {
        if items.len() < capacity {
            items.push(item);
            send producer.ProduceAck();
        } else {
            producers_waiting += 1;
            // Wait for space...
        }
    }
    
    handler Consume(consumer) {
        if !items.is_empty() {
            let item = items.pop();
            send consumer.ConsumeAck(item);
        } else {
            consumers_waiting += 1;
            // Wait for item...
        }
    }
    
    // Safety: Buffer never overflows
    invariant items.len() <= capacity
    
    // Liveness: If producers are waiting, buffer will eventually have space
    liveness producer_progress {
        always(producers_waiting > 0 => eventually(items.len() < capacity))
    }
    
    // Liveness: If consumers are waiting, buffer will eventually have items
    liveness consumer_progress {
        always(consumers_waiting > 0 => eventually(!items.is_empty()))
    }
}
```

**LTL Properties File (`ltl_props.json`):**

```json
{
  "properties": [
    {
      "name": "no_buffer_overflow",
      "type": "safety",
      "formula": "G(buffer.items.len() <= buffer.capacity)",
      "description": "Buffer never exceeds capacity",
      "critical": true
    },
    {
      "name": "eventual_consumption",
      "type": "liveness",
      "formula": "G(buffer.items.len() > 0 -> F(buffer.items.len() < old(buffer.items.len())))",
      "description": "Items are eventually consumed",
      "critical": true
    },
    {
      "name": "producer_starvation_freedom",
      "type": "liveness",
      "formula": "G(producers_waiting > 0 -> F(producers_waiting == 0))",
      "description": "Producers don't starve indefinitely",
      "critical": false
    },
    {
      "name": "consumer_starvation_freedom",
      "type": "liveness",
      "formula": "G(consumers_waiting > 0 -> F(consumers_waiting == 0))",
      "description": "Consumers don't starve indefinitely",
      "critical": false
    }
  ],
  "fairness_assumptions": [
    "WF(producer.produce_action)",
    "WF(consumer.consume_action)",
    "SF(buffer.notify_waiters)"
  ]
}
```

### CTL Property Verification

```bash
# Verify CTL properties using NuSMV
gal-model-check temporal elevator_system.gal --properties ctl_props.json --backend nusmv
```

**CTL Properties File (`ctl_props.json`):**

```json
{
  "properties": [
    {
      "name": "reachability",
      "type": "reachability",
      "formula": "EF(elevator.floor == 10)",
      "description": "Elevator can reach floor 10",
      "critical": false
    },
    {
      "name": "universal_safety",
      "type": "safety",
      "formula": "AG(elevator.door_open -> elevator.velocity == 0)",
      "description": "Door only opens when elevator is stopped",
      "critical": true
    },
    {
      "name": "existential_progress",
      "type": "liveness",
      "formula": "AG(request_pending -> EF(request_served))",
      "description": "There exists a path where every request is served",
      "critical": true
    }
  ]
}
```

## Communication Pattern Verification

GAL can verify standard communication patterns used in actor systems:

### Request-Response Pattern

```gal
actor Client {
    state {
        server: ActorRef<Server>,
        pending_requests: Map<RequestId, Request> = Map::new(),
        responses_received: Set<RequestId> = Set::new(),
    }
    
    message Response(request_id: RequestId, data: ResponseData)
    
    handler SendRequest(request: Request) {
        let request_id = generate_id();
        pending_requests[request_id] = request;
        send server.Request(request_id, request, self);
    }
    
    handler Response(request_id, data) {
        requires pending_requests.contains_key(request_id)
        ensures responses_received.contains(request_id)
        
        pending_requests.remove(request_id);
        responses_received.insert(request_id);
        // Process response...
    }
    
    // Pattern invariant: No response without request
    invariant forall(id in responses_received, 
                    exists(req in pending_requests.keys() ∪ responses_received, req == id))
}

actor Server {
    state {
        request_count: Int = 0,
    }
    
    message Request(id: RequestId, data: RequestData, client: ActorRef<Client>)
    
    handler Request(id, data, client) {
        request_count += 1;
        
        // Process request...
        let response_data = process(data);
        
        send client.Response(id, response_data);
    }
    
    // Liveness: All requests get responses
    liveness request_response {
        always(received_request(id) => eventually(sent_response(id)))
    }
}
```

**Verify Request-Response Pattern:**

```bash
gal-model-check communication client_server.gal --patterns request-response --ordering
```

### Publish-Subscribe Pattern

```gal
actor Publisher {
    state {
        subscribers: Set<ActorRef<Subscriber>> = Set::new(),
        published_messages: List<Message> = List::new(),
    }
    
    message Subscribe(subscriber: ActorRef<Subscriber>)
    message Unsubscribe(subscriber: ActorRef<Subscriber>)
    
    handler Subscribe(subscriber) {
        subscribers.insert(subscriber);
    }
    
    handler Unsubscribe(subscriber) {
        subscribers.remove(subscriber);
    }
    
    handler Publish(message: Message) {
        published_messages.push(message);
        for subscriber in subscribers {
            send subscriber.Notify(message);
        }
    }
    
    // Pattern invariant: All active subscribers receive all messages
    invariant forall(msg in published_messages,
                    forall(sub in subscribers,
                          sent_to(sub, msg)))
}

actor Subscriber {
    state {
        publisher: ActorRef<Publisher>,
        received_messages: List<Message> = List::new(),
        subscribed: Bool = false,
    }
    
    message Notify(message: Message)
    
    handler Subscribe() {
        requires !subscribed
        ensures subscribed
        
        subscribed = true;
        send publisher.Subscribe(self);
    }
    
    handler Notify(message) {
        requires subscribed
        
        received_messages.push(message);
        // Process notification...
    }
    
    // Liveness: Subscribed actors eventually receive messages
    liveness message_delivery {
        always(subscribed && publisher.published_new_message() 
               => eventually(received_new_message()))
    }
}
```

### Pipeline Pattern

```gal
actor PipelineStage<InputType, OutputType> {
    state {
        next_stage: Option<ActorRef<PipelineStage<OutputType, NextType>>>,
        processed_count: Int = 0,
    }
    
    message Process(input: InputType, pipeline_id: PipelineId)
    
    handler Process(input, pipeline_id) {
        let output = transform(input);
        processed_count += 1;
        
        match next_stage {
            Some(next) => send next.Process(output, pipeline_id),
            None => complete_pipeline(pipeline_id, output),
        }
    }
    
    // Pattern invariant: Pipeline ordering preserved
    invariant forall(id1, id2: PipelineId,
                    started(id1) < started(id2) => completed(id1) <= completed(id2))
}
```

## State Space Exploration

### Exploration Strategies

GAL provides multiple state space exploration strategies:

```bash
# Breadth-first search (default)
gal-model-check analyze system.gal --analysis-type state-space

# Depth-first search
gal-model-check analyze system.gal --analysis-type state-space --strategy dfs

# Best-first search with heuristics
gal-model-check analyze system.gal --analysis-type state-space --strategy best-first

# Random walk exploration
gal-model-check analyze system.gal --analysis-type state-space --strategy random-walk
```

### State Space Reduction Techniques

#### Partial Order Reduction

Reduces the state space by exploring only representative interleavings:

```bash
# Enable partial order reduction
gal-model-check deadlock system.gal --por
```

**How it works:**
- Identifies independent transitions
- Explores only one representative interleaving
- Preserves correctness for verified properties

#### Symmetry Reduction

Exploits symmetries in the system to reduce equivalent states:

```gal
actor Process {
    state {
        id: ProcessId,
        state: ProcessState = Idle,
    }
    
    // Symmetric system: processes are identical except for ID
    symmetry process_symmetry {
        permutation_group(ProcessId)
    }
}
```

#### Abstraction Techniques

```gal
actor Counter {
    state {
        count: Int = 0,
    }
    
    // Data abstraction: abstract exact count to ranges
    abstraction count_abstraction {
        count -> if count == 0 then Zero 
                 else if count < 10 then Small 
                 else Large
    }
}
```

### State Space Analysis

```bash
# Comprehensive state space analysis
gal-model-check analyze distributed_system.gal --analysis-type state-space --max-depth 10000
```

**Analysis Output:**

```
State Space Analysis Results
===========================
States explored: 1,234,567
Unique states: 987,654
Transitions: 5,432,109
Maximum depth: 1,247
Average branching factor: 4.39

State Distribution:
- Initial states: 1
- Deadlock states: 0
- Terminal states: 23,456
- Livelock cycles: 2

Memory Usage:
- Peak memory: 2.3 GB
- States cached: 850,000
- Cache hit ratio: 87.3%

Performance:
- States/second: 15,234
- Exploration time: 81.2 seconds
- Reduction effectiveness: 67.8%

Recommendations:
- Consider increasing abstraction level
- Partial order reduction is effective
- State caching is working well
```

## Counterexample Analysis

When a property is violated, GAL generates detailed counterexamples:

### Counterexample Structure

```json
{
  "property": "mutual_exclusion",
  "violation_type": "safety",
  "trace": {
    "length": 15,
    "steps": [
      {
        "step": 1,
        "state": {
          "process1": { "state": "idle", "wants_lock": false },
          "process2": { "state": "idle", "wants_lock": false },
          "lock": { "owner": null }
        },
        "action": "process1.request_lock()",
        "actor": "process1"
      },
      {
        "step": 2,
        "state": {
          "process1": { "state": "waiting", "wants_lock": true },
          "process2": { "state": "idle", "wants_lock": false },
          "lock": { "owner": null }
        },
        "action": "process2.request_lock()",
        "actor": "process2"
      },
      // ... more steps ...
      {
        "step": 15,
        "state": {
          "process1": { "state": "critical", "wants_lock": false },
          "process2": { "state": "critical", "wants_lock": false },
          "lock": { "owner": "process1" }  // BUG: Both in critical section!
        },
        "action": "race_condition_trigger",
        "actor": "system"
      }
    ]
  },
  "analysis": {
    "root_cause": "Race condition in lock acquisition",
    "critical_steps": [12, 13, 14],
    "involved_actors": ["process1", "process2", "lock"],
    "suggested_fixes": [
      "Use atomic compare-and-swap for lock acquisition",
      "Add proper synchronization in lock handler",
      "Implement priority-based lock arbitration"
    ]
  }
}
```

### Counterexample Minimization

GAL automatically minimizes counterexamples to focus on essential steps:

```bash
# Generate minimal counterexamples
gal-model-check temporal system.gal --properties props.json --counterexamples --minimize
```

**Before minimization (23 steps):**
```
1. process1.start()
2. process1.idle_wait() 
3. process1.idle_wait()
4. process2.start()
5. process1.request_lock()
6. process1.idle_wait()
7. process2.request_lock()
8. lock.grant_to_process1()
9. process1.enter_critical()
10. process1.work()
11. process1.work()
12. process2.timeout()
13. process2.force_acquire()  // BUG HERE
14. process2.enter_critical()
15. process1.still_working()
16. process2.work()
17. assert_mutual_exclusion() // VIOLATION
```

**After minimization (5 steps):**
```
1. process1.request_lock()
2. lock.grant_to_process1()
3. process1.enter_critical()
4. process2.force_acquire()    // BUG HERE
5. process2.enter_critical()   // VIOLATION
```

### Interactive Counterexample Exploration

```bash
# Interactive exploration mode
gal-model-check interactive system.gal
```

```
gal-verify> load counterexample violation_trace.json
gal-verify> step 1
State: { process1: idle, process2: idle, lock: free }
Action: process1.request_lock()

gal-verify> step 5
State: { process1: critical, process2: waiting, lock: owned_by_process1 }
Action: process2.force_acquire()

gal-verify> explain step 5
Analysis: This step violates the mutual exclusion protocol.
- process2 should wait for process1 to release the lock
- force_acquire() bypasses normal lock checking
- This is likely a race condition or protocol violation

gal-verify> suggest fix step 5
Suggestions:
1. Remove force_acquire() method entirely
2. Add timeout handling that respects the lock protocol
3. Implement proper priority-based preemption
```

## Performance Optimization

### Optimization Strategies

1. **State Space Reduction**
   - Partial order reduction
   - Symmetry reduction
   - Abstraction refinement

2. **Memory Optimization**
   - Hash compaction
   - State compression
   - Garbage collection of unreachable states

3. **Parallel Verification**
   - Multi-threaded exploration
   - Distributed model checking
   - GPU acceleration (experimental)

### Configuration Tuning

```json
{
  "optimization": {
    "state_exploration": {
      "max_states": 10000000,
      "hash_compaction": true,
      "compression_level": 2,
      "garbage_collection": true,
      "gc_threshold": 0.8
    },
    "reduction_techniques": {
      "partial_order_reduction": true,
      "symmetry_reduction": true,
      "abstraction_level": "moderate",
      "stubborn_sets": true
    },
    "performance": {
      "parallel_threads": 8,
      "memory_limit": "8GB",
      "disk_storage": true,
      "storage_path": "/tmp/gal_verification"
    }
  }
}
```

### Benchmarking

```bash
# Benchmark verification performance
gal-model-check benchmark system.gal --runs 10 --suite all

# Benchmark specific verification types
gal-model-check benchmark system.gal --runs 5 --suite deadlock
gal-model-check benchmark system.gal --runs 5 --suite temporal
```

**Benchmark Output:**

```
Benchmark Results (all)
===================
Runs: 10

Performance Metrics:
Average time: 24.567s
Min time: 22.134s
Max time: 28.901s
Standard deviation: 2.345s

Verification Breakdown:
- Deadlock detection: 8.2s (33.4%)
- Livelock detection: 6.8s (27.7%)
- Temporal properties: 9.5s (38.9%)

State Space Metrics:
- States explored: 1,234,567 ± 45,678
- States/second: 50,234 ± 3,456
- Memory usage: 2.1GB ± 0.3GB

Optimization Effectiveness:
- POR reduction: 67.8%
- Symmetry reduction: 23.4%
- Cache hit ratio: 89.2%

Recommendations:
- Consider increasing abstraction for temporal properties
- Memory usage is within limits
- POR is highly effective for this system
```

## Integration with SMT Solvers

GAL integrates with SMT solvers for decidable verification problems:

### SMT-Based Verification

```gal
actor BoundedCounter {
    state {
        count: Int = 0,
        max_value: Int = 100,
    }
    
    message Increment()
    message Decrement()
    
    handler Increment() {
        requires count < max_value
        ensures count == old(count) + 1
        
        count += 1;
    }
    
    handler Decrement() {
        requires count > 0  
        ensures count == old(count) - 1
        
        count -= 1;
    }
    
    // SMT-verifiable invariant
    invariant 0 <= count <= max_value
    
    // SMT-verifiable property
    property increment_effect {
        forall(pre_state, post_state: State,
               pre_state.count < max_value &&
               transition(pre_state, Increment(), post_state)
               => post_state.count == pre_state.count + 1)
    }
}
```

### SMT Configuration

```json
{
  "smt_config": {
    "solver": "z3",
    "timeout": 30,
    "logic": "LIA",
    "options": {
      "timeout": "30000",
      "model": "true",
      "unsat_core": "true"
    }
  }
}
```

### Automatic SMT Translation

GAL automatically translates suitable properties to SMT-LIB format:

```smt2
; Generated SMT-LIB for BoundedCounter
(set-logic LIA)
(set-option :timeout 30000)
(set-option :produce-models true)

; State variables
(declare-fun count () Int)
(declare-fun max_value () Int)

; Invariant: 0 <= count <= max_value
(assert (and (<= 0 count) (<= count max_value)))

; Property: increment effect
(assert (forall ((pre_count Int) (post_count Int))
  (=> (and (< pre_count max_value)
           (= post_count (+ pre_count 1)))
      (= post_count (+ pre_count 1)))))

(check-sat)
(get-model)
```

## CLI Reference

### Global Options

```bash
gal-model-check [OPTIONS] <COMMAND>

Global Options:
  -v, --verbose           Enable verbose output
  -c, --config <FILE>     Configuration file
  -t, --timeout <SECS>    Timeout in seconds [default: 300]
      --format <FORMAT>   Output format [default: human]
                          [possible values: human, json, xml, html]
  -h, --help             Print help information
  -V, --version          Print version information
```

### Commands

#### Deadlock Detection

```bash
gal-model-check deadlock [OPTIONS] <INPUT>

Options:
      --find-all         Find all deadlocks (not just the first one)
      --max-states <N>   Maximum states to explore [default: 100000]
      --por              Enable partial order reduction

Arguments:
  <INPUT>  GAL source file to analyze
```

#### Livelock Detection

```bash
gal-model-check livelock [OPTIONS] <INPUT>

Options:
      --fairness <FILE>          Fairness assumptions file
      --max-cycle-length <N>     Maximum cycle length to detect [default: 100]

Arguments:
  <INPUT>  GAL source file to analyze
```

#### Temporal Property Verification

```bash
gal-model-check temporal [OPTIONS] <INPUT> <PROPERTIES>

Options:
      --backend <BACKEND>    Model checking backend [default: auto]
                            [possible values: auto, tla-plus, spin, nusmv, state-explorer, smt]
      --counterexamples     Generate counterexamples

Arguments:
  <INPUT>       GAL source file to analyze
  <PROPERTIES>  Properties file (JSON format)
```

#### Communication Pattern Verification

```bash
gal-model-check communication [OPTIONS] <INPUT>

Options:
      --patterns <PATTERNS>...  Communication patterns to verify
                               [possible values: request-response, publish-subscribe, pipeline, broadcast]
      --ordering               Check message ordering

Arguments:
  <INPUT>  GAL source file to analyze
```

#### Model Export

```bash
gal-model-check export [OPTIONS] <INPUT> <OUTPUT>

Options:
      --formats <FORMATS>...  Formats to export
                             [possible values: tla-plus, spin, nusmv, petri-net, graphviz]

Arguments:
  <INPUT>   GAL source file to analyze
  <OUTPUT>  Output directory for exported models
```

#### State Space Analysis

```bash
gal-model-check analyze [OPTIONS] <INPUT>

Options:
      --analysis-type <TYPE>  Analysis type [default: overview]
                             [possible values: overview, state-space, communication, performance, complexity]
      --max-depth <N>        Maximum exploration depth [default: 1000]

Arguments:
  <INPUT>  GAL source file to analyze
```

#### Benchmarking

```bash
gal-model-check benchmark [OPTIONS] <INPUT>

Options:
      --runs <N>      Number of benchmark runs [default: 5]
      --suite <SUITE> Benchmarking suite [default: all]
                      [possible values: all, deadlock, livelock, temporal, communication]

Arguments:
  <INPUT>  GAL source file to analyze
```

#### Interactive Mode

```bash
gal-model-check interactive <INPUT>

Arguments:
  <INPUT>  GAL source file to analyze
```

## Configuration

### Configuration File Format

GAL model checking can be configured using JSON configuration files:

```json
{
  "verification": {
    "use_smt_solver": true,
    "smt_timeout": 30,
    "use_model_checking": true,
    "model_check_bound": 1000,
    "interactive_proving": false,
    "verbosity": "normal",
    "max_proof_depth": 100,
    "cache_proofs": true
  },
  "state_exploration": {
    "max_states": 1000000,
    "enable_partial_order_reduction": true,
    "enable_symmetry_reduction": false,
    "find_all_deadlocks": false,
    "use_hash_compaction": true,
    "search_strategy": "breadth_first",
    "abstraction_level": "none"
  },
  "tla_config": {
    "tlc_path": "tlc",
    "memory_limit": 2048,
    "worker_count": 4,
    "deadlock_checking": true,
    "cleanup": true,
    "max_depth": null,
    "symmetry_reduction": false
  },
  "spin_config": {
    "spin_path": "spin",
    "max_actor_instances": 5,
    "channel_capacity": 10,
    "search_depth": 10000,
    "safety_mode": false,
    "exhaustive_search": false,
    "compression": true,
    "memory_limit": 2048
  },
  "nusmv_config": {
    "nusmv_path": "nusmv",
    "bdd_memory_limit": 1024,
    "enable_cone_of_influence": true,
    "dynamic_reordering": true,
    "fairness_constraints": true
  },
  "smt_config": {
    "backend": "z3",
    "logic": "LIA", 
    "timeout": 30,
    "generate_models": true,
    "options": {
      "timeout": "30000",
      "model": "true"
    }
  },
  "output": {
    "format": "human",
    "verbosity": "normal",
    "export_counterexamples": true,
    "minimize_counterexamples": true,
    "generate_reports": true,
    "report_format": "html"
  },
  "performance": {
    "parallel_verification": false,
    "worker_threads": 4,
    "memory_limit": "8GB",
    "disk_storage": false,
    "storage_path": "/tmp/gal_verification"
  }
}
```

### Environment Variables

```bash
# Tool paths
export TLC_PATH=/path/to/tla2tools.jar
export SPIN_PATH=/usr/bin/spin
export NUSMV_PATH=/usr/bin/nusmv
export Z3_PATH=/usr/bin/z3

# Memory limits
export GAL_MEMORY_LIMIT=8GB
export GAL_TLC_MEMORY=4GB

# Temporary storage
export GAL_TEMP_DIR=/tmp/gal_verification

# Logging
export GAL_LOG_LEVEL=info
export GAL_LOG_FILE=/var/log/gal-model-check.log
```

## Examples

### Example 1: Simple Mutual Exclusion

```gal
// mutex.gal
actor Mutex {
    state {
        owner: Option<ActorId> = None,
        waiting: Queue<ActorId> = Queue::new(),
    }
    
    message Acquire(requester: ActorId)
    message Release(releaser: ActorId)
    
    handler Acquire(requester) {
        if owner.is_none() {
            owner = Some(requester);
        } else {
            waiting.push(requester);
        }
    }
    
    handler Release(releaser) {
        requires owner == Some(releaser)
        
        owner = None;
        if !waiting.is_empty() {
            owner = Some(waiting.pop());
        }
    }
    
    invariant waiting.len() <= 10  // Bounded waiting
    
    safety mutual_exclusion {
        always(owner.is_some() => waiting.is_empty() || 
               !waiting.contains(owner.unwrap()))
    }
    
    liveness starvation_freedom {
        always(waiting.contains(id) => eventually(!waiting.contains(id)))
    }
}
```

**Verification commands:**

```bash
# Check for deadlocks
gal-model-check deadlock mutex.gal

# Verify temporal properties
echo '{
  "properties": [
    {
      "name": "mutual_exclusion",
      "type": "safety", 
      "formula": "G(count(actor.state == critical) <= 1)",
      "critical": true
    }
  ]
}' > mutex_props.json

gal-model-check temporal mutex.gal mutex_props.json --backend tla-plus
```

### Example 2: Producer-Consumer with Bounded Buffer

```gal
// producer_consumer.gal
actor BoundedBuffer<T> {
    state {
        items: Queue<T> = Queue::new(),
        capacity: Int = 10,
        producers_blocked: Set<ActorId> = Set::new(),
        consumers_blocked: Set<ActorId> = Set::new(),
    }
    
    message Put(item: T, producer: ActorId)
    message Get(consumer: ActorId)
    message PutResponse(success: Bool)
    message GetResponse(item: Option<T>)
    
    handler Put(item, producer) {
        if items.len() < capacity {
            items.push(item);
            send actor_ref(producer).PutResponse(true);
            
            // Notify waiting consumers
            for consumer in consumers_blocked {
                send actor_ref(consumer).GetResponse(Some(items.pop()));
            }
            consumers_blocked.clear();
        } else {
            producers_blocked.insert(producer);
            send actor_ref(producer).PutResponse(false);
        }
    }
    
    handler Get(consumer) {
        if !items.is_empty() {
            let item = items.pop();
            send actor_ref(consumer).GetResponse(Some(item));
            
            // Notify waiting producers
            for producer in producers_blocked {
                send actor_ref(producer).PutResponse(true);
            }
            producers_blocked.clear();
        } else {
            consumers_blocked.insert(consumer);
            send actor_ref(consumer).GetResponse(None);
        }
    }
    
    invariant items.len() <= capacity
    invariant items.len() >= 0
    
    // No lost items
    invariant items.len() + producers_blocked.len() + consumers_blocked.len() 
              <= capacity + MAX_ACTORS
    
    liveness producer_progress {
        always(producers_blocked.contains(id) => eventually(!producers_blocked.contains(id)))
    }
    
    liveness consumer_progress {
        always(consumers_blocked.contains(id) => eventually(!consumers_blocked.contains(id)))
    }
}

actor Producer {
    state {
        buffer: ActorRef<BoundedBuffer<Item>>,
        items_produced: Int = 0,
        blocked: Bool = false,
    }
    
    message Produce()
    message PutResponse(success: Bool)
    
    handler Produce() {
        requires !blocked
        
        let item = create_item();
        send buffer.Put(item, self.id());
        blocked = true;
    }
    
    handler PutResponse(success) {
        blocked = false;
        if success {
            items_produced += 1;
        }
        // Continue producing...
    }
}

actor Consumer {
    state {
        buffer: ActorRef<BoundedBuffer<Item>>,
        items_consumed: Int = 0,
        waiting: Bool = false,
    }
    
    message Consume()
    message GetResponse(item: Option<Item>)
    
    handler Consume() {
        requires !waiting
        
        send buffer.Get(self.id());
        waiting = true;
    }
    
    handler GetResponse(item) {
        waiting = false;
        match item {
            Some(i) => {
                items_consumed += 1;
                process_item(i);
            }
            None => {
                // Buffer was empty, try again later
            }
        }
    }
}
```

**Verification commands:**

```bash
# Check for deadlocks with partial order reduction
gal-model-check deadlock producer_consumer.gal --por --max-states 500000

# Verify communication patterns
gal-model-check communication producer_consumer.gal --patterns request-response --ordering

# Comprehensive temporal property verification
echo '{
  "properties": [
    {
      "name": "no_buffer_overflow",
      "type": "safety",
      "formula": "G(buffer.items.len() <= buffer.capacity)",
      "description": "Buffer never exceeds capacity",
      "critical": true
    },
    {
      "name": "eventual_consumption",
      "type": "liveness", 
      "formula": "G(buffer.items.len() > 0 -> F(X(buffer.items.len() < buffer.items.len())))",
      "description": "Items are eventually consumed",
      "critical": true
    },
    {
      "name": "producer_progress",
      "type": "liveness",
      "formula": "G(producer.blocked -> F(!producer.blocked))",
      "description": "Blocked producers eventually make progress",
      "critical": false
    }
  ],
  "fairness_assumptions": [
    "WF(producer.produce)",
    "WF(consumer.consume)",
    "SF(buffer.notify_waiters)"
  ]
}' > pc_props.json

gal-model-check temporal producer_consumer.gal pc_props.json --counterexamples
```

### Example 3: Distributed Consensus (Raft)

```gal
// raft_consensus.gal
actor RaftNode {
    state {
        id: NodeId,
        peers: Set<ActorRef<RaftNode>>,
        role: Role = Follower,
        current_term: Int = 0,
        voted_for: Option<NodeId> = None,
        log: List<LogEntry> = List::new(),
        commit_index: Int = 0,
        last_applied: Int = 0,
        
        // Leader state
        next_index: Map<NodeId, Int> = Map::new(),
        match_index: Map<NodeId, Int> = Map::new(),
        
        // Election state
        votes_received: Set<NodeId> = Set::new(),
        election_timeout: Duration = random_timeout(),
    }
    
    enum Role {
        Follower,
        Candidate, 
        Leader,
    }
    
    message RequestVote(term: Int, candidate_id: NodeId, last_log_index: Int, last_log_term: Int)
    message VoteResponse(term: Int, vote_granted: Bool)
    message AppendEntries(term: Int, leader_id: NodeId, prev_log_index: Int, 
                         prev_log_term: Int, entries: List<LogEntry>, leader_commit: Int)
    message AppendResponse(term: Int, success: Bool)
    message ElectionTimeout()
    
    handler ElectionTimeout() {
        requires role != Leader
        
        // Start election
        current_term += 1;
        role = Candidate;
        voted_for = Some(id);
        votes_received = set![id];
        
        // Request votes from all peers
        for peer in peers {
            let last_log_index = log.len() - 1;
            let last_log_term = if log.is_empty() { 0 } else { log.last().term };
            send peer.RequestVote(current_term, id, last_log_index, last_log_term);
        }
    }
    
    handler RequestVote(term, candidate_id, last_log_index, last_log_term) {
        let vote_granted = term >= current_term && 
                          (voted_for.is_none() || voted_for == Some(candidate_id)) &&
                          log_is_up_to_date(last_log_index, last_log_term);
        
        if term > current_term {
            current_term = term;
            role = Follower;
            voted_for = None;
        }
        
        if vote_granted {
            voted_for = Some(candidate_id);
        }
        
        send actor_ref(candidate_id).VoteResponse(current_term, vote_granted);
    }
    
    handler VoteResponse(term, vote_granted) {
        requires role == Candidate
        
        if term > current_term {
            current_term = term;
            role = Follower;
            voted_for = None;
            return;
        }
        
        if vote_granted {
            votes_received.insert(sender_id());
        }
        
        // Check if we have majority
        if votes_received.len() > (peers.len() + 1) / 2 {
            role = Leader;
            initialize_leader_state();
            send_heartbeats();
        }
    }
    
    // Raft safety properties
    invariant current_term >= 0
    invariant log.len() >= commit_index
    invariant role == Leader => votes_received.len() > (peers.len() + 1) / 2
    
    // Election safety: at most one leader per term
    safety election_safety {
        always(forall(other: RaftNode, 
                     other.role == Leader && other.current_term == current_term
                     => other.id == id))
    }
    
    // Leader append-only: leader never overwrites or deletes entries
    safety leader_append_only {
        always(role == Leader => 
               forall(i: Int, i < log.len() => log[i] == old(log[i])))
    }
    
    // Log matching: if two logs contain entry with same index and term,
    // then logs are identical in all preceding entries
    safety log_matching {
        always(forall(other: RaftNode, i: Int,
                     i < log.len() && i < other.log.len() &&
                     log[i].term == other.log[i].term
                     => forall(j: Int, j <= i => log[j] == other.log[j])))
    }
    
    // Liveness: eventually someone becomes leader
    liveness eventual_leader {
        eventually(exists(node: RaftNode, node.role == Leader))
    }
}
```

**Verification commands:**

```bash
# Check for deadlocks in consensus protocol
gal-model-check deadlock raft_consensus.gal --find-all --max-states 2000000

# Verify Raft safety properties
echo '{
  "properties": [
    {
      "name": "election_safety",
      "type": "safety",
      "formula": "G(count(node.role == Leader && node.term == T) <= 1)",
      "description": "At most one leader per term",
      "critical": true
    },
    {
      "name": "leader_append_only", 
      "type": "safety",
      "formula": "G((node.role == Leader) -> (len(node.log) >= len(old(node.log))))",
      "description": "Leader log is append-only",
      "critical": true
    },
    {
      "name": "log_matching",
      "type": "safety", 
      "formula": "G(forall nodes n1,n2: (n1.log[i].term == n2.log[i].term) -> (n1.log[0..i] == n2.log[0..i]))",
      "description": "Log matching property",
      "critical": true
    },
    {
      "name": "eventual_leader",
      "type": "liveness",
      "formula": "F(exists node: node.role == Leader)",
      "description": "Eventually someone becomes leader", 
      "critical": false
    }
  ],
  "fairness_assumptions": [
    "WF(election_timeout)",
    "WF(message_delivery)",
    "SF(leader_election)"
  ]
}' > raft_props.json

gal-model-check temporal raft_consensus.gal raft_props.json --backend tla-plus --counterexamples

# Export models for detailed analysis
gal-model-check export raft_consensus.gal models/ --formats tla-plus spin nusmv

# Performance benchmark
gal-model-check benchmark raft_consensus.gal --runs 5 --suite all
```

## Advanced Topics

### Custom Verification Algorithms

GAL allows implementing custom verification algorithms:

```rust
// custom_verifier.rs
use gal::verification::model_checker::*;

pub struct CustomBoundedVerifier {
    bound: u32,
    custom_heuristic: Box<dyn CustomHeuristic>,
}

impl CustomBoundedVerifier {
    pub fn verify_custom_property(&self, model: &ActorSystemModel, property: &CustomProperty) -> Result<VerificationResult> {
        // Implement custom verification logic
        let mut states_explored = 0;
        let mut current_bound = 0;
        
        while current_bound <= self.bound {
            let result = self.bounded_search(model, property, current_bound)?;
            
            match result {
                SearchResult::PropertyViolated(trace) => {
                    return Ok(VerificationResult::Violated(trace));
                }
                SearchResult::BoundExhausted => {
                    current_bound += 1;
                }
                SearchResult::PropertyHolds => {
                    return Ok(VerificationResult::Satisfied);
                }
            }
        }
        
        Ok(VerificationResult::Unknown)
    }
}
```

### Compositional Verification

Verify large systems by decomposing them into smaller components:

```gal
// Component verification
component DatabaseCluster {
    actors {
        primary: DatabaseNode,
        replicas: Set<DatabaseNode>,
        load_balancer: LoadBalancer,
    }
    
    composition_invariant {
        // Primary is always in replicas set
        replicas.contains(primary)
        
        // Load balancer knows about all nodes
        load_balancer.known_nodes == replicas
    }
    
    interface {
        messages {
            Read(query: Query) -> Response,
            Write(update: Update) -> Acknowledgment,
        }
        
        guarantees {
            // Linearizability
            linearizable_reads_writes(),
            
            // Fault tolerance
            survives_minority_failures(),
        }
    }
}

component WebService {
    actors {
        web_servers: Set<WebServer>,
        database: DatabaseCluster,
        cache: CacheCluster,
    }
    
    // Assume-guarantee reasoning
    assumes {
        // Database guarantees
        database.linearizable_reads_writes(),
        database.survives_minority_failures(),
        
        // Cache guarantees  
        cache.eventual_consistency(),
    }
    
    guarantees {
        // Service level properties
        response_time_under_load(),
        graceful_degradation(),
    }
}
```

### Property-Driven Development

Use verification to guide system design:

```gal
// Start with properties, then implement
specification BankingSystem {
    properties {
        // Safety properties
        safety account_integrity {
            always(forall(account: Account, account.balance >= account.minimum_balance))
        }
        
        safety transaction_atomicity {
            always(forall(tx: Transaction, 
                         tx.completed => (tx.debited && tx.credited) || (!tx.debited && !tx.credited)))
        }
        
        // Liveness properties
        liveness transaction_completion {
            always(forall(tx: Transaction, 
                         tx.submitted => eventually(tx.completed || tx.failed)))
        }
        
        // Security properties
        security authorization {
            always(forall(tx: Transaction,
                         tx.executed => authorized(tx.user, tx.account)))
        }
    }
    
    // Generate actor skeleton from properties
    generate_actors_from_properties();
}

// Implementation guided by properties
actor Account implements BankingSystem.account_integrity, BankingSystem.authorization {
    state {
        balance: Money,
        minimum_balance: Money,
        authorized_users: Set<UserId>,
    }
    
    // Auto-generated pre/post conditions from properties
    handler Withdraw(amount: Money, user: UserId) {
        requires authorized_users.contains(user)           // From security property
        requires balance - amount >= minimum_balance       // From safety property
        ensures balance == old(balance) - amount           // From atomicity property
        
        balance -= amount;
    }
}
```

### Machine Learning Integration

Use ML to guide verification and property discovery:

```gal
// ML-assisted property discovery
ml_property_discovery {
    // Learn patterns from execution traces
    execution_patterns: PatternLearner = PatternLearner::new(),
    
    // Discover potential invariants
    invariant_discovery: InvariantLearner = InvariantLearner::new(),
    
    // Anomaly detection for property violations
    anomaly_detector: AnomalyDetector = AnomalyDetector::new(),
}

actor SmartContract {
    state {
        balance: Map<Address, Uint256>,
        allowances: Map<Address, Map<Address, Uint256>>,
    }
    
    // ML discovers this pattern automatically
    @discovered_invariant
    invariant total_supply_conservation {
        balance.values().sum() == INITIAL_TOTAL_SUPPLY
    }
    
    // ML suggests this property based on common vulnerabilities
    @ml_suggested_property
    security reentrancy_protection {
        always(!exists(tx1, tx2: Transaction,
                      tx1.in_progress && tx2.in_progress && 
                      tx1.target == tx2.source))
    }
}
```

## Troubleshooting

### Common Issues

#### 1. State Space Explosion

**Problem:** Verification takes too long or runs out of memory.

**Solutions:**

```bash
# Increase abstraction level
gal-model-check deadlock system.gal --abstraction-level high

# Enable all reduction techniques  
gal-model-check deadlock system.gal --por --symmetry --hash-compaction

# Use bounded verification
gal-model-check temporal system.gal props.json --max-depth 1000

# Try different backend
gal-model-check temporal system.gal props.json --backend spin  # Often faster for safety
```

#### 2. Tool Installation Issues

**Problem:** Model checking tools not found.

**Solutions:**

```bash
# Check tool availability
which tlc spin nusmv z3

# Set tool paths explicitly
export TLC_PATH=/path/to/tla2tools.jar
export SPIN_PATH=/usr/local/bin/spin

# Use configuration file
echo '{
  "tla_config": { "tlc_path": "/opt/tla/tla2tools.jar" },
  "spin_config": { "spin_path": "/usr/local/bin/spin" }
}' > tools.json

gal-model-check --config tools.json deadlock system.gal
```

#### 3. Property Specification Errors

**Problem:** Properties are incorrectly specified or not being verified.

**Solutions:**

```bash
# Validate property syntax
gal-model-check validate properties.json

# Use simpler properties first
echo '{
  "properties": [
    {
      "name": "simple_safety",
      "type": "safety",
      "formula": "G(true)",
      "critical": false
    }
  ]
}' > test_props.json

# Check property translation
gal-model-check export system.gal models/ --formats tla-plus
# Inspect generated TLA+ specification
```

#### 4. Counterexample Interpretation

**Problem:** Counterexamples are difficult to understand.

**Solutions:**

```bash
# Generate minimal counterexamples
gal-model-check temporal system.gal props.json --counterexamples --minimize

# Use interactive mode
gal-model-check interactive system.gal

# Export detailed traces
gal-model-check temporal system.gal props.json --format json > results.json
```

### Performance Tuning

#### Memory Optimization

```json
{
  "optimization": {
    "memory": {
      "limit": "16GB",
      "hash_compaction": true,
      "compression_level": 3,
      "garbage_collection": true,
      "gc_aggressive": true
    }
  }
}
```

#### Parallel Verification

```json
{
  "performance": {
    "parallel_threads": 16,
    "worker_distribution": "dynamic",
    "load_balancing": true
  }
}
```

#### Disk Storage

```json
{
  "storage": {
    "use_disk": true,
    "storage_path": "/fast/ssd/gal_verification",
    "compression": true,
    "cache_size": "4GB"
  }
}
```

### Debugging Verification Issues

#### Enable Debug Logging

```bash
export GAL_LOG_LEVEL=debug
gal-model-check --verbose deadlock system.gal
```

#### Step-by-Step Verification

```bash
gal-model-check interactive system.gal
```

```
gal-verify> load system.gal
gal-verify> step init
gal-verify> step 1
gal-verify> analyze current
gal-verify> deadlock check
```

#### Incremental Verification

```gal
// Verify components in isolation first
actor SimpleComponent {
    // Start with minimal functionality
    state { counter: Int = 0 }
    
    handler Increment() {
        counter += 1;
    }
    
    invariant counter >= 0
}

// Then add complexity gradually
actor ComplexComponent {
    state {
        simple: ActorRef<SimpleComponent>,
        additional_state: ComplexState,
    }
    
    // Verify composition properties
    composition_invariant {
        simple.counter <= MAX_SIMPLE_COUNTER
    }
}
```

### Getting Help

#### Documentation and Resources

- Official GAL documentation: https://gal-lang.org/docs/model-checking
- Model checking tutorials: https://gal-lang.org/tutorials/verification
- Community forum: https://forum.gal-lang.org/c/model-checking
- GitHub issues: https://github.com/gal-lang/gal/issues

#### Reporting Bugs

When reporting verification issues, include:

1. GAL version: `gal-model-check --version`
2. System information: OS, memory, CPU
3. Complete command used
4. Configuration file (if any)
5. Minimal reproducing example
6. Expected vs. actual behavior
7. Relevant log output with `--verbose`

```bash
# Generate debug report
gal-model-check debug-report system.gal > debug_report.txt
```

#### Community Support

- Discord: https://discord.gg/gal-lang
- Stack Overflow: Use tag `gal-model-checking`
- Reddit: r/gal_programming

---

This comprehensive documentation covers all aspects of GAL's model checking system. The integration provides world-class formal verification capabilities that enable developers to build reliable, correct concurrent actor systems with mathematical guarantees about their behavior.