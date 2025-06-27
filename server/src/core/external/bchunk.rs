use std::ffi::c_char;

extern "C" {
    pub fn bchunk(
        cuefile_path: *const c_char,
        binfile_path: *const c_char,
        outfile_path: *const c_char,
    ) -> i8;
}
