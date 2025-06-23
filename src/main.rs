use std::{
    env,
    fs::File,
    io::{BufReader, Read},
    path::Path,
    process::exit,
};

const VERSION: &str = std::env!("CARGO_PKG_VERSION");

const USAGE: &str = r#"
usage: uniqopy <file>

Create a copy of a file incorporating its MD5 hash and the current
UTC timestamp into the new file's name. The file's extension will
be retained.

Examples:
    example -> example.2022-02-02-22:22:22.d41d8cd98f00b204e9800998ecf8427e
    example.txt -> example.2022-02-02-22:22:22.d41d8cd98f00b204e9800998ecf8427e.txt
"#;

/// Calculate the MD5 of a file using buffered reading. Used to get a (reasonably)
/// unique signature for each input file.
///
/// Note that MD5 is [not cryptographically
/// secure](https://en.wikipedia.org/wiki/MD5#Security), so you shouldn't rely
/// on the uniqueness of this hash when accepting un-trusted input.
fn md5_of_file(file_path: &Path) -> Result<String, std::io::Error> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut context = md5::Context::new();
    let mut buffer = vec![0; 10 * 1024 * 1024]; // 10MB buffer

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        context.consume(&buffer[..bytes_read]);
    }

    let digest = context.compute();
    Ok(format!("{:x}", digest))
}

/// Generate a date-and-time-stamp using the system's local time.
fn timestamp() -> String {
    use chrono::{DateTime, Local};
    let now: DateTime<Local> = Local::now();
    format!("{}", now.format("%F-%X"))
}

/// Construct a new filename, preserving file extension.
///
/// For example:
///
/// * `foo.jpg` becomes `foo.<timestamp>.<md5>.jpg`
/// * `bar` becomes `bar.<timestamp>.<md5>`
fn new_name(fname: &Path, ts: &str, md5: &str) -> Result<String, &'static str> {
    let fpath = std::path::Path::new(&fname);
    if !fpath.is_file() {
        return Err("uniqopy only works on files");
    }

    let fname: String = match fpath.file_stem() {
        Some(nm) => nm.to_string_lossy().into(),
        None => return Err("File has no name?"),
    };

    let new_name = match fpath.extension() {
        Some(ext) => format!("{}.{}.{}.{}", &fname, ts, md5, ext.to_string_lossy()),
        None => format!("{}.{}.{}", fname, ts, md5),
    };
    Ok(new_name)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let fpath = match &args[..] {
        [_, fname] => Path::new(fname),
        _ => {
            eprint!("uniqopy version {}\n{}", VERSION, USAGE);
            exit(1);
        }
    };

    // Get md5 of file contents
    let md5 = match md5_of_file(fpath) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!("Error reading {}: {}", &fpath.to_string_lossy(), e);
            exit(2);
        }
    };

    // Get timestamp
    let ts = timestamp();

    // Copy file to new name
    let destination = match new_name(fpath, &ts, &md5) {
        Ok(nm) => nm,
        Err(e) => {
            eprintln!("{}", e);
            exit(3)
        }
    };
    println!("Copying {} to {}", fpath.to_string_lossy(), destination);
    match std::fs::copy(fpath, &destination) {
        Ok(bytes) => {
            println!("Copyied {} bytes", bytes);
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(4);
        }
    };
}
