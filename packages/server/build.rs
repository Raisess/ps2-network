fn main() -> () {
    cc::Build::new()
        .file("src/core/external/bchunk.c")
        .flag("-O2")
        .compile("bchunk");
}
