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

extern crate libc;
extern crate rfmod;

use std::io::timer::sleep;
use std::os;
use std::io::SeekSet;
use std::time::duration::Duration;

#[allow(unused_variables)]
fn my_open(music_name: &str, unicode: i32) -> Option<(rfmod::FmodFile, Option<rfmod::FmodUserData>)> {
    println!("Let's start !");
    let file = match rfmod::FmodFile::open(music_name) {
        Some(f) => f,
        None => panic!("Couldn't open: {}", music_name)
    };

    Some((file, None))
}

#[allow(unused_variables)]
fn my_close(music_file: &mut rfmod::FmodFile, user_data: Option<&mut rfmod::FmodUserData>) {
    println!("This is the end !");
    music_file.close();
}

#[allow(unused_variables)]
fn my_read(handle: &mut rfmod::FmodFile, buffer: &mut [u8], size_to_read: u32, user_data: Option<&mut rfmod::FmodUserData>) -> uint {
    handle.read(buffer)
}

#[allow(unused_variables)]
fn my_seek(handle: &mut rfmod::FmodFile, pos: u32, user_data: Option<&mut rfmod::FmodUserData>) {
    handle.seek(pos as i64, SeekSet);
}

fn main() {
    let args = os::args();
    let tmp = args.tail();

    if tmp.len() < 1 {
        panic!("USAGE: ./file_callback [music_file]");
    }
    let fmod = match rfmod::FmodSys::new() {
        Ok(f) => f,
        Err(e) => {
            panic!("FmodSys.new : {}", e);
        }
    };

    match fmod.init_with_parameters(1i32, rfmod::FmodInitFlag(rfmod::FMOD_INIT_NORMAL)) {
        rfmod::Result::Ok => {}
        e => {
            panic!("FmodSys.init failed : {}", e);
        }
    };

    match fmod.set_file_system(Some(my_open), Some(my_close), Some(my_read), Some(my_seek), 2048i32) {
        rfmod::Result::Ok => {}
        e => {
            panic!("FmodSys.set_file_system failed : {}", e);
        }
    };

    println!("============================================================================");
    println!("================ File Callbacks Example from FMOD examples =================");
    println!("============================================================================");

    let arg1 = tmp.get(0).unwrap();
    let sound = match fmod.create_stream((*arg1).as_slice(),
        Some(rfmod::FmodMode(rfmod::FMOD_2D | rfmod::FMOD_HARDWARE | rfmod::FMOD_LOOP_OFF)), None)
    {
        Ok(s) => s,
        Err(e) => panic!("create sound error: {}", e)
    };

    let chan = match sound.play() {
        Ok(c) => c,
        Err(e) => panic!("sound.play error: {}", e)
    };

    let length = sound.get_length(rfmod::FMOD_TIMEUNIT_MS).unwrap();
    while chan.is_playing().unwrap() {
        let position = chan.get_position(rfmod::FMOD_TIMEUNIT_MS).unwrap();

        print!("{:02}:{:02} / {:02}:{:02}\r", position / 1000 / 60, position / 1000 % 60, length / 1000 / 60, length / 1000 % 60);
        sleep(Duration::milliseconds(30))
    }
}