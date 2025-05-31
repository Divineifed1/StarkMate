use utoipa::OpenApi;
use crate::{players, games, auth, ai};
use utoipa::openapi::security::{SecurityScheme, HttpAuthScheme, HttpBuilder};
use utoipa::Modify;

// Security scheme definition for JWT authentication
pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "jwt_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .description(Some("JWT token authentication. Example: Bearer {token}"))
                        .build(),
                ),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        // Player endpoints
        players::add_player,
        players::find_player_by_id,
        players::update_player,
        players::delete_player,
        
        // Game endpoints
        games::create_game,
        games::get_game,
        games::make_move,
        games::list_games,
        games::join_game,
        games::abandon_game,
        
        // Authentication endpoints
        auth::login,
        auth::register,
        auth::refresh_token,
        auth::logout,
        
        // AI suggestion endpoints
        ai::get_ai_suggestion,
        ai::analyze_position,
    ),
    components(
        schemas(
            // Player schemas
            dto::players::NewPlayer,
            dto::players::UpdatePlayer,
            dto::players::DisplayPlayer,
            dto::players::UpdatedPlayer,
            
            // Game schemas
            dto::games::CreateGameRequest,
            dto::games::GameDisplayDTO,
            dto::games::MakeMoveRequest,
            dto::games::JoinGameRequest,
            dto::games::GameStatus,
            dto::games::GameResult,
            
            // Auth schemas
            dto::auth::LoginRequest,
            dto::auth::LoginResponse,
            dto::auth::RegisterRequest,
            dto::auth::RefreshTokenRequest,
            dto::auth::TokenResponse,
            dto::auth::UserInfo,
            
            // AI schemas
            dto::ai::AiSuggestionRequest,
            dto::ai::AiSuggestionResponse,
            dto::ai::PositionAnalysisRequest,
            dto::ai::PositionAnalysisResponse,
            dto::ai::AlternativeMove,
            
            // Response schemas
            dto::responses::PlayerAdded,
            dto::responses::PlayerFound,
            dto::responses::PlayerUpdated,
            dto::responses::PlayerDeleted,
            dto::responses::InvalidCredentialsResponse,
            dto::responses::NotFoundResponse,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Players", description = "Player management operations"),
        (name = "Games", description = "Game management operations"),
        (name = "Authentication", description = "Authentication operations"),
        (name = "AI", description = "AI suggestion operations"),
        (name = "WebSocket", description = "WebSocket communication protocol")
    ),
    info(
        title = "StarkMate Chess Platform API",
        version = "1.0.0",
        description = "API for the StarkMate chess platform built on Starknet",
        contact(
            name = "StarkMate Team",
            url = "https://starkmate.com/contact",
            email = "support@starkmate.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    )
)]
pub struct ApiDoc;

// Define WebSocket event schema documentation (can't be automatically generated with utoipa)
pub fn websocket_documentation() -> String {
    r#"
# WebSocket Protocol Documentation

## Connection

Connect to the WebSocket server:
```
ws://hostname:port/ws/game/{game_id}?token={jwt_token}
```

## Event Types

### Player Joins Game
```json
{
  "type": "join",
  "data": {
    "player_id": "uuid",
    "username": "string",
    "game_id": "uuid"
  }
}
```

### Player Leaves Game
```json
{
  "type": "leave",
  "data": {
    "player_id": "uuid",
    "game_id": "uuid"
  }
}
```

### Move Made
```json
{
  "type": "move",
  "data": {
    "player_id": "uuid",
    "game_id": "uuid",
    "move": "e2e4", 
    "fen": "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2",
    "time_remaining": 298 
  }
}
```

### Game State Update
```json
{
  "type": "state_update",
  "data": {
    "game_id": "uuid",
    "status": "in_progress | checkmate | stalemate | draw | time_forfeit",
    "current_turn": "white | black",
    "white_time_remaining": 290,
    "black_time_remaining": 300
  }
}
```

### Chat Message
```json
{
  "type": "chat",
  "data": {
    "player_id": "uuid",
    "username": "string",
    "message": "string",
    "timestamp": "ISO 8601 timestamp"
  }
}
```

## Error Messages
```json
{
  "type": "error",
  "data": {
    "code": "authentication_error | invalid_move | not_your_turn | game_not_found",
    "message": "string"
  }
}
```
"#
}
