use std::{path::Path, fmt::Debug};

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Build shader(s)
    BuildShader {
        /// Build all shaders
        #[arg(short, long)]
        all: bool,
        /// Build specific shader
        #[arg(short, long, value_name = "PATH_TO_CRATE")]
        path: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::BuildShader { all, path: name }) => {
            if *all {
                build_all_shaders();
            } else if let Some(name) = name {
                build_shader(name);
            } else {
                println!("BuildShader subcommand needs `--all` or `--path <PATH_TO_CRATE>`");
            }
        },
        None => {}
    }

}


fn build_all_shaders() {
    shader_builder::build_all_shaders();
}

fn build_shader(path_to_crate: impl AsRef<Path> + Debug) {
    shader_builder::build_shader(path_to_crate);
}
