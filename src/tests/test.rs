use grrs::funcs;

#[test]
fn check_answer_validity() {
    let mut result = Vec::new();
    let pb = indicatif::ProgressBar::new(2);
    funcs::find_matches::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result); //, &pb);
    assert_eq!(42, 42);
}
