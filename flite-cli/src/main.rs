fn main() {
    let mut args = std::env::args();
    args.next();
    let args: String = args.collect();
    let wave = flite::text_to_wave(args, "eng-USA-male").expect("Error");
    let spec = hound::WavSpec {
        channels: wave,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    for t in (0..44100).map(|x| x as f32 / 44100.0) {
        let sample = (t * 440.0 * 2.0 * PI).sin();
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
}
