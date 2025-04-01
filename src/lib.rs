/*
 * @Author: Mahires loritas.personal@gmail.com
 * @Date: 2025-04-02 02:57:02
 * @LastEditors: Mahires loritas.personal@gmail.com
 * @LastEditTime: 2025-04-02 03:08:39
 * @FilePath: \rcli\src\lib.rs
 * @Description:
 * Copyright (c) 2025 by Mahires, All Rights Reserved.
 */
mod opts;
mod process;

pub use opts::{Opts, SubCommand};
pub use process::process_csv;
