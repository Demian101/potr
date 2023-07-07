mod opts;
mod potr;
mod translators;

use anyhow::Result;
use clap::Parser;
use opts::*;
use potr::*;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let opts = Opts::parse();
    let translator_config = opts.to_translator_config();
    let potr = Potr::new(opts.po_file, opts.output_file, translator_config);
    potr.run().await
}
