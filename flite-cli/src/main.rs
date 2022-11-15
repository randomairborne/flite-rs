fn main() {
    let mut args = std::env::args();
    args.next();
    let args: String = args.collect();
    let wave = flite::text_to_wave(args, "slt").expect("Error");
    let spec = hound::WavSpec {
        channels: wave.channels().try_into().expect("too many channels"),
        sample_rate: wave.sample_rate().try_into().expect("negative channels"),
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("output.wav", spec).unwrap();
    for sample in wave.samples() {
        writer.write_sample(*sample).unwrap();
    }
}
