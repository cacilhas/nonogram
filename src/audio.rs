use raylib::prelude::*;

pub enum SfxType {
    ERROR,
    LOCK,
    SET,
    UNSET,
}

pub struct Sfx {
    error: Option<Sound>,
    lock: Option<Sound>,
    set: Option<Sound>,
    unset: Option<Sound>,
}

impl Default for Sfx {
    fn default() -> Self {
        Self {
            error: Self::load_sound(include_bytes!("assets/error.wav")),
            lock: Self::load_sound(include_bytes!("assets/lock.wav")),
            set: Self::load_sound(include_bytes!("assets/set.wav")),
            unset: Self::load_sound(include_bytes!("assets/unset.wav")),
        }
    }
}

impl Sfx {
    pub fn play(&self, audio: &mut RaylibAudio, tpe: &SfxType) {
        let sound = match tpe {
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
        let data = bytes.iter().map(|e| e.to_owned()).collect::<Vec<u8>>();
        match Wave::load_wave_from_mem(".wav", &data, data.len() as i32) {
            Err(_) => None,
            Ok(wave) => Sound::load_sound_from_wave(&wave).ok(),
        }
    }
}
