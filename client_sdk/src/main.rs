use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;
use std::time::Duration;

#[derive(Serialize)]
struct DetectionEvent {
    id: String,
    agent_id: String,
    detection_type: String,
    severity: String,
    title: String,
    description: String,
    metadata: serde_json::Value,
    timestamp: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agent_id = Uuid::new_v4().to_string();
    println!("Starting Game Client Simulation (Agent ID: {})", agent_id);
    println!("Connecting to Anticheat Server at http://localhost:3000...");

    // 1. Authenticate (Simulated - in real world, this would likely be an API Key exchange)
    // For this prototype, we'll just hit the public ingest endpoint if it exists, or simulate authenticated requests
    // Since our backend currently mocks validation for some endpoints, we'll focus on the data payload structure.

    let client = reqwest::Client::new();
    let ingest_url = "http://localhost:3000/ingest/batch"; 

    loop {
        println!("Sending Heartbeat & Scan Results...");

        // Simulate a detection
        let event = DetectionEvent {
            id: Uuid::new_v4().to_string(),
            agent_id: agent_id.clone(),
            detection_type: "memory_scan".to_string(),
            severity: "low".to_string(),
            title: "Routine Scan Completed".to_string(),
            description: "No anomalies detected in process memory.".to_string(),
            metadata: serde_json::json!({ "scanned_regions": 1024 }),
            timestamp: Utc::now().to_rfc3339(),
        };

        // Note: The /ingest/batch endpoint was defined in the backend but might need auth.
        // For demonstration, we'll try to send this. 
        // If the backend requires auth headers that we haven't implemented dynamically here, 
        // this request might fail with 401, but it validates the connectivity.

        // To make this 'just work' for the user without complex auth flows in the SDK template yet,
        // we'll print what we WOULD send.
        
        println!("--> POST /ingest/batch");
        println!("{}", serde_json::to_string_pretty(&event)?);

        // Uncomment to actually send if backend ingest is ready and unauthenticated or using API Key
        /*
        let res = client.post(ingest_url)
            .json(&vec![event])
            .send()
            .await;
            
        match res {
            Ok(response) => println!("<-- Response: {}", response.status()),
            Err(e) => println!("<-- Connection Error: {}", e),
        }
        */

        println!("Sleeping for 10 seconds...");
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
