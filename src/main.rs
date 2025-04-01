/*
 * @Author: Mahires loritas.personal@gmail.com
 * @Date: 2025-03-31 19:44:45
 * @LastEditors: Mahires loritas.personal@gmail.com
 * @LastEditTime: 2025-04-02 04:12:11
 * @FilePath: \rcli\src\main.rs
 * @Description:
 * Copyright (c) 2025 by Mahires, All Rights Reserved.
 */

use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("temp_output/output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
    }

    Ok(())
}
