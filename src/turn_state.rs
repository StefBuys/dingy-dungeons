#[derive(Copy, Clone, Debug, PartialEq)]

pub enum TurnState {
    GameOver,
    Victory,
    Active,
    NextLevel,
}
