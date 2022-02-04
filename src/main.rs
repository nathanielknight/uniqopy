use std::{env, path::Path, process::exit};

const USAGE: &str = r#"
usage: uniqopy <file>

Create a copy of a file incorporating its MD5 hash and the current
UTC timestamp into the new file's name. The file's extension will
be retained.

Examples:
    example -> example.2022-02-02-22:22:22.d41d8cd98f00b204e9800998ecf8427e
    example.txt -> example.2022-02-02-22:22:22.d41d8cd98f00b204e9800998ecf8427e.txt
"#;

/// Calculate the MD5 of a stream of input data. Used to get a (reasonably)
/// unique signature for each input file.
///
/// Note that MD5 is [not cryptographically
/// secure](https://en.wikipedia.org/wiki/MD5#Security), so you shouldn't rely
/// on the uniqueness of this hash when accepting un-trusted input.
fn md5_of(val: &[u8]) -> String {
    let digest = md5::compute(val);
    format!("{:x}", digest)
}

#[test]
fn test_md5_of() {
    assert_eq!(md5_of(b"foobar"), "3858f62230ac3c915f300c664312c63f");
    assert_eq!(md5_of(b"fibblesnork"), "ebcceb2950ed7e58c00b60a701efeb98");
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
            eprint!("{}", USAGE);
            exit(1);
        }
    };

    // Get md5 of file contents
    let file_content = match std::fs::read(&fpath) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error opening {}: {}", &fpath.to_string_lossy(), e);
            exit(2);
        }
    };
    let md5 = md5_of(&file_content);

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
    match std::fs::copy(&fpath, &destination) {
        Ok(bytes) => {
            println!("Copyied {} bytes", bytes);
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(4);
        }
    };
}
