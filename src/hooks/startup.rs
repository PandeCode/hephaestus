use hephaestus::{STARTUP_PROGRAMS, STARTUP_PROGRAMS_ONE_INSTANCE};
use penrose::{
	core::{hooks::StateHook, State},
	util::{spawn, spawn_with_args},
	x::XConn,
	Result,
};
use sysinfo::{ProcessExt, SystemExt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MyStartupHook {}
impl<X> StateHook<X> for MyStartupHook
where
	X: XConn,
{
	fn call(&mut self, _state: &mut State<X>, _x: &X) -> Result<()> {
		STARTUP_PROGRAMS.iter().for_each(|&prog| {
			let prog_split = prog.split(" ").collect::<Vec<&str>>();
			let ps_name: &str = prog_split[0];
			{
				if prog_split.len() > 1 {
					spawn_with_args(ps_name, &prog_split[1..])
				} else {
					spawn(ps_name.to_owned())
				}
			}
			.expect(&format!("Failed to start '{}'.", prog));
		});

		STARTUP_PROGRAMS_ONE_INSTANCE.into_iter().for_each(|prog| {
			let prog_split = prog.split(" ").collect::<Vec<&str>>();
			let ps_name: &str = prog_split[0];
			let display = std::env!("DISPLAY");

			let sys = sysinfo::System::new_all();
			let processes = sys.processes_by_exact_name(ps_name);
			for ps in processes {
				if let Ok(environ) = std::fs::read_to_string(format!("/proc/{}/environ", ps.pid()))
				{
					for e in environ.split("\0").collect::<Vec<&str>>() {
						if e.starts_with("DISPLAY=:") {
							let other_ps_display = &e[8..];
							println!(
								"other_ps_display {}, display: {}",
								other_ps_display, display
							);
							if other_ps_display == display {
								println!("{ps_name} is already running.");
								return;
							}
						}
					}
				}
			}

			{
				if prog_split.len() > 1 {
					println!("spawn_args: {ps_name}, {:?}", &prog_split[1..]);
					spawn_with_args(ps_name, &prog_split[1..])
				} else {
					println!("spawn: {prog}");
					spawn(prog.to_owned())
				}
			}
			.expect(&format!("Failed to start '{}'.", prog));
		});
		Ok(())
	}
}
