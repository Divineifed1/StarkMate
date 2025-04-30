use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::models::*;
use super::service::{MatchmakingService};

#[derive(Debug, Deserialize)]
pub struct JoinQueueRequest {
    pub wallet_address: String,
    pub elo: u32,
    pub match_type: MatchType,
    pub invite_address: Option<String>,
    pub max_elo_diff: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct AcceptInviteRequest {
    pub wallet_address: String,
    pub elo: u32,
    pub inviter_request_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct CancelRequest {
    pub request_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct StatusResponse {
    pub status: String,
    pub queue_status: Option<QueueStatus>,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/matchmaking")
            .route("/join", web::post().to(join_queue))
            .route("/status/{request_id}", web::get().to(get_status))
            .route("/cancel", web::post().to(cancel_request))
            .route("/accept-invite", web::post().to(accept_invite))
            .route("/match/{match_id}", web::get().to(get_match)),
    );
}

async fn join_queue(
    service: web::Data<MatchmakingService>,
    req: web::Json<JoinQueueRequest>,
) -> impl Responder {
    let request_id = Uuid::new_v4();

    let player = Player {
        wallet_address: req.wallet_address.clone(),
        elo: req.elo,
        join_time: Utc::now(),
    };

    let match_request = MatchRequest {
        id: request_id,
        player,
        match_type: req.match_type.clone(),
        invite_address: req.invite_address.clone(),
        max_elo_diff: req.max_elo_diff,
    };

    let response = service.join_queue(match_request);
    HttpResponse::Ok().json(response)
}

async fn get_status(
    service: web::Data<MatchmakingService>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let request_id = path.into_inner();

    if let Some(status) = service.get_queue_status(request_id) {
        HttpResponse::Ok().json(StatusResponse {
            status: "In queue".to_string(),
            queue_status: Some(status),
        })
    } else {
        HttpResponse::NotFound().json(StatusResponse {
            status: "Request not found".to_string(),
            queue_status: None,
        })
    }
}

async fn cancel_request(
    service: web::Data<MatchmakingService>,
    req: web::Json<CancelRequest>,
) -> impl Responder {
    let success = service.cancel_request(req.request_id);

    if success {
        HttpResponse::Ok().json(serde_json::json!({
            "status": "Request cancelled successfully"
        }))
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "status": "Request not found"
        }))
    }
}

async fn accept_invite(
    service: web::Data<MatchmakingService>,
    req: web::Json<AcceptInviteRequest>,
) -> impl Responder {
    let player = Player {
        wallet_address: req.wallet_address.clone(),
        elo: req.elo,
        join_time: Utc::now(),
    };

    match service.accept_private_invite(req.inviter_request_id, player) {
        Some(response) => HttpResponse::Ok().json(response),
        None => HttpResponse::NotFound().json(serde_json::json!({
            "status": "Invite not found"
        })),
    }
}

async fn get_match(
    service: web::Data<MatchmakingService>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let match_id = path.into_inner();

    if let Some(match_data) = service.get_match(match_id) {
        HttpResponse::Ok().json(match_data)
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "status": "Match not found"
        }))
    }
}
