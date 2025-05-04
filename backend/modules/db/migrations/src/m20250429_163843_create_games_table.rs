use sea_orm_migration::{prelude::*, schema::*};
// Import Player Iden from the player creation migration
use super::m20250428_121011_create_players_table::Player;
use sea_orm_migration::prelude::ForeignKeyAction; // Import ForeignKeyAction

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Ensure the schema exists
        manager
            .create_schema(Schema::new("smdb").if_not_exists())
            .await?;

        // Create the game table within the smdb schema
        manager
            .create_table(
                Table::create()
                    .table((Smdb, Game::Table))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Game::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Game::WhitePlayer).uuid().not_null())
                    .col(ColumnDef::new(Game::BlackPlayer).uuid().not_null())
                    .col(ColumnDef::new(Game::Fen).text().not_null())
                    .col(
                        ColumnDef::new(Game::Pgn)
                            .json_binary()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Game::Result).string().not_null())
                    .col(ColumnDef::new(Game::Variant).string().not_null())
                    .col(
                        ColumnDef::new(Game::StartedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Game::DurationSec).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_white_player")
                            .from(Game::Table, Game::WhitePlayer)
                            .to(Player::Table, Player::Id)
                            .on_delete(ForeignKeyAction::Cascade) // Add Cascade
                            .on_update(ForeignKeyAction::Cascade), // Add Cascade
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_game_black_player")
                            .from(Game::Table, Game::BlackPlayer)
                            .to(Player::Table, Player::Id)
                            .on_delete(ForeignKeyAction::Cascade) // Add Cascade
                            .on_update(ForeignKeyAction::Cascade), // Add Cascade
                    )
                    .to_owned(),
            )
            .await?;

        // Add CHECK constraint using raw SQL
        manager
            .get_connection()
            .execute_unprepared(
                r#"ALTER TABLE "game" ADD CONSTRAINT "check_game_result" CHECK ("result" IN ('white', 'black', 'draw'))"#,
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_games_started_at")
                    .table((Smdb, Game::Table))
                    .col(Game::StartedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_games_variant")
                    .table((Smdb, Game::Table))
                    .col(Game::Variant)
                    .to_owned(),
            )
            .await?;

        // Create GIN index using raw SQL
        manager
            .get_connection()
            .execute_unprepared(r#"CREATE INDEX "idx_games_pgn_gin" ON "game" USING GIN ("pgn")"#)
            .await?;

        println!("Game table created successfully.");
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop indexes (including GIN)
        manager
            .drop_index(Index::drop().name("idx_games_started_at").table((Smdb, Game::Table)).to_owned())
            .await?;
        manager
            .drop_index(Index::drop().name("idx_games_variant").table((Smdb, Game::Table)).to_owned())
            .await?;
        manager
            .get_connection()
            .execute_unprepared(r#"DROP INDEX IF EXISTS "idx_games_pgn_gin""#)
            .await?;

        // Drop CHECK constraint (might need specific syntax depending on DB)
        // Assuming PostgreSQL:
        manager
            .get_connection()
            .execute_unprepared(r#"ALTER TABLE "game" DROP CONSTRAINT IF EXISTS "check_game_result""#)
            .await?;

        // Drop Foreign Keys (use the names defined in `up`)
        manager
            .drop_foreign_key(ForeignKey::drop().name("fk_game_white_player").table((Smdb, Game::Table)).to_owned())
            .await?;
        manager
            .drop_foreign_key(ForeignKey::drop().name("fk_game_black_player").table((Smdb, Game::Table)).to_owned())
            .await?;

        // Drop the table
        manager
            .drop_table(Table::drop().table((Smdb, Game::Table)).to_owned())
            .await?;
        println!("Game table dropped successfully.");
        Ok(())
    }
}

// Define the Game table structure for use within this migration
#[derive(DeriveIden)]
enum Game {
    Table,
    Id,
    WhitePlayer,
    BlackPlayer,
    Fen,
    Pgn,
    Result,
    Variant,
    StartedAt,
    DurationSec,
}

// Define the schema identifier
#[derive(DeriveIden)]
struct Smdb; 