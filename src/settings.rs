// Window size
pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;

// GUI size
pub const BAR_WIDTH: i32 = 20;
pub const PANEL_HEIGHT: i32 = 7;
pub const PANEL_Y: i32 = SCREEN_HEIGHT - PANEL_HEIGHT;

// Messages
pub const MSG_X: i32 = BAR_WIDTH + 2;
pub const MSG_WIDTH: i32 = SCREEN_WIDTH - BAR_WIDTH - 2;
pub const MSG_HEIGHT: usize = PANEL_HEIGHT as usize - 1;

// Menus width
pub const INVENTORY_WIDTH: i32 = 50;
pub const CHARACTER_SCREEN_WIDTH: i32 = 30;

// FPS Limit
pub const LIMIT_FPS: i32 = 20;

// Player pos in vec
pub const PLAYER: usize = 0;

// Experience and level-ups
pub const LEVEL_UP_BASE: i32 = 200;
pub const LEVEL_UP_FACTOR: i32 = 150;
pub const LEVEL_SCREEN_WIDTH: i32 = 40;
