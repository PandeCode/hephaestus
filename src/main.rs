use penrose::{
	core::{Config, WindowManager},
	extensions::hooks::add_ewmh_hooks,
	x11rb::RustConn,
};

use std::collections::HashMap;
use tracing_subscriber::{self, prelude::*};

mod actions;
mod hooks;
mod keybinds;
mod layout;
mod status_bar;

use keybinds::keybindings;
use layout::layouts;
use status_bar::status_bar;

use crate::hooks::{event::MyEventHook, manager::MyManageHook, refresh::MyRefreshHook, startup::MyStartupHook};

use hephaestus::{
	BORDER_WIDTH, FLOATING_CLASSES, FOCUSED_BORDER, FOCUS_FOLLOW_MOUSE, NORMAL_BORDER, TAGS,
};

fn main() -> anyhow::Result<()> {
	tracing_subscriber::fmt()
		.json()
		.with_env_filter("debug")
		.finish()
		.init();
	println!("Start");

	let config = add_ewmh_hooks(Config {
		normal_border: NORMAL_BORDER.into(),
		focused_border: FOCUSED_BORDER.into(),
		border_width: BORDER_WIDTH,
		focus_follow_mouse: FOCUS_FOLLOW_MOUSE,
		default_layouts: layouts(),
		tags: TAGS.iter().map(|&s| s.to_owned()).collect(),
		floating_classes: FLOATING_CLASSES.iter().map(|&s| s.to_owned()).collect(),
		startup_hook: Some(Box::new(MyStartupHook {})),
        event_hook: Some(Box::new(MyEventHook {})),
        manage_hook: Some(Box::new(MyManageHook {})),
		refresh_hook: Some(Box::new(MyRefreshHook {})),
		..Config::default()
	});

	let conn = RustConn::new()?;
	let keybindings = keybindings()?;

	let mut wm = WindowManager::new(config, keybindings, HashMap::new(), conn)?;

	let bar = status_bar()?;
	wm = bar.add_to(wm);

	wm.run()?;

	Ok(())
}
