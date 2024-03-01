use std::path::PathBuf;

use clap::{Args, Subcommand};
use nexus_config::vm as vm_config;

#[derive(Debug, Args)]
pub struct PublicParamsArgs {
    #[command(subcommand)]
    pub command: Option<PublicParamsAction>,
}

#[derive(Debug, Subcommand)]
pub enum PublicParamsAction {
    /// Generate public parameters to file.
    Setup(SetupArgs),
    /// Sample SRS for testing to file: NOT SECURE, and memory-heavy operation.
    SampleTestSRS(SRSSetupArgs),
}

#[derive(Debug, Default, Args)]
pub struct SRSSetupArgs {
    /// Number of variables
    #[arg(short = 'n', long = "num-vars", default_value = "26")]
    pub num_vars: usize,

    /// File to save test SRS
    #[arg(short, long)]
    pub file: Option<PathBuf>,

    /// Overwrite the file if it already exists.
    #[arg(long)]
    pub force: bool,
}

#[derive(Debug, Default, Args)]
pub struct SetupArgs {
    /// Number of vm instructions per fold.
    #[arg(short, name = "k")]
    pub k: Option<usize>,

    #[arg(long("impl"))]
    pub nova_impl: Option<vm_config::NovaImpl>,

    /// Where to save the file.
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Overwrite the file if it already exists.
    #[arg(long)]
    pub force: bool,

    /// Path to the SRS file (only required for compressible PCD proofs).
    #[arg(long("srs_file"))]
    pub srs_file: Option<PathBuf>,
}

// TODO: make it accessible to all crates.
pub fn format_params_file(nova_impl: vm_config::NovaImpl, k: usize) -> String {
    format!("nexus-public-{nova_impl}-{k}.zst")
}

pub fn format_srs_file(num_vars: usize) -> String {
    format!("nexus-srs-{num_vars}.zst")
}

