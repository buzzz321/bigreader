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

fn get_text_window(curr_pos: usize, rows: usize, buffer: &String) -> Option<(usize, &str)> {
    let bytebuff = &buffer.as_bytes()[curr_pos..];
    let mut lines: usize = 0;
    let mut pos = curr_pos;
    for char in bytebuff.bytes() {
        if char.unwrap() == '\n' as u8 {
            lines += 1;
        }
        pos += 1;
        if lines >= rows {
            return Some((curr_pos + pos, &buffer[curr_pos..pos]));
        }
    }
    return None;
}

fn main() {
    let filename = "data.txt";
    let res = open_file(filename.to_string());

    let new_lines = get_text_window(78888890, 5, &res);
    match new_lines {
        Some((new_pos, myslice)) => println!("new_pos {} \nslice: \n{}", new_pos, myslice),
        None => println!("could get more windows"),
    };

    let mut lines: u64 = 0;
    let mut pos: u64 = 0;
    for char in res.bytes() {
        // guess this breaks in utf8..
        if char == '\n' as u8 {
            lines += 1;
        }
        pos += 1;

        if lines == 8000000{
            println!("pos {}", pos);
        }
    }

    println!("Lines in file {}", lines);
}
