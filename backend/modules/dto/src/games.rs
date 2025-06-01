use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::{Validate, ValidationError};
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use regex::Regex;

// Define a regex for validating chess moves in algebraic notation
static CHESS_MOVE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-h][1-8][a-h][1-8][qrbnQRBN]?$").unwrap()
});

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum PlayerColor {
    #[serde(rename = "white")]
    White,
    #[serde(rename = "black")]
    Black,
    #[serde(rename = "random")]
    Random,
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum GameStatus {
    #[serde(rename = "waiting")]
    Waiting,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "aborted")]
    Aborted,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum GameResult {
    #[serde(rename = "white_win")]
    WhiteWin,
    #[serde(rename = "black_win")]
    BlackWin,
    #[serde(rename = "draw")]
    Draw,
    #[serde(rename = "in_progress")]
    InProgress,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateGameRequest {
    #[validate(range(min = 60, max = 7200, message = "Time control must be between 1 minute and 2 hours"))]
    pub time_control: i32,
    
    #[validate(range(min = 0, max = 60, message = "Increment must be between 0 and 60 seconds"))]
    pub increment: i32,
    
    pub player_color: Option<PlayerColor>,
    pub opponent_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GameDisplayDTO {
    #[schema(value_type = String, format = "uuid", example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: Uuid,
    
    #[schema(value_type = String, format = "uuid", example = "123e4567-e89b-12d3-a456-426614174001")]
    pub white_player_id: Uuid,
    
    #[schema(value_type = Option<String>, format = "uuid", example = "123e4567-e89b-12d3-a456-426614174002")]
    pub black_player_id: Option<Uuid>,
    
    pub status: GameStatus,
    pub result: GameResult,
    
    #[schema(example = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")]
    pub current_fen: String,
    
    pub move_history: Vec<String>,
    pub time_control: i32,
    pub increment: i32,
    pub white_time_remaining: i32,
    pub black_time_remaining: i32,
    
    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
    
    #[schema(value_type = Option<String>, format = "date-time")]
    pub started_at: Option<DateTime<Utc>>,
    
    #[schema(value_type = String, format = "date-time")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct MakeMoveRequest {
    #[validate(regex(
        path = "CHESS_MOVE_REGEX",
        message = "Move must be in valid algebraic notation (e.g., 'e2e4', 'g7g8q')"
    ))]
    #[schema(example = "e2e4")]
    pub chess_move: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct JoinGameRequest {
    #[validate(custom = "validate_uuid")]
    #[schema(value_type = String, format = "uuid", example = "123e4567-e89b-12d3-a456-426614174000")]
    pub player_id: Uuid,
}

// UUID validation function
pub fn validate_uuid(uuid: &Uuid) -> Result<(), ValidationError> {
    if uuid.is_nil() {
        return Err(ValidationError::new("Nil UUID is not allowed"));
    }
    Ok(())
}
