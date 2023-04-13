//
//   This Source Code Form is subject to the terms of the Mozilla Public
//   License, v. 2.0. If a copy of the MPL was not distributed with this
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use std::{error::Error, fmt::Display};

use clap::{Parser, ValueEnum};
use type_description::{render::render_to_markdown, render::render_to_terminal, TypeDescription};

#[derive(Debug, ValueEnum, PartialEq, Clone, Copy)]
enum OutputFormat {
    Markdown,
    Terminal,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Markdown => write!(f, "markdown"),
            OutputFormat::Terminal => write!(f, "terminal"),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, value_parser)]
    output_format: OutputFormat,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let input: TypeDescription = serde_json::from_reader(std::io::stdin())?;

    match args.output_format {
        OutputFormat::Markdown => {
            println!("{}", render_to_markdown(&input)?);
        }
        OutputFormat::Terminal => {
            let terminal_width = term_size::dimensions().map(|(w, _)| w).unwrap_or(80);
            let arena = pretty::Arena::new();

            let rendered_doc = render_to_terminal(&input, &arena);

            let mut output = String::new();
            rendered_doc.render_fmt(terminal_width, &mut output)?;
            println!("{}", output);
        }
    }

    Ok(())
}
