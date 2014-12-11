/*
* Rust-FMOD - Copyright (c) 2014 Gomez Guillaume.
*
* The Original software, FMOD library, is provided by FIRELIGHT TECHNOLOGIES.
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

#![crate_type = "bin"]

extern crate rfmod;

use std::os;
use std::io::timer::sleep;
use std::time::duration::Duration;

fn play_to_the_end(sound: rfmod::Sound, len: uint) -> rfmod::Result {
    let length = match sound.get_length(rfmod::FMOD_TIMEUNIT_MS) {
        Ok(l) => l,
        Err(e) => panic!("sound.get_length error: {}", e)
    };
    let name = match sound.get_name(len as u32) {
        Ok(n) => n,
        Err(e) => panic!("sound.get_name error: {}", e)
    };
    let mut old_position = 100u;

    match sound.play() {
        Ok(chan) => {
            loop {
                match chan.is_playing() {
                    Ok(b) => {
                        if b == true {
                            let position = chan.get_position(rfmod::FMOD_TIMEUNIT_MS).unwrap();

                            if position != old_position {
                                old_position = position;
                                print!("\r{} : {:02}:{:02} / {:02}:{:02}", name, position / 1000 / 60, position / 1000 % 60,
                                    length / 1000 / 60, length / 1000 % 60);
                            }
                            sleep(Duration::milliseconds(30))
                        } else {
                            break;
                        }
                    },
                    Err(e) => return e,
                }
            }
            rfmod::Result::Ok
        }
        Err(err) => err,
    }
}

fn main() {
    let args = os::args();
    let tmp = args.tail();

    if tmp.len() < 1 {
        panic!("USAGE: ./simple_music_player [music_file]");
    }
    let fmod = match rfmod::FmodSys::new() {
        Ok(f) => f,
        Err(e) => {
            panic!("FmodSys.new : {}", e);
        }
    };

    match fmod.init() {
        rfmod::Result::Ok => {}
        e => {
            panic!("FmodSys.init failed : {}", e);
        }
    };

    let arg1 = tmp.get(0).unwrap();

    let sound = match fmod.create_sound((*arg1).as_slice(), None, None) {
        Ok(s) => s,
        Err(err) => {
            panic!("FmodSys.create_sound failed : {}", err);
        },
    };

    match play_to_the_end(sound, arg1.len()) {
        rfmod::Result::Ok => {
            println!("Ok !");
        },
        err => {
            panic!("FmodSys.play_to_the_end : {}", err);
        }
    };
}