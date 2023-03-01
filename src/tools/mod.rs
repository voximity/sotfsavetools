mod kelvin;
mod virginia;

use self::{kelvin::ToolKelvin, virginia::ToolVirginia};
use crate::save::Save;
use std::fmt::Debug;

pub trait SaveTool: Debug + Clone {
    fn new(save: &Save) -> Self;
    fn render(&mut self, save: &mut Save, ui: &mut egui::Ui);
}

macro_rules! save_tools {
    ($($name:ident => $type:ty),*,) => {
        #[derive(Debug, Clone)]
        pub struct SaveTools {
            $(pub $name: $type,)+
        }

        impl SaveTools {
            pub fn new(save: &Save) -> Self {
                Self {
                    $($name: <$type>::new(save),)+
                }
            }
        }
    }
}

save_tools!(
    kelvin => ToolKelvin,
    virginia => ToolVirginia,
);
