use crate::Vector;

use super::*;
#[test]
fn this_adds() {
    let mut vect = Vector::new();
    vect.add(3);

    assert_eq!(3, vect.tab[0]);
}

#[test]
fn this_removes() {
    let mut vect = Vector::new();
    vect.add(3);
    vect.add(2);
    vect.add(5);

    vect.remove(2);
    assert_eq!(3, vect.tab[0]);
}

#[test]
fn this_shows_premiers() {
    let mut vect = Vector::new();
    vect.add(2);
    vect.add(1);
    vect.add(22);
    vect.add(37);
    vect.add(12);

    let mut vect_pr = Vector::new();
    vect_pr = vect.no_premiers();

    assert_eq!(2, vect_pr.tab[0]);
    //assert_eq!(4,vect_pr.tab[1]);
    assert_eq!(37, vect_pr.tab[1]);
}

#[test]
fn this_shows_compris() {
    let mut vect = Vector::new();

    vect.add(5);
    vect.add(10);
    vect.add(7);
    vect.add(8);
    vect.add(33);
    vect.add(2);
    vect.add(1);
    vect.add(12);

    let mut vect_compr = Vector::new();
    vect_compr = vect.no_compris_entre(1, 7);

    assert_eq!(2, vect_compr.tab[0]);
    assert_eq!(5, vect_compr.tab[1]);
}
