#[macro_use]
extern crate manifest_dir_macros;

pub mod components;
pub mod graphics;
pub mod style;

mod autoenum;
mod interface;

pub use material_icons;

pub mod traits {
    pub use crate::autoenum::InlineWidgetAutoEnum;
    pub use crate::interface::AutoInlineWidgetMenu;
    pub use crate::interface::ConfigurationWidget;
    pub use crate::interface::InlineWidget;
    pub use crate::interface::InlineWidgetMenu;
}

mod ip;
mod primitive;
mod str;
mod timecode;

#[cfg(feature = "examples")]
pub mod demo;
