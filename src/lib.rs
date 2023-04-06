pub mod error_handler;
pub mod models;
pub mod ui;

pub enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
    Logs,
    WorldMap,
    Configurations,
    Quests,
    Exit,
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
