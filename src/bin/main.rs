use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use loudgain_rust::args::ARGS;
use loudgain_rust::args::build_file_list;
use loudgain_rust::decode_audio::decode_file;
use loudgain_rust::replaygain_scanner::{get_track_gain, scan_file, TrackGain};
use loudgain_rust::tags::save_tags;

fn main() {
    let songs = build_file_list(ARGS.files.clone());
    let scan_results: Vec<TrackGain> = songs.into_par_iter().map(|song| {
        let decoded = decode_file(&song).expect("To be a decoding result");
        let scan = scan_file(decoded).expect("To be a scan result.");
        get_track_gain(song, scan)
    }).collect();

    scan_results.into_par_iter().for_each(|res| {
        println!("{:#?}", res);
        save_tags(&res).expect("To work");
    })
}
