use loudgain_rust::args::ARGS;
use loudgain_rust::args::build_file_list;
use loudgain_rust::decode_audio::decode_file;
use loudgain_rust::replaygain_scanner::{get_track_gain, scan_file};

fn main() {
    let songs = build_file_list(ARGS.files.clone());
    songs.iter().for_each(|song| {
        let decoded = decode_file(song).expect("To be a decoding result");
        let scan = scan_file(decoded).expect("To be a scan result.");
        println!("{:#?}", get_track_gain(song, scan));
    });
}
