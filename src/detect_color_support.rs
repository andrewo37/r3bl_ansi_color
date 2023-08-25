/*
 *   Copyright (c) 2023 R3BL LLC
 *   All rights reserved.
 *
 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
 */

use std::env;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

static COLOR_SUPPORT_SET_VALUE_MUTEX_LOCK: Lazy<Arc<Mutex<i8>>> = Lazy::new(|| {
    Arc::new(Mutex::new(-1 as i8))
});


/// Global [ColorSupport] override.
// static mut COLOR_SUPPORT_SET_VALUE: AtomicI8 =
//     AtomicI8::new(ColorSupport::NotSet as i8);

#[test]
fn test_supports_color() {
    let level = supports_color(Stream::Stdout);
    println!("🍎🍎🍎  supports_color: {:?}", level);
}

/// The stream to check for color support.
#[derive(Clone, Copy, Debug)]
pub enum Stream {
    Stdout,
    Stderr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum ColorSupport {
    Ansi256,
    Truecolor,
    NoColor,
    NotSet,
}

mod convert_between_color_and_i8 {
    impl From<i8> for super::ColorSupport {
        fn from(value: i8) -> Self {
            match value {
                1 => super::ColorSupport::Ansi256,
                2 => super::ColorSupport::Truecolor,
                3 => super::ColorSupport::NoColor,
                _ => super::ColorSupport::NotSet,
            }
        }
    }

    impl From<super::ColorSupport> for i8 {
        fn from(value: super::ColorSupport) -> Self {
            match value {
                super::ColorSupport::Ansi256 => 1,
                super::ColorSupport::Truecolor => 2,
                super::ColorSupport::NoColor => 3,
                _ => -1,
            }
        }
    }
}

pub fn supports_color(stream: Stream) -> ColorSupport {
    if env_no_color()
        || as_str(&env::var("TERM")) == Ok("dumb")
        || !(is_a_tty(stream) || env::var("IGNORE_IS_TERMINAL").map_or(false, |v| v != "0"))
    {
        return ColorSupport::NoColor;
    }

    if env::consts::OS == "macos" {
        if as_str(&env::var("TERM_PROGRAM")) == Ok("Apple_Terminal")
            && env::var("TERM").map(|term| check_256_color(&term)) == Ok(true)
        {
            return ColorSupport::Ansi256;
        }

        if as_str(&env::var("TERM_PROGRAM")) == Ok("iTerm.app")
            || as_str(&env::var("COLORTERM")) == Ok("truecolor")
        {
            return ColorSupport::Truecolor;
        }
    }

    if env::consts::OS == "linux" && as_str(&env::var("COLORTERM")) == Ok("truecolor") {
        return ColorSupport::Truecolor;
    }

    if env::consts::OS == "windows"
    {
        return ColorSupport::Truecolor;
    }

    if env::var("COLORTERM").is_ok()
        || env::var("TERM").map(|term| check_ansi_color(&term)) == Ok(true)
        || env::var("CLICOLOR").map_or(false, |v| v != "0")
        || is_ci::uncached()
    {
        return ColorSupport::Truecolor;
    }

    ColorSupport::NoColor
}

pub fn color_support_override_set(value: ColorSupport) {
    let color_support = Arc::clone(&COLOR_SUPPORT_SET_VALUE_MUTEX_LOCK);
    let mut support_set = color_support.lock().unwrap();
    *support_set = value.into();

    // unsafe {
    //     COLOR_SUPPORT_SET_VALUE.store(value.into(), Ordering::SeqCst);
    // };
}

pub fn color_support_set_get() -> ColorSupport {
    ColorSupport::from(*COLOR_SUPPORT_SET_VALUE_MUTEX_LOCK.lock().unwrap())
    // unsafe { COLOR_SUPPORT_SET_VALUE.load(Ordering::SeqCst).into() }
}

fn is_a_tty(stream: Stream) -> bool {
    use is_terminal::*;
    match stream {
        Stream::Stdout => std::io::stdout().is_terminal(),
        Stream::Stderr => std::io::stderr().is_terminal(),
    }
}

fn check_256_color(term: &str) -> bool {
    term.ends_with("256") || term.ends_with("256color")
}

pub fn check_ansi_color(term: &str) -> bool {
    term.starts_with("screen")
        || term.starts_with("xterm")
        || term.starts_with("vt100")
        || term.starts_with("vt220")
        || term.starts_with("rxvt")
        || term.contains("color")
        || term.contains("ansi")
        || term.contains("cygwin")
        || term.contains("linux")
}

pub fn env_no_color() -> bool {
    match as_str(&env::var("NO_COLOR")) {
        Ok("0") | Err(_) => false,
        Ok(_) => true,
    }
}

fn as_str<E>(option: &Result<String, E>) -> Result<&str, &E> {
    match option {
        Ok(inner) => Ok(inner),
        Err(e) => Err(e),
    }
}
