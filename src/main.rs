use std::vec;

fn main() {
    let mut vect = Vector::new();
    let mut vect_premier = Vector::new();

    vect.add(5);
    vect.add(10);
    vect.add(7);
    vect.add(8);
    vect.add(33);
    vect.add(2);
    vect.add(1);
    vect.add(12);

    vect.display();

    vect.remove(8);
    vect.remove(10);

    vect.display();

    vect_premier = vect.no_premiers();
    vect_premier.display();
}

struct Vector {
    tab: Vec<i32>,
}

impl Vector {
    fn new() -> Vector {
        Vector { tab: vec![] }
    }

    fn add(&mut self, value: i32) {
        let mut pozitie: usize = 0;

        for index in 0..self.tab.len() {
            if self.tab[index] < value {
                pozitie += 1;
            }
        }
        self.tab.insert(pozitie, value);
    }

    fn remove(&mut self, val: i32) {
        let mut pozitie = 0;
        let mut boolean = false;
        for index in 0..self.tab.len() {
            if self.tab[index] == val {
                pozitie = index;
                boolean = true;
            }
        }
        if boolean == true {
            self.tab.remove(pozitie);
        }
    }

    fn display(&mut self) {
        for index in 0..self.tab.len() {
            print!("{} ", self.tab[index]);
        }
        println!();
    }

    fn no_premiers(&mut self) -> Vector {
        let mut nr_div;
        let mut vect = Vector::new();
        for index in 0..self.tab.len() {
            nr_div = 0;
            for indexdiv in 2..self.tab[index] / 2 {
                if self.tab[index] % indexdiv == 0 {
                    nr_div += 1;
                }
            }
            if nr_div == 0 && self.tab[index] != 1 {
                vect.add(self.tab[index]);
            }
        }
        return vect;
    }

    fn no_compris_entre(&mut self, min: i32, max: i32) ->Vector {
        let mut vect = Vector::new();
        for index in 0..self.tab.len() {
            if self.tab[index] > min && self.tab[index] < max {
                vect.add(self.tab[index]);
            }
        }
        return vect;
    }
}

#[cfg(test)]
mod tests;

// {
//     use super::*;
//     #[test]
//     fn this_adds() {
//         let mut vect = Vector::new();
//         vect.add(3);

//         assert_eq!(3, vect.tab[0]);
//     }

//     #[test]
//     fn this_removes() {
//         let mut vect = Vector::new();
//         vect.add(3);
//         vect.add(2);
//         vect.add(5);

//         vect.remove(2);
//         assert_eq!(3, vect.tab[0]);
//     }

//     #[test]
//     fn this_shows_premiers() {
//         let mut vect = Vector::new();
//         vect.add(2);
//         vect.add(1);
//         vect.add(22);
//         vect.add(37);
//         vect.add(12);

//         let mut vect_pr = Vector::new();
//         vect_pr = vect.no_premiers();

//         assert_eq!(2, vect_pr.tab[0]);
//         //assert_eq!(4,vect_pr.tab[1]);
//         assert_eq!(37, vect_pr.tab[1]);
//     }

//     #[test]
//     fn this_shows_compris() {
//         let mut vect = Vector::new();

//         vect.add(5);
//         vect.add(10);
//         vect.add(7);
//         vect.add(8);
//         vect.add(33);
//         vect.add(2);
//         vect.add(1);
//         vect.add(12);

//         let mut vect_compr = Vector::new();
//         vect_compr=vect.no_compris_entre(1, 7);

//         assert_eq!(2, vect_compr.tab[0]);
//         assert_eq!(5,vect_compr.tab[1]);
//     }
// }
