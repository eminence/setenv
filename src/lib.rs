//! It's well known that a program cannot modify the environment of its parent shell.
//! But this is useful to do, and we can use some tricks to do this.  Almost all shells support
//! some way to evaluate the output of programs (even Windows), so by returning the right commands
//! to be eval'd by the parent shell, we can apply these changes.
//!
//! This library provides two things:
//!
//! 1. Some code to try to detect what shell is in use
//! 2. What syntax is needed to do certain actions.
//!
//! At the moment, the only two commands supported are `cd` for changing directories, and `setenv`
//! for setting environment variables.
//!
//! Two other functions are also provided as a convienence: `split_env` which is just a wrapper
//! around std::env::split_paths, and `set_env_list` which is a wrapper around
//! std::env::join_paths.
//!

use std::ffi::{OsString, OsStr};
use std::convert::AsRef;
use std::env::var_os;


pub enum Shell {
    Windows,
    Bash,
    Tcsh,
    Zsh,
    Ksh,
}

/// Figure out what shell we are using.  If we can't figure it out, fallback to `Bash`, since many
/// shells support the same `export foo=bar` syntax from bash.
pub fn get_shell() -> Shell {
    if cfg!(windows) {
        Shell::Windows
    } else {

        if let Some(shell) = var_os("BASH") {
            if shell.to_string_lossy().ends_with("/bash") {
                return Shell::Bash;
            }
        }
        if let Some(zsh) = var_os("ZSH_NAME") {
            if zsh.to_string_lossy() == "zsh" {
                return Shell::Zsh;
            }
        }
        if let Some(shell) = var_os("shell") {
            if shell.to_string_lossy().ends_with("/tcsh") {
                return Shell::Tcsh;
            }
        }
        return match var_os("SHELL") {
            None => Shell::Bash,
            Some(oss) => {
                if oss.to_string_lossy().ends_with("/bash") {
                    Shell::Bash
                } else if oss.to_string_lossy().ends_with("/ksh") {
                    Shell::Ksh
                } else if oss.to_string_lossy().ends_with("/zsh") {
                    Shell::Zsh
                } else if oss.to_string_lossy().ends_with("/tcsh") {
                    Shell::Tcsh
                } else {
                    Shell::Bash
                } // many shells support export foo=bar
            }
        };
    }
}

impl Shell {
    /// Prints to stdout the necessary command to change directory.
    pub fn cd<P: AsRef<OsStr>>(&self, p: P) {
        match *self {
            Shell::Windows => {
                println!("cd /d {}", p.as_ref().to_string_lossy());
            }
            _ => {
                println!("cd '{}';", p.as_ref().to_string_lossy());
            }
        }
    }

    /// Prints to stdout the necessary command to set an envionrment variable
    pub fn setenv<K: AsRef<OsStr>, V: AsRef<OsStr>>(&self, k: K, v: V) {
        match *self {
            Shell::Windows => {
                println!("set {}={}",
                         k.as_ref().to_string_lossy(),
                         v.as_ref().to_string_lossy())
            }
            Shell::Tcsh => {
                println!("setenv {} '{}';",
                         k.as_ref().to_string_lossy(),
                         v.as_ref().to_string_lossy())
            }
            _ => {
                println!("export {}='{}';",
                         k.as_ref().to_string_lossy(),
                         v.as_ref().to_string_lossy())
            }
        }
    }

    /// A simple wrapper around `std::env::split_paths`
    pub fn split_env<K: AsRef<OsStr>>(&self, k: K) -> Vec<OsString> {
        match std::env::var(k) {
            Err(..) => Vec::new(),
            Ok(ref v) => std::env::split_paths(v).map(|p| p.into_os_string()).collect(),
        }
    }

    /// A simple wrapper around `std::env::join_paths` and `setenv`
    pub fn setenv_list<K, I, T>(&self, k: K, v: I)
        where K: AsRef<OsStr>,
              I: IntoIterator<Item = T>,
              T: AsRef<OsStr>
    {
        let paths = std::env::join_paths(v).unwrap();
        self.setenv(k, paths);
    }
}
