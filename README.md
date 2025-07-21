# a2a-rs

A Rust implementation of the A2A (Agent-to-Agent) protocol, which enables communication between AI agents.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Important: Understanding A2A Key Concepts

**Before using this crate, please read the [A2A Protocol Key Concepts](https://a2a-protocol.org/latest/topics/key-concepts/).** This documentation contains critical information about the A2A protocol that you need to understand to effectively use this library.

## About

a2a-rs is a comprehensive Rust implementation of the A2A (Agent-to-Agent) protocol version 0.2.6. The A2A protocol is designed to enable standardized communication between AI agents, allowing them to exchange messages, manage tasks, and coordinate activities.

This library provides:
- Complete type definitions for all A2A protocol components
- Struct methods for creating and working with A2A messages
- Serialization/deserialization support via serde
- Comprehensive error handling and validation
- Security scheme implementations (API Key, HTTP, OAuth2, OpenID Connect)
- OAuth2 flow support with validation
- Streaming event handling for real-time updates
- Field validation for URLs, media types, task IDs, and more
- Task state transition validation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
a2a-rs = "0.2.7"
```

## Usage

### Basic Operations

The library provides struct methods for common operations:

- `AgentCard::new()` - Create an agent card with capabilities and skills
- `SendMessageRequest::new()` - Create a request to send a message to an agent
- `GetTaskRequest::new()` - Create a request to get the status of a task
- `CancelTaskRequest::new()` - Create a request to cancel a task

### Security and Authentication

- `ApiKeySecurityScheme::new()` - Create API key authentication schemes
- `HttpSecurityScheme::new()` - Create HTTP authentication schemes
- `OAuth2SecurityScheme::new()` - Create OAuth2 authentication schemes
- `SecurityScheme::validate()` - Validate security scheme configurations

### Validation Functions

The library includes comprehensive validation for:
- URLs and media types
- Task IDs and message IDs
- Agent names and versions
- Task state transitions
- Extension configurations

### Streaming and Events

- `TaskArtifactUpdateEvent::new()` - Create task artifact update events
- `TaskStatusUpdateEvent::new()` - Create task status update events
- Support for real-time streaming of task updates and artifacts

### Basic Usage Pattern

1. Import the necessary types from a2a-rs
2. Create requests using the provided constructor methods
3. Validate configurations using built-in validation functions
4. Serialize requests to JSON using serde
5. Send JSON to agents and handle responses

## Features

- **Complete Protocol Implementation**: Implements all aspects of the A2A protocol version 0.2.6
- **Type Safety**: Uses Rust's strong type system to ensure protocol correctness
- **Serialization/Deserialization**: Full support for JSON serialization and deserialization
- **Comprehensive Validation**: Built-in validation for URLs, media types, task IDs, agent names, and more
- **Security Schemes**: Full support for API Key, HTTP, OAuth2, and OpenID Connect authentication
- **OAuth2 Flows**: Complete implementation of all OAuth2 flows with validation
- **Streaming Support**: Real-time streaming events for task updates and artifacts
- **Task Management**: Complete task lifecycle management with state transition validation
- **Extension System**: Support for agent extensions with validation
- **Struct Methods**: Convenient methods on structs for creating common A2A messages
- **Error Handling**: Comprehensive error types for all protocol-related errors
- **Backward Compatibility**: Supports both new method names (e.g., "message/send") and old method names (e.g., "sendMessage")

## API Documentation

For detailed API documentation, please refer to the documentation comments in the source code.

## Protocol Specification

For implementation details, refer to the specification in the `src/a2a.json` file.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
