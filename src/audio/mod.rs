use rodio::{decoder::Decoder, Source};
use std::{
    fs::{remove_file, File},
    io::{prelude::*, BufReader, BufWriter},
};

pub fn play_audio(sound_file: &[u8; 7486]) {
    let path = "sound.ogg";
    let device = rodio::default_output_device().unwrap();

    let file = File::create(path).unwrap();
    {
        let mut writer = BufWriter::new(file);

        writer.write_all(sound_file).unwrap();
    }

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let source = Decoder::new(BufReader::new(reader)).unwrap();
    rodio::play_raw(&device, source.convert_samples());

    remove_file(path).unwrap();
}
