use std::{env, path::PathBuf};

const FILES: &[&str] = &[
    "rcheevos/src/rapi/rc_api_common.c",
    "rcheevos/src/rapi/rc_api_editor.c",
    "rcheevos/src/rapi/rc_api_info.c",
    "rcheevos/src/rapi/rc_api_runtime.c",
    "rcheevos/src/rapi/rc_api_user.c",
    "rcheevos/src/rc_client.c",
    "rcheevos/src/rc_client_raintegration.c",
    "rcheevos/src/rc_compat.c",
    "rcheevos/src/rcheevos/alloc.c",
    "rcheevos/src/rcheevos/condition.c",
    "rcheevos/src/rcheevos/condset.c",
    "rcheevos/src/rcheevos/consoleinfo.c",
    "rcheevos/src/rcheevos/format.c",
    "rcheevos/src/rcheevos/lboard.c",
    "rcheevos/src/rcheevos/memref.c",
    "rcheevos/src/rcheevos/operand.c",
    "rcheevos/src/rcheevos/rc_validate.c",
    "rcheevos/src/rcheevos/richpresence.c",
    "rcheevos/src/rcheevos/runtime.c",
    "rcheevos/src/rcheevos/runtime_progress.c",
    "rcheevos/src/rcheevos/trigger.c",
    "rcheevos/src/rcheevos/value.c",
    "rcheevos/src/rc_util.c",
    "rcheevos/src/rc_version.c",
    "rcheevos/src/rhash/aes.c",
    "rcheevos/src/rhash/cdreader.c",
    "rcheevos/src/rhash/hash.c",
    "rcheevos/src/rhash/md5.c",
    "rcheevos/src/rurl/url.c",
];

fn main() {
    let mut build = cc::Build::new();
    build
        .define("RC_DISABLE_LUA", None)
        .define("RC_CLIENT_SUPPORTS_HASH", None)
        .define("RC_STATIC", None)
        .include("rcheevos/include")
        .files(FILES);
    build.compile("rcheevos");

    println!("cargo::rustc-link-lib=static=rcheevos");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .use_core()
        .layout_tests(true)
        .impl_debug(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
