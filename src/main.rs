use std::path::PathBuf;

use clap::Parser;

mod cli;
use cli::Cli;

mod jobs;

fn main() {
	let args = Cli::parse();
	let dir = std::env::current_dir().unwrap();

	if args.list {
		let o = jobs::is_make(dir);
		if o.is_some() {
			jobs::get_jobs(o.unwrap());
		}
	}
}
