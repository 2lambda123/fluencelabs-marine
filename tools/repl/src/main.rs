/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![deny(
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]
#![warn(rust_2018_idioms)]

/// Command-line tool intended to test Fluence App services.

mod editor;
mod logger;
mod repl;

use logger::init_logger;
use editor::init_editor;

use repl::REPL;

use rustyline::error::ReadlineError;

const HISTORY_FILE_PATH: &str = ".repl_history";

pub(crate) type ReplResult<T> = std::result::Result<T, anyhow::Error>;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> ReplResult<()> {
    init_logger();

    let (args, _) = rustop::opts! {
        synopsis "Fluence Application service REPL";
        version env!("CARGO_PKG_VERSION");
        param config_file_path: Option<String>, desc: "Path to a service config";
        opt quiet: bool=false, desc: "Suppress unnecessary welcome message";
        opt working_dir: Option<String>, desc: "Set working dir for service, default = \".\"";
    }
    .parse_or_exit();

    let mut rl = init_editor()?;
    let _ = rl.load_history(HISTORY_FILE_PATH);

    if !args.quiet {
        print_welcome_message();
    }

    let mut repl = REPL::new(args.config_file_path, args.working_dir, args.quiet).await?;

    let mut count = 1;
    loop {
        let p = format!("\n{}> ", count);
        rl.helper_mut()
            .expect("No helper")
            .set_prompt_color(format!("\x1b[1;32m{}\x1b[0m", p));
        let readline = rl.readline(&p);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if !repl.execute(line.split_whitespace()).await {
                    break;
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
        count += 1;
    }

    if let Err(e) = rl.save_history(HISTORY_FILE_PATH) {
        eprintln!("failed to save history: {}", e);
    }

    Ok(())
}

fn print_welcome_message() {
    use crossterm::style::Stylize;

    println!(
        "Welcome to the Marine REPL (version {})",
        env!("CARGO_PKG_VERSION")
    );

    println!(
        "Minimal supported versions\n  sdk: {}\n  interface-types: {}\n",
        fluence_app_service::min_sdk_version().to_string().blue(),
        fluence_app_service::min_it_version().to_string().blue(),
    );

    #[cfg(feature = "check-latest")]
    if let Ok(Some(new_version)) = check_latest::check_max!() {
        println!(
            "New version is available! {} -> {}",
            check_latest::crate_version!().red(),
            new_version.to_string().blue()
        );
        println!(
            "To update run: {}\n",
            "cargo +nightly install mrepl".yellow(),
        );
    }
}
