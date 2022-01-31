// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use anyhow::{Context, Result};
use argh::FromArgs;
use std::path::PathBuf;
use tokio::runtime::Runtime;
use url::Url;

#[derive(FromArgs)]
#[argh(subcommand, name = "put")]
/// upload via HTTP Put
struct Put {
    /// name of the file to write to on local system
    #[argh(positional)]
    filename: PathBuf,

    /// upload via HTTP PUT
    #[argh(positional)]
    url: Url,
}

#[derive(FromArgs)]
#[argh(subcommand, name = "upload-blob")]
/// upload via HTTP Put
struct BlobUpload {
    /// name of the file to write to on local system
    #[argh(positional)]
    filename: PathBuf,

    /// upload via HTTP PUT
    #[argh(positional)]
    url: Url,

    /// specify maximum block size in MiB
    #[argh(option)]
    sas_block_size: Option<usize>,

    /// specify blob upload concurrency
    #[cfg(feature = "blobstore")]
    #[argh(option)]
    sas_block_concurrency: Option<usize>,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum SubCommands {
    Put(Put),
    BlobUpload(BlobUpload),
}

#[derive(FromArgs)]
/// A portable volatile memory acquisition tool for Linux
struct Cmd {
    #[argh(subcommand)]
    subcommand: SubCommands,
}

async fn run(cmd: Cmd) -> Result<()> {
    match cmd.subcommand {
        SubCommands::Put(config) => avml::upload::put(&config.filename, &config.url)
            .await
            .context("unable to upload via PUT"),
        SubCommands::BlobUpload(config) => avml::blobstore::upload_sas(
            &config.filename,
            &config.url,
            config.sas_block_size,
            config.sas_block_concurrency,
        )
        .await
        .context("upload via sas URL failed"),
    }
}

fn main() -> Result<()> {
    let cmd: Cmd = argh::from_env();

    Runtime::new()?.block_on(run(cmd))?;

    Ok(())
}
