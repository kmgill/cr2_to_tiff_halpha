
static mut IS_VERBOSE: bool = false;


pub fn set_verbose(v:bool) {
    unsafe {
        IS_VERBOSE = v;
    }
}