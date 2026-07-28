use crate::autoenum::InlineWidgetAutoEnum;
use std::fmt::Display;

#[derive(Clone, PartialEq)]
enum ShortEnum {
    First,
    Second,
    Third,
}

impl InlineWidgetAutoEnum for ShortEnum {
    fn options() -> Vec<Self> {
        vec![Self::First, Self::Second, Self::Third]
    }
}

impl Display for ShortEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::First => write!(f, "First"),
            Self::Second => write!(f, "Second"),
            Self::Third => write!(f, "Third"),
        }
    }
}

#[derive(Clone, PartialEq)]
enum LongEnum {
    Aardvark,
    Bear,
    Cormorant,
    Dog,
    Elephant,
    Fox,
    Giraffe,
    Horse,
    Iguana,
    Jaguar,
    Kangaroo,
    Ladybug,
    Magpie,
    Nightingale,
    Opossum,
    Panther,
    Quetzal,
    Rabbit,
    Sheep,
    Turtle,
    Unicorn,
    Vulture,
    Whale,
    Xoloitzcuintli,
    Yak,
    Zebra,
}
impl InlineWidgetAutoEnum for LongEnum {
    fn options() -> Vec<Self> {
        vec![
            Self::Aardvark,
            Self::Bear,
            Self::Cormorant,
            Self::Dog,
            Self::Elephant,
            Self::Fox,
            Self::Giraffe,
            Self::Horse,
            Self::Iguana,
            Self::Jaguar,
            Self::Kangaroo,
            Self::Ladybug,
            Self::Magpie,
            Self::Nightingale,
            Self::Opossum,
            Self::Panther,
            Self::Quetzal,
            Self::Rabbit,
            Self::Sheep,
            Self::Turtle,
            Self::Unicorn,
            Self::Vulture,
            Self::Whale,
            Self::Xoloitzcuintli,
            Self::Yak,
            Self::Zebra,
        ]
    }
}

impl Display for LongEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Aardvark => write!(f, "Aardvark"),
            Self::Bear => write!(f, "Bear"),
            Self::Cormorant => write!(f, "Cormorant"),
            Self::Dog => write!(f, "Dog"),
            Self::Elephant => write!(f, "Elephant"),
            Self::Fox => write!(f, "Fox"),
            Self::Giraffe => write!(f, "Giraffe"),
            Self::Horse => write!(f, "Horse"),
            Self::Iguana => write!(f, "Iguana"),
            Self::Jaguar => write!(f, "Jaguar"),
            Self::Kangaroo => write!(f, "Kangaroo"),
            Self::Ladybug => write!(f, "Ladybug"),
            Self::Magpie => write!(f, "Magpie"),
            Self::Nightingale => write!(f, "Nightingale"),
            Self::Opossum => write!(f, "Opossum"),
            Self::Panther => write!(f, "Panther"),
            Self::Quetzal => write!(f, "Quetzal"),
            Self::Rabbit => write!(f, "Rabbit"),
            Self::Sheep => write!(f, "Sheep"),
            Self::Turtle => write!(f, "Turtle"),
            Self::Unicorn => write!(f, "Unicorn"),
            Self::Vulture => write!(f, "Vulture"),
            Self::Whale => write!(f, "Whale"),
            Self::Xoloitzcuintli => write!(f, "Xoloitzcuintli"),
            Self::Yak => write!(f, "Yak"),
            Self::Zebra => write!(f, "Zebra"),
        }
    }
}

pub fn demo_autoenum(ui: &mut egui::Ui) {
    let mut val = ui.memory(|r| {
        r.data
            .get_temp::<(ShortEnum, LongEnum)>(ui.id())
            .unwrap_or((ShortEnum::First, LongEnum::Aardvark))
    });

    val.0.autoenum_inline_widget(ui, "Short RO");
    val.0.autoenum_inline_widget_menu(ui, "Short RW");
    val.1.autoenum_inline_widget(ui, "Long RO");
    val.1.autoenum_inline_widget_menu(ui, "Long RW");

    ui.memory_mut(|w| {
        *w.data
            .get_temp_mut_or(ui.id(), (ShortEnum::First, LongEnum::Aardvark)) = val
    });
}
