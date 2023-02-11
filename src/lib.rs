pub const FONT: &str = "FantasqueSansMono Nerd Font Mono:size=10:pixelsize=15:antialias=true:autohint=true";
pub const TERMINAL : &str = "st";

// Deep Ocean
pub const THEME_BACKGROUND           : u32 = 0x0f111aff;
pub const THEME_FOREGROUND           : u32 = 0x8f93a2ff;
pub const THEME_TEXT                 : u32 = 0x4b526dff;
pub const THEME_SELECTION_BACKGROUND : u32 = 0x717cb480;
pub const THEME_SELECTION_FOREGROUND : u32 = 0xffffffff;
pub const THEME_BUTTONS              : u32 = 0x191a21ff;
pub const THEME_SECOND_BACKGROUND    : u32 = 0x181a1fff;
pub const THEME_DISABLED             : u32 = 0x464b5dff;
pub const THEME_CONTRAST             : u32 = 0x090b10ff;
pub const THEME_ACTIVE               : u32 = 0x1a1c25ff;
pub const THEME_BORDER               : u32 = 0x0f111aff;
pub const THEME_HIGHLIGHT            : u32 = 0x1f2233ff;
pub const THEME_TREE                 : u32 = 0x717cb430;
pub const THEME_NOTIFICATIONS        : u32 = 0x090b10ff;
pub const THEME_ACCENT               : u32 = 0x84ffffff;
pub const THEME_EXCLUDED_FILES       : u32 = 0x292d3eff;
pub const THEME_GREEN                : u32 = 0xc3e88dff;
pub const THEME_YELLOW               : u32 = 0xffcb6bff;
pub const THEME_BLUE                 : u32 = 0x82aaffff;
pub const THEME_RED                  : u32 = 0xf07178ff;
pub const THEME_PURPLE               : u32 = 0xc792eaff;
pub const THEME_ORANGE               : u32 = 0xf78c6cff;
pub const THEME_CYAN                 : u32 = 0x89ddffff;
pub const THEME_GRAY                 : u32 = 0x717cb4ff;
pub const THEME_WHITE_BLACK          : u32 = 0xeeffffff;
pub const THEME_ERROR                : u32 = 0xff5370ff;
pub const THEME_COMMENTS             : u32 = 0x717cb4ff;
pub const THEME_VARIABLES            : u32 = 0xeeffffff;
pub const THEME_LINKS                : u32 = 0x80cbc4ff;
pub const THEME_FUNCTIONS            : u32 = 0x82aaffff;
pub const THEME_KEYWORDS             : u32 = 0xc792eaff;
pub const THEME_TAGS                 : u32 = 0xf07178ff;
pub const THEME_STRINGS              : u32 = 0xc3e88dff;
pub const THEME_OPERATORS            : u32 = 0x89ddffff;
pub const THEME_ATTRIBUTES           : u32 = 0xffcb6bff;
pub const THEME_NUMBERS              : u32 = 0xf78c6cff;
pub const THEME_PARAMETERS           : u32 = 0xf78c6cff;

pub const TAGS: [&'static str; 9] = ["¹ ", "² ", "³ ", "⁴ ", "⁵ ", "⁶ ", "⁷ ", "⁸", "⁹"];
pub const TAGS_COLORS: [u32; 9] = [THEME_RED, THEME_GREEN, THEME_BLUE, THEME_BLUE, THEME_BLUE, THEME_BLUE, THEME_BLUE, THEME_BLUE, THEME_BLUE];

// BAR
pub const BAR_HEIGHT_PX: u32 = 18;
pub const BAR_POSITION: penrose_ui::Position = penrose_ui::Position::Top;

pub const BORDER_WIDTH: u32 = 10;
pub const FOCUSED_BORDER: u32 = BLACK;

pub const FOCUS_FOLLOW_MOUSE: bool = false;

pub const INNER_PX: u32 = 5;
pub const OUTER_PX: u32 = 5;
pub const MAX_MAIN: u32 = 1;

pub const RATIO: f32 = 0.6;
pub const RATIO_STEP: f32 = 0.1;


pub const FLOATING_CLASSES: [&'static str; 1] = ["Picture-in-Picture"];

pub const WHITE: u32 = 0xebdbb2ff;
pub const GREY: u32 = 0x3c3836ff;
pub const BLACK: u32 = 0x282828ff;
pub const BLUE: u32 = 0x458588ff;

pub const NORMAL_BORDER: u32 = BLACK;

pub const STARTUP_PROGRAMS: [&'static str; 1] = ["lastbg"];
// pub const STARTUP_PROGRAMS_ONE_INSTANCE: [&'static str; 4] = ["nm-applet", "barrier", "kdeconnect-indicator", "sxhkd -c ~/.config/sxhkd/hephaestus.sxhkdrc"];
pub const STARTUP_PROGRAMS_ONE_INSTANCE: [&'static str; 2] = ["st -e bash", "sxhkd -c ~/.config/sxhkd/hephaestus.sxhkdrc"];

pub const DEFAULT_FLOAT_GEOMETRY: penrose::pure::geometry::Rect = penrose::pure::geometry::Rect {
	x: 100,
	y: 100,
	w: 480,
	h: 360,
};

pub const RULES_CENTER_FLOAT_CLASS_NAME: [&str; 11] = [
	"opengl_testing",
	"Vimb",
	"Xmessage",
	"Gimp",
	"Open File",
	"leagueclientux.exe",
	"riotclientux.exe",
	"riotclientservices.exe",
	"League of Legends",
	"Xephyr",
	"barrier",
];
pub const RULES_FLOAT_CLASS_NAME: [&str; 1] = ["Picture-in-picture"];
pub const RULES_FLOAT_TITLE: [&str; 1] = ["Picture-in-picture"];
pub const RULES_CENTER_FLOAT_ROLE: [&str; 1] = ["GtkFileChooserDialog"];
pub const RULES_IGNORE_RESOURCE: [&str; 2] = ["desktop", "desktop_window"];
pub const RULES_IGNORE_ROLE: [&str; 1] = ["popup"];
pub const RULES_IGNORE_TITLE: [&str; 1] = ["Ozone X11"];
pub const RULES_SHIFT_WORKSPACE_CLASS_NAME_1: [&str; 4] =
	["Browser", "Firefox", "Google-chrome", "Opera"];
pub const RULES_SHIFT_WORKSPACE_CLASS_NAME_2: [&str; 6] =
	["St", "st", "terminal", "st-256color", "alacritty", "kitty"];
pub const RULES_SHIFT_WORKSPACE_CLASS_NAME_3: [&str; 8] = [
	"ModernGL",
	"Emacs",
	"emacs",
	"neovide",
	"Code",
	"Code - Insiders",
	"opengl_testing",
	"Xephyr",
];
pub const RULES_SHIFT_WORKSPACE_CLASS_NAME_4: [&str; 5] =
	["hakuneko-desktop", "Unity", "unityhub", "UnityHub", "zoom"];
pub const RULES_SHIFT_WORKSPACE_CLASS_NAME_5: [&str; 3] = ["Spotify", "spotify", "vlc"];
pub const RULES_SHIFT_WORKSPACE_CLASS_NAME_6: [&str; 2] = ["Mail", "Thunderbird"];
pub const RULES_SHIFT_WORKSPACE_CLASS_NAME_7: [&str; 14] = [
	"riotclientux.exe",
	"leagueclient.exe",
	"Zenity",
	"zenity",
	"wineboot.exe",
	"Wine",
	"wine",
	"wine.exe",
	"explorer.exe",
	"Albion Online Launcher",
	"Albion Online",
	"Albion-Online",
	"riotclientservices.exe",
	"League of Legends",
];
