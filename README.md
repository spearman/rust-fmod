rust-fmod
=========

This is a rust binding for FMOD, developped by FIRELIGHT TECHNOLOGIES.

FMOD website : www.fmod.org


##Installation

You must install on your computer the FMOD library which is used for the binding.

To build it, please use :

```Shell
> make
```

This command build rfmod, the examples, and the documentation.

You can build them separatly too.

```Shell
> make rfmod
> make examples
> make docs
```

##Short example

Here is a short example on how to create a file and to play it :

```Rust
#![feature(globs)]

extern crate libc;
extern crate rfmod;

use rfmod::enums::FMOD_OK;
use rfmod::*;
use std::os;

fn main() {
    let fmod = match FmodSys::new() {
        Ok(f) => f,
        Err(e) => {
       	    fail!("Error code : {}", e);
        }
    };

    match fmod.init() {
        FMOD_OK => {}
        e => {
            fmod.release();
            fail!("FmodSys.init failed : {}", e);
        }
    };

    let mut sound = match fmod.create_sound(StrBuf::from_str("music.mp3"), None, None) {
		              Ok(s) => s,
		              Err(err) => {fail!("Error code : {}", err);},
		            };

    match sound.play_to_the_end() {
        FMOD_OK => {println!("Ok !");}
        err => {fail!("Error code : {}", err);}
    };

    sound.release();
    fmod.release();
}
```

##License

    Copyright (c) 2014 Guillaume Gomez
    
    The license of this project is the same of the FMOD non-commercial use. 
    Please refer to it. Here is the website for FMOD : http://www.fmod.org/
