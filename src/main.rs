/*
 * @Author: Mahires loritas.personal@gmail.com
 * @Date: 2025-03-31 19:44:45
 * @LastEditors: Mahires loritas.personal@gmail.com
 * @LastEditTime: 2025-04-14 02:19:18
 * @FilePath: \rcli\src\main.rs
 * @Description:
 * Copyright (c) 2025 by Mahires, All Rights Reserved.
 */

use clap::Parser;
use rcli::{process_csv, process_genpass, Opts, SubCommand};

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
        SubCommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }
    }

    Ok(())
}
