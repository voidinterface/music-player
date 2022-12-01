pub mod output;

#[cfg(test)]
mod tests {
    use cpal::Sample;

    use crate::output::OutputStream;
    #[test]
    fn sin_wave() {
        let mut output_stream = OutputStream::<f32>::try_default();
        let mut c = 0f32;
        output_stream.start();
        for i in 1..10000000 {
            let rads = c * std::f32::consts::PI / 180f32;
            match output_stream.write(Sample::from(&rads)) {
                Ok(()) => {
                    c += 45f32;
                    if c >= 360f32 {
                        c -= 360f32;
                    }
                },
                _ => (),
            }
        }
    }
}