fn main() {
    extern crate chardet;
    extern crate encoding;
    use chardet::detect;
    use std::env;
    use std::fs::OpenOptions;
    use std::io::prelude::*;

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // open text file
    let mut fh = OpenOptions::new()
        .read(true)
        .open(file_path)
        .expect("Impossible d'ouvrir ce fichier");
    let mut reader: Vec<u8> = Vec::new();

    // read file
    fh.read_to_end(&mut reader)
        .expect("Impossible d'ouvrir ce fichier");

    // detect charset of the file
    let result = detect(&reader);

    let encoding = result.0;
    println!("{}", encoding)
}
