extern crate crossbeam;

fn fork<'a, F1, R1, F2, R2>(f1: F1, f2: F2) -> (R1, R2)
    where F1: FnOnce() -> R1 + Send + 'a,
          F2: FnOnce() -> R2 + Send + 'a,
          R1: Send + 'a,
          R2: Send + 'a
{
    crossbeam::scope(|scope| {
        let t1 = scope.spawn(move || f1());
        let t2 = scope.spawn(move || f2());
        (t1.join(), t2.join())
    })
}

#[test]
fn test_fork() {
    let (a, b) = fork(|| 1, || 2);
    assert!(a == 1);
    assert!(b == 2);
}
