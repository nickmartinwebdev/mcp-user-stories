use crate::{
    database::initialize_database, models::*, repositories::Repositories, services::Services,
};
use rmcp::{
    handler::server::{wrapper::Parameters, ServerHandler},
    model::{
        CallToolRequestParam, CallToolResult, ErrorCode, ErrorData as McpError, ListToolsResult,
        PaginatedRequestParam, Tool, *,
    },
    schemars,
    service::{RequestContext, RoleServer},
    transport::stdio,
    ServiceExt,
};
use serde::Deserialize;
use serde_json::{Map, Value};
use std::{borrow::Cow, sync::Arc};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct UserStoryServer {
    services: Arc<Mutex<Services>>,
}

// Request types for structured parameters
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateUserStoryParams {
    #[schemars(description = "Unique identifier for the user story")]
    pub id: String,
    #[schemars(description = "Title of the user story")]
    pub title: String,
    #[schemars(description = "Description of the user story")]
    pub description: String,
    #[schemars(description = "Persona associated with the user story")]
    pub persona: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetUserStoryParams {
    #[schemars(description = "ID of the user story to retrieve")]
    pub id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SearchUserStoriesParams {
    #[schemars(description = "Search query text")]
    pub query: String,
}

impl UserStoryServer {
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let pool = initialize_database(database_url).await?;
        let repositories = Repositories::new(pool);
        let services = Services::new(repositories);

        Ok(Self {
            services: Arc::new(Mutex::new(services)),
        })
    }

    async fn create_user_story(
        &self,
        params: Parameters<CreateUserStoryParams>,
    ) -> Result<CallToolResult, McpError> {
        let request = CreateUserStoryRequest {
            id: params.0.id,
            title: params.0.title,
            description: params.0.description,
            persona: params.0.persona,
        };

        let services = self.services.lock().await;
        match services.user_stories.create(request).await {
            Ok(story) => Ok(CallToolResult::success(vec![Content::text(
                serde_json::to_string_pretty(&story).map_err(|e| McpError {
                    code: ErrorCode(-32603),
                    message: Cow::from(format!("Serialization error: {}", e)),
                    data: None,
                })?,
            )])),
            Err(e) => Err(McpError {
                code: ErrorCode(-32000),
                message: Cow::from(e.to_string()),
                data: None,
            }),
        }
    }

    async fn get_user_story(
        &self,
        params: Parameters<GetUserStoryParams>,
    ) -> Result<CallToolResult, McpError> {
        let services = self.services.lock().await;
        match services.user_stories.get_by_id(&params.0.id).await {
            Ok(story) => Ok(CallToolResult::success(vec![Content::text(
                serde_json::to_string_pretty(&story).map_err(|e| McpError {
                    code: ErrorCode(-32603),
                    message: Cow::from(format!("Serialization error: {}", e)),
                    data: None,
                })?,
            )])),
            Err(e) => Err(McpError {
                code: ErrorCode(-32000),
                message: Cow::from(e.to_string()),
                data: None,
            }),
        }
    }

    async fn get_all_user_stories(&self) -> Result<CallToolResult, McpError> {
        let services = self.services.lock().await;
        match services.user_stories.get_all().await {
            Ok(stories) => Ok(CallToolResult::success(vec![Content::text(
                serde_json::to_string_pretty(&stories).map_err(|e| McpError {
                    code: ErrorCode(-32603),
                    message: Cow::from(format!("Serialization error: {}", e)),
                    data: None,
                })?,
            )])),
            Err(e) => Err(McpError {
                code: ErrorCode(-32000),
                message: Cow::from(e.to_string()),
                data: None,
            }),
        }
    }

    async fn search_user_stories(
        &self,
        params: Parameters<SearchUserStoriesParams>,
    ) -> Result<CallToolResult, McpError> {
        let services = self.services.lock().await;
        match services.user_stories.search(&params.0.query).await {
            Ok(stories) => Ok(CallToolResult::success(vec![Content::text(
                serde_json::to_string_pretty(&stories).map_err(|e| McpError {
                    code: ErrorCode(-32603),
                    message: Cow::from(format!("Serialization error: {}", e)),
                    data: None,
                })?,
            )])),
            Err(e) => Err(McpError {
                code: ErrorCode(-32000),
                message: Cow::from(e.to_string()),
                data: None,
            }),
        }
    }

    async fn get_statistics(&self) -> Result<CallToolResult, McpError> {
        let services = self.services.lock().await;
        match services.user_stories.get_statistics().await {
            Ok(stats) => Ok(CallToolResult::success(vec![Content::text(
                serde_json::to_string_pretty(&stats).map_err(|e| McpError {
                    code: ErrorCode(-32603),
                    message: Cow::from(format!("Serialization error: {}", e)),
                    data: None,
                })?,
            )])),
            Err(e) => Err(McpError {
                code: ErrorCode(-32000),
                message: Cow::from(e.to_string()),
                data: None,
            }),
        }
    }
}

impl ServerHandler for UserStoryServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "A comprehensive user story and acceptance criteria management system. \
                Use the available tools to create, read, update, delete, and search user stories, \
                as well as manage their acceptance criteria. Perfect for agile development teams \
                and AI-assisted project management."
                    .to_string(),
            ),
        }
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        // Helper function to create empty object schema
        let empty_object_schema = || {
            let mut schema = Map::new();
            schema.insert("type".to_string(), Value::String("object".to_string()));
            schema.insert("properties".to_string(), Value::Object(Map::new()));
            schema
        };

        Ok(ListToolsResult {
            tools: vec![
                Tool {
                    name: "create_user_story".into(),
                    description: Some(
                        "Create a new user story with ID, title, description, and persona".into(),
                    ),
                    input_schema: std::sync::Arc::new(
                        serde_json::to_value(schemars::schema_for!(CreateUserStoryParams))
                            .unwrap()
                            .as_object()
                            .unwrap()
                            .clone(),
                    ),
                    annotations: None,
                    output_schema: None,
                },
                Tool {
                    name: "get_user_story".into(),
                    description: Some("Retrieve a user story by its ID".into()),
                    input_schema: std::sync::Arc::new(
                        serde_json::to_value(schemars::schema_for!(GetUserStoryParams))
                            .unwrap()
                            .as_object()
                            .unwrap()
                            .clone(),
                    ),
                    annotations: None,
                    output_schema: None,
                },
                Tool {
                    name: "get_all_user_stories".into(),
                    description: Some("Get all user stories in the system".into()),
                    input_schema: std::sync::Arc::new(empty_object_schema()),
                    annotations: None,
                    output_schema: None,
                },
                Tool {
                    name: "search_user_stories".into(),
                    description: Some(
                        "Search user stories by text in title, description, or persona".into(),
                    ),
                    input_schema: std::sync::Arc::new(
                        serde_json::to_value(schemars::schema_for!(SearchUserStoriesParams))
                            .unwrap()
                            .as_object()
                            .unwrap()
                            .clone(),
                    ),
                    annotations: None,
                    output_schema: None,
                },
                Tool {
                    name: "get_user_stories_statistics".into(),
                    description: Some(
                        "Get statistics about user stories including counts and metrics".into(),
                    ),
                    input_schema: std::sync::Arc::new(empty_object_schema()),
                    annotations: None,
                    output_schema: None,
                },
            ],
            next_cursor: None,
        })
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        match request.name.as_ref() {
            "create_user_story" => {
                let params: CreateUserStoryParams = serde_json::from_value(
                    serde_json::Value::Object(request.arguments.unwrap_or_default()),
                )
                .map_err(|e| McpError {
                    code: ErrorCode(-32602),
                    message: Cow::from(format!("Invalid parameters: {}", e)),
                    data: None,
                })?;
                self.create_user_story(Parameters(params)).await
            }
            "get_user_story" => {
                let params: GetUserStoryParams = serde_json::from_value(serde_json::Value::Object(
                    request.arguments.unwrap_or_default(),
                ))
                .map_err(|e| McpError {
                    code: ErrorCode(-32602),
                    message: Cow::from(format!("Invalid parameters: {}", e)),
                    data: None,
                })?;
                self.get_user_story(Parameters(params)).await
            }
            "get_all_user_stories" => self.get_all_user_stories().await,
            "search_user_stories" => {
                let params: SearchUserStoriesParams = serde_json::from_value(
                    serde_json::Value::Object(request.arguments.unwrap_or_default()),
                )
                .map_err(|e| McpError {
                    code: ErrorCode(-32602),
                    message: Cow::from(format!("Invalid parameters: {}", e)),
                    data: None,
                })?;
                self.search_user_stories(Parameters(params)).await
            }
            "get_user_stories_statistics" => self.get_statistics().await,
            _ => Err(McpError {
                code: ErrorCode(-32601),
                message: Cow::from("Method not found"),
                data: None,
            }),
        }
    }
}

// Main server runner function using rmcp
pub async fn run_server(database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let server = UserStoryServer::new(database_url).await?;

    eprintln!("User Stories MCP Server started with rmcp");
    eprintln!("Database: {}", database_url);
    eprintln!("Available tools:");
    eprintln!("  - create_user_story");
    eprintln!("  - get_user_story");
    eprintln!("  - get_all_user_stories");
    eprintln!("  - search_user_stories");
    eprintln!("  - get_user_stories_statistics");

    let service = server.serve(stdio()).await?;
    service.waiting().await?;

    Ok(())
}
