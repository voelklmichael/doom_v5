// Track 16 scaffolding: the aggregate state struct that replaces `static mut`
// globals module by module. `GAME_STATE`/`game_state()` are a transitional
// bridge for call sites not yet converted to receive `&mut GameState`
// explicitly; both are deleted once the whole codebase is converted.
// See /docs/track16-gamestate-plan.md for the full plan.
//
// GAME_STATE is lazily built via OnceLock rather than a const-evaluated
// static initializer: some module states (e.g. WFileState) need to embed
// the address of another static, which `const fn` can't do.

use std::sync::OnceLock;

use crate::src::d_event::DEventState;
use crate::src::d_iwad::DIwadState;
use crate::src::f_finale::FFinaleState;
use crate::src::i_cdmus::ICdMusState;
use crate::src::i_joystick::IJoystickState;
use crate::src::i_timer::ITimerState;
use crate::src::p_user::PUserState;
use crate::src::st_lib::StLibState;
use crate::src::statdump::StatDumpState;
use crate::src::w_checksum::WChecksumState;
use crate::src::w_file::WFileState;

pub struct GameState {
    pub d_event: DEventState,
    pub d_iwad: DIwadState,
    pub f_finale: FFinaleState,
    pub i_cdmus: ICdMusState,
    pub i_joystick: IJoystickState,
    pub i_timer: ITimerState,
    pub p_user: PUserState,
    pub st_lib: StLibState,
    pub statdump: StatDumpState,
    pub w_checksum: WChecksumState,
    pub w_file: WFileState,
}

impl GameState {
    fn new() -> Self {
        GameState {
            d_event: DEventState::new(),
            d_iwad: DIwadState::new(),
            f_finale: FFinaleState::new(),
            i_cdmus: ICdMusState::new(),
            i_joystick: IJoystickState::new(),
            i_timer: ITimerState::new(),
            p_user: PUserState::new(),
            st_lib: StLibState::new(),
            statdump: StatDumpState::new(),
            w_checksum: WChecksumState::new(),
            w_file: WFileState::new(),
        }
    }
}

static mut GAME_STATE: OnceLock<GameState> = OnceLock::new();

pub unsafe fn game_state() -> &'static mut GameState {
    let cell: &'static mut OnceLock<GameState> = &mut GAME_STATE;
    cell.get_or_init(GameState::new);
    cell.get_mut().unwrap()
}
