It's well known that a program cannot modify the environment of its parent shell.
But this is useful to do, and we can use some tricks to do this.  Almost all shells support
some way to evaluate the output of programs (even Windows), so by returning the right commands
to be eval'd by the parent shell, we can apply these changes.
                                                                                               
# Install
                                                                                               
Add the following to your `Cargo.toml` file:
                                                                                               
```toml
[dependencies]
setenv = "0.1"
```
                                                                                               
# Usage
                                                                                               
This library provides two things:
                                                                                               
1. Some code to try to detect what shell is in use
2. What syntax is needed to do certain actions.
                                                                                               
At the moment, the only two commands supported are [`cd`] for changing directories, and
[`setenv`] for setting environment variables.
                                                                                               
Two other functions are also provided as a convienence: `split_env` which is just a wrapper
around std::env::split_paths, and `set_env_list` which is a wrapper around
std::env::join_paths.
                                                                                               
# Examples
                                                                                               
To make use of all this, each executable using `setenv` should be wrapped in an
alias/function/bat file.  Here are some examples:
                                                                                               
## Windows:
                                                                                               
```batch
for /f "tokens=*" %%I in ('d:\target\debug\myapp.exe  %*') do (
  %%I
)
```
                                                                                               
### Bash:
                                                                                               
```bash
function dothing() {
  eval `/target/debug/myapp "$@"`
}
```
                                                                                               
## Ksh:
                                                                                               
```ksh
dothing() {
  eval `/target/debug/myapp "$@"`
}
```
                                                                                               
## Zsh:
                                                                                               
```zsh
function dothing() {
  eval `/target/debug/myapp $*`
}
```
                                                                                               
## Tcsh:
                                                                                               
```csh
alias dothing 'eval `/target/debug/myapp \!*`'
```
                                                                                               
# Notes
                                                                                               
Since all text send to stdout is eval'd by the shell, great care must be taken to control what
is printed to stdout.  All user-facing messages should go to stderr instead.


# License
![http://creativecommons.org/publicdomain/zero/1.0/](https://licensebuttons.net/p/zero/1.0/88x31.png)

To the extent possible under law, Andrew Chin has waived all copyright and related or neighboring rights to setenv. This work is published from: United States.
                                                                                               
[`cd`]: enum.Shell.html#method.cd
[`setenv`]: enum.Shell.html#method.setenv
