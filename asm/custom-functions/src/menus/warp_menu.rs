use crate::game::reloader;
use crate::game::stage_info::*;
use crate::menus::main_menu::MainMenu;
use crate::menus::simple_menu::SimpleMenu;
use crate::system::button::*;

const NUM_ENTRIES_MAIN: u8 = 7;

#[derive(Copy, Clone, PartialEq, Eq)]
enum WarpState {
    Off,
    Main,
    Stage,
    Details,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum WarpStage {
    Sky,
    Faron,
    Eldin,
    Lanayru,
    SealedGrounds,
    Dungeon,
    SilentRealm,
    None,
}

impl WarpStage {
    fn from_idx(idx: usize) -> Self {
        match idx {
            0 => WarpStage::Sky,
            1 => WarpStage::Faron,
            2 => WarpStage::Eldin,
            3 => WarpStage::Lanayru,
            4 => WarpStage::SealedGrounds,
            5 => WarpStage::Dungeon,
            6 => WarpStage::SilentRealm,
            _ => WarpStage::None,
        }
    }
    fn get_name(&self) -> &'static str {
        match self {
            WarpStage::Sky => THE_SKY.name,
            WarpStage::Faron => FARON.name,
            WarpStage::Eldin => ELDIN.name,
            WarpStage::Lanayru => LANAYRU.name,
            WarpStage::SealedGrounds => SEALED_GROUNDS.name,
            WarpStage::Dungeon => DUNGEONS.name,
            WarpStage::SilentRealm => SILENT_REALMS.name,
            WarpStage::None => "None",
        }
    }

    fn get_num_stages(&self) -> u8 {
        match self {
            WarpStage::Sky => THE_SKY.stages.len(),
            WarpStage::Faron => FARON.stages.len(),
            WarpStage::Eldin => ELDIN.stages.len(),
            WarpStage::Lanayru => LANAYRU.stages.len(),
            WarpStage::SealedGrounds => SEALED_GROUNDS.stages.len(),
            WarpStage::Dungeon => DUNGEONS.stages.len(),
            WarpStage::SilentRealm => SILENT_REALMS.stages.len(),
            WarpStage::None => 0,
        }
        .try_into()
        .unwrap()
    }

    fn get_stage_info(&self, idx: u8) -> StageInfo {
        match self {
            WarpStage::Sky => THE_SKY.stages[idx as usize],
            WarpStage::Faron => FARON.stages[idx as usize],
            WarpStage::Eldin => ELDIN.stages[idx as usize],
            WarpStage::Lanayru => LANAYRU.stages[idx as usize],
            WarpStage::SealedGrounds => SEALED_GROUNDS.stages[idx as usize],
            WarpStage::Dungeon => DUNGEONS.stages[idx as usize],
            WarpStage::SilentRealm => SILENT_REALMS.stages[idx as usize],
            WarpStage::None => StageInfo::default(),
        }
    }

    fn get_stage_name(&self, idx: u8) -> &'static str {
        self.get_stage_info(idx).name
    }

    fn get_stage_pretty_name(&self, idx: u8) -> &'static str {
        self.get_stage_info(idx).pretty_name
    }
}

pub struct WarpMenu {
    state:             WarpState,
    stage_state:       WarpStage,
    stage_selected:    [u8; 8],
    main_cursor:       u8,
    stage_cursor:      u8,
    detail_cursor:     u8,
    selected_room:     u8,
    selected_layer:    u8,
    selected_entrance: u8,
}

impl WarpMenu {
    fn get_room(&self) -> u8 {
        self.stage_state.get_stage_info(self.stage_cursor).rooms[self.selected_room as usize]
    }
    fn get_layer(&self) -> u8 {
        self.stage_state.get_stage_info(self.stage_cursor).layers[self.selected_layer as usize]
    }
    fn get_entrance(&self) -> u8 {
        self.selected_entrance
    }
    fn warp(&mut self) {
        let stage_name = self.stage_state.get_stage_name(self.stage_cursor);
        for n in 0..8 {
            self.stage_selected[n] = if n < stage_name.len() {
                stage_name.as_bytes()[n] as u8
            } else {
                0
            };
        }
        let room = self.get_room();
        let layer = self.get_layer();
        let entrance = self.get_entrance();
        let forced_night: u8 = match self.stage_state {
            WarpStage::Sky => {
                if layer % 2 == 0 {
                    0
                } else {
                    1
                }
            },
            _ => 0,
        };
        let forced_trial: u8 = match self.stage_state {
            WarpStage::SilentRealm => 1,
            _ => 0,
        };
        let transition_type = 0;
        reloader::trigger_entrance(
            self.stage_selected.as_ptr(),
            room,
            layer,
            entrance,
            forced_night,
            forced_trial,
            transition_type,
            0xF,  // transition_fade_frames:  u8
            0xFF, // param_9: u8
        );
        reloader::set_reload_trigger(5);
    }

    fn change_room(&mut self, num: i8) {
        let num_rooms = self
            .stage_state
            .get_stage_info(self.stage_cursor)
            .rooms
            .len();
        self.selected_room =
            (self.selected_room as i8 + num_rooms as i8 + num) as u8 % num_rooms as u8;
    }
    fn change_layer(&mut self, num: i8) {
        let num_layers = self
            .stage_state
            .get_stage_info(self.stage_cursor)
            .layers
            .len();
        self.selected_layer =
            (self.selected_layer as i8 + num_layers as i8 + num) as u8 % num_layers as u8;
    }
    fn change_entrance(&mut self, num: i8) {
        self.selected_entrance = (self.selected_entrance as i8 + num) as u8;
    }
}

#[link_section = "data"]
#[no_mangle]
static mut WARP_MENU: WarpMenu = WarpMenu {
    state:             WarpState::Off,
    stage_state:       WarpStage::None,
    stage_selected:    [0u8; 8],
    main_cursor:       0,
    stage_cursor:      0,
    detail_cursor:     0,
    selected_room:     0,
    selected_layer:    0,
    selected_entrance: 0,
};

impl WarpMenu {
    pub fn enable() {
        unsafe { WARP_MENU.state = WarpState::Main };
    }

    pub fn input() -> bool {
        let b_pressed = is_pressed(B);
        let a_pressed = is_pressed(A);
        let up_pressed = is_pressed(DPAD_UP);
        let down_pressed = is_pressed(DPAD_DOWN);
        let right_pressed = is_pressed(DPAD_RIGHT);
        let left_pressed = is_pressed(DPAD_LEFT);

        let mut next_state = unsafe { WARP_MENU.state };

        match next_state {
            WarpState::Off => {},
            WarpState::Main => {
                if b_pressed {
                    next_state = WarpState::Off;
                } else if a_pressed {
                    next_state = WarpState::Stage;
                    unsafe {
                        WARP_MENU.stage_state = WarpStage::from_idx(WARP_MENU.main_cursor.into());
                        if WARP_MENU.stage_cursor >= WARP_MENU.stage_state.get_num_stages() {
                            WARP_MENU.stage_cursor = 0;
                        }
                    }
                } else if up_pressed {
                    unsafe {
                        WARP_MENU.main_cursor =
                            (WARP_MENU.main_cursor + NUM_ENTRIES_MAIN - 1) % NUM_ENTRIES_MAIN;
                    }
                } else if down_pressed {
                    unsafe {
                        WARP_MENU.main_cursor = (WARP_MENU.main_cursor + 1) % NUM_ENTRIES_MAIN;
                    }
                }
            },
            WarpState::Stage => {
                if b_pressed {
                    next_state = WarpState::Main;
                    unsafe { WARP_MENU.stage_state = WarpStage::None };
                } else if a_pressed {
                    next_state = WarpState::Details;
                } else if up_pressed {
                    unsafe {
                        let num_entries = WARP_MENU.stage_state.get_num_stages();
                        WARP_MENU.stage_cursor =
                            (WARP_MENU.stage_cursor + num_entries - 1) % num_entries;
                    }
                } else if down_pressed {
                    unsafe {
                        let num_entries = WARP_MENU.stage_state.get_num_stages();
                        WARP_MENU.stage_cursor = (WARP_MENU.stage_cursor + 1) % num_entries;
                    }
                }
            },
            WarpState::Details => {
                if b_pressed {
                    next_state = WarpState::Stage;
                    unsafe {
                        WARP_MENU.selected_entrance = 0;
                        WARP_MENU.selected_room = 0;
                        WARP_MENU.selected_layer = 0;
                    }
                } else if a_pressed {
                    unsafe { WARP_MENU.warp() };
                    unsafe { WARP_MENU.stage_state = WarpStage::None };
                    next_state = WarpState::Off;
                    MainMenu::disable();
                    unsafe {
                        WARP_MENU.selected_entrance = 0;
                        WARP_MENU.selected_room = 0;
                        WARP_MENU.selected_layer = 0;
                    }
                } else if up_pressed {
                    unsafe {
                        WARP_MENU.detail_cursor = (WARP_MENU.detail_cursor + 3 - 1) % 3;
                    }
                } else if down_pressed {
                    unsafe {
                        WARP_MENU.detail_cursor = (WARP_MENU.detail_cursor + 1) % 3;
                    }
                } else if right_pressed || left_pressed {
                    unsafe {
                        match WARP_MENU.detail_cursor {
                            0 => WARP_MENU.change_room(if right_pressed { 1 } else { -1 }),
                            1 => WARP_MENU.change_layer(if right_pressed { 1 } else { -1 }),
                            2 => WARP_MENU.change_entrance(if right_pressed { 1 } else { -1 }),
                            _ => {},
                        }
                    }
                }
            },
        }

        unsafe { WARP_MENU.state = next_state };
        return next_state == WarpState::Off;
    }

    pub fn display() {
        match unsafe { WARP_MENU.state } {
            WarpState::Details => {
                let mut detail_menu = SimpleMenu::<5, 25>::new(10, 10, 10, unsafe {
                    WARP_MENU.stage_state.get_stage_name(WARP_MENU.stage_cursor)
                });
                detail_menu.current_line = unsafe { WARP_MENU.detail_cursor.into() };
                let (room, layer, entrance) = unsafe {
                    (
                        WARP_MENU.get_room(),
                        WARP_MENU.get_layer(),
                        WARP_MENU.get_entrance(),
                    )
                };
                detail_menu.add_entry_args(format_args!("Room: {room}"));
                detail_menu.add_entry_args(format_args!("Layer: {layer}"));
                detail_menu.add_entry_args(format_args!("Entrance: {entrance}"));
                detail_menu.draw();
            },
            _ => {
                let mut main_menu = SimpleMenu::<{ NUM_ENTRIES_MAIN as usize + 2 }, 17>::new(
                    10,
                    10,
                    10,
                    "Select Stage",
                );
                main_menu.current_line = unsafe { WARP_MENU.main_cursor as u32 };
                main_menu.add_entry("Sky");
                main_menu.add_entry("Faron");
                main_menu.add_entry("Eldin");
                main_menu.add_entry("Lanayru");
                main_menu.add_entry("Sealed Grounds");
                main_menu.add_entry("Dungeons");
                main_menu.add_entry("Silent Realms");
                main_menu.draw();

                let stage_state = unsafe { WARP_MENU.stage_state };
                match stage_state {
                    WarpStage::None => {},
                    _ => {
                        let mut sub_menu =
                            SimpleMenu::<25, 39>::new(200, 5, 10, stage_state.get_name());
                        sub_menu.current_line = unsafe { WARP_MENU.stage_cursor as u32 };
                        for n in 0..stage_state.get_num_stages() as u8 {
                            sub_menu.add_entry(stage_state.get_stage_pretty_name(n));
                        }
                        sub_menu.draw();
                    },
                }
            },
        }
    }
}
