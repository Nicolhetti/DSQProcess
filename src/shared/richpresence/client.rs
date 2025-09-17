use discord_rich_presence::{
    activity::{ Activity, Assets, Timestamps },
    DiscordIpc,
    DiscordIpcClient,
};
use std::time::{ SystemTime, UNIX_EPOCH };

const CLIENT_ID: &str = "1391260707542143046";

pub struct RichPresenceManager {
    client: Option<DiscordIpcClient>,
    is_connected: bool,
    start_time: i64,
}

impl RichPresenceManager {
    pub fn new() -> Self {
        let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

        Self {
            client: None,
            is_connected: false,
            start_time,
        }
    }

    pub fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = DiscordIpcClient::new(CLIENT_ID);
        client.connect()?;
        self.client = Some(client);
        self.is_connected = true;
        Ok(())
    }

    pub fn disconnect(&mut self) {
        if let Some(mut client) = self.client.take() {
            let _ = client.close();
        }
        self.is_connected = false;
    }

    pub fn is_connected(&self) -> bool {
        self.is_connected
    }

    pub fn set_activity(
        &mut self,
        game_name: Option<String>
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(client) = &mut self.client {
            let mut activity = Activity::new()
                .details("Simulando juego")
                .timestamps(Timestamps::new().start(self.start_time))
                .assets(
                    Assets::new()
                        .large_image("dsqprocess_logo")
                        .large_text("DSQProcess - Discord Quest Process")
                );

            // TODO: Botones comentados temporalmente - no funcionan correctamente
            // let button = Button::new("Ver repositorio", "https://github.com/Nicolhetti/DSQProcess");
            // activity = activity.buttons(vec![button]);

            if let Some(game) = game_name {
                let state_text = format!("Jugando: {}", game);
                activity = activity.state(&state_text);
                client.set_activity(activity)?;
            } else {
                activity = activity.state("Esperando...");
                client.set_activity(activity)?;
            }
        }
        Ok(())
    }

    pub fn clear_activity(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(client) = &mut self.client {
            client.clear_activity()?;
        }
        Ok(())
    }
}

impl Drop for RichPresenceManager {
    fn drop(&mut self) {
        self.disconnect();
    }
}
