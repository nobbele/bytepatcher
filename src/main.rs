use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::Path,
};

use clap::{App, Arg};

fn main() {
    let matches = App::new("BytePatcher")
        .version("1.0")
        .author("nobbele <realnobbele@gmail.com>")
        .about("Patches bytes in binaries")
        .arg(
            Arg::with_name("file")
                .index(1)
                .required(true)
                .help("File to patched bytes in."),
        )
        .arg(
            Arg::with_name("signature")
                .long("signature")
                .short("s")
                .takes_value(true)
                .required(true)
                .display_order(1)
                .help("Signature to find the bytes to patch."),
        )
        .arg(
            Arg::with_name("patch")
                .long("patch")
                .short("p")
                .takes_value(true)
                .required(true)
                .help("Bytes to replace signature with."),
        )
        .usage("bytepatcher <file> -s <signature> -p <patch>")
        .get_matches();

    let file_path = Path::new(matches.value_of("file").unwrap());

    let signature = matches.value_of("signature").unwrap();
    let signature = signature
        .as_bytes()
        .chunks(2)
        .map(|c| std::str::from_utf8(c).unwrap())
        .map(|s| u8::from_str_radix(s, 16).unwrap())
        .flat_map(|n| n.to_le_bytes())
        .collect::<Vec<u8>>();

    let patch = matches.value_of("patch").unwrap();
    let patch = patch
        .as_bytes()
        .chunks(2)
        .map(|c| std::str::from_utf8(c).unwrap())
        .map(|s| u8::from_str_radix(s, 16).unwrap())
        .flat_map(|n| n.to_le_bytes())
        .collect::<Vec<u8>>();

    println!("Signature: {:X?}", signature);
    println!("Patch: {:X?}", patch);

    let file = std::fs::read(file_path).expect("Unable to open file");
    let mut content = file.bytes().collect::<Result<Vec<_>, _>>().unwrap();

    let offset = content
        .windows(signature.len())
        .position(|w| w == signature)
        .expect("Unable to find signature");
    println!("Found signature at {:X?}!", offset);
    for (idx, b) in content[offset..][..patch.len()].iter_mut().enumerate() {
        *b = patch[idx];
    }

    let file_path_stem = file_path.file_stem().unwrap().to_str().unwrap();
    let file_path_ext = file_path.extension().unwrap().to_str().unwrap();
    let new_file_path =
        file_path.with_file_name(format!("{}-patched.{}", file_path_stem, file_path_ext));

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&new_file_path)
        .expect("Unable to open file");
    file.write_all(&content).expect("Unable to write to file");

    println!(
        "Patched file has been writted to {}",
        new_file_path.display()
    );
}
