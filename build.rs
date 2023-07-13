use std::{fs, path::PathBuf};
use spirv_builder::{MetadataPrintout, SpirvBuilder, ModuleResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target = "spirv-unknown-spv1.4";
    let shaders_dir: PathBuf = [std::env::var("CARGO_MANIFEST_DIR")?, String::from("assets/shaders/")].iter().collect();
    fn copy_spv(src: &PathBuf, dest_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let dest: PathBuf = [dest_dir, &PathBuf::from(src.file_name().unwrap())].iter().collect();
        fs::copy(src, dest)?;
        Ok(())
    }

    let shader_crate = "shaders/custom_material";
    let built_shader = SpirvBuilder::new(shader_crate, target)
        .print_metadata(MetadataPrintout::Full)
        .preserve_bindings(true)
        .build()?;
    match built_shader.module {
        ModuleResult::SingleModule(path) => copy_spv(&path, &shaders_dir)?,
        ModuleResult::MultiModule(tree) => for path in tree.values() { copy_spv(&path, &shaders_dir)? },
    }

    Ok(())
}