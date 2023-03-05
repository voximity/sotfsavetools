use crate::save::Save;

use super::SaveTool;

#[derive(Debug, Clone, Default)]
pub struct ToolInventory {}

impl SaveTool for ToolInventory {
    fn new(save: &Save) -> Self {
        Self {}
    }

    fn render(&mut self, save: &mut Save, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            for item in &save
                .inventory
                .data
                .player_inventory
                .item_instance_manager_data
                .item_blocks
            {
                ui.label(format!("{} x{}", item.item_id, item.total_count));
            }
        });
    }
}
