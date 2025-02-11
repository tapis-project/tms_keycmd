// build.rs

use vergen_gitcl::{BuildBuilder,GitclBuilder,Emitter};

fn main() {
    let build = BuildBuilder::all_build().unwrap();
    let gitcl = GitclBuilder::all_git().unwrap();
    Emitter::default()
        .add_instructions(&build).unwrap()
        .add_instructions(&gitcl).unwrap()
        .emit().expect("ERROR: Failed to extract build info");
}