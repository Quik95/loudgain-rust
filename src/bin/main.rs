use clap::Parser;

use loudgain_rust::args::{Args, build_file_list};
use loudgain_rust::decode_audio::decode_file;
use loudgain_rust::replaygain_scanner::{scan_file};

fn main() {
    println!("Hello, world!");
    let args = Args::parse();
    let songs = build_file_list(args.files);
    songs.iter().for_each(|song| {
        let decoded = decode_file(song).expect("To be a decoding result");
        dbg!(song);
        dbg!(&decoded);
        println!("{}", scan_file(decoded).expect("To be a global loudness in LUFS"));
    });
}
