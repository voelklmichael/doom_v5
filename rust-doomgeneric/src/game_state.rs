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
use crate::src::i_input::IInputState;
use crate::src::i_joystick::IJoystickState;
use crate::src::i_timer::ITimerState;
use crate::src::m_config::MConfigState;
use crate::src::m_random::MRandomState;
use crate::src::p_ceilng::PCeilngState;
use crate::src::p_plats::PPlatsState;
use crate::src::p_switch::PSwitchState;
use crate::src::p_user::PUserState;
use crate::src::sounds::SoundsState;
use crate::src::st_lib::StLibState;
use crate::src::statdump::StatDumpState;
use crate::src::w_checksum::WChecksumState;
use crate::src::w_file::WFileState;

pub struct GameState {
    pub d_event: DEventState,
    pub d_iwad: DIwadState,
    pub f_finale: FFinaleState,
    pub i_cdmus: ICdMusState,
    pub i_input: IInputState,
    pub i_joystick: IJoystickState,
    pub i_timer: ITimerState,
    pub m_config: MConfigState,
    pub m_random: MRandomState,
    pub p_ceilng: PCeilngState,
    pub p_plats: PPlatsState,
    pub p_switch: PSwitchState,
    pub p_user: PUserState,
    pub sounds: SoundsState,
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
            i_input: IInputState::new(),
            i_joystick: IJoystickState::new(),
            i_timer: ITimerState::new(),
            m_config: MConfigState::new(),
            m_random: MRandomState::new(),
            p_ceilng: PCeilngState::new(),
            p_plats: PPlatsState::new(),
            p_switch: PSwitchState::new(),
            p_user: PUserState::new(),
            sounds: SoundsState::new(),
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
    let just_initialized = cell.get().is_none();
    cell.get_or_init(GameState::new);
    let state = cell.get_mut().unwrap();
    if just_initialized {
        // Self-referential pointers (e.g. sounds.S_sfx's one aliased entry)
        // can only be computed once the value is at its final, permanently-
        // stable 'static address -- i.e. here, not inside any XxxState::new().
        state.sounds.fixup_self_links();
    }
    state
}
