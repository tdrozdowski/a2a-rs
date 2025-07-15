# a2a-rs

A Rust implementation of the A2A (Agent-to-Agent) protocol, which enables communication between AI agents.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## About

a2a-rs is a comprehensive Rust implementation of the A2A (Agent-to-Agent) protocol version 0.2.5. The A2A protocol is designed to enable standardized communication between AI agents, allowing them to exchange messages, manage tasks, and coordinate activities.

This library provides:
- Complete type definitions for all A2A protocol components
- Helper functions for creating and working with A2A messages
- Serialization/deserialization support via serde
- Comprehensive error handling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
a2a-rs = "0.1.0"
```

## Usage

The library provides helper functions for common operations:

- `create_agent_card()` - Create an agent card with capabilities and skills
- `create_send_message_request()` - Create a request to send a message to an agent
- `create_get_task_request()` - Create a request to get the status of a task
- `create_cancel_task_request()` - Create a request to cancel a task

For example, to send a message to an agent, you would:
1. Import the necessary types from a2a-rs
2. Use the create_send_message_request helper function with appropriate parameters
3. Serialize the request to JSON
4. Send the JSON to the agent

## Features

- **Complete Protocol Implementation**: Implements all aspects of the A2A protocol version 0.2.5
- **Type Safety**: Uses Rust's strong type system to ensure protocol correctness
- **Serialization/Deserialization**: Full support for JSON serialization and deserialization
- **Helper Functions**: Convenient functions for creating common A2A messages
- **Error Handling**: Comprehensive error types for all protocol-related errors
- **Backward Compatibility**: Supports both new method names (e.g., "message/send") and old method names (e.g., "sendMessage")

## API Documentation

For detailed API documentation, please refer to the documentation comments in the source code.

## A2A Protocol

The A2A (Agent-to-Agent) protocol is a standardized way for AI agents to communicate with each other. It defines:

- Agent capabilities and skills
- Message formats for requests and responses
- Task management (creation, status checking, cancellation)
- Security schemes for authentication
- Push notification configurations

For more information about the A2A protocol, refer to the specification in the `src/a2a.json` file.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.