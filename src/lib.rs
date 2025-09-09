use argh::FromArgs;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(FromArgs, Serialize, Deserialize)]
#[argh(subcommand)]
pub enum Command {
	Level(LevelArgs),
	Name(NameArgs),
	Stamina(StaminaArgs),
	Sloth(SlothArgs),
	Pause(PauseArgs),
	Resume(ResumeArgs),
	Toggle(ToggleArgs),
	Next(NextArgs),
	Prev(PrevArgs),
	Finish(FinishArgs),
	Jump(JumpArgs),
	Reload(ReloadArgs),
	Fetch(FetchArgs),
	Listen(ListenArgs),
}

#[derive(FromArgs, Serialize, Deserialize)]
/// Obtain Player's Level.
#[argh(subcommand, name = "level")]
pub struct LevelArgs{}

#[derive(FromArgs, Serialize, Deserialize)]
/// Obtain Player's Name.
#[argh(subcommand, name = "name")]
pub struct NameArgs{}

#[derive(FromArgs, Serialize, Deserialize)]
/// Obtain Player's Stamina Value.
#[argh(subcommand, name = "stamina")]
pub struct StaminaArgs{}

#[derive(FromArgs, Serialize, Deserialize)]
/// Obtain Player's Lazy Value.
#[argh(subcommand, name = "sloth")]
pub struct SlothArgs{}

#[derive(FromArgs, Serialize, Deserialize)]
/// Pause the timer.
#[argh(subcommand, name = "pause")]
pub struct PauseArgs {}

#[derive(FromArgs, Serialize, Deserialize)]
/// Resume the timer.
#[argh(subcommand, name = "resume")]
pub struct ResumeArgs {}

#[derive(FromArgs, Serialize, Deserialize)]
/// Toggle the state of the timer.
#[argh(subcommand, name = "toggle")]
pub struct ToggleArgs {}

#[derive(FromArgs, Serialize, Deserialize)]
/// Jump to the next session.
#[argh(subcommand, name = "next")]
pub struct NextArgs {}

#[derive(FromArgs, Serialize, Deserialize)]
/// Jump to the previous session.
#[argh(subcommand, name = "prev")]
pub struct PrevArgs {}

#[derive(FromArgs, Serialize, Deserialize)]
/// Instantly finishes the current session, invoking the session's specified command.
#[argh(subcommand, name = "finish")]
pub struct FinishArgs {}

#[derive(FromArgs, Serialize, Deserialize)]
/// Jump to the session with the given id.
#[argh(subcommand, name = "jump")]
pub struct JumpArgs {
	/// id of the session
	#[argh(positional)]
	pub id: String,
}

#[derive(FromArgs, Serialize, Deserialize)]
/// Reload the config file.
#[argh(subcommand, name = "reload")]
pub struct ReloadArgs {}

#[derive(FromArgs, Serialize, Deserialize)]
/// Fetch timer information.
#[argh(subcommand, name = "fetch")]
pub struct FetchArgs {
	/// output format
	#[argh(positional)]
	pub format: String,
}

#[derive(FromArgs, Serialize, Deserialize)]
/// Output time continuously, while remaining in sync with the main 'uair' instance.
#[argh(subcommand, name = "listen")]
pub struct ListenArgs {
	/// override to apply
	#[argh(option, short = 'o', long = "override")]
	pub overrid: Option<String>,
	/// output time and exit listening instance immediately
	#[argh(switch, short = 'e')]
	pub exit: bool,
}

pub fn get_socket_path() -> String {
	if let Ok(xdg_runtime_dir) = env::var("XDG_RUNTIME_DIR") {
		xdg_runtime_dir + "/uair.sock"
	} else if let Ok(tmp_dir) = env::var("TMPDIR") {
		tmp_dir + "/uair.sock"
	} else {
		"/tmp/uair.sock".into()
	}
}
