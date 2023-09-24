use std::{
	fs::File,
	io::{BufRead, BufReader},
	path::PathBuf,
};

use regex::Regex;

// https://www.gnu.org/software/make/manual/html_node/Makefile-Names.html
pub fn is_make(dir: PathBuf) -> Option<PathBuf> {
	let files = vec!["GNUmakefile", "makefile", "Makefile"];
	for file in files {
		let path = dir.join(file);
		if path.exists() {
			return Some(path);
		}
	}

	None
}

pub fn get_jobs(file: PathBuf) -> Vec<String> {
	let task_re = Regex::new("^[a-z][A-Z]:$").unwrap();

	let file = File::open(file).unwrap();
	let reader = BufReader::new(file);
	for line in reader.buffer().lines() {
		let l = line.unwrap();

		println!("{}", l);
		if task_re.is_match(l.as_str()) {
			println!("match: {}", l);
		}
	}

	let v = vec![];
	return v;
}
