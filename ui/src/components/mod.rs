pub mod numpad;
pub mod selector_list;
pub mod slider;

pub(crate) const SELECTOR_LIST_WIDTH: f32 = 250.0;
pub(crate) const SELECTOR_LIST_MIN_HEIGHT: f32 = 500.0;
pub(crate) const SELECTOR_LIST_MIN_ITEM_HEIGHT: f32 = 64.0;

pub(crate) const SQUARE_BUTTON_SIZE: f32 = 64.0;

pub use selector_list::selector_list_value;
pub use slider::big_slider;
