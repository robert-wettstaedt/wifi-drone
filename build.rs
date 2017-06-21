use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(&env::var("OUT_DIR").unwrap());
    let glutin_out_dir = out_dir.parent().unwrap().parent().unwrap();
    let out_file = out_dir.join("test_gl_bindings.rs");

    for entry in glutin_out_dir.read_dir().unwrap() {
        if let Ok(entry) = entry {
            let folder_name = entry.file_name().into_string().unwrap();

            if folder_name.contains("glutin-") {
                let glutin_out_file = entry.path().join("out").join("test_gl_bindings.rs");

                if glutin_out_file.exists() {
                    fs::copy(glutin_out_file, out_file);
                    break;
                }
            }
        }
    }
}
