//! A2A (Agent-to-Agent) protocol implementation in Rust
//!
//! This crate provides types and functionality for working with the A2A protocol,
//! which enables communication between AI agents.
//!
//! The implementation is based on the A2A specification version 0.2.5.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The current version of the A2A protocol implemented by this crate.
pub const PROTOCOL_VERSION: &str = "0.2.5";

// ============================================================================
// PHASE 1: CORE MESSAGE TYPES
// ============================================================================

/// Represents a single message exchanged between user and agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Event type
    pub kind: String, // Always "message"
    /// Identifier created by the message creator
    #[serde(rename = "messageId")]
    pub message_id: String,
    /// Message content
    pub parts: Vec<Part>,
    /// Message sender's role
    pub role: MessageRole,
    /// The context the message is associated with
    #[serde(rename = "contextId", skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
    /// The URIs of extensions that are present or contributed to this Message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<String>>,
    /// Extension metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// List of tasks referenced as context by this message
    #[serde(rename = "referenceTaskIds", skip_serializing_if = "Option::is_none")]
    pub reference_task_ids: Option<Vec<String>>,
    /// Identifier of task the message is related to
    #[serde(rename = "taskId", skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// Message sender's role
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    Agent,
    User,
}

/// Represents a part of a message, which can be text, a file, or structured data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Part {
    #[serde(rename = "text")]
    Text(TextPart),
    #[serde(rename = "file")]
    File(FilePart),
    #[serde(rename = "data")]
    Data(DataPart),
}

/// Represents a text segment within parts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPart {
    /// Text content
    pub text: String,
    /// Optional metadata associated with the part
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Represents a File segment within parts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePart {
    /// File content either as url or bytes
    pub file: FileContent,
    /// Optional metadata associated with the part
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Represents a structured data segment within a message part.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPart {
    /// Structured data content
    pub data: serde_json::Value,
    /// Optional metadata associated with the part
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// File content variants
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FileContent {
    WithBytes(FileWithBytes),
    WithUri(FileWithUri),
}

/// File with base64 encoded bytes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileWithBytes {
    /// base64 encoded content of the file
    pub bytes: String,
    /// Optional name for the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional mimeType for the file
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

/// File with URI reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileWithUri {
    /// URL for the File content
    pub uri: String,
    /// Optional name for the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional mimeType for the file
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

// ============================================================================
// PHASE 2: TASK SYSTEM OVERHAUL
// ============================================================================

/// Represents the possible states of a Task.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TaskState {
    Submitted,
    Working,
    InputRequired,
    Completed,
    Canceled,
    Failed,
    Rejected,
    AuthRequired,
    Unknown,
}

/// TaskState and accompanying message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatus {
    /// Current state of the task
    pub state: TaskState,
    /// Additional status updates for client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    /// ISO 8601 datetime string when the status was recorded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

/// Request methods supported by the A2A protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestMethod {
    /// Send a message to an agent.
    #[serde(rename = "message/send")]
    MessageSend,
    /// Send a streaming message to an agent.
    #[serde(rename = "message/stream")]
    MessageStream,
    /// Get a task.
    #[serde(rename = "tasks/get")]
    TasksGet,
    /// Cancel a task.
    #[serde(rename = "tasks/cancel")]
    TasksCancel,
    /// Set a push notification config for a task.
    #[serde(rename = "tasks/pushNotificationConfig/set")]
    TasksPushNotificationConfigSet,
    /// Get a push notification config for a task.
    #[serde(rename = "tasks/pushNotificationConfig/get")]
    TasksPushNotificationConfigGet,
    /// List push notification configs for a task.
    #[serde(rename = "tasks/pushNotificationConfig/list")]
    TasksPushNotificationConfigList,
    /// Delete a push notification config for a task.
    #[serde(rename = "tasks/pushNotificationConfig/delete")]
    TasksPushNotificationConfigDelete,
    /// Resubscribe to a task.
    #[serde(rename = "tasks/resubscribe")]
    TasksResubscribe,
}

// Implement custom deserialization for RequestMethod
impl<'de> Deserialize<'de> for RequestMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        RequestMethod::from_str(&s)
            .ok_or_else(|| serde::de::Error::custom(format!("unknown request method: {}", s)))
    }
}

impl RequestMethod {
    /// Convert the request method to a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            RequestMethod::MessageSend => "message/send",
            RequestMethod::MessageStream => "message/stream",
            RequestMethod::TasksGet => "tasks/get",
            RequestMethod::TasksCancel => "tasks/cancel",
            RequestMethod::TasksPushNotificationConfigSet => "tasks/pushNotificationConfig/set",
            RequestMethod::TasksPushNotificationConfigGet => "tasks/pushNotificationConfig/get",
            RequestMethod::TasksPushNotificationConfigList => "tasks/pushNotificationConfig/list",
            RequestMethod::TasksPushNotificationConfigDelete => "tasks/pushNotificationConfig/delete",
            RequestMethod::TasksResubscribe => "tasks/resubscribe",
        }
    }

    /// Convert a string to a request method.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "message/send" => Some(RequestMethod::MessageSend),
            "message/stream" => Some(RequestMethod::MessageStream),
            "tasks/get" => Some(RequestMethod::TasksGet),
            "tasks/cancel" => Some(RequestMethod::TasksCancel),
            "tasks/pushNotificationConfig/set" => Some(RequestMethod::TasksPushNotificationConfigSet),
            "tasks/pushNotificationConfig/get" => Some(RequestMethod::TasksPushNotificationConfigGet),
            "tasks/pushNotificationConfig/list" => Some(RequestMethod::TasksPushNotificationConfigList),
            "tasks/pushNotificationConfig/delete" => Some(RequestMethod::TasksPushNotificationConfigDelete),
            "tasks/resubscribe" => Some(RequestMethod::TasksResubscribe),
            // For backward compatibility
            "sendMessage" => Some(RequestMethod::MessageSend),
            "getTask" => Some(RequestMethod::TasksGet),
            "cancelTask" => Some(RequestMethod::TasksCancel),
            _ => None,
        }
    }
}

impl std::fmt::Display for RequestMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for RequestMethod {
    type Err = A2AError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s).ok_or_else(|| A2AError::MethodNotFound(MethodNotFoundError {
            code: -32601,
            message: format!("Method not found: {}", s),
            data: None,
        }))
    }
}

/// JSON-RPC error indicating invalid JSON was received by the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JSONParseError {
    /// A Number that indicates the error type that occurred.
    pub code: i32, // Always -32700
    /// A String providing a short description of the error.
    pub message: String,
    /// A Primitive or Structured value that contains additional information about the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// JSON-RPC error indicating the JSON sent is not a valid Request object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvalidRequestError {
    /// A Number that indicates the error type that occurred.
    pub code: i32, // Always -32600
    /// A String providing a short description of the error.
    pub message: String,
    /// A Primitive or Structured value that contains additional information about the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// JSON-RPC error indicating the method does not exist / is not available.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodNotFoundError {
    /// A Number that indicates the error type that occurred.
    pub code: i32, // Always -32601
    /// A String providing a short description of the error.
    pub message: String,
    /// A Primitive or Structured value that contains additional information about the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// JSON-RPC error indicating invalid method parameter(s).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvalidParamsError {
    /// A Number that indicates the error type that occurred.
    pub code: i32, // Always -32602
    /// A String providing a short description of the error.
    pub message: String,
    /// A Primitive or Structured value that contains additional information about the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// JSON-RPC error indicating an internal JSON-RPC error on the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalError {
    /// A Number that indicates the error type that occurred.
    pub code: i32, // Always -32603
    /// A String providing a short description of the error.
    pub message: String,
    /// A Primitive or Structured value that contains additional information about the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// A2A specific error indicating the requested task ID was not found.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskNotFoundError {
    /// A Number that indicates the error type that occurred.
    pub code: i32, // Always -32001
    /// A String providing a short description of the error.
    pub message: String,
    /// A Primitive or Structured value that contains additional information about the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// A2A specific error indicating the task is in a state where it cannot be canceled.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskNotCancelableError {
    /// A Number that indicates the error type that occurred.
    pub code: i32, // Always -32002
    /// A String providing a short description of the error.
    pub message: String,
    /// A Primitive or Structured value that contains additional information about the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// A2A specific error indicating the agent does not support push notifications.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushNotificationNotSupportedError {
    /// A Number that indicates the error type that occurred.
    pub code: i32, // Always -32003
    /// A String providing a short description of the error.
    pub message: String,
    /// A Primitive or Structured value that contains additional information about the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// A2A specific error indicating the requested operation is not supported by the agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsupportedOperationError {
    /// A Number that indicates the error type that occurred.
    pub code: i32, // Always -32004
    /// A String providing a short description of the error.
    pub message: String,
    /// A Primitive or Structured value that contains additional information about the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// A2A specific error indicating incompatible content types between request and agent capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentTypeNotSupportedError {
    /// A Number that indicates the error type that occurred.
    pub code: i32, // Always -32005
    /// A String providing a short description of the error.
    pub message: String,
    /// A Primitive or Structured value that contains additional information about the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// A2A specific error indicating agent returned invalid response for the current method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvalidAgentResponseError {
    /// A Number that indicates the error type that occurred.
    pub code: i32, // Always -32006
    /// A String providing a short description of the error.
    pub message: String,
    /// A Primitive or Structured value that contains additional information about the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// A2A Error union type.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum A2AError {
    /// JSON parse error.
    JSONParse(JSONParseError),
    /// Invalid request error.
    InvalidRequest(InvalidRequestError),
    /// Method not found error.
    MethodNotFound(MethodNotFoundError),
    /// Invalid parameters error.
    InvalidParams(InvalidParamsError),
    /// Internal error.
    Internal(InternalError),
    /// Task not found error.
    TaskNotFound(TaskNotFoundError),
    /// Task not cancelable error.
    TaskNotCancelable(TaskNotCancelableError),
    /// Push notification not supported error.
    PushNotificationNotSupported(PushNotificationNotSupportedError),
    /// Unsupported operation error.
    UnsupportedOperation(UnsupportedOperationError),
    /// Content type not supported error.
    ContentTypeNotSupported(ContentTypeNotSupportedError),
    /// Invalid agent response error.
    InvalidAgentResponse(InvalidAgentResponseError),
}

impl<'de> Deserialize<'de> for A2AError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let value = serde_json::Value::deserialize(deserializer)?;
        let code = value.get("code")
            .and_then(|c| c.as_i64())
            .ok_or_else(|| D::Error::missing_field("code"))?;

        match code {
            -32700 => Ok(A2AError::JSONParse(
                JSONParseError::deserialize(value).map_err(D::Error::custom)?
            )),
            -32600 => Ok(A2AError::InvalidRequest(
                InvalidRequestError::deserialize(value).map_err(D::Error::custom)?
            )),
            -32601 => Ok(A2AError::MethodNotFound(
                MethodNotFoundError::deserialize(value).map_err(D::Error::custom)?
            )),
            -32602 => Ok(A2AError::InvalidParams(
                InvalidParamsError::deserialize(value).map_err(D::Error::custom)?
            )),
            -32603 => Ok(A2AError::Internal(
                InternalError::deserialize(value).map_err(D::Error::custom)?
            )),
            -32001 => Ok(A2AError::TaskNotFound(
                TaskNotFoundError::deserialize(value).map_err(D::Error::custom)?
            )),
            -32002 => Ok(A2AError::TaskNotCancelable(
                TaskNotCancelableError::deserialize(value).map_err(D::Error::custom)?
            )),
            -32003 => Ok(A2AError::PushNotificationNotSupported(
                PushNotificationNotSupportedError::deserialize(value).map_err(D::Error::custom)?
            )),
            -32004 => Ok(A2AError::UnsupportedOperation(
                UnsupportedOperationError::deserialize(value).map_err(D::Error::custom)?
            )),
            -32005 => Ok(A2AError::ContentTypeNotSupported(
                ContentTypeNotSupportedError::deserialize(value).map_err(D::Error::custom)?
            )),
            -32006 => Ok(A2AError::InvalidAgentResponse(
                InvalidAgentResponseError::deserialize(value).map_err(D::Error::custom)?
            )),
            _ => Err(D::Error::custom(format!("Unknown error code: {}", code))),
        }
    }
}

impl std::fmt::Display for A2AError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            A2AError::JSONParse(e) => write!(f, "JSON parse error: {}", e.message),
            A2AError::InvalidRequest(e) => write!(f, "Invalid request: {}", e.message),
            A2AError::MethodNotFound(e) => write!(f, "Method not found: {}", e.message),
            A2AError::InvalidParams(e) => write!(f, "Invalid parameters: {}", e.message),
            A2AError::Internal(e) => write!(f, "Internal error: {}", e.message),
            A2AError::TaskNotFound(e) => write!(f, "Task not found: {}", e.message),
            A2AError::TaskNotCancelable(e) => write!(f, "Task not cancelable: {}", e.message),
            A2AError::PushNotificationNotSupported(e) => write!(f, "Push notification not supported: {}", e.message),
            A2AError::UnsupportedOperation(e) => write!(f, "Unsupported operation: {}", e.message),
            A2AError::ContentTypeNotSupported(e) => write!(f, "Content type not supported: {}", e.message),
            A2AError::InvalidAgentResponse(e) => write!(f, "Invalid agent response: {}", e.message),
        }
    }
}

impl std::error::Error for A2AError {}

/// Security scheme types supported by the A2A protocol.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SecuritySchemeType {
    /// API key security scheme.
    ApiKey,
    /// HTTP security scheme.
    Http,
    /// OAuth2 security scheme.
    OAuth2,
    /// OpenID Connect security scheme.
    OpenIdConnect,
}

/// Locations where an API key can be provided.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ApiKeyLocation {
    /// API key in a cookie.
    Cookie,
    /// API key in a header.
    Header,
    /// API key in a query parameter.
    Query,
}

/// API Key security scheme.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeySecurityScheme {
    /// The type of the security scheme.
    #[serde(rename = "type")]
    pub type_: String, // Always "apiKey"
    /// The location of the API key.
    #[serde(rename = "in")]
    pub in_: ApiKeyLocation,
    /// The name of the header, query, or cookie parameter.
    pub name: String,
    /// Description of this security scheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ApiKeySecurityScheme {
    /// Create a new API Key security scheme.
    ///
    /// # Arguments
    ///
    /// * `in_` - The location of the API key.
    /// * `name` - The name of the parameter.
    ///
    /// # Returns
    ///
    /// A new `ApiKeySecurityScheme`.
    pub fn new(in_: ApiKeyLocation, name: String) -> Self {
        Self {
            type_: "apiKey".to_string(),
            in_,
            name,
            description: None,
        }
    }

    /// Validate the API Key security scheme.
    ///
    /// # Returns
    ///
    /// `Ok(())` if valid, `Err(String)` with error message if invalid.
    pub fn validate(&self) -> Result<(), String> {
        if self.type_ != "apiKey" {
            return Err("API Key security scheme type must be 'apiKey'".to_string());
        }

        if self.name.is_empty() {
            return Err("API Key parameter name cannot be empty".to_string());
        }

        // Validate parameter name based on location
        match self.in_ {
            ApiKeyLocation::Header => {
                if self.name.contains(' ') {
                    return Err("Header names cannot contain spaces".to_string());
                }
                if self.name.to_lowercase() == "authorization" {
                    return Err("Use HTTP security scheme for Authorization header".to_string());
                }
            }
            ApiKeyLocation::Query => {
                if self.name.contains(' ') || self.name.contains('&') || self.name.contains('=') {
                    return Err("Query parameter names cannot contain spaces, &, or =".to_string());
                }
            }
            ApiKeyLocation::Cookie => {
                if self.name.contains(' ') || self.name.contains(';') || self.name.contains('=') {
                    return Err("Cookie names cannot contain spaces, ;, or =".to_string());
                }
            }
        }

        // Validate description length if present
        if let Some(desc) = &self.description {
            if desc.len() > 500 {
                return Err("Security scheme description is too long (max 500 characters)".to_string());
            }
        }

        Ok(())
    }
}

/// HTTP security scheme.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpSecurityScheme {
    /// The type of the security scheme.
    #[serde(rename = "type")]
    pub type_: String, // Always "http"
    /// The name of the HTTP Authorization scheme.
    pub scheme: String,
    /// A hint to the client to identify how the bearer token is formatted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearer_format: Option<String>,
    /// Description of this security scheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl HttpSecurityScheme {
    /// Create a new HTTP security scheme.
    ///
    /// # Arguments
    ///
    /// * `scheme` - The HTTP Authorization scheme name.
    ///
    /// # Returns
    ///
    /// A new `HttpSecurityScheme`.
    pub fn new(scheme: String) -> Self {
        Self {
            type_: "http".to_string(),
            scheme,
            bearer_format: None,
            description: None,
        }
    }

    /// Create a new Bearer HTTP security scheme.
    ///
    /// # Arguments
    ///
    /// * `bearer_format` - Optional bearer token format hint.
    ///
    /// # Returns
    ///
    /// A new `HttpSecurityScheme` configured for Bearer tokens.
    pub fn bearer(bearer_format: Option<String>) -> Self {
        Self {
            type_: "http".to_string(),
            scheme: "bearer".to_string(),
            bearer_format,
            description: None,
        }
    }

    /// Validate the HTTP security scheme.
    ///
    /// # Returns
    ///
    /// `Ok(())` if valid, `Err(String)` with error message if invalid.
    pub fn validate(&self) -> Result<(), String> {
        if self.type_ != "http" {
            return Err("HTTP security scheme type must be 'http'".to_string());
        }

        if self.scheme.is_empty() {
            return Err("HTTP scheme name cannot be empty".to_string());
        }

        // Validate common HTTP authentication schemes
        let valid_schemes = ["basic", "bearer", "digest", "negotiate", "ntlm"];
        let scheme_lower = self.scheme.to_lowercase();

        if !valid_schemes.contains(&scheme_lower.as_str()) && !scheme_lower.starts_with("x-") {
            return Err(format!("Unknown HTTP authentication scheme: {}", self.scheme));
        }

        // Validate bearer format if present
        if let Some(ref format) = self.bearer_format {
            if self.scheme.to_lowercase() != "bearer" {
                return Err("Bearer format can only be specified for bearer scheme".to_string());
            }
            if format.is_empty() {
                return Err("Bearer format cannot be empty if specified".to_string());
            }
        }

        // Validate description length if present
        if let Some(desc) = &self.description {
            if desc.len() > 500 {
                return Err("Security scheme description is too long (max 500 characters)".to_string());
            }
        }

        Ok(())
    }
}

/// OAuth2 security scheme.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2SecurityScheme {
    /// The type of the security scheme.
    #[serde(rename = "type")]
    pub type_: String, // Always "oauth2"
    /// The available flows for the OAuth2 security scheme.
    pub flows: OAuth2Flows,
    /// Description of this security scheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl OAuth2SecurityScheme {
    /// Create a new OAuth2 security scheme.
    ///
    /// # Arguments
    ///
    /// * `flows` - The OAuth2 flows configuration.
    ///
    /// # Returns
    ///
    /// A new `OAuth2SecurityScheme`.
    pub fn new(flows: OAuth2Flows) -> Self {
        Self {
            type_: "oauth2".to_string(),
            flows,
            description: None,
        }
    }

    /// Validate the OAuth2 security scheme.
    ///
    /// # Returns
    ///
    /// `Ok(())` if valid, `Err(String)` with error message if invalid.
    pub fn validate(&self) -> Result<(), String> {
        if self.type_ != "oauth2" {
            return Err("OAuth2 security scheme type must be 'oauth2'".to_string());
        }

        // Validate that at least one flow is defined
        if self.flows.implicit.is_none() 
            && self.flows.password.is_none() 
            && self.flows.client_credentials.is_none() 
            && self.flows.authorization_code.is_none() {
            return Err("OAuth2 security scheme must define at least one flow".to_string());
        }

        // Validate each defined flow
        if let Some(ref flow) = self.flows.implicit {
            flow.validate().map_err(|e| format!("Invalid implicit flow: {}", e))?;
        }

        if let Some(ref flow) = self.flows.password {
            flow.validate().map_err(|e| format!("Invalid password flow: {}", e))?;
        }

        if let Some(ref flow) = self.flows.client_credentials {
            flow.validate().map_err(|e| format!("Invalid client credentials flow: {}", e))?;
        }

        if let Some(ref flow) = self.flows.authorization_code {
            flow.validate().map_err(|e| format!("Invalid authorization code flow: {}", e))?;
        }

        // Validate description length if present
        if let Some(desc) = &self.description {
            if desc.len() > 500 {
                return Err("Security scheme description is too long (max 500 characters)".to_string());
            }
        }

        Ok(())
    }

    /// Check if the OAuth2 scheme supports client-only flows.
    ///
    /// # Returns
    ///
    /// `true` if client-only flows are supported.
    pub fn supports_client_only_flows(&self) -> bool {
        self.flows.client_credentials.is_some()
    }

    /// Check if the OAuth2 scheme requires user interaction.
    ///
    /// # Returns
    ///
    /// `true` if user interaction is required.
    pub fn requires_user_interaction(&self) -> bool {
        self.flows.implicit.is_some() || self.flows.authorization_code.is_some()
    }
}

/// OAuth2 flows.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2Flows {
    /// The implicit flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit: Option<ImplicitOAuthFlow>,
    /// The password flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<PasswordOAuthFlow>,
    /// The client credentials flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials: Option<ClientCredentialsOAuthFlow>,
    /// The authorization code flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<AuthorizationCodeOAuthFlow>,
}

/// Authorization Code OAuth flow.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizationCodeOAuthFlow {
    /// The authorization URL to be used for this flow.
    pub authorization_url: String,
    /// The token URL to be used for this flow.
    pub token_url: String,
    /// The URL to be used for obtaining refresh tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    /// The available scopes for the OAuth2 security scheme.
    pub scopes: std::collections::HashMap<String, String>,
}

impl AuthorizationCodeOAuthFlow {
    /// Create a new Authorization Code OAuth flow.
    ///
    /// # Arguments
    ///
    /// * `authorization_url` - The authorization URL.
    /// * `token_url` - The token URL.
    /// * `scopes` - The available scopes.
    ///
    /// # Returns
    ///
    /// A new `AuthorizationCodeOAuthFlow`.
    pub fn new(
        authorization_url: String,
        token_url: String,
        scopes: std::collections::HashMap<String, String>,
    ) -> Self {
        Self {
            authorization_url,
            token_url,
            refresh_url: None,
            scopes,
        }
    }

    /// Validate the OAuth flow configuration.
    ///
    /// # Returns
    ///
    /// `Ok(())` if valid, `Err(String)` with error message if invalid.
    pub fn validate(&self) -> Result<(), String> {
        crate::validation::validate_url(&self.authorization_url)
            .map_err(|e| format!("Invalid authorization URL: {}", e))?;

        crate::validation::validate_url(&self.token_url)
            .map_err(|e| format!("Invalid token URL: {}", e))?;

        if let Some(ref refresh_url) = self.refresh_url {
            crate::validation::validate_url(refresh_url)
                .map_err(|e| format!("Invalid refresh URL: {}", e))?;
        }

        if self.scopes.is_empty() {
            return Err("OAuth2 flow must define at least one scope".to_string());
        }

        // Validate scope names and descriptions
        for (scope_name, scope_desc) in &self.scopes {
            if scope_name.is_empty() {
                return Err("OAuth2 scope name cannot be empty".to_string());
            }
            if scope_desc.is_empty() {
                return Err("OAuth2 scope description cannot be empty".to_string());
            }
            if scope_name.contains(' ') {
                return Err("OAuth2 scope names cannot contain spaces".to_string());
            }
        }

        Ok(())
    }
}

/// Client Credentials OAuth flow.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientCredentialsOAuthFlow {
    /// The token URL to be used for this flow.
    pub token_url: String,
    /// The URL to be used for obtaining refresh tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    /// The available scopes for the OAuth2 security scheme.
    pub scopes: std::collections::HashMap<String, String>,
}

impl ClientCredentialsOAuthFlow {
    /// Create a new Client Credentials OAuth flow.
    ///
    /// # Arguments
    ///
    /// * `token_url` - The token URL.
    /// * `scopes` - The available scopes.
    ///
    /// # Returns
    ///
    /// A new `ClientCredentialsOAuthFlow`.
    pub fn new(
        token_url: String,
        scopes: std::collections::HashMap<String, String>,
    ) -> Self {
        Self {
            token_url,
            refresh_url: None,
            scopes,
        }
    }

    /// Validate the OAuth flow configuration.
    ///
    /// # Returns
    ///
    /// `Ok(())` if valid, `Err(String)` with error message if invalid.
    pub fn validate(&self) -> Result<(), String> {
        crate::validation::validate_url(&self.token_url)
            .map_err(|e| format!("Invalid token URL: {}", e))?;

        if let Some(ref refresh_url) = self.refresh_url {
            crate::validation::validate_url(refresh_url)
                .map_err(|e| format!("Invalid refresh URL: {}", e))?;
        }

        if self.scopes.is_empty() {
            return Err("OAuth2 flow must define at least one scope".to_string());
        }

        // Validate scope names and descriptions
        for (scope_name, scope_desc) in &self.scopes {
            if scope_name.is_empty() {
                return Err("OAuth2 scope name cannot be empty".to_string());
            }
            if scope_desc.is_empty() {
                return Err("OAuth2 scope description cannot be empty".to_string());
            }
            if scope_name.contains(' ') {
                return Err("OAuth2 scope names cannot contain spaces".to_string());
            }
        }

        Ok(())
    }
}

/// Implicit OAuth flow.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplicitOAuthFlow {
    /// The authorization URL to be used for this flow.
    pub authorization_url: String,
    /// The URL to be used for obtaining refresh tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    /// The available scopes for the OAuth2 security scheme.
    pub scopes: std::collections::HashMap<String, String>,
}

impl ImplicitOAuthFlow {
    /// Create a new Implicit OAuth flow.
    ///
    /// # Arguments
    ///
    /// * `authorization_url` - The authorization URL.
    /// * `scopes` - The available scopes.
    ///
    /// # Returns
    ///
    /// A new `ImplicitOAuthFlow`.
    pub fn new(
        authorization_url: String,
        scopes: std::collections::HashMap<String, String>,
    ) -> Self {
        Self {
            authorization_url,
            refresh_url: None,
            scopes,
        }
    }

    /// Validate the OAuth flow configuration.
    ///
    /// # Returns
    ///
    /// `Ok(())` if valid, `Err(String)` with error message if invalid.
    pub fn validate(&self) -> Result<(), String> {
        crate::validation::validate_url(&self.authorization_url)
            .map_err(|e| format!("Invalid authorization URL: {}", e))?;

        if let Some(ref refresh_url) = self.refresh_url {
            crate::validation::validate_url(refresh_url)
                .map_err(|e| format!("Invalid refresh URL: {}", e))?;
        }

        if self.scopes.is_empty() {
            return Err("OAuth2 flow must define at least one scope".to_string());
        }

        // Validate scope names and descriptions
        for (scope_name, scope_desc) in &self.scopes {
            if scope_name.is_empty() {
                return Err("OAuth2 scope name cannot be empty".to_string());
            }
            if scope_desc.is_empty() {
                return Err("OAuth2 scope description cannot be empty".to_string());
            }
            if scope_name.contains(' ') {
                return Err("OAuth2 scope names cannot contain spaces".to_string());
            }
        }

        Ok(())
    }
}

/// Password OAuth flow.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordOAuthFlow {
    /// The token URL to be used for this flow.
    pub token_url: String,
    /// The URL to be used for obtaining refresh tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    /// The available scopes for the OAuth2 security scheme.
    pub scopes: std::collections::HashMap<String, String>,
}

impl PasswordOAuthFlow {
    /// Create a new Password OAuth flow.
    ///
    /// # Arguments
    ///
    /// * `token_url` - The token URL.
    /// * `scopes` - The available scopes.
    ///
    /// # Returns
    ///
    /// A new `PasswordOAuthFlow`.
    pub fn new(
        token_url: String,
        scopes: std::collections::HashMap<String, String>,
    ) -> Self {
        Self {
            token_url,
            refresh_url: None,
            scopes,
        }
    }

    /// Validate the OAuth flow configuration.
    ///
    /// # Returns
    ///
    /// `Ok(())` if valid, `Err(String)` with error message if invalid.
    pub fn validate(&self) -> Result<(), String> {
        crate::validation::validate_url(&self.token_url)
            .map_err(|e| format!("Invalid token URL: {}", e))?;

        if let Some(ref refresh_url) = self.refresh_url {
            crate::validation::validate_url(refresh_url)
                .map_err(|e| format!("Invalid refresh URL: {}", e))?;
        }

        if self.scopes.is_empty() {
            return Err("OAuth2 flow must define at least one scope".to_string());
        }

        // Validate scope names and descriptions
        for (scope_name, scope_desc) in &self.scopes {
            if scope_name.is_empty() {
                return Err("OAuth2 scope name cannot be empty".to_string());
            }
            if scope_desc.is_empty() {
                return Err("OAuth2 scope description cannot be empty".to_string());
            }
            if scope_name.contains(' ') {
                return Err("OAuth2 scope names cannot contain spaces".to_string());
            }
        }

        Ok(())
    }
}

/// OpenID Connect security scheme.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenIdConnectSecurityScheme {
    /// The type of the security scheme.
    #[serde(rename = "type")]
    pub type_: String, // Always "openIdConnect"
    /// OpenId Connect URL to discover OAuth2 configuration values.
    pub open_id_connect_url: String,
    /// Description of this security scheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl OpenIdConnectSecurityScheme {
    /// Create a new OpenID Connect security scheme.
    ///
    /// # Arguments
    ///
    /// * `open_id_connect_url` - The OpenID Connect discovery URL.
    ///
    /// # Returns
    ///
    /// A new `OpenIdConnectSecurityScheme`.
    pub fn new(open_id_connect_url: String) -> Self {
        Self {
            type_: "openIdConnect".to_string(),
            open_id_connect_url,
            description: None,
        }
    }

    /// Validate the OpenID Connect security scheme.
    ///
    /// # Returns
    ///
    /// `Ok(())` if valid, `Err(String)` with error message if invalid.
    pub fn validate(&self) -> Result<(), String> {
        if self.type_ != "openIdConnect" {
            return Err("OpenID Connect security scheme type must be 'openIdConnect'".to_string());
        }

        // Validate the OpenID Connect URL
        crate::validation::validate_url(&self.open_id_connect_url)
            .map_err(|e| format!("Invalid OpenID Connect URL: {}", e))?;

        // Validate that it's HTTPS (required for OpenID Connect)
        if !self.open_id_connect_url.starts_with("https://") {
            return Err("OpenID Connect URL must use HTTPS".to_string());
        }

        // Validate common OpenID Connect discovery endpoint patterns
        if !self.open_id_connect_url.contains("/.well-known/openid_configuration") 
            && !self.open_id_connect_url.contains("/.well-known/openid-configuration") {
            return Err("OpenID Connect URL should point to a well-known configuration endpoint".to_string());
        }

        // Validate description length if present
        if let Some(desc) = &self.description {
            if desc.len() > 500 {
                return Err("Security scheme description is too long (max 500 characters)".to_string());
            }
        }

        Ok(())
    }

    /// Get the base URL for the OpenID Connect provider.
    ///
    /// # Returns
    ///
    /// The base URL of the OpenID Connect provider.
    pub fn get_provider_base_url(&self) -> String {
        if let Some(pos) = self.open_id_connect_url.find("/.well-known/") {
            self.open_id_connect_url[..pos].to_string()
        } else {
            self.open_id_connect_url.clone()
        }
    }
}

/// Security scheme.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SecurityScheme {
    /// API key security scheme.
    ApiKey(ApiKeySecurityScheme),
    /// HTTP security scheme.
    Http(HttpSecurityScheme),
    /// OAuth2 security scheme.
    OAuth2(OAuth2SecurityScheme),
    /// OpenID Connect security scheme.
    OpenIdConnect(OpenIdConnectSecurityScheme),
}

impl SecurityScheme {
    /// Validate the security scheme configuration.
    ///
    /// # Returns
    ///
    /// `Ok(())` if valid, `Err(String)` with error message if invalid.
    pub fn validate(&self) -> Result<(), String> {
        match self {
            SecurityScheme::ApiKey(scheme) => scheme.validate(),
            SecurityScheme::Http(scheme) => scheme.validate(),
            SecurityScheme::OAuth2(scheme) => scheme.validate(),
            SecurityScheme::OpenIdConnect(scheme) => scheme.validate(),
        }
    }

    /// Get the security scheme type as a string.
    ///
    /// # Returns
    ///
    /// The security scheme type.
    pub fn scheme_type(&self) -> &str {
        match self {
            SecurityScheme::ApiKey(_) => "apiKey",
            SecurityScheme::Http(_) => "http",
            SecurityScheme::OAuth2(_) => "oauth2",
            SecurityScheme::OpenIdConnect(_) => "openIdConnect",
        }
    }

    /// Check if the security scheme requires user interaction.
    ///
    /// # Returns
    ///
    /// `true` if user interaction is required.
    pub fn requires_user_interaction(&self) -> bool {
        match self {
            SecurityScheme::ApiKey(_) => false,
            SecurityScheme::Http(_) => false,
            SecurityScheme::OAuth2(scheme) => {
                // OAuth2 flows that require user interaction
                scheme.flows.implicit.is_some() || scheme.flows.authorization_code.is_some()
            }
            SecurityScheme::OpenIdConnect(_) => true,
        }
    }
}

/// Agent extension.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentExtension {
    /// The URI of the extension.
    pub uri: String,
    /// Whether the client must follow specific requirements of the extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// A description of how this agent uses this extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional configuration for the extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

impl AgentExtension {
    /// Create a new agent extension with the specified URI.
    ///
    /// # Arguments
    ///
    /// * `uri` - The URI of the extension.
    ///
    /// # Returns
    ///
    /// A new `AgentExtension` with the specified URI.
    pub fn new(uri: String) -> Self {
        Self {
            uri,
            required: None,
            description: None,
            params: None,
        }
    }

    /// Create a new required agent extension with description and parameters.
    ///
    /// # Arguments
    ///
    /// * `uri` - The URI of the extension.
    /// * `description` - A description of how this agent uses this extension.
    /// * `required` - Whether the client must follow specific requirements.
    /// * `params` - Optional configuration for the extension.
    ///
    /// # Returns
    ///
    /// A new `AgentExtension` with the specified parameters.
    pub fn with_config(
        uri: String,
        description: Option<String>,
        required: Option<bool>,
        params: Option<serde_json::Value>,
    ) -> Self {
        Self {
            uri,
            required,
            description,
            params,
        }
    }

    /// Validate the extension URI format.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the URI is valid, `Err(String)` with error message if invalid.
    pub fn validate_uri(&self) -> Result<(), String> {
        if self.uri.is_empty() {
            return Err("Extension URI cannot be empty".to_string());
        }

        // Basic URI validation - should start with http:// or https://
        if !self.uri.starts_with("http://") && !self.uri.starts_with("https://") {
            return Err("Extension URI must be a valid HTTP or HTTPS URL".to_string());
        }

        // Check for common URI patterns
        if self.uri.len() < 10 {
            return Err("Extension URI appears to be too short".to_string());
        }

        Ok(())
    }

    /// Validate extension parameters against known extension types.
    ///
    /// # Returns
    ///
    /// `Ok(())` if parameters are valid, `Err(String)` with error message if invalid.
    pub fn validate_params(&self) -> Result<(), String> {
        if let Some(params) = &self.params {
            // Validate that params is an object
            if !params.is_object() {
                return Err("Extension params must be a JSON object".to_string());
            }

            // Validate specific extension types based on URI patterns
            if self.uri.contains("oauth") || self.uri.contains("auth") {
                self.validate_auth_extension_params(params)?;
            } else if self.uri.contains("webhook") || self.uri.contains("notification") {
                self.validate_webhook_extension_params(params)?;
            }
        }

        Ok(())
    }

    /// Validate authentication extension parameters.
    fn validate_auth_extension_params(&self, params: &serde_json::Value) -> Result<(), String> {
        let obj = params.as_object().unwrap();

        // Common auth extension parameters
        if let Some(client_id) = obj.get("clientId") {
            if !client_id.is_string() || client_id.as_str().unwrap().is_empty() {
                return Err("Auth extension clientId must be a non-empty string".to_string());
            }
        }

        if let Some(scopes) = obj.get("scopes") {
            if !scopes.is_array() {
                return Err("Auth extension scopes must be an array".to_string());
            }
        }

        if let Some(redirect_uri) = obj.get("redirectUri") {
            if !redirect_uri.is_string() {
                return Err("Auth extension redirectUri must be a string".to_string());
            }
            let uri_str = redirect_uri.as_str().unwrap();
            if !uri_str.starts_with("http://") && !uri_str.starts_with("https://") {
                return Err("Auth extension redirectUri must be a valid URL".to_string());
            }
        }

        Ok(())
    }

    /// Validate webhook extension parameters.
    fn validate_webhook_extension_params(&self, params: &serde_json::Value) -> Result<(), String> {
        let obj = params.as_object().unwrap();

        if let Some(url) = obj.get("url") {
            if !url.is_string() || url.as_str().unwrap().is_empty() {
                return Err("Webhook extension url must be a non-empty string".to_string());
            }
            let url_str = url.as_str().unwrap();
            if !url_str.starts_with("http://") && !url_str.starts_with("https://") {
                return Err("Webhook extension url must be a valid HTTP or HTTPS URL".to_string());
            }
        }

        if let Some(secret) = obj.get("secret") {
            if !secret.is_string() {
                return Err("Webhook extension secret must be a string".to_string());
            }
        }

        if let Some(events) = obj.get("events") {
            if !events.is_array() {
                return Err("Webhook extension events must be an array".to_string());
            }
        }

        Ok(())
    }

    /// Perform comprehensive validation of the extension.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the extension is valid, `Err(String)` with error message if invalid.
    pub fn validate(&self) -> Result<(), String> {
        self.validate_uri()?;
        self.validate_params()?;

        // Validate description length if present
        if let Some(desc) = &self.description {
            if desc.len() > 1000 {
                return Err("Extension description is too long (max 1000 characters)".to_string());
            }
        }

        Ok(())
    }
}

// ============================================================================
// FIELD VALIDATION MODULE
// ============================================================================

/// Validation utilities for A2A protocol fields.
pub mod validation {
    use std::collections::HashSet;

    /// Validate URL format.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL string to validate.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the URL is valid, `Err(String)` with error message if invalid.
    pub fn validate_url(url: &str) -> Result<(), String> {
        if url.is_empty() {
            return Err("URL cannot be empty".to_string());
        }

        // Basic URL validation
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err("URL must start with http:// or https://".to_string());
        }

        // Check minimum length
        if url.len() < 10 {
            return Err("URL appears to be too short".to_string());
        }

        // Check for valid domain structure
        let without_protocol = if url.starts_with("https://") {
            &url[8..]
        } else {
            &url[7..]
        };

        if without_protocol.is_empty() {
            return Err("URL must contain a domain".to_string());
        }

        // Check for invalid characters
        if url.contains(' ') {
            return Err("URL cannot contain spaces".to_string());
        }

        Ok(())
    }

    /// Validate media type format.
    ///
    /// # Arguments
    ///
    /// * `media_type` - The media type string to validate.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the media type is valid, `Err(String)` with error message if invalid.
    pub fn validate_media_type(media_type: &str) -> Result<(), String> {
        if media_type.is_empty() {
            return Err("Media type cannot be empty".to_string());
        }

        // Basic media type validation (type/subtype)
        let parts: Vec<&str> = media_type.split('/').collect();
        if parts.len() != 2 {
            return Err("Media type must be in format 'type/subtype'".to_string());
        }

        let (main_type, sub_type) = (parts[0], parts[1]);

        if main_type.is_empty() || sub_type.is_empty() {
            return Err("Media type parts cannot be empty".to_string());
        }

        // Validate common media types
        let valid_main_types: HashSet<&str> = [
            "text", "image", "audio", "video", "application", "multipart", "message"
        ].iter().cloned().collect();

        if !valid_main_types.contains(main_type) && !main_type.starts_with("x-") {
            return Err(format!("Unknown media type: {}", main_type));
        }

        Ok(())
    }

    /// Validate task state transitions.
    ///
    /// # Arguments
    ///
    /// * `from_state` - The current task state.
    /// * `to_state` - The target task state.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the transition is valid, `Err(String)` with error message if invalid.
    pub fn validate_task_state_transition(from_state: &crate::TaskState, to_state: &crate::TaskState) -> Result<(), String> {
        use crate::TaskState::*;

        let valid_transitions = match from_state {
            Submitted => vec![Working, Rejected, Canceled, AuthRequired],
            Working => vec![Completed, Failed, Canceled, InputRequired],
            InputRequired => vec![Working, Canceled, Failed],
            AuthRequired => vec![Working, Rejected, Canceled],
            Completed => vec![], // Terminal state
            Failed => vec![], // Terminal state
            Canceled => vec![], // Terminal state
            Rejected => vec![], // Terminal state
            Unknown => vec![Submitted, Working, Completed, Failed, Canceled, Rejected, AuthRequired, InputRequired],
        };

        if !valid_transitions.contains(to_state) {
            return Err(format!("Invalid task state transition from {:?} to {:?}", from_state, to_state));
        }

        Ok(())
    }

    /// Validate message ID format.
    ///
    /// # Arguments
    ///
    /// * `message_id` - The message ID to validate.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the message ID is valid, `Err(String)` with error message if invalid.
    pub fn validate_message_id(message_id: &str) -> Result<(), String> {
        if message_id.is_empty() {
            return Err("Message ID cannot be empty".to_string());
        }

        if message_id.len() > 255 {
            return Err("Message ID is too long (max 255 characters)".to_string());
        }

        // Check for valid characters (alphanumeric, hyphens, underscores)
        if !message_id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err("Message ID can only contain alphanumeric characters, hyphens, and underscores".to_string());
        }

        Ok(())
    }

    /// Validate task ID format.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The task ID to validate.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the task ID is valid, `Err(String)` with error message if invalid.
    pub fn validate_task_id(task_id: &str) -> Result<(), String> {
        if task_id.is_empty() {
            return Err("Task ID cannot be empty".to_string());
        }

        if task_id.len() > 255 {
            return Err("Task ID is too long (max 255 characters)".to_string());
        }

        // Check for valid characters (alphanumeric, hyphens, underscores)
        if !task_id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err("Task ID can only contain alphanumeric characters, hyphens, and underscores".to_string());
        }

        Ok(())
    }

    /// Validate agent name format.
    ///
    /// # Arguments
    ///
    /// * `name` - The agent name to validate.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the name is valid, `Err(String)` with error message if invalid.
    pub fn validate_agent_name(name: &str) -> Result<(), String> {
        if name.is_empty() {
            return Err("Agent name cannot be empty".to_string());
        }

        if name.len() > 100 {
            return Err("Agent name is too long (max 100 characters)".to_string());
        }

        // Check for reasonable characters
        if name.trim() != name {
            return Err("Agent name cannot start or end with whitespace".to_string());
        }

        Ok(())
    }

    /// Validate version string format.
    ///
    /// # Arguments
    ///
    /// * `version` - The version string to validate.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the version is valid, `Err(String)` with error message if invalid.
    pub fn validate_version(version: &str) -> Result<(), String> {
        if version.is_empty() {
            return Err("Version cannot be empty".to_string());
        }

        if version.len() > 50 {
            return Err("Version is too long (max 50 characters)".to_string());
        }

        // Basic semantic version validation (flexible)
        let parts: Vec<&str> = version.split('.').collect();
        if parts.is_empty() || parts.len() > 4 {
            return Err("Version should have 1-4 dot-separated parts".to_string());
        }

        for part in parts {
            if part.is_empty() {
                return Err("Version parts cannot be empty".to_string());
            }
        }

        Ok(())
    }

    /// Validate skill ID format.
    ///
    /// # Arguments
    ///
    /// * `skill_id` - The skill ID to validate.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the skill ID is valid, `Err(String)` with error message if invalid.
    pub fn validate_skill_id(skill_id: &str) -> Result<(), String> {
        if skill_id.is_empty() {
            return Err("Skill ID cannot be empty".to_string());
        }

        if skill_id.len() > 100 {
            return Err("Skill ID is too long (max 100 characters)".to_string());
        }

        // Check for valid characters (alphanumeric, hyphens, underscores, dots)
        if !skill_id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.') {
            return Err("Skill ID can only contain alphanumeric characters, hyphens, underscores, and dots".to_string());
        }

        Ok(())
    }
}

/// Agent capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCapabilities {
    /// Extensions supported by this agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<AgentExtension>>,
    /// True if the agent can notify updates to client.
    #[serde(rename = "pushNotifications", skip_serializing_if = "Option::is_none")]
    pub push_notifications: Option<bool>,
    /// True if the agent exposes status change history for tasks.
    #[serde(rename = "stateTransitionHistory", skip_serializing_if = "Option::is_none")]
    pub state_transition_history: Option<bool>,
    /// True if the agent supports SSE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming: Option<bool>,
}

/// Agent interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentInterface {
    /// The URL for this interface.
    pub url: String,
    /// The transport supported by this URL.
    pub transport: String,
}

/// Agent provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProvider {
    /// Agent provider's organization name.
    pub organization: String,
    /// Agent provider's URL.
    pub url: String,
}

/// Agent skill.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSkill {
    /// The name of the skill.
    pub name: String,
    /// A description of the skill.
    pub description: String,
    /// Input modes supported by this skill.
    #[serde(rename = "inputModes", skip_serializing_if = "Option::is_none")]
    pub input_modes: Option<Vec<String>>,
    /// Output modes supported by this skill.
    #[serde(rename = "outputModes", skip_serializing_if = "Option::is_none")]
    pub output_modes: Option<Vec<String>>,
    /// Example scenarios that the skill can perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<String>>,
}

/// Agent card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCard {
    /// Human readable name of the agent.
    pub name: String,
    /// A human-readable description of the agent.
    pub description: String,
    /// The version of the agent.
    pub version: String,
    /// The version of the A2A protocol this agent supports.
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    /// A URL to the address the agent is hosted at.
    pub url: String,
    /// The transport of the preferred endpoint. If empty, defaults to JSONRPC.
    #[serde(rename = "preferredTransport", skip_serializing_if = "Option::is_none")]
    pub preferred_transport: Option<String>,
    /// Optional capabilities supported by the agent.
    pub capabilities: AgentCapabilities,
    /// The set of interaction modes that the agent supports across all skills.
    #[serde(rename = "defaultInputModes")]
    pub default_input_modes: Vec<String>,
    /// Supported media types for output.
    #[serde(rename = "defaultOutputModes")]
    pub default_output_modes: Vec<String>,
    /// Skills are a unit of capability that an agent can perform.
    pub skills: Vec<AgentSkill>,
    /// The service provider of the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<AgentProvider>,
    /// A URL to documentation for the agent.
    #[serde(rename = "documentationUrl", skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
    /// A URL to an icon for the agent.
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// True if the agent supports providing an extended agent card when the user is authenticated.
    #[serde(rename = "supportsAuthenticatedExtendedCard", skip_serializing_if = "Option::is_none")]
    pub supports_authenticated_extended_card: Option<bool>,
    /// Announcement of additional supported transports.
    #[serde(rename = "additionalInterfaces", skip_serializing_if = "Option::is_none")]
    pub additional_interfaces: Option<Vec<AgentInterface>>,
    /// Security requirements for contacting the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<HashMap<String, Vec<String>>>>,
    /// Security scheme details used for authenticating with this agent.
    #[serde(rename = "securitySchemes", skip_serializing_if = "Option::is_none")]
    pub security_schemes: Option<HashMap<String, SecurityScheme>>,
}

impl AgentCard {
    /// Create a new agent card with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the agent.
    /// * `description` - A description of the agent.
    /// * `version` - The version of the agent.
    /// * `url` - The URL where the agent is hosted.
    /// * `capabilities` - The capabilities of the agent.
    /// * `default_input_modes` - The default input modes supported by the agent.
    /// * `default_output_modes` - The default output modes supported by the agent.
    /// * `skills` - The skills provided by the agent.
    ///
    /// # Returns
    ///
    /// A new `AgentCard` with the specified parameters.
    pub fn new(
        name: String,
        description: String,
        version: String,
        url: String,
        capabilities: AgentCapabilities,
        default_input_modes: Vec<String>,
        default_output_modes: Vec<String>,
        skills: Vec<AgentSkill>,
    ) -> Self {
        Self {
            name,
            description,
            version,
            protocol_version: PROTOCOL_VERSION.to_string(),
            url,
            preferred_transport: None,
            capabilities,
            default_input_modes,
            default_output_modes,
            skills,
            provider: None,
            documentation_url: None,
            icon_url: None,
            supports_authenticated_extended_card: None,
            additional_interfaces: None,
            security: None,
            security_schemes: None,
        }
    }
}

/// Task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// The ID of the task
    pub id: String,
    /// Event type
    pub kind: String, // Always "task"
    /// The status of the task
    pub status: TaskStatus,
    /// Server-generated id for contextual alignment across interactions
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// Collection of artifacts created by the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<Artifact>>,
    /// Message history for the task
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<Message>>,
    /// Extension metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// The result of the task, if completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// The error that occurred, if the task failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<A2AErrorResponse>,
    /// The time the task was created
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The time the task was last updated
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// The history of status changes for this task
    #[serde(rename = "statusHistory", skip_serializing_if = "Option::is_none")]
    pub status_history: Option<Vec<TaskStatus>>,
}

// ============================================================================
// PHASE 3: STREAMING AND EVENT TYPES
// ============================================================================

/// Represents an artifact generated for a task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    /// Unique identifier for the artifact
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    /// Artifact parts
    pub parts: Vec<Part>,
    /// Optional description for the artifact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The URIs of extensions that are present or contributed to this Artifact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<String>>,
    /// Extension metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// Optional name for the artifact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Sent by server during sendStream or subscribe requests for artifact updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskArtifactUpdateEvent {
    /// Event type
    pub kind: String, // Always "artifact-update"
    /// Task id
    #[serde(rename = "taskId")]
    pub task_id: String,
    /// The context the task is associated with
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// Generated artifact
    pub artifact: Artifact,
    /// Indicates if this artifact appends to a previous one
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append: Option<bool>,
    /// Indicates if this is the last chunk of the artifact
    #[serde(rename = "lastChunk", skip_serializing_if = "Option::is_none")]
    pub last_chunk: Option<bool>,
    /// Extension metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl TaskArtifactUpdateEvent {
    /// Create a new task artifact update event.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The task ID.
    /// * `context_id` - The context ID.
    /// * `artifact` - The generated artifact.
    ///
    /// # Returns
    ///
    /// A new `TaskArtifactUpdateEvent`.
    pub fn new(task_id: String, context_id: String, artifact: Artifact) -> Self {
        Self {
            kind: "artifact-update".to_string(),
            task_id,
            context_id,
            artifact,
            append: None,
            last_chunk: None,
            metadata: None,
        }
    }

    /// Validate the artifact update event.
    ///
    /// # Returns
    ///
    /// `Ok(())` if valid, `Err(String)` with error message if invalid.
    pub fn validate(&self) -> Result<(), String> {
        if self.kind != "artifact-update" {
            return Err("TaskArtifactUpdateEvent kind must be 'artifact-update'".to_string());
        }

        crate::validation::validate_task_id(&self.task_id)?;

        if self.context_id.is_empty() {
            return Err("Context ID cannot be empty".to_string());
        }

        // Validate artifact parts
        if self.artifact.parts.is_empty() {
            return Err("Artifact must contain at least one part".to_string());
        }

        // Validate streaming consistency
        if let Some(append) = self.append {
            if let Some(last_chunk) = self.last_chunk {
                if append && last_chunk {
                    return Err("Artifact cannot both append and be the last chunk".to_string());
                }
            }
        }

        Ok(())
    }

    /// Check if this is a streaming chunk.
    ///
    /// # Returns
    ///
    /// `true` if this is part of a streaming sequence.
    pub fn is_streaming_chunk(&self) -> bool {
        self.append.unwrap_or(false) || self.last_chunk.unwrap_or(false)
    }

    /// Check if this is the final chunk in a streaming sequence.
    ///
    /// # Returns
    ///
    /// `true` if this is the last chunk.
    pub fn is_final_chunk(&self) -> bool {
        self.last_chunk.unwrap_or(false)
    }
}

/// Sent by server during sendStream or subscribe requests for status updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatusUpdateEvent {
    /// Event type
    pub kind: String, // Always "status-update"
    /// Task id
    #[serde(rename = "taskId")]
    pub task_id: String,
    /// The context the task is associated with
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// Current status of the task
    pub status: TaskStatus,
    /// Indicates the end of the event stream
    #[serde(rename = "final")]
    pub final_event: bool,
    /// Extension metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl TaskStatusUpdateEvent {
    /// Create a new task status update event.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The task ID.
    /// * `context_id` - The context ID.
    /// * `status` - The current task status.
    /// * `final_event` - Whether this is the final event.
    ///
    /// # Returns
    ///
    /// A new `TaskStatusUpdateEvent`.
    pub fn new(task_id: String, context_id: String, status: TaskStatus, final_event: bool) -> Self {
        Self {
            kind: "status-update".to_string(),
            task_id,
            context_id,
            status,
            final_event,
            metadata: None,
        }
    }

    /// Validate the status update event.
    ///
    /// # Returns
    ///
    /// `Ok(())` if valid, `Err(String)` with error message if invalid.
    pub fn validate(&self) -> Result<(), String> {
        if self.kind != "status-update" {
            return Err("TaskStatusUpdateEvent kind must be 'status-update'".to_string());
        }

        crate::validation::validate_task_id(&self.task_id)?;

        if self.context_id.is_empty() {
            return Err("Context ID cannot be empty".to_string());
        }

        // Validate that final events have terminal states
        if self.final_event {
            match self.status.state {
                TaskState::Completed | TaskState::Failed | TaskState::Canceled | TaskState::Rejected => {
                    // These are valid terminal states for final events
                }
                _ => {
                    return Err("Final status update events must have terminal task states".to_string());
                }
            }
        }

        Ok(())
    }

    /// Check if this event indicates a terminal state.
    ///
    /// # Returns
    ///
    /// `true` if the task has reached a terminal state.
    pub fn is_terminal_state(&self) -> bool {
        matches!(self.status.state, 
            TaskState::Completed | TaskState::Failed | TaskState::Canceled | TaskState::Rejected
        )
    }

    /// Check if this is the final event in the stream.
    ///
    /// # Returns
    ///
    /// `true` if this is the final event.
    pub fn is_final_event(&self) -> bool {
        self.final_event
    }
}

/// Parameters containing only a task ID, used for simple task operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskIdParams {
    /// Task id
    pub id: String,
    /// Extension metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// A2A error response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct A2AErrorResponse {
    /// The error code.
    pub code: i32,
    /// The error message.
    pub message: String,
    /// Additional data about the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Configuration for the send message request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageSendConfiguration {
    /// Accepted output modalities by the client
    #[serde(rename = "acceptedOutputModes")]
    pub accepted_output_modes: Vec<String>,
    /// If the server should treat the client as a blocking request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocking: Option<bool>,
    /// Number of recent messages to be retrieved
    #[serde(rename = "historyLength", skip_serializing_if = "Option::is_none")]
    pub history_length: Option<i32>,
    /// Where the server should send notifications when disconnected
    #[serde(rename = "pushNotificationConfig", skip_serializing_if = "Option::is_none")]
    pub push_notification_config: Option<PushNotificationConfig>,
}

/// Send message request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageRequest {
    /// The method name.
    pub method: RequestMethod,
    /// The parameters for the request.
    pub params: SendMessageParams,
    /// The JSON-RPC ID.
    pub id: String,
    /// The JSON-RPC version.
    pub jsonrpc: String,
}

impl SendMessageRequest {
    /// Create a new send message request.
    ///
    /// # Arguments
    ///
    /// * `id` - The JSON-RPC ID.
    /// * `message_id` - The message ID.
    /// * `text` - The text content of the message.
    /// * `role` - The role of the message sender.
    /// * `configuration` - Send message configuration (optional).
    /// * `metadata` - Additional metadata (optional).
    ///
    /// # Returns
    ///
    /// A new `SendMessageRequest` with the specified parameters.
    pub fn new(
        id: String,
        message_id: String,
        text: String,
        role: MessageRole,
        configuration: Option<MessageSendConfiguration>,
        metadata: Option<serde_json::Value>,
    ) -> Self {
        Self {
            method: RequestMethod::MessageSend,
            params: SendMessageParams {
                message: Message {
                    kind: "message".to_string(),
                    message_id,
                    parts: vec![Part::Text(TextPart {
                        text,
                        metadata: None,
                    })],
                    role,
                    context_id: None,
                    extensions: None,
                    metadata: None,
                    reference_task_ids: None,
                    task_id: None,
                },
                configuration,
                metadata,
            },
            id,
            jsonrpc: "2.0".to_string(),
        }
    }
}

/// Send message parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageParams {
    /// The message being sent to the server
    pub message: Message,
    /// Send message configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<MessageSendConfiguration>,
    /// Extension metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Send message response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageResponse {
    /// The JSON-RPC ID.
    pub id: String,
    /// The JSON-RPC version.
    pub jsonrpc: String,
    /// The result of the request.
    pub result: SendMessageResult,
}

/// Send message result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageResult {
    /// The task ID.
    pub task_id: String,
    /// The message ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// The conversation ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
}

/// Send streaming message request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendStreamingMessageRequest {
    /// The method name.
    pub method: RequestMethod,
    /// The parameters for the request.
    pub params: SendMessageParams,
    /// The JSON-RPC ID.
    pub id: String,
    /// The JSON-RPC version.
    pub jsonrpc: String,
}

/// Get task request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTaskRequest {
    /// The method name.
    pub method: RequestMethod,
    /// The parameters for the request.
    pub params: GetTaskParams,
    /// The JSON-RPC ID.
    pub id: String,
    /// The JSON-RPC version.
    pub jsonrpc: String,
}

impl GetTaskRequest {
    /// Create a new get task request.
    ///
    /// # Arguments
    ///
    /// * `id` - The JSON-RPC ID.
    /// * `task_id` - The ID of the task to get.
    ///
    /// # Returns
    ///
    /// A new `GetTaskRequest` with the specified parameters.
    pub fn new(id: String, task_id: String) -> Self {
        Self {
            method: RequestMethod::TasksGet,
            params: GetTaskParams { task_id },
            id,
            jsonrpc: "2.0".to_string(),
        }
    }
}

/// Get task parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTaskParams {
    /// The task ID.
    pub task_id: String,
}

/// Get task response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTaskResponse {
    /// The JSON-RPC ID.
    pub id: String,
    /// The JSON-RPC version.
    pub jsonrpc: String,
    /// The result of the request.
    pub result: Task,
}

/// Cancel task request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelTaskRequest {
    /// The method name.
    pub method: RequestMethod,
    /// The parameters for the request.
    pub params: CancelTaskParams,
    /// The JSON-RPC ID.
    pub id: String,
    /// The JSON-RPC version.
    pub jsonrpc: String,
}

impl CancelTaskRequest {
    /// Create a new cancel task request.
    ///
    /// # Arguments
    ///
    /// * `id` - The JSON-RPC ID.
    /// * `task_id` - The ID of the task to cancel.
    ///
    /// # Returns
    ///
    /// A new `CancelTaskRequest` with the specified parameters.
    pub fn new(id: String, task_id: String) -> Self {
        Self {
            method: RequestMethod::TasksCancel,
            params: CancelTaskParams { task_id },
            id,
            jsonrpc: "2.0".to_string(),
        }
    }
}

/// Cancel task parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelTaskParams {
    /// The task ID.
    pub task_id: String,
}

/// Cancel task response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelTaskResponse {
    /// The JSON-RPC ID.
    pub id: String,
    /// The JSON-RPC version.
    pub jsonrpc: String,
    /// The result of the request.
    pub result: Task,
}

/// Set task push notification config request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTaskPushNotificationConfigRequest {
    /// The method name.
    pub method: RequestMethod,
    /// The parameters for the request.
    pub params: SetTaskPushNotificationConfigParams,
    /// The JSON-RPC ID.
    pub id: String,
    /// The JSON-RPC version.
    pub jsonrpc: String,
}

/// Set task push notification config parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTaskPushNotificationConfigParams {
    /// The task ID.
    pub task_id: String,
    /// The push notification config.
    pub config: PushNotificationConfig,
}

/// Push notification authentication info
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushNotificationAuthenticationInfo {
    /// Supported authentication schemes - e.g. Basic, Bearer
    pub schemes: Vec<String>,
    /// Optional credentials
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
}

/// Push notification config.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushNotificationConfig {
    /// URL for sending the push notifications
    pub url: String,
    /// Authentication details for push notifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<PushNotificationAuthenticationInfo>,
    /// Push Notification ID - created by server to support multiple callbacks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Token unique to this task/session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// Set task push notification config response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTaskPushNotificationConfigResponse {
    /// The JSON-RPC ID.
    pub id: String,
    /// The JSON-RPC version.
    pub jsonrpc: String,
    /// The result of the request.
    pub result: PushNotificationConfigResult,
}

/// Push notification config result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PushNotificationConfigResult {
    /// The task ID.
    pub task_id: String,
    /// The push notification config ID.
    pub config_id: String,
}

/// Get task push notification config request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTaskPushNotificationConfigRequest {
    /// The method name.
    pub method: RequestMethod,
    /// The parameters for the request.
    pub params: GetTaskPushNotificationConfigParams,
    /// The JSON-RPC ID.
    pub id: String,
    /// The JSON-RPC version.
    pub jsonrpc: String,
}

/// Get task push notification config parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTaskPushNotificationConfigParams {
    /// The task ID.
    pub task_id: String,
    /// The push notification config ID.
    pub config_id: String,
}

/// Get task push notification config response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTaskPushNotificationConfigResponse {
    /// The JSON-RPC ID.
    pub id: String,
    /// The JSON-RPC version.
    pub jsonrpc: String,
    /// The result of the request.
    pub result: PushNotificationConfig,
}

/// Task resubscription request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskResubscriptionRequest {
    /// The method name.
    pub method: RequestMethod,
    /// The parameters for the request.
    pub params: TaskResubscriptionParams,
    /// The JSON-RPC ID.
    pub id: String,
    /// The JSON-RPC version.
    pub jsonrpc: String,
}

/// Task resubscription parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskResubscriptionParams {
    /// The task ID.
    pub task_id: String,
}

/// Task resubscription response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskResubscriptionResponse {
    /// The JSON-RPC ID.
    pub id: String,
    /// The JSON-RPC version.
    pub jsonrpc: String,
    /// The result of the request.
    pub result: Task,
}

/// List task push notification config request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTaskPushNotificationConfigRequest {
    /// The method name.
    pub method: RequestMethod,
    /// The parameters for the request.
    pub params: ListTaskPushNotificationConfigParams,
    /// The JSON-RPC ID.
    pub id: String,
    /// The JSON-RPC version.
    pub jsonrpc: String,
}

/// List task push notification config parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTaskPushNotificationConfigParams {
    /// The task ID.
    pub task_id: String,
}

/// List task push notification config response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTaskPushNotificationConfigResponse {
    /// The JSON-RPC ID.
    pub id: String,
    /// The JSON-RPC version.
    pub jsonrpc: String,
    /// The result of the request.
    pub result: Vec<PushNotificationConfigInfo>,
}

/// Push notification config info.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PushNotificationConfigInfo {
    /// The push notification config ID.
    pub config_id: String,
    /// The URL to send push notifications to.
    pub url: String,
}

/// Delete task push notification config request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTaskPushNotificationConfigRequest {
    /// The method name.
    pub method: RequestMethod,
    /// The parameters for the request.
    pub params: DeleteTaskPushNotificationConfigParams,
    /// The JSON-RPC ID.
    pub id: String,
    /// The JSON-RPC version.
    pub jsonrpc: String,
}

/// Delete task push notification config parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTaskPushNotificationConfigParams {
    /// The task ID.
    pub task_id: String,
    /// The push notification config ID.
    pub config_id: String,
}

/// Delete task push notification config response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTaskPushNotificationConfigResponse {
    /// The JSON-RPC ID.
    pub id: String,
    /// The JSON-RPC version.
    pub jsonrpc: String,
    /// The result of the request.
    pub result: bool,
}

/// Helper functions for working with the A2A protocol.
pub mod helpers {
    use super::*;

    /// Parse a JSON string into an A2A request.
    ///
    /// # Arguments
    ///
    /// * `json` - The JSON string to parse.
    ///
    /// # Returns
    ///
    /// A `Result` containing either the parsed request or an error.
    pub fn parse_request(json: &str) -> Result<serde_json::Value, A2AError> {
        serde_json::from_str(json).map_err(|e| A2AError::JSONParse(JSONParseError {
            code: -32700,
            message: format!("Invalid JSON payload: {}", e),
            data: None,
        }))
    }

    /// Serialize an A2A response to a JSON string.
    ///
    /// # Arguments
    ///
    /// * `response` - The response to serialize.
    ///
    /// # Returns
    ///
    /// A `Result` containing either the serialized JSON string or an error.
    pub fn serialize_response<T: Serialize>(response: &T) -> Result<String, A2AError> {
        serde_json::to_string(response).map_err(|e| A2AError::Internal(InternalError {
            code: -32603,
            message: format!("Internal error: {}", e),
            data: None,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_agent_card() {
        let card = AgentCard::new(
            "Test Agent".to_string(),
            "A test agent".to_string(),
            "1.0.0".to_string(),
            "https://example.com/agent".to_string(),
            AgentCapabilities {
                extensions: None,
                push_notifications: Some(false),
                state_transition_history: Some(true),
                streaming: Some(false),
            },
            vec!["text/plain".to_string()],
            vec!["text/plain".to_string()],
            vec![AgentSkill {
                name: "test".to_string(),
                description: "A test skill".to_string(),
                input_modes: None,
                output_modes: None,
                examples: None,
            }],
        );

        assert_eq!(card.name, "Test Agent");
        assert_eq!(card.description, "A test agent");
        assert_eq!(card.version, "1.0.0");
        assert_eq!(card.protocol_version, PROTOCOL_VERSION);
        assert_eq!(card.url, "https://example.com/agent");
        assert_eq!(card.capabilities.push_notifications, Some(false));
        assert_eq!(card.capabilities.state_transition_history, Some(true));
        assert_eq!(card.capabilities.streaming, Some(false));
        assert_eq!(card.default_input_modes, vec!["text/plain"]);
        assert_eq!(card.default_output_modes, vec!["text/plain"]);
        assert_eq!(card.skills.len(), 1);
        assert_eq!(card.skills[0].name, "test");
        assert_eq!(card.skills[0].description, "A test skill");
    }

    #[test]
    fn test_create_send_message_request() {
        let request = SendMessageRequest::new(
            "1".to_string(),
            "msg-123".to_string(),
            "Hello, world!".to_string(),
            MessageRole::User,
            None,
            None,
        );

        assert_eq!(request.method, RequestMethod::MessageSend);
        assert_eq!(request.id, "1");
        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.params.message.kind, "message");
        assert_eq!(request.params.message.message_id, "msg-123");
        assert_eq!(request.params.message.role, MessageRole::User);
        assert_eq!(request.params.message.parts.len(), 1);
        if let Part::Text(text_part) = &request.params.message.parts[0] {
            assert_eq!(text_part.text, "Hello, world!");
        } else {
            panic!("Expected TextPart");
        }
        assert_eq!(request.params.configuration, None);
        assert_eq!(request.params.metadata, None);
    }

    #[test]
    fn test_create_get_task_request() {
        let request = GetTaskRequest::new("1".to_string(), "task1".to_string());

        assert_eq!(request.method, RequestMethod::TasksGet);
        assert_eq!(request.id, "1");
        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.params.task_id, "task1");
    }

    #[test]
    fn test_create_cancel_task_request() {
        let request = CancelTaskRequest::new("1".to_string(), "task1".to_string());

        assert_eq!(request.method, RequestMethod::TasksCancel);
        assert_eq!(request.id, "1");
        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.params.task_id, "task1");
    }

    #[test]
    fn test_request_method_serialization() {
        // Test MessageSend serialization and deserialization
        let method = RequestMethod::MessageSend;
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(serialized, "\"message/send\"");

        let deserialized: RequestMethod = serde_json::from_str("\"message/send\"").unwrap();
        assert_eq!(deserialized, RequestMethod::MessageSend);

        // Test backward compatibility for MessageSend
        let deserialized: RequestMethod = serde_json::from_str("\"sendMessage\"").unwrap();
        assert_eq!(deserialized, RequestMethod::MessageSend);

        // Test TasksGet serialization and deserialization
        let method = RequestMethod::TasksGet;
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(serialized, "\"tasks/get\"");

        let deserialized: RequestMethod = serde_json::from_str("\"tasks/get\"").unwrap();
        assert_eq!(deserialized, RequestMethod::TasksGet);

        // Test backward compatibility for TasksGet
        let deserialized: RequestMethod = serde_json::from_str("\"getTask\"").unwrap();
        assert_eq!(deserialized, RequestMethod::TasksGet);

        // Test TasksCancel serialization and deserialization
        let method = RequestMethod::TasksCancel;
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(serialized, "\"tasks/cancel\"");

        let deserialized: RequestMethod = serde_json::from_str("\"tasks/cancel\"").unwrap();
        assert_eq!(deserialized, RequestMethod::TasksCancel);

        // Test backward compatibility for TasksCancel
        let deserialized: RequestMethod = serde_json::from_str("\"cancelTask\"").unwrap();
        assert_eq!(deserialized, RequestMethod::TasksCancel);

        // Test MessageStream serialization and deserialization
        let method = RequestMethod::MessageStream;
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(serialized, "\"message/stream\"");

        let deserialized: RequestMethod = serde_json::from_str("\"message/stream\"").unwrap();
        assert_eq!(deserialized, RequestMethod::MessageStream);

        // Test TasksPushNotificationConfigSet serialization and deserialization
        let method = RequestMethod::TasksPushNotificationConfigSet;
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(serialized, "\"tasks/pushNotificationConfig/set\"");

        let deserialized: RequestMethod = serde_json::from_str("\"tasks/pushNotificationConfig/set\"").unwrap();
        assert_eq!(deserialized, RequestMethod::TasksPushNotificationConfigSet);
    }

    // ============================================================================
    // A2A SPECIFICATION VALIDATION TESTS
    // ============================================================================

    #[test]
    fn test_message_spec_compliance() {
        let message = Message {
            kind: "message".to_string(),
            message_id: "msg-123".to_string(),
            parts: vec![Part::Text(TextPart {
                text: "Hello, world!".to_string(),
                metadata: None,
            })],
            role: MessageRole::User,
            context_id: Some("ctx-456".to_string()),
            extensions: None,
            metadata: None,
            reference_task_ids: None,
            task_id: Some("task-789".to_string()),
        };

        let json = serde_json::to_value(&message).unwrap();

        // Validate required fields
        assert_eq!(json["kind"], "message");
        assert_eq!(json["messageId"], "msg-123");
        assert_eq!(json["role"], "user");
        assert_eq!(json["contextId"], "ctx-456");
        assert_eq!(json["taskId"], "task-789");

        // Validate parts structure
        assert!(json["parts"].is_array());
        assert_eq!(json["parts"][0]["kind"], "text");
        assert_eq!(json["parts"][0]["text"], "Hello, world!");

        // Test round-trip serialization
        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: Message = serde_json::from_str(&serialized).unwrap();
        assert_eq!(message.kind, deserialized.kind);
        assert_eq!(message.message_id, deserialized.message_id);
        assert_eq!(message.role, deserialized.role);
    }

    #[test]
    fn test_task_status_spec_compliance() {
        let message = Message {
            kind: "message".to_string(),
            message_id: "status-msg".to_string(),
            parts: vec![Part::Text(TextPart {
                text: "Task is working".to_string(),
                metadata: None,
            })],
            role: MessageRole::Agent,
            context_id: None,
            extensions: None,
            metadata: None,
            reference_task_ids: None,
            task_id: None,
        };

        let task_status = TaskStatus {
            state: TaskState::Working,
            message: Some(message),
            timestamp: Some("2023-10-27T10:00:00Z".to_string()),
        };

        let json = serde_json::to_value(&task_status).unwrap();

        // Validate TaskState serialization uses kebab-case
        assert_eq!(json["state"], "working");
        assert_eq!(json["timestamp"], "2023-10-27T10:00:00Z");
        assert!(json["message"].is_object());

        // Test all TaskState variants
        let states = vec![
            (TaskState::Submitted, "submitted"),
            (TaskState::Working, "working"),
            (TaskState::InputRequired, "input-required"),
            (TaskState::Completed, "completed"),
            (TaskState::Canceled, "canceled"),
            (TaskState::Failed, "failed"),
            (TaskState::Rejected, "rejected"),
            (TaskState::AuthRequired, "auth-required"),
            (TaskState::Unknown, "unknown"),
        ];

        for (state, expected_json) in states {
            let json = serde_json::to_string(&state).unwrap();
            assert_eq!(json, format!("\"{}\"", expected_json));

            let deserialized: TaskState = serde_json::from_str(&json).unwrap();
            assert_eq!(std::mem::discriminant(&state), std::mem::discriminant(&deserialized));
        }
    }

    #[test]
    fn test_agent_card_spec_compliance() {
        let agent_card = AgentCard::new(
            "Test Agent".to_string(),
            "A test agent for A2A protocol".to_string(),
            "1.0.0".to_string(),
            "https://example.com/agent".to_string(),
            AgentCapabilities {
                extensions: None,
                push_notifications: Some(true),
                state_transition_history: Some(true),
                streaming: Some(false),
            },
            vec!["text/plain".to_string(), "application/json".to_string()],
            vec!["text/plain".to_string(), "application/json".to_string()],
            vec![AgentSkill {
                name: "text_processing".to_string(),
                description: "Process and analyze text content".to_string(),
                input_modes: Some(vec!["text/plain".to_string()]),
                output_modes: Some(vec!["text/plain".to_string()]),
                examples: Some(vec!["Analyze this text".to_string(), "Summarize this document".to_string()]),
            }],
        );

        let json = serde_json::to_value(&agent_card).unwrap();

        // Validate camelCase field names as per spec
        assert_eq!(json["protocolVersion"], "0.2.5");
        assert_eq!(json["defaultInputModes"][0], "text/plain");
        assert_eq!(json["defaultOutputModes"][0], "text/plain");
        assert_eq!(json["capabilities"]["pushNotifications"], true);
        assert_eq!(json["capabilities"]["stateTransitionHistory"], true);
        assert_eq!(json["capabilities"]["streaming"], false);

        // Validate skill structure
        assert_eq!(json["skills"][0]["inputModes"][0], "text/plain");
        assert_eq!(json["skills"][0]["outputModes"][0], "text/plain");
        assert_eq!(json["skills"][0]["examples"][0], "Analyze this text");

        // Test round-trip serialization
        let serialized = serde_json::to_string(&agent_card).unwrap();
        let deserialized: AgentCard = serde_json::from_str(&serialized).unwrap();
        assert_eq!(agent_card.name, deserialized.name);
        assert_eq!(agent_card.protocol_version, deserialized.protocol_version);
    }

    #[test]
    fn test_message_send_params_spec_compliance() {
        let message = Message {
            kind: "message".to_string(),
            message_id: "msg-456".to_string(),
            parts: vec![Part::Text(TextPart {
                text: "Test message".to_string(),
                metadata: None,
            })],
            role: MessageRole::User,
            context_id: None,
            extensions: None,
            metadata: None,
            reference_task_ids: None,
            task_id: None,
        };

        let send_params = SendMessageParams {
            message: message.clone(),
            configuration: Some(MessageSendConfiguration {
                accepted_output_modes: vec!["text/plain".to_string()],
                blocking: Some(true),
                history_length: Some(10),
                push_notification_config: Some(PushNotificationConfig {
                    url: "https://example.com/webhook".to_string(),
                    authentication: Some(PushNotificationAuthenticationInfo {
                        schemes: vec!["Bearer".to_string()],
                        credentials: Some("token123".to_string()),
                    }),
                    id: Some("webhook-1".to_string()),
                    token: Some("session-token".to_string()),
                }),
            }),
            metadata: None,
        };

        let json = serde_json::to_value(&send_params).unwrap();

        // Validate camelCase field names
        assert_eq!(json["configuration"]["acceptedOutputModes"][0], "text/plain");
        assert_eq!(json["configuration"]["historyLength"], 10);
        assert_eq!(json["configuration"]["pushNotificationConfig"]["url"], "https://example.com/webhook");

        // Test round-trip serialization
        let serialized = serde_json::to_string(&send_params).unwrap();
        let deserialized: SendMessageParams = serde_json::from_str(&serialized).unwrap();
        assert_eq!(send_params.message.message_id, deserialized.message.message_id);
    }

    #[test]
    fn test_part_variants_spec_compliance() {
        // Test TextPart
        let text_part = Part::Text(TextPart {
            text: "Hello world".to_string(),
            metadata: Some(serde_json::json!({"source": "user"})),
        });

        let json = serde_json::to_value(&text_part).unwrap();
        assert_eq!(json["kind"], "text");
        assert_eq!(json["text"], "Hello world");
        assert_eq!(json["metadata"]["source"], "user");

        // Test FilePart with bytes
        let file_part = Part::File(FilePart {
            file: FileContent::WithBytes(FileWithBytes {
                bytes: "SGVsbG8gd29ybGQ=".to_string(), // "Hello world" in base64
                name: Some("test.txt".to_string()),
                mime_type: Some("text/plain".to_string()),
            }),
            metadata: None,
        });

        let json = serde_json::to_value(&file_part).unwrap();
        assert_eq!(json["kind"], "file");
        assert_eq!(json["file"]["bytes"], "SGVsbG8gd29ybGQ=");
        assert_eq!(json["file"]["name"], "test.txt");
        assert_eq!(json["file"]["mimeType"], "text/plain");

        // Test FilePart with URI
        let file_part_uri = Part::File(FilePart {
            file: FileContent::WithUri(FileWithUri {
                uri: "https://example.com/file.txt".to_string(),
                name: Some("remote.txt".to_string()),
                mime_type: Some("text/plain".to_string()),
            }),
            metadata: None,
        });

        let json = serde_json::to_value(&file_part_uri).unwrap();
        assert_eq!(json["kind"], "file");
        assert_eq!(json["file"]["uri"], "https://example.com/file.txt");

        // Test DataPart
        let data_part = Part::Data(DataPart {
            data: serde_json::json!({"key": "value", "number": 42}),
            metadata: None,
        });

        let json = serde_json::to_value(&data_part).unwrap();
        assert_eq!(json["kind"], "data");
        assert_eq!(json["data"]["key"], "value");
        assert_eq!(json["data"]["number"], 42);
    }

    #[test]
    fn test_push_notification_config_spec_compliance() {
        let config = PushNotificationConfig {
            url: "https://example.com/webhook".to_string(),
            authentication: Some(PushNotificationAuthenticationInfo {
                schemes: vec!["Bearer".to_string(), "Basic".to_string()],
                credentials: Some("secret-token".to_string()),
            }),
            id: Some("notification-1".to_string()),
            token: Some("session-abc123".to_string()),
        };

        let json = serde_json::to_value(&config).unwrap();

        // Validate structure matches spec
        assert_eq!(json["url"], "https://example.com/webhook");
        assert_eq!(json["authentication"]["schemes"][0], "Bearer");
        assert_eq!(json["authentication"]["schemes"][1], "Basic");
        assert_eq!(json["authentication"]["credentials"], "secret-token");
        assert_eq!(json["id"], "notification-1");
        assert_eq!(json["token"], "session-abc123");

        // Test round-trip serialization
        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: PushNotificationConfig = serde_json::from_str(&serialized).unwrap();
        assert_eq!(config.url, deserialized.url);
        assert_eq!(config.id, deserialized.id);
        assert_eq!(config.token, deserialized.token);
    }

    #[test]
    fn test_agent_provider_spec_compliance() {
        let provider = AgentProvider {
            organization: "Test Organization".to_string(),
            url: "https://test-org.com".to_string(),
        };

        let json = serde_json::to_value(&provider).unwrap();

        // Validate field names match spec exactly
        assert_eq!(json["organization"], "Test Organization");
        assert_eq!(json["url"], "https://test-org.com");

        // Test round-trip serialization
        let serialized = serde_json::to_string(&provider).unwrap();
        let deserialized: AgentProvider = serde_json::from_str(&serialized).unwrap();
        assert_eq!(provider.organization, deserialized.organization);
        assert_eq!(provider.url, deserialized.url);
    }

    #[test]
    fn test_a2a_error_types_spec_compliance() {
        // Test TaskNotFoundError
        let task_not_found = TaskNotFoundError {
            code: -32001,
            message: "Task not found".to_string(),
            data: None,
        };
        let serialized = serde_json::to_string(&task_not_found).unwrap();
        let deserialized: TaskNotFoundError = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.code, -32001);
        assert_eq!(deserialized.message, "Task not found");

        // Test InternalError
        let internal_error = InternalError {
            code: -32603,
            message: "Internal error".to_string(),
            data: None,
        };
        let serialized = serde_json::to_string(&internal_error).unwrap();
        let deserialized: InternalError = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.code, -32603);
        assert_eq!(deserialized.message, "Internal error");

        // Test JSONParseError
        let json_parse_error = JSONParseError {
            code: -32700,
            message: "Invalid JSON payload".to_string(),
            data: None,
        };
        let serialized = serde_json::to_string(&json_parse_error).unwrap();
        let deserialized: JSONParseError = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.code, -32700);
        assert_eq!(deserialized.message, "Invalid JSON payload");

        // Test A2AError union type
        let error = A2AError::TaskNotFound(TaskNotFoundError {
            code: -32001,
            message: "Task not found".to_string(),
            data: None,
        });
        let serialized = serde_json::to_string(&error).unwrap();
        let deserialized: A2AError = serde_json::from_str(&serialized).unwrap();
        match deserialized {
            A2AError::TaskNotFound(e) => {
                assert_eq!(e.code, -32001);
                assert_eq!(e.message, "Task not found");
            }
            _ => panic!("Expected TaskNotFound error"),
        }
    }

    #[test]
    fn test_oauth2_flows_spec_compliance() {
        // Test AuthorizationCodeOAuthFlow
        let auth_code_flow = AuthorizationCodeOAuthFlow {
            authorization_url: "https://example.com/auth".to_string(),
            token_url: "https://example.com/token".to_string(),
            refresh_url: Some("https://example.com/refresh".to_string()),
            scopes: {
                let mut scopes = HashMap::new();
                scopes.insert("read".to_string(), "Read access".to_string());
                scopes.insert("write".to_string(), "Write access".to_string());
                scopes
            },
        };

        let json = serde_json::to_value(&auth_code_flow).unwrap();
        assert_eq!(json["authorizationUrl"], "https://example.com/auth");
        assert_eq!(json["tokenUrl"], "https://example.com/token");
        assert_eq!(json["refreshUrl"], "https://example.com/refresh");
        assert_eq!(json["scopes"]["read"], "Read access");
        assert_eq!(json["scopes"]["write"], "Write access");

        // Test round-trip serialization
        let serialized = serde_json::to_string(&auth_code_flow).unwrap();
        let deserialized: AuthorizationCodeOAuthFlow = serde_json::from_str(&serialized).unwrap();
        assert_eq!(auth_code_flow.authorization_url, deserialized.authorization_url);
        assert_eq!(auth_code_flow.token_url, deserialized.token_url);
        assert_eq!(auth_code_flow.refresh_url, deserialized.refresh_url);
        assert_eq!(auth_code_flow.scopes, deserialized.scopes);

        // Test PasswordOAuthFlow
        let password_flow = PasswordOAuthFlow {
            token_url: "https://example.com/token".to_string(),
            refresh_url: Some("https://example.com/refresh".to_string()),
            scopes: {
                let mut scopes = HashMap::new();
                scopes.insert("api".to_string(), "API access".to_string());
                scopes
            },
        };

        let json = serde_json::to_value(&password_flow).unwrap();
        assert_eq!(json["tokenUrl"], "https://example.com/token");
        assert_eq!(json["refreshUrl"], "https://example.com/refresh");
        assert_eq!(json["scopes"]["api"], "API access");

        // Test ImplicitOAuthFlow
        let implicit_flow = ImplicitOAuthFlow {
            authorization_url: "https://example.com/auth".to_string(),
            refresh_url: None,
            scopes: HashMap::new(),
        };

        let json = serde_json::to_value(&implicit_flow).unwrap();
        assert_eq!(json["authorizationUrl"], "https://example.com/auth");
        assert!(json["refreshUrl"].is_null());
        assert!(json["scopes"].as_object().unwrap().is_empty());

        // Test ClientCredentialsOAuthFlow
        let client_creds_flow = ClientCredentialsOAuthFlow {
            token_url: "https://example.com/token".to_string(),
            refresh_url: None,
            scopes: HashMap::new(),
        };

        let json = serde_json::to_value(&client_creds_flow).unwrap();
        assert_eq!(json["tokenUrl"], "https://example.com/token");
        assert!(json["refreshUrl"].is_null());
    }

    #[test]
    fn test_comprehensive_error_types_spec_compliance() {
        // Test all error types with their specific codes
        let test_cases = vec![
            (A2AError::JSONParse(JSONParseError {
                code: -32700,
                message: "Parse error".to_string(),
                data: None,
            }), -32700),
            (A2AError::InvalidRequest(InvalidRequestError {
                code: -32600,
                message: "Invalid Request".to_string(),
                data: None,
            }), -32600),
            (A2AError::MethodNotFound(MethodNotFoundError {
                code: -32601,
                message: "Method not found".to_string(),
                data: None,
            }), -32601),
            (A2AError::InvalidParams(InvalidParamsError {
                code: -32602,
                message: "Invalid params".to_string(),
                data: None,
            }), -32602),
            (A2AError::Internal(InternalError {
                code: -32603,
                message: "Internal error".to_string(),
                data: None,
            }), -32603),
            (A2AError::TaskNotFound(TaskNotFoundError {
                code: -32001,
                message: "Task not found".to_string(),
                data: None,
            }), -32001),
            (A2AError::TaskNotCancelable(TaskNotCancelableError {
                code: -32002,
                message: "Task not cancelable".to_string(),
                data: None,
            }), -32002),
            (A2AError::PushNotificationNotSupported(PushNotificationNotSupportedError {
                code: -32003,
                message: "Push notifications not supported".to_string(),
                data: None,
            }), -32003),
            (A2AError::UnsupportedOperation(UnsupportedOperationError {
                code: -32004,
                message: "Unsupported operation".to_string(),
                data: None,
            }), -32004),
            (A2AError::ContentTypeNotSupported(ContentTypeNotSupportedError {
                code: -32005,
                message: "Content type not supported".to_string(),
                data: None,
            }), -32005),
            (A2AError::InvalidAgentResponse(InvalidAgentResponseError {
                code: -32006,
                message: "Invalid agent response".to_string(),
                data: None,
            }), -32006),
        ];

        for (error, expected_code) in test_cases {
            // Test serialization
            let serialized = serde_json::to_string(&error).unwrap();
            let json: serde_json::Value = serde_json::from_str(&serialized).unwrap();
            assert_eq!(json["code"].as_i64().unwrap(), expected_code);

            // Test deserialization
            let deserialized: A2AError = serde_json::from_str(&serialized).unwrap();

            // Verify the error type matches
            match (&error, &deserialized) {
                (A2AError::JSONParse(_), A2AError::JSONParse(_)) => {},
                (A2AError::InvalidRequest(_), A2AError::InvalidRequest(_)) => {},
                (A2AError::MethodNotFound(_), A2AError::MethodNotFound(_)) => {},
                (A2AError::InvalidParams(_), A2AError::InvalidParams(_)) => {},
                (A2AError::Internal(_), A2AError::Internal(_)) => {},
                (A2AError::TaskNotFound(_), A2AError::TaskNotFound(_)) => {},
                (A2AError::TaskNotCancelable(_), A2AError::TaskNotCancelable(_)) => {},
                (A2AError::PushNotificationNotSupported(_), A2AError::PushNotificationNotSupported(_)) => {},
                (A2AError::UnsupportedOperation(_), A2AError::UnsupportedOperation(_)) => {},
                (A2AError::ContentTypeNotSupported(_), A2AError::ContentTypeNotSupported(_)) => {},
                (A2AError::InvalidAgentResponse(_), A2AError::InvalidAgentResponse(_)) => {},
                _ => panic!("Error type mismatch during deserialization"),
            }
        }
    }

    #[test]
    fn test_json_schema_examples_validation() {
        // Test with realistic JSON examples that match the schema

        // Test Message with all fields
        let message_json = r#"{
            "kind": "message",
            "messageId": "msg-123",
            "parts": [
                {
                    "kind": "text",
                    "text": "Hello, world!"
                }
            ],
            "role": "user",
            "contextId": "ctx-456",
            "taskId": "task-789"
        }"#;

        let message: Message = serde_json::from_str(message_json).unwrap();
        assert_eq!(message.message_id, "msg-123");
        assert_eq!(message.role, MessageRole::User);
        assert_eq!(message.context_id, Some("ctx-456".to_string()));
        assert_eq!(message.task_id, Some("task-789".to_string()));

        // Test round-trip
        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: Message = serde_json::from_str(&serialized).unwrap();
        assert_eq!(message.message_id, deserialized.message_id);

        // Test AgentCard with comprehensive fields
        let agent_card_json = r#"{
            "name": "Test Agent",
            "description": "A comprehensive test agent",
            "version": "1.0.0",
            "protocolVersion": "0.2.5",
            "url": "https://example.com/agent",
            "capabilities": {
                "pushNotifications": true,
                "stateTransitionHistory": true,
                "streaming": false
            },
            "defaultInputModes": ["text/plain", "application/json"],
            "defaultOutputModes": ["text/plain", "application/json"],
            "skills": [
                {
                    "name": "text_processing",
                    "description": "Process text content",
                    "inputModes": ["text/plain"],
                    "outputModes": ["text/plain"],
                    "examples": ["Analyze this text"]
                }
            ]
        }"#;

        let agent_card: AgentCard = serde_json::from_str(agent_card_json).unwrap();
        assert_eq!(agent_card.name, "Test Agent");
        assert_eq!(agent_card.protocol_version, "0.2.5");
        assert_eq!(agent_card.capabilities.push_notifications, Some(true));
        assert_eq!(agent_card.skills.len(), 1);
        assert_eq!(agent_card.skills[0].name, "text_processing");
    }

    #[test]
    fn test_performance_serialization_deserialization() {
        use std::time::Instant;

        // Create a complex AgentCard for performance testing
        let agent_card = AgentCard::new(
            "Performance Test Agent".to_string(),
            "A complex agent for performance testing".to_string(),
            "1.0.0".to_string(),
            "https://example.com/agent".to_string(),
            AgentCapabilities {
                extensions: Some(vec![
                    AgentExtension {
                        uri: "https://example.com/ext1".to_string(),
                        required: Some(true),
                        description: Some("Extension 1".to_string()),
                        params: Some(serde_json::json!({"param1": "value1"})),
                    },
                    AgentExtension {
                        uri: "https://example.com/ext2".to_string(),
                        required: Some(false),
                        description: Some("Extension 2".to_string()),
                        params: Some(serde_json::json!({"param2": "value2"})),
                    },
                ]),
                push_notifications: Some(true),
                state_transition_history: Some(true),
                streaming: Some(true),
            },
            vec!["text/plain".to_string(), "application/json".to_string(), "image/png".to_string()],
            vec!["text/plain".to_string(), "application/json".to_string(), "image/png".to_string()],
            vec![
                AgentSkill {
                    name: "text_processing".to_string(),
                    description: "Advanced text processing capabilities".to_string(),
                    input_modes: Some(vec!["text/plain".to_string(), "text/markdown".to_string()]),
                    output_modes: Some(vec!["text/plain".to_string(), "text/html".to_string()]),
                    examples: Some(vec![
                        "Analyze sentiment".to_string(),
                        "Extract entities".to_string(),
                        "Summarize content".to_string(),
                    ]),
                },
                AgentSkill {
                    name: "image_processing".to_string(),
                    description: "Image analysis and processing".to_string(),
                    input_modes: Some(vec!["image/png".to_string(), "image/jpeg".to_string()]),
                    output_modes: Some(vec!["application/json".to_string()]),
                    examples: Some(vec![
                        "Detect objects".to_string(),
                        "Extract text".to_string(),
                    ]),
                },
            ],
        );

        // Performance test: Serialize 1000 times
        let start = Instant::now();
        for _ in 0..1000 {
            let _serialized = serde_json::to_string(&agent_card).unwrap();
        }
        let serialize_duration = start.elapsed();

        // Performance test: Deserialize 1000 times
        let serialized = serde_json::to_string(&agent_card).unwrap();
        let start = Instant::now();
        for _ in 0..1000 {
            let _deserialized: AgentCard = serde_json::from_str(&serialized).unwrap();
        }
        let deserialize_duration = start.elapsed();

        // Basic performance assertions (should complete within reasonable time)
        assert!(serialize_duration.as_millis() < 1000, "Serialization took too long: {:?}", serialize_duration);
        assert!(deserialize_duration.as_millis() < 1000, "Deserialization took too long: {:?}", deserialize_duration);

        println!("Performance test results:");
        println!("  Serialization (1000x): {:?}", serialize_duration);
        println!("  Deserialization (1000x): {:?}", deserialize_duration);
    }

    #[test]
    fn test_security_scheme_spec_compliance() {
        // Test API Key security scheme
        let api_key_scheme = SecurityScheme::ApiKey(ApiKeySecurityScheme {
            type_: "apiKey".to_string(),
            in_: ApiKeyLocation::Header,
            name: "X-API-Key".to_string(),
            description: Some("API key authentication".to_string()),
        });

        let json = serde_json::to_value(&api_key_scheme).unwrap();
        assert_eq!(json["type"], "apiKey");
        assert_eq!(json["in"], "header");
        assert_eq!(json["name"], "X-API-Key");

        // Test round-trip serialization
        let serialized = serde_json::to_string(&api_key_scheme).unwrap();
        let deserialized: SecurityScheme = serde_json::from_str(&serialized).unwrap();
        if let SecurityScheme::ApiKey(scheme) = deserialized {
            assert_eq!(scheme.name, "X-API-Key");
        } else {
            panic!("Expected ApiKey security scheme");
        }
    }

    #[test]
    fn test_artifact_spec_compliance() {
        let artifact = Artifact {
            artifact_id: "artifact-123".to_string(),
            parts: vec![Part::Text(TextPart {
                text: "Generated content".to_string(),
                metadata: None,
            })],
            description: Some("Test artifact".to_string()),
            extensions: Some(vec!["https://example.com/extension".to_string()]),
            metadata: Some(serde_json::json!({"source": "agent"})),
            name: Some("test-artifact.txt".to_string()),
        };

        let json = serde_json::to_value(&artifact).unwrap();

        // Validate required fields
        assert_eq!(json["artifactId"], "artifact-123");
        assert!(json["parts"].is_array());
        assert_eq!(json["parts"][0]["kind"], "text");
        assert_eq!(json["parts"][0]["text"], "Generated content");

        // Validate optional fields
        assert_eq!(json["description"], "Test artifact");
        assert_eq!(json["extensions"][0], "https://example.com/extension");
        assert_eq!(json["metadata"]["source"], "agent");
        assert_eq!(json["name"], "test-artifact.txt");

        // Test round-trip serialization
        let serialized = serde_json::to_string(&artifact).unwrap();
        let deserialized: Artifact = serde_json::from_str(&serialized).unwrap();
        assert_eq!(artifact.artifact_id, deserialized.artifact_id);
        assert_eq!(artifact.parts.len(), deserialized.parts.len());
        assert_eq!(artifact.description, deserialized.description);
        assert_eq!(artifact.name, deserialized.name);
    }

    #[test]
    fn test_task_artifact_update_event_spec_compliance() {
        let artifact = Artifact {
            artifact_id: "artifact-456".to_string(),
            parts: vec![Part::Text(TextPart {
                text: "Updated content".to_string(),
                metadata: None,
            })],
            description: None,
            extensions: None,
            metadata: None,
            name: None,
        };

        let event = TaskArtifactUpdateEvent {
            kind: "artifact-update".to_string(),
            task_id: "task-789".to_string(),
            context_id: "ctx-123".to_string(),
            artifact,
            append: Some(true),
            last_chunk: Some(false),
            metadata: Some(serde_json::json!({"timestamp": "2023-10-27T10:00:00Z"})),
        };

        let json = serde_json::to_value(&event).unwrap();

        // Validate required fields
        assert_eq!(json["kind"], "artifact-update");
        assert_eq!(json["taskId"], "task-789");
        assert_eq!(json["contextId"], "ctx-123");
        assert!(json["artifact"].is_object());
        assert_eq!(json["artifact"]["artifactId"], "artifact-456");

        // Validate optional fields
        assert_eq!(json["append"], true);
        assert_eq!(json["lastChunk"], false);
        assert_eq!(json["metadata"]["timestamp"], "2023-10-27T10:00:00Z");

        // Test round-trip serialization
        let serialized = serde_json::to_string(&event).unwrap();
        let deserialized: TaskArtifactUpdateEvent = serde_json::from_str(&serialized).unwrap();
        assert_eq!(event.kind, deserialized.kind);
        assert_eq!(event.task_id, deserialized.task_id);
        assert_eq!(event.context_id, deserialized.context_id);
        assert_eq!(event.append, deserialized.append);
        assert_eq!(event.last_chunk, deserialized.last_chunk);
    }

    #[test]
    fn test_task_status_update_event_spec_compliance() {
        let status = TaskStatus {
            state: TaskState::Working,
            message: None,
            timestamp: Some("2023-10-27T10:00:00Z".to_string()),
        };

        let event = TaskStatusUpdateEvent {
            kind: "status-update".to_string(),
            task_id: "task-abc".to_string(),
            context_id: "ctx-def".to_string(),
            status,
            final_event: false,
            metadata: Some(serde_json::json!({"source": "agent"})),
        };

        let json = serde_json::to_value(&event).unwrap();

        // Validate required fields
        assert_eq!(json["kind"], "status-update");
        assert_eq!(json["taskId"], "task-abc");
        assert_eq!(json["contextId"], "ctx-def");
        assert_eq!(json["final"], false);
        assert!(json["status"].is_object());
        assert_eq!(json["status"]["state"], "working");

        // Validate optional fields
        assert_eq!(json["metadata"]["source"], "agent");

        // Test round-trip serialization
        let serialized = serde_json::to_string(&event).unwrap();
        let deserialized: TaskStatusUpdateEvent = serde_json::from_str(&serialized).unwrap();
        assert_eq!(event.kind, deserialized.kind);
        assert_eq!(event.task_id, deserialized.task_id);
        assert_eq!(event.context_id, deserialized.context_id);
        assert_eq!(event.final_event, deserialized.final_event);
    }

    #[test]
    fn test_task_id_params_spec_compliance() {
        let params = TaskIdParams {
            id: "task-xyz".to_string(),
            metadata: Some(serde_json::json!({"priority": "high"})),
        };

        let json = serde_json::to_value(&params).unwrap();

        // Validate required fields
        assert_eq!(json["id"], "task-xyz");

        // Validate optional fields
        assert_eq!(json["metadata"]["priority"], "high");

        // Test round-trip serialization
        let serialized = serde_json::to_string(&params).unwrap();
        let deserialized: TaskIdParams = serde_json::from_str(&serialized).unwrap();
        assert_eq!(params.id, deserialized.id);
        assert_eq!(params.metadata, deserialized.metadata);
    }

    #[test]
    fn test_updated_task_spec_compliance() {
        let message = Message {
            kind: "message".to_string(),
            message_id: "msg-456".to_string(),
            parts: vec![Part::Text(TextPart {
                text: "Task progress update".to_string(),
                metadata: None,
            })],
            role: MessageRole::Agent,
            context_id: None,
            extensions: None,
            metadata: None,
            reference_task_ids: None,
            task_id: None,
        };

        let artifact = Artifact {
            artifact_id: "artifact-789".to_string(),
            parts: vec![Part::Text(TextPart {
                text: "Task output".to_string(),
                metadata: None,
            })],
            description: None,
            extensions: None,
            metadata: None,
            name: None,
        };

        let task = Task {
            id: "task-123".to_string(),
            kind: "task".to_string(),
            status: TaskStatus {
                state: TaskState::Completed,
                message: Some(message.clone()),
                timestamp: Some("2023-10-27T10:00:00Z".to_string()),
            },
            context_id: "ctx-456".to_string(),
            artifacts: Some(vec![artifact]),
            history: Some(vec![message]),
            metadata: Some(serde_json::json!({"priority": "normal"})),
            result: Some(serde_json::json!({"success": true})),
            error: None,
            created_at: Some("2023-10-27T09:00:00Z".to_string()),
            updated_at: Some("2023-10-27T10:00:00Z".to_string()),
            status_history: None,
        };

        let json = serde_json::to_value(&task).unwrap();

        // Validate required fields
        assert_eq!(json["id"], "task-123");
        assert_eq!(json["kind"], "task");
        assert_eq!(json["contextId"], "ctx-456");
        assert!(json["status"].is_object());
        assert_eq!(json["status"]["state"], "completed");

        // Validate optional fields
        assert!(json["artifacts"].is_array());
        assert_eq!(json["artifacts"][0]["artifactId"], "artifact-789");
        assert!(json["history"].is_array());
        assert_eq!(json["history"][0]["messageId"], "msg-456");
        assert_eq!(json["metadata"]["priority"], "normal");
        assert_eq!(json["result"]["success"], true);
        assert_eq!(json["createdAt"], "2023-10-27T09:00:00Z");
        assert_eq!(json["updatedAt"], "2023-10-27T10:00:00Z");

        // Test round-trip serialization
        let serialized = serde_json::to_string(&task).unwrap();
        let deserialized: Task = serde_json::from_str(&serialized).unwrap();
        assert_eq!(task.id, deserialized.id);
        assert_eq!(task.kind, deserialized.kind);
        assert_eq!(task.context_id, deserialized.context_id);
        assert_eq!(task.artifacts.is_some(), deserialized.artifacts.is_some());
        assert_eq!(task.history.is_some(), deserialized.history.is_some());
        assert_eq!(task.metadata, deserialized.metadata);
    }
}
