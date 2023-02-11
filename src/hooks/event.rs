use penrose::{
	core::{hooks::EventHook, State},
	x::{XConn, XEvent},
	Result,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MyEventHook {}
impl<X> EventHook<X> for MyEventHook
where
	X: XConn,
{
	fn call(&mut self, _event: &XEvent, _state: &mut State<X>, _x: &X) -> Result<bool> {
		/* todo!() */
		Ok(true)
	}
}
