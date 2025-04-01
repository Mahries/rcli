/*
 * @Author: Mahires loritas.personal@gmail.com
 * @Date: 2025-04-02 03:03:23
 * @LastEditors: Mahires loritas.personal@gmail.com
 * @LastEditTime: 2025-04-02 04:32:57
 * @FilePath: \rcli\src\process.rs
 * @Description:
 * Copyright (c) 2025 by Mahires, All Rights Reserved.
 */
use anyhow::Ok;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

use crate::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();

    for result in reader.records() {
        let record = result?;
        // zip() -> 将两个迭代器合并成一个元组的迭代器
        // collect() -> 将元组迭代器转换为JSON Value (由serde_json提供)
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        _ => "Unsupported format".to_string(),
    };

    fs::write(output, content)?;

    Ok(())
}
