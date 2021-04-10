#[rustversion::attr(not(nightly), ignore)]
#[test]
fn docbox() {
    trybuild::TestCases::new().compile_fail("tests/ui/docbox/*.rs");
}

#[rustversion::attr(not(nightly), ignore)]
#[test]
fn short_docbox() {
    trybuild::TestCases::new().compile_fail("tests/ui/short_docbox/*.rs");
}

#[rustversion::attr(not(nightly), ignore)]
#[test]
fn since() {
    trybuild::TestCases::new().compile_fail("tests/ui/since/*.rs");
}
