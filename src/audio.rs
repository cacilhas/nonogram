use std::ffi::CString;

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
        let data = bytes.iter().map(|e| e.to_owned()).collect::<Vec<u8>>();
        match unsafe { Sfx::load_wave_from_mem(".wav", &data, data.len() as i32) } {
            Err(_) => None,
            Ok(wave) => Sound::load_sound_from_wave(&wave).ok(),
        }
    }

    // TODO: simplify this
    unsafe fn load_wave_from_mem(
        filetype: &str,
        bytes: &Vec<u8>,
        size: i32,
    ) -> Result<Wave, String> {
        let c_filetype = CString::new(filetype).unwrap();
        let c_bytes = bytes.as_ptr();
        let w = ffi::LoadWaveFromMemory(c_filetype.as_ptr(), c_bytes, size);
        if w.data.is_null() {
            return Err(format!("Wave data is null. Check provided buffer data"));
        };
        Ok(Wave::from_raw(w))
    }
}
