use nexrad::file::is_compressed;
use nexrad::decompress::decompress_file;

pub fn decompress(bytes: Vec<u8>) -> Vec<u8> {
    if is_compressed(bytes.as_slice()) {
        return decompress_file(&bytes).expect("decompresses file");
    }

    bytes
}
