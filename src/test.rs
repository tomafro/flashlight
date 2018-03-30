extern crate test;

use self::test::Bencher;
use super::*;
use std::io::{self, BufRead, BufReader, Write};
use std::str;

#[test]
fn test_matching_simple_request() {
    let log = indoc!(
        r#"
        [web] [a123b…] Aardvaark
        [web] [a123b…] Buffalo
        [web] [a123b…] Chinchilla
    "#
    );

    assert_eq!(log, match_pattern("Aardvaark", log));
    assert_eq!(log, match_pattern("Buffalo", log));
    assert_eq!(log, match_pattern("Chinchilla", log));

    assert_eq!("", match_pattern("Dolphin", log));
}

#[test]
fn test_match_interleaved_requests() {
    let mammals = indoc!(
        r#"
        [web] [a123b…] Aardvaark
        [web] [a123b…] Buffalo
        [web] [a123b…] Chinchilla
    "#
    );

    let fish = indoc!(
        r#"
        [web] [b123d…] Albacore
        [web] [b123d…] Beluga
        [web] [b123d…] Carp
    "#
    );

    let log = indoc!(
        r#"
        [web] [a123b…] Aardvaark
        [web] [b123d…] Albacore
        [web] [b123d…] Beluga
        [web] [a123b…] Buffalo
        [web] [b123d…] Carp
        [web] [a123b…] Chinchilla
    "#
    );

    assert_eq!(mammals, match_pattern("Aardvaark", log));
    assert_eq!(mammals, match_pattern("Buffalo", log));
    assert_eq!(mammals, match_pattern("Chinchilla", log));

    assert_eq!(fish, match_pattern("Albacore", log));
    assert_eq!(fish, match_pattern("Beluga", log));
    assert_eq!(fish, match_pattern("Carp", log));
}

#[test]
fn test_match_context_requests() {
    let web = indoc!(
        r#"
        [web] [a123b…] Aardvaark
        [web] [a123b…] Buffalo
        [web] [a123b…] Chinchilla
    "#
    );

    let assets = indoc!(
        r#"
        [assets] [b123d…] Albacore
        [assets] [b123d…] Beluga
        [assets] [b123d…] Carp
    "#
    );

    let log = indoc!(
        r#"
        [web] [a123b…] Aardvaark
        [assets] [b123d…] Albacore
        [assets] [b123d…] Beluga
        [web] [a123b…] Buffalo
        [assets] [b123d…] Carp
        [web] [a123b…] Chinchilla
    "#
    );

    assert_eq!(web, match_contexts(vec![Context::Web], log));
    assert_eq!(assets, match_contexts(vec![Context::Asset], log));
    assert_eq!(log, match_contexts(vec![], log));
    assert_eq!(log, match_contexts(vec![Context::Web, Context::Asset], log));
}

#[test]
fn test_match_without_request_id() {
    let log = indoc!(
        r#"
        [no-account] [rake] [none] Aardvaark
        [web] [a123b…] Buffalo
        [no-account] [rake] [none] Chinchilla
    "#
    );

    assert_eq!(
        "[no-account] [rake] [none] Aardvaark\n",
        match_pattern("Aardvaark", log)
    );
    assert_eq!(
        "[no-account] [rake] [none] Chinchilla\n",
        match_pattern("Chinchilla", log)
    );
}

#[bench]
fn bench_parse_1000_lines_of_generated_log(b: &mut Bencher) {
    let log = include_str!("test/1000-lines.log");

    b.iter(|| {
        let config = Config::default().matching("Buffalo");
        run_flashlight(config, log)
    });
}

// #[bench]
// fn bench_parse_100_000_lines_of_generated_log(b: &mut Bencher) {
//     let log = include_str!("test/100_000-lines.log");

//     b.iter(|| {
//         let config = Config::default().matching("Buffalo");
//         run_flashlight(config, log)
//     });
// }

fn match_pattern(pattern: &str, log: &str) -> String {
    let config = Config::default().matching(pattern);
    run_flashlight(config, log)
}

fn match_contexts(contexts: Vec<Context>, log: &str) -> String {
    let config = Config::default().match_contexts(contexts);
    run_flashlight(config, log)
}

fn run_flashlight(config: Config, log: &str) -> String {
    let reader = BufReader::new(log.as_bytes());
    let mut output: Vec<u8> = Vec::new();
    run(config, reader, &mut output);
    str::from_utf8(&output).unwrap().to_string()
}
