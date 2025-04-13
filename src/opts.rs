use clap::{Parser, Subcommand};
use std::{fmt, path::Path, str::FromStr};

/*
 * @Author: Mahires loritas.personal@gmail.com
 * @Date: 2025-04-02 02:57:27
 * @LastEditors: Mahires loritas.personal@gmail.com
 * @LastEditTime: 2025-04-03 09:12:33
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

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
}

#[derive(Parser, Debug)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format: {}", s)),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
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

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    // parse可以将字符串转换为指定类型, 前提是该类型实现了FromStr trait
    format.parse::<OutputFormat>()
}
