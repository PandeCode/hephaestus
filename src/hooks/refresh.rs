use penrose::{
	core::{
		State, hooks::StateHook,
	},
	x::{
		XConn
	},
	Result
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MyRefreshHook {}
impl<X> StateHook<X> for MyRefreshHook
where
	X: XConn,
{
	fn call(&mut self, _state: &mut State<X>, _x: &X) -> Result<()> {
		Ok(())
	}
}
