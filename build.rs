extern crate pkg_config;
extern crate vergen;

use std::process::{Command};
use vergen::*;

fn main() {
    let mut flags = OutputFns::all();
    flags.toggle(NOW);
    assert!(vergen(flags).is_ok());

    let has_pkgconfig = Command::new("pkg-config").output().is_ok();

    if has_pkgconfig && pkg_config::find_library("libtokyocabinet").is_ok() {
        return;
    } else {
	// should exit with error
    }
}