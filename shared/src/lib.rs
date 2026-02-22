use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// WebSocket message protocol shared between backend and frontend.
///
/// Uses serde tagged enum so messages serialize as `{"type": "Variant", ...}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    /// Heartbeat to keep connection alive
    Heartbeat,

    /// Error from server
    Error { message: String },

    /// Server is shutting down
    ServerShutdown {
        reason: String,
        reconnect_delay_ms: u64,
    },
}

/// Health check response from `/api/health`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
}

/// Example API item (matches the `items` database table).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
}

/// Request body for creating a new item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ws_message_heartbeat_roundtrip() {
        let msg = WsMessage::Heartbeat;
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: WsMessage = serde_json::from_str(&json).unwrap();
        assert!(matches!(parsed, WsMessage::Heartbeat));
    }

    #[test]
    fn ws_message_error_roundtrip() {
        let msg = WsMessage::Error {
            message: "something broke".to_string(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: WsMessage = serde_json::from_str(&json).unwrap();
        match parsed {
            WsMessage::Error { message } => assert_eq!(message, "something broke"),
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn ws_message_shutdown_roundtrip() {
        let msg = WsMessage::ServerShutdown {
            reason: "restarting".to_string(),
            reconnect_delay_ms: 1000,
        };
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: WsMessage = serde_json::from_str(&json).unwrap();
        match parsed {
            WsMessage::ServerShutdown {
                reason,
                reconnect_delay_ms,
            } => {
                assert_eq!(reason, "restarting");
                assert_eq!(reconnect_delay_ms, 1000);
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn item_roundtrip() {
        let item = Item {
            id: Uuid::new_v4(),
            name: "test item".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
        };
        let json = serde_json::to_string(&item).unwrap();
        let parsed: Item = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.id, item.id);
        assert_eq!(parsed.name, item.name);
    }
}
