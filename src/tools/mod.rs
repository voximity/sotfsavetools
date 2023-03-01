use crate::save::Save;

use self::kelvin::ToolKelvin;

mod kelvin;

pub trait SaveTool {
    fn new(save: &Save) -> Self;
}

macro_rules! save_tools {
    ($($name:ident => $type:ty),*,) => {
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
);
