// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::env;
use std::ffi::CString;
use std::ptr;

use hcore::env as henv;
use hcore::fs::find_command;
use libc;

use error::{Error, Result};
use util::path::busybox_paths;

/// Our output key
static LOGKEY: &'static str = "SH";

/// Start a bash shell
pub fn bash() -> Result<()> {
    try!(set_path());
    outputln!("Starting your bashlike shell; enjoy!");
    exec_shell("bash")
}

/// Start a sh shell
pub fn sh() -> Result<()> {
    try!(set_path());
    outputln!("Starting your bourne shell; enjoy!");
    exec_shell("sh")
}

fn set_path() -> Result<()> {
    let mut paths = String::new();
    for path in try!(busybox_paths()).iter() {
        if !paths.is_empty() {
            paths.push(':');
        }
        paths.push_str(&path.to_string_lossy());
    }
    if let Some(val) = henv::var_os("PATH") {
        paths.push(':');
        paths.push_str(&val.to_string_lossy());
    }
    debug!("Setting the PATH to {}", &paths);
    env::set_var("PATH", &paths);
    Ok(())
}

fn exec_shell(cmd: &str) -> Result<()> {
    let cmd_path = match find_command(cmd) {
        Some(p) => p,
        None => return Err(sup_error!(Error::ExecCommandNotFound(cmd.to_string()))),
    };
    let c_cmd = try!(CString::new(cmd_path.to_string_lossy().into_owned()));
    let mut argv = [c_cmd.as_ptr(), ptr::null()];
    debug!("Exec {:?}", &cmd_path.display());
    unsafe {
        libc::execvp(c_cmd.as_ptr(), argv.as_mut_ptr());
    }
    Ok(())
}
