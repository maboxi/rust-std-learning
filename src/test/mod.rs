use log::trace;

mod droptest;
pub use droptest::DropTest;

pub fn test_repeated<F, R>(name: &str, iterations: usize, test: F) -> Vec<R>
where
    F: Fn(usize) -> R,
{
    assert!(
        iterations > 0,
        "Test {name} has to be repeated at least once!"
    );
    (0..iterations).map(|iteration| {
        trace!(
            "---------------------- {name} Test Iteration {iteration} start ----------------------"
        );
        let res = test(iteration);
        trace!(
            "---------------------- {name} Test Iteration {iteration} end ----------------------"
        );
        res
    }).collect()
}
