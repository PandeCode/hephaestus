use std::collections::HashMap;

use hephaestus::TAGS;
use penrose::{
	builtin::{
		actions::{exit, log_current_state, modify_with, send_layout_message, spawn},
		layout::messages::{ExpandMain, IncMain, ShrinkMain},
	},
	core::bindings::{parse_keybindings_with_xmodmap, KeyCode, KeyEventHandler},
	map,
	x11rb::RustConn,
};

// HashMap<String, Box<dyn KeyEventHandler<RustConn>>>

pub fn keybindings(
) -> Result<HashMap<KeyCode, Box<(dyn KeyEventHandler<RustConn> + 'static)>>, penrose::Error> {
	let mut raw_bindings = map! {
		map_keys: |k: &str| k.to_owned();
		"M-j"            => modify_with(|cs| cs.focus_down()),
		"M-k"            => modify_with(|cs| cs.focus_up()),
		"A-F4"          => modify_with(|cs| cs.kill_focused()),

		"M-C-j"          => modify_with(|cs| cs.swap_down()),
		"M-C-k"          => modify_with(|cs| cs.swap_up()),

		"M-S-k"           => send_layout_message(|| IncMain(2)),
		"M-S-j"         => send_layout_message(|| IncMain(0)),
		"M-S-l"        => send_layout_message(|| ExpandMain),
		"M-S-h"         => send_layout_message(|| ShrinkMain),

		"M-Tab"          => modify_with(|cs| cs.toggle_tag()),

		"M-grave"        => modify_with(|cs| cs.next_layout()),
		"M-S-grave"      => modify_with(|cs| cs.previous_layout()),

		"M-bracketright" => modify_with(|cs| cs.next_screen()),
		"M-bracketleft"  => modify_with(|cs| cs.previous_screen()),

		"M-semicolon"    => spawn("dmenu_run"),
		"M-S-s"          => log_current_state(),
		"M-S-Return"     => spawn("st"),
		"M-A-Escape"     => exit(),
	};

	for (i, tag) in TAGS.iter().enumerate() {
		raw_bindings.extend([
			(
				format!("M-{}", i + 1),
				modify_with(move |client_set| client_set.focus_tag(tag)),
			),
			(
				format!("M-S-{}", i + 1),
				modify_with(move |client_set| client_set.move_focused_to_tag(tag)),
			),
		]);
	}

	parse_keybindings_with_xmodmap(raw_bindings)
}
