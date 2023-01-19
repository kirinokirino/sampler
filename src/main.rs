use quad_snd::{AudioContext, Sound, PlaySoundParams};

fn main() {
    let mut ctx = AudioContext::new();
    let mut sound = Sound::load(&mut ctx, include_bytes!("t.wav"));
	let params = PlaySoundParams { looped: true, volume: 0.1 };
    sound.play(&mut ctx, params);

    loop {}
}
