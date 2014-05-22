/*
* Rust-FMOD - Copyright (c) 2014 Gomez Guillaume.
*
* The Original software, FmodEx library, is provided by FIRELIGHT TECHNOLOGIES.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
*
* 3. This notice may not be removed or altered from any source distribution.
*/

use enums::*;
use types::*;
use ffi;
use sound;
use fmod_sys;
use libc::{c_int};

pub struct SoundGroup {
    sound_group: ffi::FMOD_SOUNDGROUP,
}

pub fn get_ffi(sound_group : &SoundGroup) -> ffi::FMOD_SOUNDGROUP {
    sound_group.sound_group
}

pub fn from_ptr(sound_group : ffi::FMOD_SOUNDGROUP) -> SoundGroup {
    SoundGroup{sound_group: sound_group}
}

impl Drop for SoundGroup {
    fn drop(&mut self) {
        self.release();
    }
}

impl SoundGroup {
    pub fn release(&mut self) -> FMOD_RESULT {
        if self.sound_group != ::std::ptr::null() {
            match unsafe { ffi::FMOD_SoundGroup_Release(self.sound_group) } {
                FMOD_OK => {
                    self.sound_group = ::std::ptr::null();
                    FMOD_OK
                }
                e => e
            }
        } else {
            FMOD_OK
        }
    }

    pub fn set_max_audible(&self, max_audible: i32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_SoundGroup_SetMaxAudible(self.sound_group, max_audible) }
    }

    pub fn get_max_audible(&self) -> Result<i32, FMOD_RESULT> {
        let max_audible = 0i32;

        match unsafe { ffi::FMOD_SoundGroup_GetMaxAudible(self.sound_group, &max_audible) } {
            FMOD_OK => Ok(max_audible),
            e => Err(e)
        }
    }

    pub fn set_max_audible_behavior(&self, max_audible_behavior: FMOD_SOUNDGROUP_BEHAVIOR) -> FMOD_RESULT {
        unsafe { ffi::FMOD_SoundGroup_SetMaxAudibleBehavior(self.sound_group, max_audible_behavior) }
    }

    pub fn get_max_audible_behavior(&self) -> Result<FMOD_SOUNDGROUP_BEHAVIOR, FMOD_RESULT> {
        let max_audible_behavior = FMOD_SOUNDGROUP_BEHAVIOR_FAIL;

        match unsafe { ffi::FMOD_SoundGroup_GetMaxAudibleBehavior(self.sound_group, &max_audible_behavior) } {
            FMOD_OK => Ok(max_audible_behavior),
            e => Err(e)
        }
    }

    pub fn set_mute_fade_speed(&self, speed: f32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_SoundGroup_SetMuteFadeSpeed(self.sound_group, speed) }
    }

    pub fn get_mute_fade_speed(&self) -> Result<f32, FMOD_RESULT> {
        let speed = 0f32;

        match unsafe { ffi::FMOD_SoundGroup_GetMuteFadeSpeed(self.sound_group, &speed) } {
            FMOD_OK => Ok(speed),
            e => Err(e)
        }
    }

    pub fn set_volume(&self, volume: f32) -> FMOD_RESULT {
        unsafe { ffi::FMOD_SoundGroup_SetVolume(self.sound_group, volume) }
    }

    pub fn get_volume(&self) -> Result<f32, FMOD_RESULT> {
        let volume = 0f32;

        match unsafe { ffi::FMOD_SoundGroup_GetVolume(self.sound_group, &volume) } {
            FMOD_OK => Ok(volume),
            e => Err(e)
        }
    }

    pub fn stop(&self) -> FMOD_RESULT {
        unsafe { ffi::FMOD_SoundGroup_Stop(self.sound_group) }
    }

    pub fn get_name(&self, name_len: u32) -> Result<StrBuf, FMOD_RESULT> {
        let name = StrBuf::with_capacity(name_len as uint).into_owned();

        name.with_c_str(|c_name|{
            match unsafe { ffi::FMOD_SoundGroup_GetName(self.sound_group, c_name, name_len as i32) } {
                FMOD_OK => Ok(StrBuf::from_owned_str(unsafe { ::std::str::raw::from_c_str(c_name) }).clone()),
                e => Err(e)
            }
        })
    }

    pub fn get_num_sounds(&self) -> Result<i32, FMOD_RESULT> {
        let num_sounds = 0i32;

        match unsafe { ffi::FMOD_SoundGroup_GetNumSounds(self.sound_group, &num_sounds) } {
            FMOD_OK => Ok(num_sounds),
            e => Err(e)
        }
    }

    pub fn get_sound(&self, index: i32) -> Result<sound::Sound, FMOD_RESULT> {
        let sound = ::std::ptr::null();

        match unsafe { ffi::FMOD_SoundGroup_GetSound(self.sound_group, index, &sound) } {
            FMOD_OK => Ok(sound::Sound::from_ptr(sound)),
            e => Err(e)
        }
    }

    pub fn get_num_playing(&self) -> Result<i32, FMOD_RESULT> {
        let num_playing = 0i32;

        match unsafe { ffi::FMOD_SoundGroup_GetNumPlaying(self.sound_group, &num_playing) } {
            FMOD_OK => Ok(num_playing),
            e => Err(e)
        }
    }
}