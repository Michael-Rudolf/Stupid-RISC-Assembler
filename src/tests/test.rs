use crate::tests::math_test;

pub trait Test{
    fn execute() -> bool;
}

pub fn test_all() -> bool{
    let math_tests_pass = math_test::MathTest::execute();
    return math_tests_pass;
}