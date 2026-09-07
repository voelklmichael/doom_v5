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

use crate::src::am_map::AmMapState;
use crate::src::d_event::DEventState;
use crate::src::d_iwad::DIwadState;
use crate::src::d_loop::DLoopState;
use crate::src::d_net::DNetState;
use crate::src::doomstat::DoomstatState;
use crate::src::f_finale::FFinaleState;
use crate::src::g_game::GGameState;
use crate::src::hu_stuff::HuStuffState;
use crate::src::i_cdmus::ICdMusState;
use crate::src::i_input::IInputState;
use crate::src::i_joystick::IJoystickState;
use crate::src::i_scale::IScaleState;
use crate::src::i_sound::ISoundState;
use crate::src::i_system::ISystemState;
use crate::src::i_timer::ITimerState;
use crate::src::m_argv::MArgvState;
use crate::src::m_config::MConfigState;
use crate::src::m_controls::MControlsState;
use crate::src::m_menu::MMenuState;
use crate::src::m_random::MRandomState;
use crate::src::p_ceilng::PCeilngState;
use crate::src::p_enemy::PEnemyState;
use crate::src::p_map::PMapState;
use crate::src::p_maputl::fixup_intercepts_overrun;
use crate::src::p_maputl::PMaputlState;
use crate::src::p_mobj::PMobjState;
use crate::src::p_plats::PPlatsState;
use crate::src::p_pspr::PPsprState;
use crate::src::p_saveg::PSavegState;
use crate::src::p_sight::PSightState;
use crate::src::p_spec::PSpecState;
use crate::src::p_switch::PSwitchState;
use crate::src::p_user::PUserState;
use crate::src::r_sky::RSkyState;
use crate::src::r_things::RThingsState;
use crate::src::sounds::SoundsState;
use crate::src::st_lib::StLibState;
use crate::src::st_stuff::StStuffState;
use crate::src::statdump::StatDumpState;
use crate::src::v_video::VVideoState;
use crate::src::w_checksum::WChecksumState;
use crate::src::w_file::WFileState;
use crate::src::w_wad::WWadState;
use crate::src::wi_stuff::WiStuffState;
use crate::src::z_zone::ZZoneState;

pub struct GameState {
    pub am_map: AmMapState,
    pub d_event: DEventState,
    pub d_iwad: DIwadState,
    pub d_loop: DLoopState,
    pub d_net: DNetState,
    pub doomstat: DoomstatState,
    pub f_finale: FFinaleState,
    pub g_game: GGameState,
    pub hu_stuff: HuStuffState,
    pub i_cdmus: ICdMusState,
    pub i_input: IInputState,
    pub i_joystick: IJoystickState,
    pub i_scale: IScaleState,
    pub i_sound: ISoundState,
    pub i_system: ISystemState,
    pub i_timer: ITimerState,
    pub m_argv: MArgvState,
    pub m_config: MConfigState,
    pub m_controls: MControlsState,
    pub m_menu: MMenuState,
    pub m_random: MRandomState,
    pub p_ceilng: PCeilngState,
    pub p_enemy: PEnemyState,
    pub p_map: PMapState,
    pub p_maputl: PMaputlState,
    pub r_sky: RSkyState,
    pub p_mobj: PMobjState,
    pub p_plats: PPlatsState,
    pub p_pspr: PPsprState,
    pub p_saveg: PSavegState,
    pub p_sight: PSightState,
    pub p_spec: PSpecState,
    pub p_switch: PSwitchState,
    pub p_user: PUserState,
    pub r_things: RThingsState,
    pub sounds: SoundsState,
    pub st_lib: StLibState,
    pub st_stuff: StStuffState,
    pub statdump: StatDumpState,
    pub v_video: VVideoState,
    pub w_checksum: WChecksumState,
    pub w_file: WFileState,
    pub w_wad: WWadState,
    pub wi_stuff: WiStuffState,
    pub z_zone: ZZoneState,
}

impl GameState {
    fn new() -> Self {
        GameState {
            am_map: AmMapState::new(),
            d_event: DEventState::new(),
            d_iwad: DIwadState::new(),
            d_loop: DLoopState::new(),
            d_net: DNetState::new(),
            doomstat: DoomstatState::new(),
            f_finale: FFinaleState::new(),
            g_game: GGameState::new(),
            hu_stuff: HuStuffState::new(),
            i_cdmus: ICdMusState::new(),
            i_input: IInputState::new(),
            i_joystick: IJoystickState::new(),
            i_scale: IScaleState::new(),
            i_sound: ISoundState::new(),
            i_system: ISystemState::new(),
            i_timer: ITimerState::new(),
            m_argv: MArgvState::new(),
            m_config: MConfigState::new(),
            m_controls: MControlsState::new(),
            m_menu: MMenuState::new(),
            m_random: MRandomState::new(),
            p_ceilng: PCeilngState::new(),
            p_enemy: PEnemyState::new(),
            p_map: PMapState::new(),
            p_maputl: PMaputlState::new(),
            r_sky: RSkyState::new(),
            p_mobj: PMobjState::new(),
            p_plats: PPlatsState::new(),
            p_pspr: PPsprState::new(),
            p_saveg: PSavegState::new(),
            p_sight: PSightState::new(),
            p_spec: PSpecState::new(),
            p_switch: PSwitchState::new(),
            p_user: PUserState::new(),
            r_things: RThingsState::new(),
            sounds: SoundsState::new(),
            st_lib: StLibState::new(),
            st_stuff: StStuffState::new(),
            statdump: StatDumpState::new(),
            v_video: VVideoState::new(),
            w_checksum: WChecksumState::new(),
            w_file: WFileState::new(),
            w_wad: WWadState::new(),
            wi_stuff: WiStuffState::new(),
            z_zone: ZZoneState::new(),
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
        fixup_intercepts_overrun(state);
        state.m_config.fixup_defaults();
        state.m_controls.fixup_weapon_keys();
        state.m_menu.fixup_menu_links();
        state.wi_stuff.fixup_anims();
    }
    state
}
