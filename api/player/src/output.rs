use cpal::{traits::{HostTrait, DeviceTrait, StreamTrait}, StreamConfig, Sample};
use ringbuf::{HeapProducer, HeapRb, Consumer, ring_buffer::{RbRef, RbRead}};


const BUF_SIZE: usize = 4096 as usize;

pub struct OutputStream<T: cpal::Sample> {
    stream: cpal::Stream,
    rb_producer: HeapProducer<T>,
}

impl<T: cpal::Sample> OutputStream<T> {
    pub fn try_default() -> OutputStream<f32> {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output devices");
        let config = device.default_output_config().expect("no supported config");

        let rb = HeapRb::<f32>::new(BUF_SIZE);
        let (rb_producer, mut rb_consumer) = rb.split();

        let stream = device.build_output_stream(
            &StreamConfig::from(config),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for sample in data.iter_mut() {
                    *sample = match rb_consumer.pop() {
                        Some(s) => Sample::from(&s),
                        None => Sample::from(&0f32),
                    }
                }
            },
            move |err| {
                eprint!("{}", err);
            }
        ).expect("unable to create stream");

        OutputStream {stream, rb_producer}

    }

    pub fn start(&self) {
        self.stream.play().unwrap();
    }

    pub fn write(&mut self, sample: T) -> Result<(), T> {
        self.rb_producer.push(sample)
    }
}
