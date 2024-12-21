use super::*;

#[test]
fn test_isin() {
    assert!(true);
    let b = Beam {
        dir: (-1, 0),
        cur: (23, 12),
    };
    println!("{:?}", &b);
    let hist = vec![b];
    println!("{:?}", &hist);
    assert!(Beam::is_in_hist(&hist, 23, 12, (-1, 0)));
}

#[test]
fn test_notin() {
    let b = Beam {
        dir: (-1, 0),
        cur: (23, 12),
    };
    println!("{:?}", &b);
    let hist = vec![b];
    println!("{:?}", &hist);
    assert!(!Beam::is_in_hist(&hist, 3, 12, (-1, 0)));
}
