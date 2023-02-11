use hephaestus::BAR_POSITION;
use penrose::x::XConn;

pub use penrose_ui::{
	bar::{
		widgets::{
			amixer_volume, battery_summary, current_date_and_time, wifi_network, ActiveWindowName,
			CurrentLayout, RefreshText, RootWindowName, Workspaces,
		},
		Position, StatusBar,
	},
	core::{Context, Draw, TextStyle},
	Result,
};

use hephaestus::{BAR_HEIGHT_PX, BLACK, BLUE, FONT, GREY, WHITE};

pub fn status_bar<X: XConn>() -> Result<StatusBar<X>> {
	let style = &TextStyle {
		font: FONT.to_string(),
		point_size: 8,
		fg: WHITE.into(),
		bg: Some(BLACK.into()),
		padding: (2.0, 2.0),
	};

	let highlight = BLUE;
	let empty_ws = GREY;

	let max_active_window_chars = 80;
	let highlight = highlight.into();

	StatusBar::try_new(
		BAR_POSITION,
		BAR_HEIGHT_PX,
		style.bg.unwrap_or_else(|| 0x000000.into()),
		&[&style.font],
		vec![
			Box::new(Workspaces::new(style, highlight, empty_ws)),
			Box::new(CurrentLayout::new(style)),
			Box::new(ActiveWindowName::new(
				max_active_window_chars,
				&TextStyle {
					bg: Some(highlight),
					padding: (6.0, 4.0),
					..style.clone()
				},
				true,
				false,
			)),
			Box::new(wifi_network(&style)),
			Box::new(battery_summary("BAT1", &style)),
			Box::new(battery_summary("BAT0", &style)),
			Box::new(amixer_volume("Master", &style)),
			Box::new(current_date_and_time(&style)),
		],
	)
}
