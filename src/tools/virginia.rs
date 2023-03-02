use egui::Button;

use crate::save::{ActorStats, Save};

use super::SaveTool;

#[derive(Debug, Clone, Default)]
pub struct ToolVirginia {
    is_dead: bool,
}

impl SaveTool for ToolVirginia {
    fn new(save: &Save) -> Self {
        let mut tool = Self::default();
        tool.fetch_is_dead(save);
        tool
    }

    fn render(&mut self, save: &mut Save, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui
                .add_enabled(self.is_dead, Button::new("Resurrect"))
                .clicked()
            {
                self.resurrect(save);
            }

            if !self.is_dead {
                ui.label("Virginia is not dead.");
            }
        });
    }
}

impl ToolVirginia {
    pub fn fetch_is_dead(&mut self, save: &Save) {
        if save.game_state.data.game_state.is_virginia_dead {
            self.is_dead = true;
            return;
        }

        if let Some(virginia) = save.actor(10) {
            if virginia.state == 6 {
                self.is_dead = true;
                return;
            }

            if let Some(ActorStats { health, .. }) = virginia.stats {
                if health.is_nan() || health <= 0.0 {
                    self.is_dead = true;
                    return;
                }
            }
        }

        if let Some(kill) = save.kill_stat(10) {
            if kill.player_killed != 0 {
                self.is_dead = true;
                return;
            }
        }

        self.is_dead = false;
    }

    pub fn resurrect(&mut self, save: &mut Save) {
        // set game state flag
        save.game_state.data.game_state.is_virginia_dead = false;

        // find virginia's actor
        if let Some(virginia) = save.actor_mut(10) {
            virginia.state = 2;
            if let Some(stats) = &mut virginia.stats {
                stats.health = 120.0;
            }
        }

        // remove the player killed stat, if any
        if let Some(kill) = save.kill_stat_mut(10) {
            kill.player_killed = 0;
        }

        self.is_dead = false;
    }
}
