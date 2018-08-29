use std::env::args;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, BufWriter, Write};
use std::process::exit;

fn pad_right(string: &str, len: usize) -> String {
	if string.len() >= len {
		return string.to_string();
	}
	let diff = len - string.len();
	return format!("{}{}", string, " ".repeat(diff));
}

#[test]
fn test_pad_right() {
	assert_eq!(&pad_right("test", 5), "test ");
	assert_eq!(&pad_right("test", 10), "test      ");
	assert_eq!(&pad_right("test string", 5), "test string");
}

fn main() {
	let args: Vec<String> = args().skip(1).collect();
	if args.len() < 1 {
		eprintln!(
			"usage: line_by_line [default_padding=100] [...[path [padding=100]]>1] [> output_path]"
		);
		exit(1);
	}
	let mut paths: Vec<(_, u16)> = vec![];
	let mut default_padding = 100u16;

	for arg in args {
		let res = arg.parse::<u16>();
		if res.is_ok() {
			let res = res.unwrap();
			if paths.len() == 0 {
				default_padding = res;
				continue;
			}

			let index = paths.len();
			paths[index - 1].1 = res;
			continue;
		}

		let file = File::open(&arg);
		if file.is_err() {
			eprintln!("Cannot open path: {}", arg);
			exit(1);
		};
		let file = file.unwrap();
		let file = BufReader::with_capacity(1024 * 64, file);
		let lines = file.lines();
		paths.push((lines, default_padding));
	}

	let stdo = stdout();
	let stdo = stdo.lock();
	let mut stdo = BufWriter::with_capacity(1024 * 64, stdo);

	loop {
		let lines: Vec<(_, u16)> = (&mut paths)
			.into_iter()
			.map(|(l, padding)| (l.next(), padding.clone()))
			.collect();
		if (&lines).into_iter().by_ref().all(|(x, _)| x.is_none()) {
			break;
		}
		let padding_total = (&lines)
			.into_iter()
			.by_ref()
			.fold(0usize, |acc, (_, padding)| acc + (*padding as usize));
		let mut output = String::with_capacity(padding_total + 2);
		for (line, padding) in lines {
			if line.is_none() {
				output.push_str(&pad_right(&"", padding as usize));
			} else {
				let line = line.unwrap();
				if line.is_err() {
					eprintln!("Error while reading file");
					exit(1);
				};
				let line = line.unwrap();
				output.push_str(&pad_right(&line, padding as usize));
			}
		}
		output.push_str(&"\n");
		if let Err(_) = stdo.write(output.as_bytes()) {
			eprintln!("Error while writing file");
			exit(1);
		}
	}
	if let Err(_) = stdo.flush() {
		eprintln!("Error while writing file");
		exit(1);
	}
}
