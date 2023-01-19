use quad_snd::{AudioContext, PlaySoundParams, Sound};

use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;

const MY_KEYBOARD_OFFSET: u8 = 36;

pub struct Sampler {
    rx: mpsc::Receiver<u8>,
    ctx: AudioContext,
    one_shots: Vec<Sample>,
    loops: Vec<Sample>,
}

impl Sampler {
    pub fn new(midi_receiver: mpsc::Receiver<u8>) -> Self {
        let mut ctx = AudioContext::new();

        Self {
            rx: midi_receiver,
            ctx,
            one_shots: Vec::new(),
            loops: Vec::new(),
        }
    }

    pub fn add_one_shot(&mut self, data: &[u8]) {
        let sound = Sound::load(&self.ctx, include_bytes!("t.wav"));
        let params = PlaySoundParams {
            looped: false,
            volume: 0.1,
        };
        let sample = Sample::new(sound, params);
        self.one_shots.push(sample);
    }

    pub fn run(&mut self) -> ! {
        loop {
            for received in &self.rx {
                println!("Got: {}", received);
                if let Some(oneshot) = self
                    .one_shots
                    .get((received.checked_sub(MY_KEYBOARD_OFFSET).unwrap()) as usize)
                {
                    oneshot.play(&mut self.ctx);
                }
            }
            sleep(Duration::from_millis(5));
        }
    }
}

struct Sample {
    sound: Sound,
    params: PlaySoundParams,
}

impl Sample {
    pub fn new(sound: Sound, params: PlaySoundParams) -> Self {
        Self { sound, params }
    }

    pub fn play(&self, ctx: &mut AudioContext) {
        let PlaySoundParams { looped, volume } = self.params;
        self.sound.play(ctx, PlaySoundParams { looped, volume });
    }
}
