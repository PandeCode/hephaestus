use hephaestus::{
	RULES_CENTER_FLOAT_CLASS_NAME, RULES_CENTER_FLOAT_ROLE, RULES_FLOAT_CLASS_NAME,
	RULES_FLOAT_TITLE, RULES_IGNORE_RESOURCE, RULES_IGNORE_ROLE, RULES_IGNORE_TITLE,
	RULES_SHIFT_WORKSPACE_CLASS_NAME_1, RULES_SHIFT_WORKSPACE_CLASS_NAME_2,
	RULES_SHIFT_WORKSPACE_CLASS_NAME_3, RULES_SHIFT_WORKSPACE_CLASS_NAME_4,
	RULES_SHIFT_WORKSPACE_CLASS_NAME_5, RULES_SHIFT_WORKSPACE_CLASS_NAME_6,
	RULES_SHIFT_WORKSPACE_CLASS_NAME_7, TAGS, DEFAULT_FLOAT_GEOMETRY,
};
use penrose::{
	core::{
		hooks::ManageHook,
		State,
	},
	x::{
		query::{ClassName, StringProperty, Title},
		Query, XConn
	},
	Result, Xid,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MyManageHook {}
impl MyManageHook {

	/*
	fn m_app_name<X>(self, d: &'static str, id: Xid, x: &X) -> bool where X: XConn, { AppName(d).run(id, x).unwrap_or(false) }
	fn m_client_machine<X>(self, d: &'static str, id: Xid, x: &X) -> bool where X: XConn, { StringProperty("WM_CLIENT_MACHINE", d) .run(id, x) .unwrap_or(false) }
	fn m_icon_name<X>(self, d: &'static str, id: Xid, x: &X) -> bool where X: XConn, { StringProperty("WM_ICON_NAME", d) .run(id, x) .unwrap_or(false) }
	fn m_locale_name<X>(self, d: &'static str, id: Xid, x: &X) -> bool where X: XConn, { StringProperty("WM_LOCALE_NAME", d) .run(id, x) .unwrap_or(false) }
	fn m_name<X>(self, d: &'static str, id: Xid, x: &X) -> bool where X: XConn, { StringProperty("WM_NAME", d).run(id, x).unwrap_or(false) }
	fn m_net_icon_name<X>(self, d: &'static str, id: Xid, x: &X) -> bool where X: XConn, { StringProperty("_NET_WM_ICON_NAME", d) .run(id, x) .unwrap_or(false) }
	fn m_net_name<X>(self, d: &'static str, id: Xid, x: &X) -> bool where X: XConn, { StringProperty("_NET_WM_NAME", d) .run(id, x) .unwrap_or(false) }
	*/

	fn m_role<X>(self, d: &'static str, id: Xid, x: &X) -> bool where X: XConn, { StringProperty("WM_WINDOW_ROLE", d) .run(id, x) .unwrap_or(false) }
	fn m_title<X>(self, d: &'static str, id: Xid, x: &X) -> bool where X: XConn, { Title(d).run(id, x).unwrap_or(false) }
	fn m_class_name<X>(&self, d: &'static str, id: Xid, x: &X) -> bool where X: XConn, { ClassName(d).run(id, x).unwrap_or(false) }

	fn center_float<X>(&self, id: Xid, state: &mut State<X>, x: &X) -> ()
	where
		X: XConn, {
		let mut rect = state.client_set.current_screen().geometry();
		let cli_rect = x.client_geometry(id).unwrap_or(DEFAULT_FLOAT_GEOMETRY);
		rect.x = (rect.w - cli_rect.w) / 2;
		rect.y = (rect.h - cli_rect.h) / 2;
        rect.w = cli_rect.w;
        rect.h = cli_rect.h;
		state
			.client_set
			.float(id, rect)
			.expect("Failed to set window to float");
	}

	fn float<X>(&self, id: Xid, state: &mut State<X>, x: &X) -> ()
	where
		X: XConn,
	{
		let cli_rect = x.client_geometry(id).unwrap_or(DEFAULT_FLOAT_GEOMETRY);
		state
			.client_set
			.float(id, cli_rect)
			.expect("Failed to set window to float");
	}
}

impl<X> ManageHook<X> for MyManageHook
where
	X: XConn,
{
	#[rustfmt::skip]
	fn call(&mut self, id: Xid, state: &mut State<X>, x: &X) -> Result<()> {

		for d in RULES_CENTER_FLOAT_CLASS_NAME { if self.m_class_name(d, id, x) { self.center_float(id, state, x); }; }
		for d in RULES_CENTER_FLOAT_ROLE       { if self.m_role(d, id, x) { self.center_float(id, state, x) }; }

		for d in RULES_FLOAT_CLASS_NAME { if self.m_class_name(d, id, x) { self.float(id, state, x); }; }
		for d in RULES_FLOAT_TITLE      { if self.m_title(d, id, x) {self.float(id, state, x)}; }

		for d in RULES_IGNORE_RESOURCE { if self.m_class_name(d, id, x) {}; }
		for d in RULES_IGNORE_ROLE     { if self.m_class_name(d, id, x) {}; }
		for d in RULES_IGNORE_TITLE    { if self.m_class_name(d, id, x) {}; }

		for d in RULES_SHIFT_WORKSPACE_CLASS_NAME_1 { if self.m_class_name(d, id, x) { state.client_set.move_client_to_tag(&id, TAGS[0]); return Ok(()); }; }
		for d in RULES_SHIFT_WORKSPACE_CLASS_NAME_2 { if self.m_class_name(d, id, x) { state.client_set.move_client_to_tag(&id, TAGS[1]); return Ok(()); }; }
		for d in RULES_SHIFT_WORKSPACE_CLASS_NAME_3 { if self.m_class_name(d, id, x) { state.client_set.move_client_to_tag(&id, TAGS[2]); return Ok(()); }; }
		for d in RULES_SHIFT_WORKSPACE_CLASS_NAME_4 { if self.m_class_name(d, id, x) { state.client_set.move_client_to_tag(&id, TAGS[3]); return Ok(()); }; }
		for d in RULES_SHIFT_WORKSPACE_CLASS_NAME_5 { if self.m_class_name(d, id, x) { state.client_set.move_client_to_tag(&id, TAGS[4]); return Ok(()); }; }
		for d in RULES_SHIFT_WORKSPACE_CLASS_NAME_6 { if self.m_class_name(d, id, x) { state.client_set.move_client_to_tag(&id, TAGS[5]); return Ok(()); }; }
		for d in RULES_SHIFT_WORKSPACE_CLASS_NAME_7 { if self.m_class_name(d, id, x) { state.client_set.move_client_to_tag(&id, TAGS[6]); return Ok(()); }; }

		Ok(())
	}
}
