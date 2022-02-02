
pub enum GameState {
    Created,
    Running,
    ProposeDraw,
    Draw,
    Surrender,
    Win,
    Leave,
}

pub struct Player {
    // TODO
}

impl Player {
    pub fn new() -> Self {
        todo!()
    }

    pub fn get_name(&self) -> &str {
        todo!()
    }

    pub fn get_id(&self) -> &str {
        todo!()
    }
}

pub struct Game{
    // TODO
}

impl Game {
    pub fn new() -> Self {
        todo!()
    }

    pub fn get_id(&self) -> &str {
        todo!()
    }

    pub fn players(&self) -> &Vec<Player> {
        todo!()
    }
}
