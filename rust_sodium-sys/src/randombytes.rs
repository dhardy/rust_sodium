// randombytes.h

extern "C" {
    pub fn randombytes_buf(buf: *mut c_void, size: size_t);
}