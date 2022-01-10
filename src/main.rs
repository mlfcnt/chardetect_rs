fn main() {
    extern crate chardet;
    extern crate encoding;
    use std::env;

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // use chardet::{charset2encoding, detect};
    use chardet::detect;
    // use encoding::label::encoding_from_whatwg_label;
    // use encoding::DecoderTrap;
    use std::fs::OpenOptions;
    use std::io::prelude::*;

    // open text file
    let mut fh = OpenOptions::new()
        .read(true)
        .open(file_path)
        .expect("Could not open file");
    let mut reader: Vec<u8> = Vec::new();

    // read file
    fh.read_to_end(&mut reader).expect("Could not read file");

    // detect charset of the file
    let result = detect(&reader);

    let encoding = result.0;
    println!("{}", encoding)
}
