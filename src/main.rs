use std::{
    fs,
    io::{BufReader, Read},
};

fn open_file(filename: String) -> String {
    let mut source_text = String::new();
    let path = fs::File::open(&filename);
    match path {
        Ok(infile) => {
            let mut f = BufReader::new(infile);
            let metadata = fs::metadata(&filename).unwrap();
            source_text.reserve(metadata.len() as usize);
            if let Ok(_) = f.read_to_string(&mut source_text) {
                return source_text;
            } else {
                return source_text;
            }
        }
        Err(e) => {
            println!("Cant open file due to: {}", e);
            return source_text;
        }
    }
}

fn main() {
    let filename = "data.txt";
    let res = open_file(filename.to_string());
    let mut lines: u64 = 0;
    for char in res.bytes() {
        // guess this breaks in utf8..
        if char == '\n' as u8 {
            lines += 1;
        }
    }

    println!("Hello, world! {}", lines);
}
