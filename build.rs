use std::path::Path;

use dircpy::copy_dir;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let target_dir = Path::new(&out_dir).ancestors().nth(3).unwrap();

    let resources = Path::new("resources");

    copy_dir(resources, target_dir.join(resources)).unwrap();
}
