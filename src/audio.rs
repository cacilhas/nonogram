use rscenes::prelude::*;

#[derive(Clone, Copy)]
pub enum SfxType {
    CLAPPING,
    ERROR,
    LOCK,
    SET,
    UNSET,
}

pub struct Sfx {
    clapping: Option<Sound>,
    error: Option<Sound>,
    lock: Option<Sound>,
    set: Option<Sound>,
    unset: Option<Sound>,
}

impl Default for Sfx {
    fn default() -> Self {
        Self {
            clapping: Self::load_sound(include_bytes!("assets/clapping.wav")),
            error: Self::load_sound(include_bytes!("assets/error.wav")),
            lock: Self::load_sound(include_bytes!("assets/lock.wav")),
            set: Self::load_sound(include_bytes!("assets/set.wav")),
            unset: Self::load_sound(include_bytes!("assets/unset.wav")),
        }
    }
}

impl Sfx {
    pub fn play(&self, audio: &mut RaylibAudio, tpe: SfxType) {
        let sound = match tpe {
            SfxType::CLAPPING => &self.clapping,
            SfxType::ERROR => &self.error,
            SfxType::LOCK => &self.lock,
            SfxType::SET => &self.set,
            SfxType::UNSET => &self.unset,
        };
        if let Some(sound) = sound {
            audio.play_sound(sound);
        }
    }

    fn load_sound(bytes: &'static [u8]) -> Option<Sound> {
        mem::load_wave(mem::WaveType::Wav, bytes)
            .ok()
            .and_then(|wave| Sound::load_sound_from_wave(&wave).ok())
    }
}
