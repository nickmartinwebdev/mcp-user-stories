use crate::{
    database::initialize_database,
    models::*,
    repositories::Repositories,
    services::{user_story_service::UserStoryStatistics, Services},
};
use rmcp::{
    handler::server::{tool::ToolRouter, wrapper::Parameters, ServerHandler},
    model::{
        CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo,
    },
    schemars,
    service::RoleServer,
    tool, tool_router,
    transport::stdio,
    ErrorData, ServiceExt,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct UserStoryServer {
    services: Arc<Mutex<Services>>,
    tool_router: ToolRouter<Self>,
}

// Request types for structured parameters
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateUserStoryParams {
    /// Unique identifier for the user story
    pub id: String,
    /// Title of the user story
    pub title: String,
    /// Description of the user story
    pub description: String,
    /// Persona associated with the user story
    pub persona: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetUserStoryParams {
    /// ID of the user story to retrieve
    pub id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SearchUserStoriesParams {
    /// Search query text
    pub query: String,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub struct UserStoryResponse {
    pub id: String,
    pub title: String,
    pub description: String,
    pub persona: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, schemars::JsonSchema)]
pub struct StatisticsResponse {
    pub total_stories: i64,
    pub stories_by_persona: Vec<(String, i64)>,
}

impl From<UserStory> for UserStoryResponse {
    fn from(story: UserStory) -> Self {
        Self {
            id: story.id,
            title: story.title,
            description: story.description,
            persona: story.persona,
            created_at: story.created_at.to_string(),
            updated_at: story.updated_at.to_string(),
        }
    }
}

impl From<UserStoryStatistics> for StatisticsResponse {
    fn from(stats: UserStoryStatistics) -> Self {
        Self {
            total_stories: stats.total_stories,
            stories_by_persona: stats.stories_by_persona.into_iter().collect(),
        }
    }
}

#[tool_router]
impl UserStoryServer {
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let pool = initialize_database(database_url).await?;
        let repositories = Repositories::new(pool);
        let services = Services::new(repositories);

        Ok(Self {
            services: Arc::new(Mutex::new(services)),
            tool_router: Self::tool_router(),
        })
    }

    #[tool(description = "Create a new user story with ID, title, description, and persona")]
    async fn create_user_story(
        &self,
        params: Parameters<CreateUserStoryParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let request = CreateUserStoryRequest {
            id: params.0.id,
            title: params.0.title,
            description: params.0.description,
            persona: params.0.persona,
        };

        let services = self.services.lock().await;
        match services.user_stories.create(request).await {
            Ok(story) => {
                let response: UserStoryResponse = story.into();
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&response).unwrap(),
                )]))
            }
            Err(e) => Err(ErrorData {
                code: rmcp::model::ErrorCode(-32000),
                message: e.to_string().into(),
                data: None,
            }),
        }
    }

    #[tool(description = "Retrieve a user story by its ID")]
    async fn get_user_story(
        &self,
        params: Parameters<GetUserStoryParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let services = self.services.lock().await;
        match services.user_stories.get_by_id(&params.0.id).await {
            Ok(story) => {
                let response: UserStoryResponse = story.into();
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&response).unwrap(),
                )]))
            }
            Err(e) => Err(ErrorData {
                code: rmcp::model::ErrorCode(-32000),
                message: e.to_string().into(),
                data: None,
            }),
        }
    }

    #[tool(description = "Get all user stories in the system")]
    async fn get_all_user_stories(&self) -> Result<CallToolResult, ErrorData> {
        let services = self.services.lock().await;
        match services.user_stories.get_all().await {
            Ok(stories) => {
                let responses: Vec<UserStoryResponse> =
                    stories.into_iter().map(|s| s.into()).collect();
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&responses).unwrap(),
                )]))
            }
            Err(e) => Err(ErrorData {
                code: rmcp::model::ErrorCode(-32000),
                message: e.to_string().into(),
                data: None,
            }),
        }
    }

    #[tool(description = "Search user stories by text in title, description, or persona")]
    async fn search_user_stories(
        &self,
        params: Parameters<SearchUserStoriesParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let services = self.services.lock().await;
        match services.user_stories.search(&params.0.query).await {
            Ok(stories) => {
                let responses: Vec<UserStoryResponse> =
                    stories.into_iter().map(|s| s.into()).collect();
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&responses).unwrap(),
                )]))
            }
            Err(e) => Err(ErrorData {
                code: rmcp::model::ErrorCode(-32000),
                message: e.to_string().into(),
                data: None,
            }),
        }
    }

    #[tool(description = "Get statistics about user stories including counts and metrics")]
    async fn get_user_stories_statistics(&self) -> Result<CallToolResult, ErrorData> {
        let services = self.services.lock().await;
        match services.user_stories.get_statistics().await {
            Ok(stats) => {
                let response: StatisticsResponse = stats.into();
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&response).unwrap(),
                )]))
            }
            Err(e) => Err(ErrorData {
                code: rmcp::model::ErrorCode(-32000),
                message: e.to_string().into(),
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
        _request: Option<rmcp::model::PaginatedRequestParam>,
        _context: rmcp::service::RequestContext<RoleServer>,
    ) -> Result<rmcp::model::ListToolsResult, ErrorData> {
        Ok(rmcp::model::ListToolsResult {
            tools: self.tool_router.list_all(),
            next_cursor: None,
        })
    }

    async fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParam,
        context: rmcp::service::RequestContext<RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        use rmcp::handler::server::tool::ToolCallContext;
        let ctx = ToolCallContext::new(self, request, context);
        self.tool_router.call(ctx).await
    }
}

/// Main server runner function using rmcp
pub async fn run_server(database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let server = UserStoryServer::new(database_url).await?;

    eprintln!("User Stories MCP Server started");
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
