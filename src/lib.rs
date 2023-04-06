pub mod error_handler;
pub mod models;
pub mod ui;

pub enum Event<I> {
    Input(I),
    Tick,
}

/// This enum is for the items that the menu in the top of the app will contain
#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
    Configurations,
    Exit,
    Logs,
    Quests,
    WorldMap,
}

impl From<MenuItem> for usize {
    fn from(value: MenuItem) -> Self {
        match value {
            MenuItem::Logs => 0,
            MenuItem::WorldMap => 1,
            MenuItem::Configurations => 2,
            MenuItem::Quests => 3,
            MenuItem::Exit => 4,
        }
    }
}

/// This enum is for the items that the menu in the bottom of the app will contain
#[derive(Copy, Clone, Debug)]
pub enum InfoItem {
    Army,
    Food,
    Gold,
    Iron,
    InvPoints,
    Poblation,
    Water,
    Wood,
}

impl From<InfoItem> for usize {
    fn from(value: InfoItem) -> Self {
        match value {
            InfoItem::Poblation => 0,
            InfoItem::Food => 1,
            InfoItem::Water => 2,
            InfoItem::Wood => 3,
            InfoItem::Iron => 4,
            InfoItem::Gold => 5,
            InfoItem::InvPoints => 6,
            InfoItem::Army => 7,
        }
    }
}
