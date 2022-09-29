//
//   This Source Code Form is subject to the terms of the Mozilla Public
//   License, v. 2.0. If a copy of the MPL was not distributed with this
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use std::{error::Error, fmt::Display};

use clap::{Parser, ValueEnum};
use type_description::{render::render_to_markdown, TypeDescription};

#[derive(Debug, ValueEnum, PartialEq, Clone, Copy)]
enum OutputFormat {
    Markdown,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Markdown => write!(f, "markdown"),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, value_parser, default_value_t = OutputFormat::Markdown)]
    output_format: OutputFormat,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let input: TypeDescription = serde_json::from_reader(std::io::stdin())?;

    match args.output_format {
        OutputFormat::Markdown => {
            println!("{}", render_to_markdown(&input)?);
        }
    }

    Ok(())
}
