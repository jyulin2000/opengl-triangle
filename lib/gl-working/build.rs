// extern crate gl_generator;
// extern crate gl_generator_profiling_struct;

// use gl_generator::{Api, Fallbacks, StructGenerator, Profile, Registry};
// use gl_generator_profiling_struct::ProfilingStructGenerator;
// use std::env;
// use std::fs::File;
// use std::path::Path;
/*
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut file_gl = File::create(&Path::new(&out_dir).join("bindings.rs")).unwrap();

    let registry = Registry::new(
        Api::Gl,
        (4, 5),
        Profile::Core,
        Fallbacks::All,
        ["GL_NV_command_list"],
    );

    registry
        .write_bindings(ProfilingStructGenerator, &mut file_gl)
        .unwrap();
} */

extern crate gl_generator;

use gl_generator::{Registry, Fallbacks, StructGenerator, Api, Profile};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut file_gl = File::create(&Path::new(&out_dir).join("bindings.rs")).unwrap();

    Registry::new(Api::Gl, (4, 5), Profile::Core, Fallbacks::All, [
        "GL_NV_command_list", // additional extension we want to use
    ])
        .write_bindings(
            StructGenerator, // different generator
            &mut file_gl
        )
        .unwrap();
}

