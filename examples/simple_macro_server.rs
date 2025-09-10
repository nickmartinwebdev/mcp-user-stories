//! Simple demonstration of rmcp macro usage
//!
//! This example shows how rmcp macros can simplify MCP server development
//! compared to the manual approach. This is a conceptual demonstration.

use rmcp::{
    handler::server::ServerHandler,
    model::{CallToolResult, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo},
    schemars,
    service::RoleServer,
    tool, tool_router,
    transport::stdio,
    ErrorData, Json, ServiceExt,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct SimpleCounter {
    count: Arc<Mutex<i32>>,
    messages: Arc<Mutex<HashMap<String, String>>>,
    tool_router: rmcp::handler::server::tool::ToolRouter<Self>,
}

// Request/Response types with automatic schema generation
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct IncrementRequest {
    /// Amount to increment by (defaults to 1)
    pub amount: Option<i32>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SetMessageRequest {
    /// Message key
    pub key: String,
    /// Message content
    pub message: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetMessageRequest {
    /// Message key to retrieve
    pub key: String,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub struct CounterResponse {
    pub current_count: i32,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub struct MessageResponse {
    pub key: String,
    pub message: String,
}

// This would use #[tool_router] in a working implementation
impl SimpleCounter {
    pub fn new() -> Self {
        Self {
            count: Arc::new(Mutex::new(0)),
            messages: Arc::new(Mutex::new(HashMap::new())),
            tool_router: rmcp::handler::server::tool::ToolRouter::new(),
        }
    }

    // This would use #[tool] attribute in a working implementation
    /// Increment the counter by the specified amount
    pub async fn increment(
        &self,
        request: IncrementRequest,
    ) -> Result<Json<CounterResponse>, String> {
        let amount = request.amount.unwrap_or(1);
        let mut count = self.count.lock().await;
        *count += amount;

        Ok(Json(CounterResponse {
            current_count: *count,
        }))
    }

    // This would use #[tool] attribute in a working implementation
    /// Get the current counter value
    pub async fn get_count(&self) -> Result<Json<CounterResponse>, String> {
        let count = self.count.lock().await;

        Ok(Json(CounterResponse {
            current_count: *count,
        }))
    }

    // This would use #[tool] attribute in a working implementation
    /// Store a message with a key
    pub async fn set_message(
        &self,
        request: SetMessageRequest,
    ) -> Result<Json<MessageResponse>, String> {
        let mut messages = self.messages.lock().await;
        messages.insert(request.key.clone(), request.message.clone());

        Ok(Json(MessageResponse {
            key: request.key,
            message: request.message,
        }))
    }

    // This would use #[tool] attribute in a working implementation
    /// Retrieve a stored message by key
    pub async fn get_message(
        &self,
        request: GetMessageRequest,
    ) -> Result<Json<MessageResponse>, String> {
        let messages = self.messages.lock().await;

        match messages.get(&request.key) {
            Some(message) => Ok(Json(MessageResponse {
                key: request.key,
                message: message.clone(),
            })),
            None => Err(format!("Message with key '{}' not found", request.key)),
        }
    }
}

impl ServerHandler for SimpleCounter {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "Simple counter and message storage server demonstrating rmcp macro concepts"
                    .to_string(),
            ),
        }
    }

    async fn list_tools(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParam>,
        _context: rmcp::service::RequestContext<RoleServer>,
    ) -> Result<rmcp::model::ListToolsResult, ErrorData> {
        // In a macro implementation, this would be auto-generated
        Ok(rmcp::model::ListToolsResult {
            tools: vec![
                rmcp::model::Tool {
                    name: "increment".into(),
                    description: Some("Increment the counter by the specified amount".into()),
                    input_schema: std::sync::Arc::new(
                        serde_json::to_value(rmcp::schemars::schema_for!(IncrementRequest))
                            .unwrap()
                            .as_object()
                            .unwrap()
                            .clone(),
                    ),
                    annotations: None,
                    output_schema: Some(std::sync::Arc::new(
                        serde_json::to_value(rmcp::schemars::schema_for!(CounterResponse))
                            .unwrap()
                            .as_object()
                            .unwrap()
                            .clone(),
                    )),
                },
                rmcp::model::Tool {
                    name: "get_count".into(),
                    description: Some("Get the current counter value".into()),
                    input_schema: std::sync::Arc::new({
                        let mut schema = serde_json::Map::new();
                        schema.insert(
                            "type".to_string(),
                            serde_json::Value::String("object".to_string()),
                        );
                        schema.insert(
                            "properties".to_string(),
                            serde_json::Value::Object(serde_json::Map::new()),
                        );
                        schema
                    }),
                    annotations: None,
                    output_schema: Some(std::sync::Arc::new(
                        serde_json::to_value(rmcp::schemars::schema_for!(CounterResponse))
                            .unwrap()
                            .as_object()
                            .unwrap()
                            .clone(),
                    )),
                },
                rmcp::model::Tool {
                    name: "set_message".into(),
                    description: Some("Store a message with a key".into()),
                    input_schema: std::sync::Arc::new(
                        serde_json::to_value(rmcp::schemars::schema_for!(SetMessageRequest))
                            .unwrap()
                            .as_object()
                            .unwrap()
                            .clone(),
                    ),
                    annotations: None,
                    output_schema: Some(std::sync::Arc::new(
                        serde_json::to_value(rmcp::schemars::schema_for!(MessageResponse))
                            .unwrap()
                            .as_object()
                            .unwrap()
                            .clone(),
                    )),
                },
                rmcp::model::Tool {
                    name: "get_message".into(),
                    description: Some("Retrieve a stored message by key".into()),
                    input_schema: std::sync::Arc::new(
                        serde_json::to_value(rmcp::schemars::schema_for!(GetMessageRequest))
                            .unwrap()
                            .as_object()
                            .unwrap()
                            .clone(),
                    ),
                    annotations: None,
                    output_schema: Some(std::sync::Arc::new(
                        serde_json::to_value(rmcp::schemars::schema_for!(MessageResponse))
                            .unwrap()
                            .as_object()
                            .unwrap()
                            .clone(),
                    )),
                },
            ],
            next_cursor: None,
        })
    }

    async fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParam,
        _context: rmcp::service::RequestContext<RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        // In a macro implementation, this would be auto-generated routing
        match request.name.as_ref() {
            "increment" => {
                let params: IncrementRequest = serde_json::from_value(serde_json::Value::Object(
                    request.arguments.unwrap_or_default(),
                ))
                .map_err(|e| ErrorData {
                    code: rmcp::model::ErrorCode(-32602),
                    message: format!("Invalid parameters: {}", e).into(),
                    data: None,
                })?;

                match self.increment(params).await {
                    Ok(result) => Ok(CallToolResult::success(vec![rmcp::model::Content::text(
                        serde_json::to_string_pretty(&result.0).map_err(|e| ErrorData {
                            code: rmcp::model::ErrorCode(-32603),
                            message: format!("Serialization error: {}", e).into(),
                            data: None,
                        })?,
                    )])),
                    Err(e) => Err(ErrorData {
                        code: rmcp::model::ErrorCode(-32000),
                        message: e.into(),
                        data: None,
                    }),
                }
            }
            "get_count" => match self.get_count().await {
                Ok(result) => Ok(CallToolResult::success(vec![rmcp::model::Content::text(
                    serde_json::to_string_pretty(&result.0).map_err(|e| ErrorData {
                        code: rmcp::model::ErrorCode(-32603),
                        message: format!("Serialization error: {}", e).into(),
                        data: None,
                    })?,
                )])),
                Err(e) => Err(ErrorData {
                    code: rmcp::model::ErrorCode(-32000),
                    message: e.into(),
                    data: None,
                }),
            },
            "set_message" => {
                let params: SetMessageRequest = serde_json::from_value(serde_json::Value::Object(
                    request.arguments.unwrap_or_default(),
                ))
                .map_err(|e| ErrorData {
                    code: rmcp::model::ErrorCode(-32602),
                    message: format!("Invalid parameters: {}", e).into(),
                    data: None,
                })?;

                match self.set_message(params).await {
                    Ok(result) => Ok(CallToolResult::success(vec![rmcp::model::Content::text(
                        serde_json::to_string_pretty(&result.0).map_err(|e| ErrorData {
                            code: rmcp::model::ErrorCode(-32603),
                            message: format!("Serialization error: {}", e).into(),
                            data: None,
                        })?,
                    )])),
                    Err(e) => Err(ErrorData {
                        code: rmcp::model::ErrorCode(-32000),
                        message: e.into(),
                        data: None,
                    }),
                }
            }
            "get_message" => {
                let params: GetMessageRequest = serde_json::from_value(serde_json::Value::Object(
                    request.arguments.unwrap_or_default(),
                ))
                .map_err(|e| ErrorData {
                    code: rmcp::model::ErrorCode(-32602),
                    message: format!("Invalid parameters: {}", e).into(),
                    data: None,
                })?;

                match self.get_message(params).await {
                    Ok(result) => Ok(CallToolResult::success(vec![rmcp::model::Content::text(
                        serde_json::to_string_pretty(&result.0).map_err(|e| ErrorData {
                            code: rmcp::model::ErrorCode(-32603),
                            message: format!("Serialization error: {}", e).into(),
                            data: None,
                        })?,
                    )])),
                    Err(e) => Err(ErrorData {
                        code: rmcp::model::ErrorCode(-32000),
                        message: e.into(),
                        data: None,
                    }),
                }
            }
            _ => Err(ErrorData {
                code: rmcp::model::ErrorCode(-32601),
                message: "Method not found".into(),
                data: None,
            }),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = SimpleCounter::new();

    eprintln!("Simple Counter MCP Server (Macro Demonstration)");
    eprintln!("Available tools:");
    eprintln!("  - increment: Increment counter by specified amount");
    eprintln!("  - get_count: Get current counter value");
    eprintln!("  - set_message: Store a message with a key");
    eprintln!("  - get_message: Retrieve a stored message");

    let service = server.serve(stdio()).await?;
    service.waiting().await?;

    Ok(())
}
