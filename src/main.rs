// Section B: API Systems Design
use axum::{
    extract::State,
    http::StatusCode,
    Json, Router,
    routing::post,
};

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};


//Request type, 
#[derive(Deserialize)]
struct ReserveRequest{
    user_id: String
}

#[derive(Serialize)]
struct ReserveResponse{
    status: String,
    message: String,
}

//shared state
struct AppState {
    available_stocks: i32,
    reservations: HashSet<String>,
}

#[tokio::main]
async fn main() {
    //state intialization . eg : 10 items
    let shared_state = Arc::new(Mutex::new(AppState{
        available_stocks: 10,
        reservations: HashSet::new(),
    }));

    //routing 
    let app = Router::new()
        .route("/reserve", post(reserve_handler))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tokio::runtime::Handle::current().block_on(async {
        axum::serve(listener, app).await.unwrap();
    });
}

//  handlers

async fn reserve_handler(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<ReserveRequest>,
)-> (StatusCode, Json<ReserveResponse>) {
    
    let mut app_state = match state.lock(){
        Ok(gaurd) => gaurd,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ReserveResponse{
                    status: "error".to_string(),
                    message: "Internal server  lock".to_string(),
                }),
            );
    }
};

    //idempotency check
    if app_state.reservations.contains(&payload.user_id){
        return (
            StatusCode::OK,
            Json(ReserveResponse{
                status: "success".to_string(),
                message: "Already reserved".to_string(),
            }),
        );
    };

    //checking conflict
    if app_state.reservations.contains(&payload.user_id){
        return (
            StatusCode::OK,
            Json(ReserveResponse{
                status: "success".to_string(),
                message: "User already has a reservation".to_string(),
            }),
        );
    };

    // DEDUCT STOCK AND COMMIT RESERVATION
    app_state.available_stocks -= 1;
    app_state.reservations.insert(payload.user_id.clone()); 
    
    (
        StatusCode::OK,
        Json(ReserveResponse{
            status: "success".to_string(),
            message: "Reservation successful".to_string(),
        }),     
    )

};


