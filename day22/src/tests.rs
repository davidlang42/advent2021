use super::*;

#[test]
fn segments_1d_overlap_full_inner() {
    let res = Cubeoid::segments_1d(1,6,3,4);
    assert_eq!(res, [(1,2),(3,4),(5,6)]);
}

#[test]
fn segments_1d_overlap_full_outer() {
    let res = Cubeoid::segments_1d(1,6,0,7);
    assert_eq!(res, [(1,6)]);
}

#[test]
fn segments_1d_overlap_match() {
    let res = Cubeoid::segments_1d(1,6,1,6);
    assert_eq!(res, [(1,6)]);
}

#[test]
fn segments_1d_overlap_left() {
    let res = Cubeoid::segments_1d(1,6,0,3);
    assert_eq!(res, [(1,3),(4,6)]);
}

#[test]
fn segments_1d_overlap_right() {
    let res = Cubeoid::segments_1d(1,6,4,7);
    assert_eq!(res, [(1,3),(4,6)]);
}

#[test]
fn segments_1d_no_overlap_left() {
    let res = Cubeoid::segments_1d(1,6,-1,0);
    assert_eq!(res, [(1,6)]);
}

#[test]
fn segments_1d_no_overlap_right() {
    let res = Cubeoid::segments_1d(1,6,7,8);
    assert_eq!(res, [(1,6)]);
}

#[test]
fn segments_1d_overlap_left_border() {
    let res = Cubeoid::segments_1d(1,6,1,1);
    assert_eq!(res, [(1,1),(2,6)]);
}

#[test]
fn segments_1d_overlap_right_border() {
    let res = Cubeoid::segments_1d(1,6,6,6);
    assert_eq!(res, [(1,5),(6,6)]);
}