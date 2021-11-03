use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=ext/libc/include/libc.h");
    println!("cargo:rerun-if-changed=ext/libc/lib/crypto/fnv.c");
    println!("cargo:rerun-if-changed=ext/libc/lib/crypto/adler32.c");
    println!("cargo:rerun-if-changed=ext/libc/lib/crypto/djb2.c");
    println!("cargo:rerun-if-changed=ext/libc/lib/collections/tuples.c");
    println!("cargo:rerun-if-changed=ext/libc/lib/collections/vector.c");
    println!("cargo:rerun-if-changed=ext/libc/lib/math/max.c");
    println!("cargo:rerun-if-changed=ext/libc/lib/math/min.c");
    println!("cargo:rerun-if-changed=ext/libc/lib/crypto/md5.c");

    cc::Build::new()
        .flag_if_supported("-std=c99")
        .include("ext/libc/lib")
        .file("ext/libc/lib/crypto/adler32.c")
        .file("ext/libc/lib/crypto/fnv.c")
        .file("ext/libc/lib/crypto/djb2.c")
        .file("ext/libc/lib/crypto/md5.c")
        .file("ext/libc/lib/collections/tuples.c")
        .file("ext/libc/lib/collections/vector.c")
        .file("ext/libc/lib/math/max.c")
        .file("ext/libc/lib/math/min.c")
        .compile("liblibc.a");

    let bindings = bindgen::Builder::default()
        .header("ext/libc/include/libc.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
