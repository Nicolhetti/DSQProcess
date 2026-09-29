use discord_rich_presence::{
    activity::{Activity, Assets, Button, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use std::time::{SystemTime, UNIX_EPOCH};

const CLIENT_ID: &str = "1391260707542143046";

pub struct RichPresenceManager {
    client: Option<DiscordIpcClient>,
    start_time: i64,
    last_activity: Option<String>, // Track last activity to avoid redundant updates
}

impl RichPresenceManager {
    pub fn new() -> Self {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        Self {
            client: None,
            start_time,
            last_activity: None,
        }
    }

    pub fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.client.is_some() {
            log::warn!("Rich Presence already connected");
            return Ok(());
        }

        let mut client = DiscordIpcClient::new(CLIENT_ID);

        match client.connect() {
            Ok(_) => {
                log::info!("Rich Presence connected successfully");
                self.client = Some(client);
                Ok(())
            }
            Err(e) => {
                log::error!("Failed to connect Rich Presence: {}", e);
                Err(Box::new(e))
            }
        }
    }

    pub fn disconnect(&mut self) {
        if let Some(mut client) = self.client.take() {
            match client.close() {
                Ok(_) => log::info!("Rich Presence disconnected successfully"),
                Err(e) => log::warn!("Error disconnecting Rich Presence: {}", e),
            }
        }
        self.last_activity = None;
    }

    pub fn set_activity(
        &mut self,
        game_name: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.client.is_none() {
            return Err("Rich Presence not connected".into());
        }

        // Avoid redundant updates
        if self.last_activity == game_name {
            log::debug!("Skipping redundant activity update");
            return Ok(());
        }

        let Some(client) = self.client.as_mut() else {
            return Err("Rich Presence not connected".into());
        };

        let mut activity = Activity::new()
            .timestamps(Timestamps::new().start(self.start_time))
            .assets(
                Assets::new()
                    .large_image("dsqprocess_logo")
                    .large_text("DSQProcess - Discord Quest Process"),
            )
            .buttons(vec![Button::new(
                "Ver repositorio",
                "https://github.com/Nicolhetti/DSQProcess",
            )]);

        if let Some(ref game) = game_name {
            let state_text = format!("Jugando: {}", game);
            let leaked_state: &'static str = Box::leak(state_text.into_boxed_str());
            activity = activity.state(leaked_state).details("Simulando juego");
            log::info!("Setting Rich Presence to: {}", game);
        } else {
            activity = activity.state("Esperando...").details("Sin juego activo");
            log::info!("Setting Rich Presence to idle");
        }

        match client.set_activity(activity) {
            Ok(_) => {
                self.last_activity = game_name;
                Ok(())
            }
            Err(e) => {
                log::error!("Failed to set activity: {}", e);
                self.client = None; // Mark as disconnected on error
                Err(Box::new(e))
            }
        }
    }

    pub fn clear_activity(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let Some(client) = self.client.as_mut() else {
            return Ok(());
        };

        match client.clear_activity() {
            Ok(_) => {
                log::info!("Rich Presence activity cleared");
                self.last_activity = None;
                Ok(())
            }
            Err(e) => {
                log::error!("Failed to clear activity: {}", e);
                self.client = None;
                Err(Box::new(e))
            }
        }
    }
}

impl Drop for RichPresenceManager {
    fn drop(&mut self) {
        log::debug!("Dropping RichPresenceManager");
        self.disconnect();
    }
}
