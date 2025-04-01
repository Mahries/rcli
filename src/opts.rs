use clap::{Parser, Subcommand};
use std::path::Path;

/*
 * @Author: Mahires loritas.personal@gmail.com
 * @Date: 2025-04-02 02:57:27
 * @LastEditors: Mahires loritas.personal@gmail.com
 * @LastEditTime: 2025-04-02 03:02:56
 * @FilePath: \rcli\src\opts.rs
 * @Description:
 * Copyright (c) 2025 by Mahires, All Rights Reserved.
 */
#[derive(Parser, Debug)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV or Convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if !Path::new(filename).exists() {
        Err("File does not exist".into())
    } else if !filename.ends_with(".csv") {
        Err(format!("Invalid file type: {}", filename))
    } else {
        Ok(filename.to_string())
    }
}
