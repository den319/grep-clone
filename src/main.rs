use std::{fs, path::Path};
use walkdir::WalkDir;
use colored::*;
use rayon::prelude::*;

use clap::Parser;

use crate::search::{
    search,
    search_case_insensitive,
};

mod config;
mod search;
fn main() -> std::io::Result<()> {
    let config= config::Config::parse();


    let results= if config.recursive {
        let entries: Vec<_> = WalkDir::new(&config.file_path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
            .collect();
        entries
            .par_iter()
            .flat_map(|entry| {
                let path = entry.path();

                let bytes = fs::read(path).ok()?;

                if search::is_binary(&bytes) {
                    return None;
                }

                let contents = String::from_utf8_lossy(&bytes);

                let file_results = if config.ignore_case {
                    search::search_case_insensitive(
                        &config.query,
                        &contents,
                        path,
                    )
                } else {
                    search::search(
                        &config.query,
                        &contents,
                        path,
                    )
                };

                Some(file_results)
            })
            .flatten()
            .collect()
    } else {
        let contents = fs::read_to_string(&config.file_path)?;
        let path= Path::new(&config.file_path);

        if config.ignore_case {
            search_case_insensitive(&config.query, &contents, &path)
        } else {
            search(&config.query, &contents, &path)
        }
    };

    for (line_num, line, path) in results {
        let colored_query = line.replace(
            &config.query,
            &format!("{}", &config.query.on_black().green()),
        );

        let colored_path= path.replace(&path, &format!("{}", &path.magenta().bold()));
        
        if config.line_numbers {
            println!("{} line-{}: {}", colored_path, line_num + 1, colored_query);
        }else {
            println!("{} {}", colored_path, colored_query);
        }
    }

    Ok(())
}

// hello
