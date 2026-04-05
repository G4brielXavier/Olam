
// SPDX-License-Identifier: BSD-3-Clause

mod models;
mod cli;
mod utils;

use models::User;

use clap::Parser;
use cli::args::Args;
use cli::matches::matches;

use utils::hey::Hey;
use utils::fiman::Fiman;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    
    let mut fiman = Fiman::new()?;
    fiman.setup()?;

    let mut user: User = fiman.read(&fiman.user_data_path.clone())?;

    let hey = Hey::hi("olam".to_string(), user.clone());
    let cli: Args = Args::parse();

    matches(&cli.command, &hey, &mut fiman, &mut user)?;

    Ok(())

}
