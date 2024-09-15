use clap::Parser;
use mdrss::{generate_rss, RssConf};
use std::process;

/// A simple CLI tool to generate RSS feed from markdown files.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the directory with markdown files
    #[arg(short, long)]
    input_dir: String,

    /// Path to the output rss.xml file
    #[arg(short, long)]
    output_file: String,
}

fn main() {
    // Parse the command line arguments
    let args = Args::parse();

    let rss_conf = RssConf {
        title: String::from("Custom RSS Title"),
        link: String::from("https://example.com"),
        description: String::from("A test description."),
        delimiter: String::from("-rss-"),
    };

    // Call the function from mdrss to generate the RSS feed
    match generate_rss(&args.input_dir, &args.output_file, &rss_conf) {
        Ok(_) => {
            println!("RSS feed generated successfully: {}", args.output_file);
        }
        Err(e) => {
            eprint!("Failed to generate RSS feed: {:?}", e);
            process::exit(1);
        }
    }
}
