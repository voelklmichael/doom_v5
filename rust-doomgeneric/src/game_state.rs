// Track 16 scaffolding: the aggregate state struct that replaces `static mut`
// globals module by module. `GAME_STATE`/`game_state()` are a transitional
// bridge for call sites not yet converted to receive `&mut GameState`
// explicitly; both are deleted once the whole codebase is converted.
// See /docs/track16-gamestate-plan.md for the full plan.

pub struct GameState {}

impl GameState {
    pub const fn new() -> Self {
        GameState {}
    }
}

static mut GAME_STATE: GameState = GameState::new();

pub unsafe fn game_state() -> &'static mut GameState {
    &mut GAME_STATE
}
