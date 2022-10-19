#!/usr/bin/env rust-script
// cargo-deps: nix = "0.25.0"

fn main() {
    loop {
        nix::unistd::getppid();
    }
}
