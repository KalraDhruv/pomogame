use crate::app::Event;
use crate::session::Session;
use crate::socket::BlockingStream;
use crate::Error;
use once_cell::sync::OnceCell;
use async_io::Timer;
use std::fmt::Write as _;
use std::sync::{Arc, Mutex};
use std::io::{self, Stdout, Write};
use std::time::{Duration, Instant};


pub struct UairTimer {
	interval: Duration,
	pub writer: Writer,
	pub state: State,
	pub overtime_ran_once : Arc<Mutex<bool>>,
	pub overtimer: OverTimer,
	pub overtime_val: i32,
}
pub struct OverTimer {
	pub overtime: OnceCell<Duration>,
}
impl OverTimer {
	fn set_overtime(&self, duration:Duration)-> Duration{
		*self.overtime.get_or_init(|| {
			duration
		})
	}
}

impl UairTimer {
	pub fn new(interval: Duration, quiet: bool) -> Self {
		UairTimer {
			interval,
			writer: Writer::new(quiet),
			state: State::PreInit,
			overtime_ran_once: Arc::new(Mutex::new(false)),
			overtime_val: 0,
			overtimer: OverTimer{
				overtime: OnceCell::new(),
			},
		}
	}

	pub async fn start(
		&mut self,
		session: &Session,
		start: Instant,
		dest: Instant,
	) -> Result<Event, Error> {
		let _guard = StateGuard(&mut self.state);

		let duration = dest - start;
		let first_interval = Duration::from_nanos(duration.subsec_nanos().into());
		let mut end = start + first_interval;
		
		let overtime = self.overtimer.set_overtime(duration);
		
		while end <= dest {
			Timer::at(end).await;
			self.writer.write::<true>(session, dest - end)?;
			end += self.interval;
		}
		
		
		if !*self.overtime_ran_once.lock().unwrap(){
			*self.overtime_ran_once.lock().unwrap() = true;
			while end.duration_since(dest) <= overtime{
				
				Timer::at(end).await;
				self.writer.write::<true>(session,  end - dest)?;
				end += self.interval;
				self.overtime_val += 1
			}
			//Send overtime_val from here to any function which requires it The same one as in Pause!
		}
		self.overtime_val = 0;
		*self.overtime_ran_once.lock().unwrap() = false;
		self.overtimer= OverTimer{
				overtime: OnceCell::new(),
		};
		Ok(Event::Finished)
	}
}

pub struct Writer {
	streams: Vec<(BlockingStream, Option<String>)>,
	stdout: Option<Stdout>,
	buf: String,
}

impl Writer {
	fn new(quiet: bool) -> Self {
		Writer {
			streams: Vec::new(),
			stdout: (!quiet).then(io::stdout),
			buf: "".into(),
		}
	}

	pub fn write<const R: bool>(
		&mut self,
		session: &Session,
		duration: Duration,
	) -> Result<(), Error> {
		if let Some(stdout) = &mut self.stdout {
			_ = write!(self.buf, "{}", session.display::<R>(duration, None));
			if write!(stdout, "{}", self.buf)
				.and_then(|_| stdout.flush())
				.is_err()
			{
				self.stdout = None;
			}
			self.buf.clear();
		}
		self.streams.retain_mut(|(stream, overrid)| {
			let overrid = overrid.as_ref().and_then(|o| session.overrides.get(o));
			_ = write!(self.buf, "{}\0", session.display::<R>(duration, overrid));
			let res = stream.write(self.buf.as_bytes()).is_ok();
			self.buf.clear();
			res
		});
		Ok(())
	}

	pub fn add_stream(&mut self, stream: BlockingStream, overrid: Option<String>) {
		self.streams.push((stream, overrid));
	}
}

pub enum State {
	PreInit,
	Paused(Duration),
	Resumed(Instant, Instant),
	Finished,
}

struct StateGuard<'s>(&'s mut State);

impl Drop for StateGuard<'_> {
	fn drop(&mut self) {
		if let State::Resumed(_, dest) = self.0 {
			*self.0 = State::Resumed(Instant::now(), *dest);
		}
	}
}
