use std::{env::args, fs::read_to_string, io::Write};

fn main() {
    let args : Vec<_> = args().collect();
    if args.len() != 3 {
        eprintln!("program <input.json> <output.json>");
        return;
    }
    let mut string = read_to_string(args.get(1).expect("No 1st argument")).expect("Cann't read string.");
    string = string.replace("\"{", "{").replace("}\"", "}").replace("\\r\\n", "").replace("\\n", "").replace("\\\"", "\"");
    let mut file = std::fs::File::create(args.get(2).expect("No 2nd argument")).expect("create failed");
    file.write_all(string.as_bytes()).expect("write failed");
}
