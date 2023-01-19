use quad_snd::{AudioContext, PlaySoundParams, Sound};

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut sampler = Sampler::new();
    sampler.add_one_shot(include_bytes!("t.wav"));
    sampler.run();
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

struct Sampler {
    ctx: AudioContext,
    one_shots: Vec<Sample>,
    loops: Vec<Sample>,
}

impl Sampler {
    pub fn new() -> Self {
        let mut ctx = AudioContext::new();

        Self {
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
            self.one_shots.get(0).unwrap().play(&mut self.ctx);
            sleep(Duration::from_secs(10));
        }
    }
}
