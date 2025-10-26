use discord_rich_presence::{
    activity::{Activity, Assets, Button, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use std::time::{SystemTime, UNIX_EPOCH};

const CLIENT_ID: &str = "1391260707542143046";

pub struct RichPresenceManager {
    client: Option<DiscordIpcClient>,
    is_connected: bool,
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
            is_connected: false,
            start_time,
            last_activity: None,
        }
    }

    pub fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_connected {
            log::warn!("Rich Presence already connected");
            return Ok(());
        }

        let mut client = DiscordIpcClient::new(CLIENT_ID);

        match client.connect() {
            Ok(_) => {
                log::info!("Rich Presence connected successfully");
                self.client = Some(client);
                self.is_connected = true;
                Ok(())
            }
            Err(e) => {
                log::error!("Failed to connect Rich Presence: {}", e);
                self.is_connected = false;
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
        self.is_connected = false;
        self.last_activity = None;
    }

    pub fn is_connected(&self) -> bool {
        self.is_connected
    }

    pub fn set_activity(
        &mut self,
        game_name: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.is_connected {
            return Err("Rich Presence not connected".into());
        }

        // Avoid redundant updates
        if self.last_activity == game_name {
            log::debug!("Skipping redundant activity update");
            return Ok(());
        }

        let client = self.client.as_mut().ok_or("Client not initialized")?;

        let mut activity = Activity::new()
            .timestamps(Timestamps::new().start(self.start_time))
            .assets(
                Assets::new()
                    .large_image("dsqprocess_logo")
                    .large_text("DSQProcess - Discord Quest Process"),
            );

        // Add button
        let button = Button::new(
            "Ver repositorio",
            "https://github.com/Nicolhetti/DSQProcess",
        );
        activity = activity.buttons(vec![button]);

        // Set state based on game
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
                self.is_connected = false; // Mark as disconnected on error
                Err(Box::new(e))
            }
        }
    }

    pub fn clear_activity(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.is_connected {
            return Ok(());
        }

        if let Some(client) = &mut self.client {
            match client.clear_activity() {
                Ok(_) => {
                    log::info!("Rich Presence activity cleared");
                    self.last_activity = None;
                    Ok(())
                }
                Err(e) => {
                    log::error!("Failed to clear activity: {}", e);
                    self.is_connected = false;
                    Err(Box::new(e))
                }
            }
        } else {
            Ok(())
        }
    }

    /// Reconnect if disconnected (useful for recovery)
    #[allow(dead_code)]
    pub fn ensure_connected(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.is_connected {
            log::info!("Attempting to reconnect Rich Presence");
            self.connect()?;
        }
        Ok(())
    }
}

impl Drop for RichPresenceManager {
    fn drop(&mut self) {
        log::debug!("Dropping RichPresenceManager");
        self.disconnect();
    }
}

impl Default for RichPresenceManager {
    fn default() -> Self {
        Self::new()
    }
}
