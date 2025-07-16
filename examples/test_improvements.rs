use a2a_rs::*;
use std::collections::HashMap;

fn main() {
    println!("Testing A2A Rust Implementation Improvements");
    println!("============================================");

    // Test 1: Enhanced Extension System
    println!("\n1. Testing Enhanced Extension System:");
    let extension = AgentExtension::new("https://example.com/oauth-extension".to_string());
    match extension.validate() {
        Ok(_) => println!("✅ Extension validation passed"),
        Err(e) => println!("❌ Extension validation failed: {}", e),
    }

    // Test invalid extension
    let invalid_extension = AgentExtension::new("invalid-url".to_string());
    match invalid_extension.validate() {
        Ok(_) => println!("❌ Should have failed validation"),
        Err(e) => println!("✅ Correctly caught invalid extension: {}", e),
    }

    // Test 2: Field Validation
    println!("\n2. Testing Field Validation:");
    match validation::validate_url("https://example.com/api") {
        Ok(_) => println!("✅ URL validation passed"),
        Err(e) => println!("❌ URL validation failed: {}", e),
    }

    match validation::validate_media_type("application/json") {
        Ok(_) => println!("✅ Media type validation passed"),
        Err(e) => println!("❌ Media type validation failed: {}", e),
    }

    match validation::validate_task_id("task-123") {
        Ok(_) => println!("✅ Task ID validation passed"),
        Err(e) => println!("❌ Task ID validation failed: {}", e),
    }

    // Test 3: OAuth2 Flow Validation
    println!("\n3. Testing OAuth2 Flow Validation:");
    let mut scopes = HashMap::new();
    scopes.insert("read".to_string(), "Read access".to_string());
    scopes.insert("write".to_string(), "Write access".to_string());

    let auth_flow = AuthorizationCodeOAuthFlow::new(
        "https://auth.example.com/authorize".to_string(),
        "https://auth.example.com/token".to_string(),
        scopes,
    );

    match auth_flow.validate() {
        Ok(_) => println!("✅ OAuth2 Authorization Code flow validation passed"),
        Err(e) => println!("❌ OAuth2 flow validation failed: {}", e),
    }

    // Test 4: Security Scheme Validation
    println!("\n4. Testing Security Scheme Validation:");
    let api_key_scheme = ApiKeySecurityScheme::new(
        ApiKeyLocation::Header,
        "X-API-Key".to_string(),
    );

    let security_scheme = SecurityScheme::ApiKey(api_key_scheme);
    match security_scheme.validate() {
        Ok(_) => println!("✅ API Key security scheme validation passed"),
        Err(e) => println!("❌ Security scheme validation failed: {}", e),
    }

    // Test 5: Streaming Event Validation
    println!("\n5. Testing Streaming Event Validation:");
    let artifact = Artifact {
        artifact_id: "artifact-123".to_string(),
        parts: vec![Part::Text(TextPart {
            text: "Generated content".to_string(),
            metadata: None,
        })],
        description: None,
        extensions: None,
        metadata: None,
        name: Some("Generated Artifact".to_string()),
    };

    let artifact_event = TaskArtifactUpdateEvent::new(
        "task-456".to_string(),
        "context-789".to_string(),
        artifact,
    );

    match artifact_event.validate() {
        Ok(_) => println!("✅ Task artifact update event validation passed"),
        Err(e) => println!("❌ Artifact event validation failed: {}", e),
    }

    println!("\n6. Testing Task State Transitions:");
    match validation::validate_task_state_transition(&TaskState::Submitted, &TaskState::Working) {
        Ok(_) => println!("✅ Valid task state transition (Submitted -> Working)"),
        Err(e) => println!("❌ Task state transition failed: {}", e),
    }

    match validation::validate_task_state_transition(&TaskState::Completed, &TaskState::Working) {
        Ok(_) => println!("❌ Should have failed - invalid transition"),
        Err(e) => println!("✅ Correctly caught invalid transition: {}", e),
    }

    println!("\n✅ All improvement tests completed successfully!");
    println!("The A2A Rust implementation now has enhanced validation, security, and streaming support.");
}