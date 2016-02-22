#[cfg(test)]
extern crate tempdir;

use std::ffi::OsStr;
use std::convert::AsRef;
use std::env::var_os;


pub enum Shell {
    Windows,
    Bash,
    Tcsh,
    Zsh,
    Ksh
}

pub fn get_shell() -> Shell {
    if cfg!(windows) {
        Shell::Windows
    } else {

        if let Some(shell) = var_os("BASH") {
            if shell.to_string_lossy().ends_with("/bash") { return Shell::Bash }
        }
        if let Some(zsh) = var_os("ZSH_NAME") {
            if zsh.to_string_lossy() == "zsh" { return Shell::Zsh }
        }
        if let Some(shell) = var_os("shell") {
            if shell.to_string_lossy().ends_with("/tcsh") { return Shell::Tcsh }
        }
        return match var_os("SHELL") {
            None => Shell::Bash,
            Some(oss) => {
                if oss.to_string_lossy().ends_with("/bash") { Shell::Bash }
                else if oss.to_string_lossy().ends_with("/ksh") { Shell::Ksh }
                else if oss.to_string_lossy().ends_with("/zsh") { Shell::Zsh }
                else if oss.to_string_lossy().ends_with("/tcsh") { Shell::Tcsh }
                else { Shell::Bash } // many shells support export foo=bar
            }
        }
    }
}

impl Shell {
    pub fn cd<P: AsRef<OsStr>>(&self, p: P) {
        match *self {
            Shell::Windows => { println!("cd /d {}", p.as_ref().to_string_lossy()); }
            _ => { println!("cd '{}';", p.as_ref().to_string_lossy()); }
        }
    }

    pub fn setenv<K: AsRef<OsStr>, V: AsRef<OsStr>>(&self, k: K, v: V) {
        match *self {
            Shell::Windows => {        println!("set {}={}", k.as_ref().to_string_lossy(), v.as_ref().to_string_lossy()) }
            Shell::Tcsh => {         println!("setenv {} '{}';", k.as_ref().to_string_lossy(), v.as_ref().to_string_lossy()) }
            _ => {         println!("export {}='{}';", k.as_ref().to_string_lossy(), v.as_ref().to_string_lossy()) }
        }
    }
}

