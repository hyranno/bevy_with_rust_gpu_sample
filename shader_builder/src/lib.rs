use std::{fs, path::{PathBuf, Path}, fmt::Debug};
use spirv_builder::{MetadataPrintout, SpirvBuilder, ModuleResult};

const SHADERS_DIR: &str = "shaders/";
const TARGET: &str = "spirv-unknown-spv1.5";   // NOTE: should be "spirv-unknown-webgpu0"
const GAME_DIR: &str = "game/";
const SHADERS_ASSET_DIR: &str = "assets/shaders/";



pub fn build_shader(path_to_crate: impl AsRef<Path> + Debug) {
    let workspace_dir: PathBuf = [  // "CARGO_WORKSPACE_DIR"
        std::env::var("CARGO_MANIFEST_DIR").expect("env not found."),
        String::from("..")
    ].iter().collect();
    let game_dir = workspace_dir.join(GAME_DIR);
    let shaders_out_dir = game_dir.join(SHADERS_ASSET_DIR);
    let errmsg = format!("failed to build {:?} .", path_to_crate);

    let built_shader = SpirvBuilder::new(path_to_crate, TARGET)
        .print_metadata(MetadataPrintout::Full)
        .preserve_bindings(true)
        // .spirv_metadata(spirv_builder::SpirvMetadata::Full) // for debug
        .build()
        .expect(&errmsg)
    ;
    match built_shader.module {
        ModuleResult::SingleModule(path) => copy_spv(&path, &shaders_out_dir),
        ModuleResult::MultiModule(tree) => for path in tree.values() {
            copy_spv(&path, &shaders_out_dir)
        },
    }
}

pub fn build_all_shaders() {
    let workspace_dir: PathBuf = [  // "CARGO_WORKSPACE_DIR"
        std::env::var("CARGO_MANIFEST_DIR").expect("env not found."),
        String::from("..")
    ].iter().collect();
    let shaders_dir = workspace_dir.join(SHADERS_DIR);

    let errmsg_dir = format!("cannot read shaders_dir {:?} .", shaders_dir);
    let errmsg_entry = format!("cannot read entry in shaders_dir {:?} .", shaders_dir);

    for entry in fs::read_dir(shaders_dir).expect(&errmsg_dir) {
        let entry = entry.expect(&errmsg_entry);
        let path = entry.path();
        if path.is_dir() {
            build_shader(path);
        }
    }
}

fn copy_spv(src: &PathBuf, dest_dir: &PathBuf) {
    let dest: PathBuf = [dest_dir, &PathBuf::from(src.file_name().unwrap())].iter().collect();
    fs::copy(src.clone(), dest.clone()).expect(&format!("failed to copy {:?} to {:?} .", src, dest));
}
