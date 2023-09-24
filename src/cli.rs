use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
	/// Name of the person to greet
	#[arg(short, long)]
	pub list: bool,

	/// Number of times to greet
	pub job_name: String,
}
