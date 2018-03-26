extern crate test;

use super::*;
use std::io::{self, BufRead, BufReader, Write};
use std::str;

#[test]
fn test_matching_simple_request() {
    let log = indoc!(r#"
        [web] [a123b…] Aardvaark
        [web] [a123b…] Buffalo
        [web] [a123b…] Chinchilla
    "#);

    assert_eq!(log, match_pattern("Aardvaark", log));
    assert_eq!(log, match_pattern("Buffalo", log));
    assert_eq!(log, match_pattern("Chinchilla", log));

    assert_eq!("", match_pattern("Dolphin", log));
}

#[test]
fn test_match_interleaved_requests() {
    let mammals = indoc!(r#"
        [web] [a123b…] Aardvaark
        [web] [a123b…] Buffalo
        [web] [a123b…] Chinchilla
    "#);

    let fish = indoc!(r#"
        [web] [b123d…] Albacore
        [web] [b123d…] Beluga
        [web] [b123d…] Carp
    "#);

    let log = indoc!(r#"
        [web] [a123b…] Aardvaark
        [web] [b123d…] Albacore
        [web] [b123d…] Beluga
        [web] [a123b…] Buffalo
        [web] [b123d…] Carp
        [web] [a123b…] Chinchilla
    "#);

    assert_eq!(mammals, match_pattern("Aardvaark", log));
    assert_eq!(mammals, match_pattern("Buffalo", log));
    assert_eq!(mammals, match_pattern("Chinchilla", log));

    assert_eq!(fish, match_pattern("Albacore", log));
    assert_eq!(fish, match_pattern("Beluga", log));
    assert_eq!(fish, match_pattern("Carp", log));
}

fn match_pattern(pattern: &str, log: &str) -> String {
    let args = Args{ flag_pattern: pattern.to_string(), flag_file: None, flag_buffer_size: 10_000, flag_web: true, flag_cable: true, flag_jobs: true, flag_assets: true};
    let reader = BufReader::new(log.as_bytes());
    let mut output: Vec<u8> = Vec::new();
    run(args, reader, &mut output);
    str::from_utf8(&output).unwrap().to_string()
}
