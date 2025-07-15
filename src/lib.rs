//! A2A (Agent-to-Agent) protocol implementation in Rust
//!
//! This crate provides types and functionality for working with the A2A protocol,
//! which enables communication between AI agents.
//!
//! The implementation is based on the A2A specification version 0.2.5.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// The current version of the A2A protocol implemented by this crate.
pub const PROTOCOL_VERSION: &str = "0.2.5";

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
        Self::from_str(s).ok_or_else(|| A2AError::MethodNotFound(s.to_string()))
    }
}

/// Errors that can occur when working with the A2A protocol.
#[derive(Debug, Error)]
pub enum A2AError {
    /// Error parsing JSON.
    #[error("JSON parse error: {0}")]
    JSONParse(String),

    /// Invalid request error.
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// Method not found error.
    #[error("Method not found: {0}")]
    MethodNotFound(String),

    /// Invalid parameters error.
    #[error("Invalid parameters: {0}")]
    InvalidParams(String),

    /// Internal error.
    #[error("Internal error: {0}")]
    Internal(String),

    /// Task not found error.
    #[error("Task not found: {0}")]
    TaskNotFound(String),

    /// Task not cancelable error.
    #[error("Task not cancelable: {0}")]
    TaskNotCancelable(String),

    /// Push notification not supported error.
    #[error("Push notification not supported: {0}")]
    PushNotificationNotSupported(String),

    /// Unsupported operation error.
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),

    /// Content type not supported error.
    #[error("Content type not supported: {0}")]
    ContentTypeNotSupported(String),

    /// Invalid agent response error.
    #[error("Invalid agent response: {0}")]
    InvalidAgentResponse(String),
}

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
#[serde(rename_all = "camelCase")]
pub struct ApiKeySecurityScheme {
    /// The type of security scheme.
    #[serde(rename = "type")]
    pub type_: SecuritySchemeType,
    /// The location of the API key.
    pub in_: ApiKeyLocation,
    /// The name of the header, query, or cookie parameter.
    pub name: String,
    /// Description of this security scheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// HTTP security scheme.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpSecurityScheme {
    /// The type of security scheme.
    #[serde(rename = "type")]
    pub type_: SecuritySchemeType,
    /// The name of the HTTP Authorization scheme.
    pub scheme: String,
    /// A hint to the client to identify how the bearer token is formatted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearer_format: Option<String>,
    /// Description of this security scheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// OAuth2 security scheme.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2SecurityScheme {
    /// The type of security scheme.
    #[serde(rename = "type")]
    pub type_: SecuritySchemeType,
    /// The available flows for the OAuth2 security scheme.
    pub flows: OAuth2Flows,
    /// Description of this security scheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// OAuth2 flows.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2Flows {
    /// The implicit flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit: Option<OAuth2Flow>,
    /// The password flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<OAuth2Flow>,
    /// The client credentials flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials: Option<OAuth2Flow>,
    /// The authorization code flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<OAuth2Flow>,
}

/// OAuth2 flow.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2Flow {
    /// The authorization URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_url: Option<String>,
    /// The token URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_url: Option<String>,
    /// The refresh URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    /// The available scopes for the OAuth2 security scheme.
    pub scopes: std::collections::HashMap<String, String>,
}

/// OpenID Connect security scheme.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenIdConnectSecurityScheme {
    /// The type of security scheme.
    #[serde(rename = "type")]
    pub type_: SecuritySchemeType,
    /// OpenId Connect URL to discover OAuth2 configuration values.
    pub open_id_connect_url: String,
    /// Description of this security scheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Security scheme.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum SecurityScheme {
    /// API key security scheme.
    #[serde(rename = "apiKey")]
    ApiKey(ApiKeySecurityScheme),
    /// HTTP security scheme.
    #[serde(rename = "http")]
    Http(HttpSecurityScheme),
    /// OAuth2 security scheme.
    #[serde(rename = "oauth2")]
    OAuth2(OAuth2SecurityScheme),
    /// OpenID Connect security scheme.
    #[serde(rename = "openIdConnect")]
    OpenIdConnect(OpenIdConnectSecurityScheme),
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

/// Agent capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentCapabilities {
    /// Extensions supported by this agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<AgentExtension>>,
    /// True if the agent supports task cancellation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation: Option<bool>,
    /// True if the agent supports push notifications for task status changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_notifications: Option<bool>,
    /// True if the agent exposes status change history for tasks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_history: Option<bool>,
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
#[serde(rename_all = "camelCase")]
pub struct AgentProvider {
    /// The name of the provider.
    pub name: String,
    /// The URL of the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Agent skill.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentSkill {
    /// The name of the skill.
    pub name: String,
    /// A description of the skill.
    pub description: String,
    /// Input modes supported by this skill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_modes: Option<Vec<String>>,
    /// Output modes supported by this skill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_modes: Option<Vec<String>>,
}

/// Agent card.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentCard {
    /// Human readable name of the agent.
    pub name: String,
    /// A human-readable description of the agent.
    pub description: String,
    /// The version of the agent.
    pub version: String,
    /// The version of the A2A protocol this agent supports.
    pub protocol_version: String,
    /// A URL to the address the agent is hosted at.
    pub url: String,
    /// The transport of the preferred endpoint. If empty, defaults to JSONRPC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_transport: Option<String>,
    /// Optional capabilities supported by the agent.
    pub capabilities: AgentCapabilities,
    /// The set of interaction modes that the agent supports across all skills.
    pub default_input_modes: Vec<String>,
    /// Supported media types for output.
    pub default_output_modes: Vec<String>,
    /// Skills are a unit of capability that an agent can perform.
    pub skills: Vec<AgentSkill>,
    /// The service provider of the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<AgentProvider>,
    /// A URL to documentation for the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
    /// A URL to an icon for the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// True if the agent supports providing an extended agent card when the user is authenticated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_authenticated_extended_card: Option<bool>,
    /// Announcement of additional supported transports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_interfaces: Option<Vec<AgentInterface>>,
    /// Security requirements for contacting the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<std::collections::HashMap<String, Vec<String>>>>,
    /// Security scheme details used for authenticating with this agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_schemes: Option<std::collections::HashMap<String, SecurityScheme>>,
}

/// Task status.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskStatus {
    /// The task is in progress.
    #[serde(rename = "in_progress")]
    InProgress,
    /// The task has completed successfully.
    #[serde(rename = "completed")]
    Completed,
    /// The task has failed.
    #[serde(rename = "failed")]
    Failed,
    /// The task has been canceled.
    #[serde(rename = "canceled")]
    Canceled,
}

/// Task.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    /// The ID of the task.
    pub id: String,
    /// The status of the task.
    pub status: TaskStatus,
    /// The result of the task, if completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// The error that occurred, if the task failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<A2AErrorResponse>,
    /// The time the task was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The time the task was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// The history of status changes for this task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_history: Option<Vec<TaskStatusChange>>,
}

/// Task status change.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskStatusChange {
    /// The status of the task.
    pub status: TaskStatus,
    /// The time the status changed.
    pub timestamp: String,
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

/// Message content.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageContent {
    /// The content type.
    pub content_type: String,
    /// The content.
    pub content: String,
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

/// Send message parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageParams {
    /// The message content.
    pub message: MessageContent,
    /// The skill to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill: Option<String>,
    /// The conversation ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    /// The parent message ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_message_id: Option<String>,
    /// Additional metadata.
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

/// Push notification config.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PushNotificationConfig {
    /// The URL to send push notifications to.
    pub url: String,
    /// The authentication header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_header: Option<String>,
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
    pub fn create_agent_card(
        name: String,
        description: String,
        version: String,
        url: String,
        capabilities: AgentCapabilities,
        default_input_modes: Vec<String>,
        default_output_modes: Vec<String>,
        skills: Vec<AgentSkill>,
    ) -> AgentCard {
        AgentCard {
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

    /// Create a new send message request.
    ///
    /// # Arguments
    ///
    /// * `id` - The JSON-RPC ID.
    /// * `content_type` - The content type of the message.
    /// * `content` - The content of the message.
    /// * `skill` - The skill to use (optional).
    /// * `conversation_id` - The conversation ID (optional).
    /// * `parent_message_id` - The parent message ID (optional).
    /// * `metadata` - Additional metadata (optional).
    ///
    /// # Returns
    ///
    /// A new `SendMessageRequest` with the specified parameters.
    pub fn create_send_message_request(
        id: String,
        content_type: String,
        content: String,
        skill: Option<String>,
        conversation_id: Option<String>,
        parent_message_id: Option<String>,
        metadata: Option<serde_json::Value>,
    ) -> SendMessageRequest {
        SendMessageRequest {
            method: RequestMethod::MessageSend,
            params: SendMessageParams {
                message: MessageContent {
                    content_type,
                    content,
                },
                skill,
                conversation_id,
                parent_message_id,
                metadata,
            },
            id,
            jsonrpc: "2.0".to_string(),
        }
    }

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
    pub fn create_get_task_request(id: String, task_id: String) -> GetTaskRequest {
        GetTaskRequest {
            method: RequestMethod::TasksGet,
            params: GetTaskParams { task_id },
            id,
            jsonrpc: "2.0".to_string(),
        }
    }

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
    pub fn create_cancel_task_request(id: String, task_id: String) -> CancelTaskRequest {
        CancelTaskRequest {
            method: RequestMethod::TasksCancel,
            params: CancelTaskParams { task_id },
            id,
            jsonrpc: "2.0".to_string(),
        }
    }

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
        serde_json::from_str(json).map_err(|e| A2AError::JSONParse(e.to_string()))
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
        serde_json::to_string(response).map_err(|e| A2AError::Internal(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_agent_card() {
        let card = helpers::create_agent_card(
            "Test Agent".to_string(),
            "A test agent".to_string(),
            "1.0.0".to_string(),
            "https://example.com/agent".to_string(),
            AgentCapabilities {
                extensions: None,
                cancellation: Some(true),
                push_notifications: Some(false),
                status_history: Some(true),
                streaming: Some(false),
            },
            vec!["text/plain".to_string()],
            vec!["text/plain".to_string()],
            vec![AgentSkill {
                name: "test".to_string(),
                description: "A test skill".to_string(),
                input_modes: None,
                output_modes: None,
            }],
        );

        assert_eq!(card.name, "Test Agent");
        assert_eq!(card.description, "A test agent");
        assert_eq!(card.version, "1.0.0");
        assert_eq!(card.protocol_version, PROTOCOL_VERSION);
        assert_eq!(card.url, "https://example.com/agent");
        assert_eq!(card.capabilities.cancellation, Some(true));
        assert_eq!(card.capabilities.push_notifications, Some(false));
        assert_eq!(card.capabilities.status_history, Some(true));
        assert_eq!(card.capabilities.streaming, Some(false));
        assert_eq!(card.default_input_modes, vec!["text/plain"]);
        assert_eq!(card.default_output_modes, vec!["text/plain"]);
        assert_eq!(card.skills.len(), 1);
        assert_eq!(card.skills[0].name, "test");
        assert_eq!(card.skills[0].description, "A test skill");
    }

    #[test]
    fn test_create_send_message_request() {
        let request = helpers::create_send_message_request(
            "1".to_string(),
            "text/plain".to_string(),
            "Hello, world!".to_string(),
            Some("test".to_string()),
            Some("conv1".to_string()),
            None,
            None,
        );

        assert_eq!(request.method, RequestMethod::MessageSend);
        assert_eq!(request.id, "1");
        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.params.message.content_type, "text/plain");
        assert_eq!(request.params.message.content, "Hello, world!");
        assert_eq!(request.params.skill, Some("test".to_string()));
        assert_eq!(request.params.conversation_id, Some("conv1".to_string()));
        assert_eq!(request.params.parent_message_id, None);
        assert_eq!(request.params.metadata, None);
    }

    #[test]
    fn test_create_get_task_request() {
        let request = helpers::create_get_task_request("1".to_string(), "task1".to_string());

        assert_eq!(request.method, RequestMethod::TasksGet);
        assert_eq!(request.id, "1");
        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.params.task_id, "task1");
    }

    #[test]
    fn test_create_cancel_task_request() {
        let request = helpers::create_cancel_task_request("1".to_string(), "task1".to_string());

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
}
