use std::path::PathBuf;
use clap::{ Parser, Subcommand, Command, arg, command };
use std::collections::HashMap;

fn main() {
    println!("wput - inspired by hagen fritsch and matthieu le brazidec");
	
	let _cli = Cli::parse();
}


#[derive(Parser)]
#[command(author="bob pop <me@nagexiucai.com>", version="0.0.1", about="pratices")]
struct Cli {
	name: Option<String>,
	
	#[command(subcommand)]
	command: Option<Commands>,
	
	#[arg(short, long)]
	config: Option<PathBuf>,
	
	#[arg(short, long)]
	protocol: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
	Test {
		#[arg(short, long)]
		list: bool,
	},
}

enum FormItemValue<'t> {
	Text(&'t str),
	Integer(isize),
	Float(f64),
}

fn formpost(url: &str, items: &HashMap<&str, FormItemValue>) {}