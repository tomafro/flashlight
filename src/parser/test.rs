extern crate test;
extern crate serde_json;

use super::*;
use parser::test::serde_json::Value;
use self::test::Bencher;

#[test]
fn test_examples() {
    let json: Value = serde_json::from_str(include_str!("examples.json")).unwrap();
    let examples = json.as_object().unwrap();

    for key in examples.keys() {
        let line = Line::from(key);

        assert_eq!(key, line.content());
        assert_eq!(examples[key]["context"].as_str().unwrap(), line.context().string());
        assert_eq!(&Some(examples[key]["request_id"].as_str().unwrap().to_string()), line.request_id());
    }
}

#[bench]
fn bench_without_parsing(b: &mut Bencher) {
    let json: Value = serde_json::from_str(include_str!("examples.json")).unwrap();
    let examples = json.as_object().unwrap();

    b.iter(|| {
        for key in examples.keys() {
            key;
        }
    });
}

#[bench]
fn bench_with_parsing(b: &mut Bencher) {
    let json: Value = serde_json::from_str(include_str!("examples.json")).unwrap();
    let examples = json.as_object().unwrap();

    b.iter(|| {
        for key in examples.keys() {
            Line::from(key);
        }
    });
}
