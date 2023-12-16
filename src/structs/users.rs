
async fn get_user_vehicles() -> impl axum::response::IntoResponse {
    // Retrieve user's vehicles data from MongoDB
    let user_id = "user_id_example"; // Replace with actual user_id
    let collection = db.collection::<Vehicle>("vehicles");
    let vehicles = collection.find(doc! {"user_id": user_id}, None)
        .await.unwrap()
        .collect::<Vec<_>>()
        .await;

    // Convert vehicles data to JSON
    Json(vehicles)
}

async fn get_vehicles() -> impl axum::response::IntoResponse {
    // Retrieve vehicles data from MongoDB
    let collection = db.collection::<Vehicle>("vehicles");
    let vehicles = collection.find(None, None)
        .await.unwrap()
        .collect::<Vec<_>>()
        .await;

    // Convert vehicles data to JSON
    Json(vehicles)
}

async fn get_user_tickets() -> impl axum::response::IntoResponse {
    // Retrieve user's tickets data from MongoDB
    let user_id = "user_id_example"; // Replace with actual user_id
    let collection = db.collection::<Ticket>("tickets");
    let tickets = collection.find(doc! {"user_id": user_id}, None)
        .await.unwrap()
        .collect::<Vec<_>>()
        .await;

    // Convert tickets data to JSON
    Json(tickets)
}

async fn add_ticket(ticket: Json<Ticket>) -> impl axum::response::IntoResponse {
    // Insert ticket data into MongoDB
    let collection = db.collection::<Ticket>("tickets");
    collection.insert_one(ticket.0, None).await.unwrap();

    "Ticket added successfully"
}

async fn get_parking_lots() -> impl axum::response::IntoResponse {
    // Retrieve parking lots data from MongoDB
    let collection = db.collection::<ParkingLot>("parking_lots");
    let parking_lots = collection.find(None, None)
        .await.unwrap()
        .collect::<Vec<_>>()
        .await;

    // Convert parking lots data to JSON
    Json(parking_lots)
}

async fn add_parking_lot(parking_lot: Json<ParkingLot>) -> impl axum::response::IntoResponse {
    // Insert parking lot data into MongoDB
    let collection = db.collection::<ParkingLot>("parking_lots");
    collection.insert_one(parking_lot.0, None).await.unwrap();

    "Parking lot added successfully"
}
