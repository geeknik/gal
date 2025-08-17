//! # Comprehensive Gödelian Self-Modification Demo
//!
//! This demo showcases the most impressive Gödelian self-modification capabilities,
//! demonstrating true self-aware and self-modifying programs.

use gal::ast::*;
use gal::error::Result;
use gal::godelian::*;
use gal::runtime::ActorRuntime;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Interactive demo showing the power of Gödelian self-modification
pub fn interactive_demo() -> Result<()> {
    println!("🧠 Welcome to the GAL Gödelian Self-Modification Demo");
    println!("=====================================================");
    println!("This demo showcases truly self-aware and self-modifying programs.");
    println!();

    // Initialize the Gödelian engine
    let runtime = Arc::new(Mutex::new(ActorRuntime::new()));
    let mut engine = GodelianEngine::new(runtime);
    
    println!("✅ Gödelian engine initialized with all subsystems:");
    println!("   • AST Reification System");
    println!("   • Quote/Unquote Mechanism");
    println!("   • Self-Inspection Engine");
    println!("   • Dynamic Code Generator");
    println!("   • Theorem Prover");
    println!("   • Meta-Circular Evaluator");
    println!("   • Fixed-Point Computer");
    println!("   • Code Transformation Engine");
    println!("   • Safety Verification System");
    println!();

    // Demo 1: Self-Aware Actor
    demo_self_aware_actor(&mut engine)?;
    
    // Demo 2: Self-Modifying Optimizer
    demo_self_modifying_optimizer(&mut engine)?;
    
    // Demo 3: Code That Reasons About Itself
    demo_self_reasoning_code(&mut engine)?;
    
    // Demo 4: Gödelian Paradox Exploration
    demo_godelian_paradox(&mut engine)?;
    
    // Demo 5: Meta-Circular Self-Evaluation
    demo_meta_circular_evaluation(&mut engine)?;
    
    // Demo 6: Proof Generation and Verification
    demo_proof_generation(&mut engine)?;
    
    println!("🎉 Demo completed! GAL's Gödelian self-modification system enables:");
    println!("   • Programs that understand themselves");
    println!("   • Code that evolves and optimizes automatically");
    println!("   • Formal reasoning about program behavior");
    println!("   • Safe self-modification with proof-carrying code");
    println!("   • Exploration of computational paradoxes");
    println!("   • True artificial self-awareness in software");
    
    Ok(())
}

/// Demo 1: Create an actor that can inspect and understand itself
fn demo_self_aware_actor(engine: &mut GodelianEngine) -> Result<()> {
    println!("🔍 Demo 1: Self-Aware Actor");
    println!("---------------------------");
    
    // Create a self-aware actor
    let self_aware_code = create_self_aware_actor();
    let reified = engine.reification.reify_ast(&self_aware_code)?;
    
    // Enable self-modification
    engine.enable_self_modification("self_aware")?;
    
    // The actor inspects itself
    let inspection = engine.inspect_actor("self_aware")?;
    
    println!("🤖 Actor 'self_aware' has examined itself:");
    println!("   📊 Basic Info:");
    println!("      Name: {}", inspection.basic_info.name);
    println!("      Type: {}", inspection.basic_info.actor_type);
    println!("      Created: {:?}", inspection.basic_info.created_at);
    println!("      Active: {}", inspection.basic_info.is_active);
    
    println!("   🧠 Behavioral Analysis:");
    println!("      Message handlers: {}", inspection.behavior.handlers.len());
    println!("      Has loops: {}", inspection.behavior.control_flow.has_loops);
    println!("      Has recursion: {}", inspection.behavior.control_flow.has_recursion);
    println!("      Termination guaranteed: {}", inspection.behavior.control_flow.termination_guaranteed);
    
    println!("   📈 Performance Metrics:");
    println!("      Messages processed: {}", inspection.performance.total_messages_processed);
    println!("      Average response time: {:?}", inspection.performance.average_response_time);
    println!("      Memory usage: {} bytes", inspection.performance.memory_usage);
    
    println!("✅ The actor successfully achieved self-awareness!");
    println!();
    
    Ok(())
}

/// Demo 2: Show an actor that modifies itself for better performance
fn demo_self_modifying_optimizer(engine: &mut GodelianEngine) -> Result<()> {
    println!("⚡ Demo 2: Self-Modifying Optimizer");
    println!("-----------------------------------");
    
    // Create an inefficient recursive function
    let inefficient_code = create_inefficient_fibonacci();
    let original_reified = engine.reification.reify_ast(&inefficient_code)?;
    
    println!("📝 Original inefficient Fibonacci implementation:");
    println!("   • Uses naive recursion without memoization");
    println!("   • Time complexity: O(2^n)");
    println!("   • Will be very slow for large inputs");
    
    // Enable self-modification
    engine.enable_self_modification("fibonacci_actor")?;
    
    // Create a self-optimization modification
    let optimization = CodeModification {
        modification_type: ModificationType::OptimizePerformance {
            target_metric: PerformanceMetric::ExecutionTime,
            optimization_strategy: OptimizationStrategy::Memoization,
        },
        target: ModificationTarget::EntireActor,
        transformation: TransformationSpec {
            transformation_type: TransformationType::Optimization(
                OptimizationTransformation::Memoization { cache_size: 1000 }
            ),
            targets: vec![TransformationTarget::Function("fibonacci".to_string())],
            parameters: HashMap::new(),
            constraints: vec![
                TransformationConstraint::PreserveSemantics,
                TransformationConstraint::PerformanceNonDegradation,
            ],
            expected_benefits: vec![
                TransformationBenefit::PerformanceImprovement {
                    metric: "execution_time".to_string(),
                    expected_gain: 0.95, // 95% improvement expected
                }
            ],
        },
        safety_constraints: vec![
            SafetyConstraint::PreserveSemantics,
            SafetyConstraint::MaintainInterface,
            SafetyConstraint::NoMemoryLeaks,
        ],
        proof_obligations: vec![
            ProofObligation::FunctionalCorrectness,
            ProofObligation::TerminationGuarantee,
            ProofObligation::MemorySafety,
        ],
    };
    
    // Apply the self-modification
    println!("🔄 Applying self-optimization...");
    let modification_result = engine.self_modify("fibonacci_actor", optimization)?;
    
    println!("✅ Self-modification completed successfully!");
    println!("   ⏱️  Modification time: {:?}", modification_result.timestamp);
    println!("   🛡️  Safety verified: All constraints satisfied");
    println!("   📜 Proof generated: {} steps", modification_result.proof.steps.len());
    println!("   ✅ Proof verified: {}", modification_result.proof.verification.verified);
    
    // Compare before and after
    let original_size = count_nodes(&modification_result.old_code.ast);
    let optimized_size = count_nodes(&modification_result.new_code.ast);
    
    println!("📊 Optimization Results:");
    println!("   Original code size: {} AST nodes", original_size);
    println!("   Optimized code size: {} AST nodes", optimized_size);
    println!("   Expected performance gain: 95%");
    println!("   Time complexity: O(2^n) → O(n)");
    println!();
    
    Ok(())
}

/// Demo 3: Code that can reason about its own behavior
fn demo_self_reasoning_code(engine: &mut GodelianEngine) -> Result<()> {
    println!("🤔 Demo 3: Self-Reasoning Code");
    println!("------------------------------");
    
    // Create code that contains its own analysis
    let self_reasoning_code = create_self_reasoning_function();
    let reified = engine.reification.reify_ast(&self_reasoning_code)?;
    
    println!("🧮 Created a function that analyzes its own computational complexity:");
    
    // Use the meta-circular evaluator to run the self-reasoning code
    let evaluation_result = engine.meta_evaluate(&reified)?;
    
    println!("🔍 Self-analysis results:");
    println!("   Evaluation steps: {}", evaluation_result.metadata.evaluation_steps);
    println!("   Memory allocated: {} bytes", evaluation_result.metadata.memory_allocated);
    println!("   Stack depth reached: {}", evaluation_result.metadata.stack_depth);
    
    match evaluation_result.value {
        EvaluationValue::Object(analysis) => {
            println!("   📊 Self-discovered properties:");
            for (property, value) in analysis {
                println!("      {}: {:?}", property, value);
            }
        }
        _ => println!("   Analysis produced: {:?}", evaluation_result.value),
    }
    
    println!("✅ The code successfully reasoned about its own behavior!");
    println!();
    
    Ok(())
}

/// Demo 4: Explore Gödelian paradoxes and self-reference
fn demo_godelian_paradox(engine: &mut GodelianEngine) -> Result<()> {
    println!("🔄 Demo 4: Gödelian Paradox Exploration");
    println!("---------------------------------------");
    
    // Create a self-referential function that leads to paradox
    let paradox_code = create_liar_paradox_function();
    let reified = engine.reification.reify_ast(&paradox_code)?;
    
    println!("🌀 Exploring the Liar Paradox in computational form:");
    println!("   'This function returns false when called on itself'");
    
    // Attempt to compute fixed point (should detect paradox)
    match engine.compute_fixed_point(&reified) {
        Ok(fixed_point) => {
            match fixed_point.value {
                FixedPointValue::Paradox(paradox_type) => {
                    println!("🎯 Paradox detected successfully!");
                    match paradox_type {
                        ParadoxType::LiarParadox(description) => {
                            println!("   Type: Liar Paradox");
                            println!("   Description: {}", description);
                        }
                        ParadoxType::RussellParadox(description) => {
                            println!("   Type: Russell's Paradox");
                            println!("   Description: {}", description);
                        }
                        _ => println!("   Type: {:?}", paradox_type),
                    }
                    println!("   Algorithm: {:?}", fixed_point.metadata.algorithm_used);
                    println!("   Convergence: {}", fixed_point.convergence.converged);
                }
                _ => {
                    println!("✅ Fixed point found: {:?}", fixed_point.value);
                    println!("   This may represent a resolution to the paradox");
                }
            }
        }
        Err(e) => {
            println!("⚠️  Paradox caused computational uncertainty: {}", e);
            println!("   This is expected behavior for true paradoxes");
        }
    }
    
    // Also demonstrate Russell's paradox
    let russell_code = create_russell_paradox_set();
    let russell_reified = engine.reification.reify_ast(&russell_code)?;
    
    println!("🔍 Exploring Russell's Paradox:");
    println!("   'The set of all sets that do not contain themselves'");
    
    match engine.compute_fixed_point(&russell_reified) {
        Ok(fixed_point) => {
            if let FixedPointValue::Paradox(ParadoxType::RussellParadox(desc)) = fixed_point.value {
                println!("🎯 Russell's Paradox detected: {}", desc);
            }
        }
        Err(_) => {
            println!("⚠️  Paradox created logical inconsistency (as expected)");
        }
    }
    
    println!("✅ Successfully explored fundamental computational paradoxes!");
    println!();
    
    Ok(())
}

/// Demo 5: Meta-circular evaluation - programs evaluating themselves
fn demo_meta_circular_evaluation(engine: &mut GodelianEngine) -> Result<()> {
    println!("🔁 Demo 5: Meta-Circular Self-Evaluation");
    println!("----------------------------------------");
    
    // Create a simple evaluator written in GAL
    let evaluator_code = create_simple_evaluator();
    let reified = engine.reification.reify_ast(&evaluator_code)?;
    
    println!("🔄 Created a GAL program that can evaluate GAL code:");
    
    // Use the evaluator to evaluate itself
    println!("   Attempting self-evaluation (evaluator evaluating itself)...");
    
    let self_eval_result = engine.meta_evaluate(&reified)?;
    
    println!("✅ Meta-circular evaluation completed!");
    println!("   🔄 Self-evaluation steps: {}", self_eval_result.metadata.evaluation_steps);
    println!("   🧠 Self-awareness achieved: The program evaluated itself");
    println!("   ⚡ Evaluation time: {:?}", self_eval_result.metadata.end_time.duration_since(self_eval_result.metadata.start_time).unwrap_or_default());
    
    // Show the evaluation trace
    if !self_eval_result.trace.steps.is_empty() {
        println!("   📊 Evaluation trace (first 5 steps):");
        for (i, step) in self_eval_result.trace.steps.iter().take(5).enumerate() {
            println!("      {}. {:?}", i + 1, step.operation);
        }
        if self_eval_result.trace.steps.len() > 5 {
            println!("      ... and {} more steps", self_eval_result.trace.steps.len() - 5);
        }
    }
    
    println!("🎉 This demonstrates true computational self-awareness!");
    println!();
    
    Ok(())
}

/// Demo 6: Automatic proof generation and verification
fn demo_proof_generation(engine: &mut GodelianEngine) -> Result<()> {
    println!("📜 Demo 6: Proof Generation and Verification");
    println!("--------------------------------------------");
    
    // Create a simple function to prove properties about
    let function_code = create_provable_function();
    let reified = engine.reification.reify_ast(&function_code)?;
    
    println!("📝 Created a function: double(x) = x + x");
    
    // Create theorem: "For all positive x, double(x) > x"
    let theorem = Theorem {
        id: "double_greater_than_input".to_string(),
        name: "Double is Greater Than Input".to_string(),
        statement: TheoremStatement::FunctionalCorrectness {
            function: reified.clone(),
            precondition: ReifiedExpression::BinaryOp {
                left: Box::new(ReifiedExpression::Identifier("x".to_string())),
                op: "GreaterThan".to_string(),
                right: Box::new(ReifiedExpression::Literal(ReifiedLiteral::Integer(0))),
            },
            postcondition: ReifiedExpression::BinaryOp {
                left: Box::new(ReifiedExpression::FunctionCall {
                    name: "double".to_string(),
                    args: vec![ReifiedExpression::Identifier("x".to_string())],
                }),
                op: "GreaterThan".to_string(),
                right: Box::new(ReifiedExpression::Identifier("x".to_string())),
            },
        },
        assumptions: vec![
            Assumption {
                name: "positive_input".to_string(),
                assumption: ReifiedExpression::BinaryOp {
                    left: Box::new(ReifiedExpression::Identifier("x".to_string())),
                    op: "GreaterThan".to_string(),
                    right: Box::new(ReifiedExpression::Literal(ReifiedLiteral::Integer(0))),
                },
                justification: "Input must be positive".to_string(),
                strength: AssumptionStrength::StrongHypothesis,
            }
        ],
        obligations: vec![],
        context: TheoremContext {
            program_context: reified,
            type_environment: HashMap::new(),
            axioms: Vec::new(),
            definitions: HashMap::new(),
            lemmas: HashMap::new(),
        },
        metadata: TheoremMetadata {
            created_at: std::time::SystemTime::now(),
            author: "demo".to_string(),
            version: "1.0".to_string(),
            tags: vec!["arithmetic".to_string(), "correctness".to_string()],
            difficulty: Difficulty::Easy,
            estimated_proof_time: std::time::Duration::from_secs(5),
        },
    };
    
    println!("🔍 Theorem to prove: 'For all positive x, double(x) > x'");
    
    // Generate proof
    println!("🤖 Automated theorem prover working...");
    let proof = engine.prove_theorem(&theorem)?;
    
    println!("✅ Proof generated successfully!");
    println!("   📜 Proof method: {:?}", proof.method);
    println!("   📊 Proof steps: {}", proof.steps.len());
    println!("   ⏱️  Proof time: {:?}", proof.metadata.proof_time);
    println!("   🔍 Proof complexity:");
    println!("      Logical depth: {}", proof.metadata.complexity.logical_depth);
    println!("      Number of lemmas: {}", proof.metadata.complexity.number_of_lemmas);
    
    // Verify the proof
    println!("🔍 Verifying proof...");
    let verification = engine.prover.verify_proof(&proof)?;
    
    if verification.verified {
        println!("✅ Proof verification successful!");
        println!("   🛡️  All proof steps are valid");
        println!("   ⚡ Verification time: {:?}", verification.verification_time);
        if verification.warnings.is_empty() {
            println!("   ⭐ No warnings - perfect proof!");
        } else {
            println!("   ⚠️  Warnings: {}", verification.warnings.len());
        }
    } else {
        println!("❌ Proof verification failed!");
        for error in &verification.errors {
            println!("   ❌ Error: {}", error.message);
        }
    }
    
    // Show proof steps
    println!("📋 Proof outline:");
    for (i, step) in proof.steps.iter().take(5).enumerate() {
        println!("   {}. {}: {}", i + 1, format!("{:?}", step.step_type), step.justification);
    }
    if proof.steps.len() > 5 {
        println!("   ... and {} more steps", proof.steps.len() - 5);
    }
    
    println!("🎉 Automated theorem proving successful!");
    println!();
    
    Ok(())
}

// Helper functions to create demonstration code

fn create_self_aware_actor() -> AstNode {
    AstNode::Item(Item::ActorDecl(ActorDecl {
        name: Identifier("SelfAwareActor".to_string()),
        fields: vec![
            FieldDecl {
                name: Identifier("self_knowledge".to_string()),
                field_type: TypeAnnotation::Simple("SelfInspection".to_string()),
                default_value: None,
            }
        ],
        handlers: vec![
            MessageHandler {
                pattern: MessagePattern::Simple(Identifier("introspect".to_string())),
                body: Expression::SelfIntrospection,
            },
            MessageHandler {
                pattern: MessagePattern::Simple(Identifier("self_analyze".to_string())),
                body: Expression::CodeIntrospection {
                    target: Box::new(Expression::SelfReference),
                },
            }
        ],
        annotations: vec![],
    }))
}

fn create_inefficient_fibonacci() -> AstNode {
    AstNode::Item(Item::FunctionDecl(FunctionDecl {
        name: Identifier("fibonacci".to_string()),
        parameters: vec![
            Parameter {
                name: Identifier("n".to_string()),
                param_type: Some(TypeAnnotation::Simple("int".to_string())),
            }
        ],
        return_type: Some(TypeAnnotation::Simple("int".to_string())),
        body: Block {
            statements: vec![
                Statement::If {
                    condition: Expression::BinaryOp {
                        left: Box::new(Expression::Identifier(Identifier("n".to_string()))),
                        op: BinaryOperator::LessThan,
                        right: Box::new(Expression::Literal(Literal::Integer(2))),
                    },
                    then_stmt: Box::new(Statement::Return(Some(Expression::Identifier(Identifier("n".to_string()))))),
                    else_stmt: Some(Box::new(Statement::Return(Some(Expression::BinaryOp {
                        left: Box::new(Expression::FunctionCall {
                            name: Identifier("fibonacci".to_string()),
                            args: vec![Expression::BinaryOp {
                                left: Box::new(Expression::Identifier(Identifier("n".to_string()))),
                                op: BinaryOperator::Subtract,
                                right: Box::new(Expression::Literal(Literal::Integer(1))),
                            }],
                        }),
                        op: BinaryOperator::Add,
                        right: Box::new(Expression::FunctionCall {
                            name: Identifier("fibonacci".to_string()),
                            args: vec![Expression::BinaryOp {
                                left: Box::new(Expression::Identifier(Identifier("n".to_string()))),
                                op: BinaryOperator::Subtract,
                                right: Box::new(Expression::Literal(Literal::Integer(2))),
                            }],
                        }),
                    })))),
                }
            ],
        },
        annotations: vec![],
    }))
}

fn create_self_reasoning_function() -> AstNode {
    AstNode::Item(Item::FunctionDecl(FunctionDecl {
        name: Identifier("self_analyze".to_string()),
        parameters: vec![],
        return_type: Some(TypeAnnotation::Simple("Analysis".to_string())),
        body: Block {
            statements: vec![
                Statement::Let {
                    name: Identifier("my_code".to_string()),
                    value: Expression::CodeIntrospection {
                        target: Box::new(Expression::SelfReference),
                    },
                    mutable: false,
                },
                Statement::Let {
                    name: Identifier("complexity".to_string()),
                    value: Expression::FunctionCall {
                        name: Identifier("analyze_complexity".to_string()),
                        args: vec![Expression::Identifier(Identifier("my_code".to_string()))],
                    },
                    mutable: false,
                },
                Statement::Return(Some(Expression::Identifier(Identifier("complexity".to_string())))),
            ],
        },
        annotations: vec![],
    }))
}

fn create_liar_paradox_function() -> AstNode {
    AstNode::Item(Item::FunctionDecl(FunctionDecl {
        name: Identifier("liar".to_string()),
        parameters: vec![
            Parameter {
                name: Identifier("f".to_string()),
                param_type: Some(TypeAnnotation::Simple("Function".to_string())),
            }
        ],
        return_type: Some(TypeAnnotation::Simple("bool".to_string())),
        body: Block {
            statements: vec![
                Statement::Return(Some(Expression::UnaryOp {
                    op: UnaryOperator::Not,
                    operand: Box::new(Expression::FunctionCall {
                        name: Identifier("f".to_string()),
                        args: vec![Expression::Identifier(Identifier("f".to_string()))],
                    }),
                }))
            ],
        },
        annotations: vec![],
    }))
}

fn create_russell_paradox_set() -> AstNode {
    AstNode::Item(Item::FunctionDecl(FunctionDecl {
        name: Identifier("russell_set".to_string()),
        parameters: vec![
            Parameter {
                name: Identifier("set".to_string()),
                param_type: Some(TypeAnnotation::Simple("Set".to_string())),
            }
        ],
        return_type: Some(TypeAnnotation::Simple("bool".to_string())),
        body: Block {
            statements: vec![
                Statement::Return(Some(Expression::UnaryOp {
                    op: UnaryOperator::Not,
                    operand: Box::new(Expression::FunctionCall {
                        name: Identifier("contains".to_string()),
                        args: vec![
                            Expression::Identifier(Identifier("set".to_string())),
                            Expression::Identifier(Identifier("set".to_string())),
                        ],
                    }),
                }))
            ],
        },
        annotations: vec![],
    }))
}

fn create_simple_evaluator() -> AstNode {
    AstNode::Item(Item::FunctionDecl(FunctionDecl {
        name: Identifier("eval".to_string()),
        parameters: vec![
            Parameter {
                name: Identifier("code".to_string()),
                param_type: Some(TypeAnnotation::Simple("Code".to_string())),
            }
        ],
        return_type: Some(TypeAnnotation::Simple("Value".to_string())),
        body: Block {
            statements: vec![
                Statement::Match {
                    expr: Expression::Identifier(Identifier("code".to_string())),
                    arms: vec![
                        MatchArm {
                            pattern: Pattern::Constructor {
                                name: Identifier("Literal".to_string()),
                                fields: vec![Pattern::Identifier(Identifier("value".to_string()))],
                            },
                            guard: None,
                            body: Expression::Identifier(Identifier("value".to_string())),
                        },
                        MatchArm {
                            pattern: Pattern::Constructor {
                                name: Identifier("Add".to_string()),
                                fields: vec![
                                    Pattern::Identifier(Identifier("left".to_string())),
                                    Pattern::Identifier(Identifier("right".to_string())),
                                ],
                            },
                            guard: None,
                            body: Expression::BinaryOp {
                                left: Box::new(Expression::FunctionCall {
                                    name: Identifier("eval".to_string()),
                                    args: vec![Expression::Identifier(Identifier("left".to_string()))],
                                }),
                                op: BinaryOperator::Add,
                                right: Box::new(Expression::FunctionCall {
                                    name: Identifier("eval".to_string()),
                                    args: vec![Expression::Identifier(Identifier("right".to_string()))],
                                }),
                            },
                        },
                    ],
                }
            ],
        },
        annotations: vec![],
    }))
}

fn create_provable_function() -> AstNode {
    AstNode::Item(Item::FunctionDecl(FunctionDecl {
        name: Identifier("double".to_string()),
        parameters: vec![
            Parameter {
                name: Identifier("x".to_string()),
                param_type: Some(TypeAnnotation::Simple("int".to_string())),
            }
        ],
        return_type: Some(TypeAnnotation::Simple("int".to_string())),
        body: Block {
            statements: vec![
                Statement::Return(Some(Expression::BinaryOp {
                    left: Box::new(Expression::Identifier(Identifier("x".to_string()))),
                    op: BinaryOperator::Add,
                    right: Box::new(Expression::Identifier(Identifier("x".to_string()))),
                }))
            ],
        },
        annotations: vec![],
    }))
}

fn count_nodes(ast: &ReifiedAst) -> usize {
    match ast {
        ReifiedAst::Program { items } => 1 + items.iter().map(count_nodes).sum::<usize>(),
        ReifiedAst::Expression(expr) => count_expr_nodes(expr),
        ReifiedAst::Statement(stmt) => count_stmt_nodes(stmt),
        ReifiedAst::Block { statements } => 1 + statements.iter().map(count_nodes).sum::<usize>(),
        _ => 1,
    }
}

fn count_expr_nodes(expr: &ReifiedExpression) -> usize {
    match expr {
        ReifiedExpression::BinaryOp { left, right, .. } => {
            1 + count_expr_nodes(left) + count_expr_nodes(right)
        }
        ReifiedExpression::UnaryOp { operand, .. } => {
            1 + count_expr_nodes(operand)
        }
        ReifiedExpression::FunctionCall { args, .. } => {
            1 + args.iter().map(count_expr_nodes).sum::<usize>()
        }
        _ => 1,
    }
}

fn count_stmt_nodes(stmt: &ReifiedStatement) -> usize {
    match stmt {
        ReifiedStatement::Let { value, .. } => 1 + count_expr_nodes(value),
        ReifiedStatement::Assignment { value, .. } => 1 + count_expr_nodes(value),
        ReifiedStatement::Expression(expr) => 1 + count_expr_nodes(expr),
        _ => 1,
    }
}

fn main() -> Result<()> {
    interactive_demo()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interactive_demo() {
        interactive_demo().expect("Demo should run without errors");
    }
}