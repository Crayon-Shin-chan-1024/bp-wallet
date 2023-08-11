// Modern, minimalistic & standard-compliant cold wallet library.
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2020-2023 by
//     Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2020-2023 LNP/BP Standards Association. All rights reserved.
// Copyright (C) 2020-2023 Dr Maxim Orlovsky. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]
extern crate amplify;
#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_crate as serde;

mod command;
mod args;

use std::fs;
use std::path::Path;
use std::process::ExitCode;

use bpw::{BoostrapError, LogLevel};
use clap::Parser;

use crate::args::Args;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "serde_crate", rename_all = "camelCase")]
pub struct Config {
    pub default_wallet: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            default_wallet: s!("default"),
        }
    }
}

impl Config {
    pub fn load(conf_path: &Path) -> Self {
        fs::read_to_string(conf_path)
            .map_err(|err| {
                error!("Unable to read config file: {err:?}");
                ()
            })
            .and_then(|s| {
                toml::from_str(&s).map_err(|err| {
                    error!("Unable to parse config file: {err}");
                    ()
                })
            })
            .unwrap_or_else(|_| {
                eprintln!("Unable to find or parse config file; using config defaults");
                let conf = Config::default();
                conf.store(conf_path);
                conf
            })
    }

    pub fn store(&self, conf_path: &Path) {
        fs::write(conf_path, toml::to_string(self).expect("config must convert to TOML")).ok();
    }
}

fn main() -> ExitCode {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn run() -> Result<(), BoostrapError> {
    let mut args = Args::parse();
    args.process();
    LogLevel::from_verbosity_flag_count(args.verbose).apply();
    trace!("Command-line arguments: {:#?}", &args);

    eprintln!("\nBP: command-line wallet for bitcoin protocol");
    eprintln!("    by LNP/BP Standards Association\n");

    let conf = Config::load(&args.conf_path());
    debug!("Executing command: {}", args.command);
    args.exec(conf)
}
