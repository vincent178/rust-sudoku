use std::collections::HashMap;
use std::fmt;

fn main() {
    let game = Game::new();

    println!("{}", game);

    game.solve();

    game.validate(1, 3);
}

struct Game {
    states: Vec<Vec<i8>>,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let m = HashMap::from([
            (0, "⬜"),
            (1, "1️ "),
            (2, "2️ "),
            (3, "3️ "),
            (4, "4️ "),
            (5, "5️ "),
            (6, "6️ "),
            (7, "7️ "),
            (8, "8️ "),
            (9, "9️ "),
        ]);

        for x in &self.states {
            for y in x {
                let s = m.get(y).expect("invalid input");
                write!(f, "{} ", s)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

impl Game {
    fn new() -> Game {
        //           [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let l1 = vec![0, 0, 0, 0, 0, 8, 0, 0, 0];
        let l2 = vec![0, 2, 5, 9, 0, 4, 0, 1, 0];
        let l3 = vec![0, 0, 0, 3, 0, 0, 0, 0, 9];
        let l4 = vec![7, 0, 0, 0, 0, 0, 3, 0, 6];
        let l5 = vec![1, 9, 6, 0, 0, 0, 0, 0, 4];
        let l6 = vec![0, 0, 0, 0, 0, 0, 5, 7, 0];
        let l7 = vec![0, 0, 7, 0, 0, 6, 0, 4, 0];
        let l8 = vec![0, 0, 3, 0, 0, 0, 6, 0, 5];
        let l9 = vec![0, 0, 0, 1, 0, 0, 0, 0, 0];

        Game {
            states: vec![l1, l2, l3, l4, l5, l6, l7, l8, l9],
        }
    }

    #[allow(dead_code)]
    fn solve(&self) {
        // validate and find possible number could merge into one loop
        for x in &self.states {
            for y in x {
                print!("{}", y);
            }
            print!("\n");
        }
    }

    #[allow(dead_code)]
    fn validate(&self, i: usize, j: usize) {
        // if it is empty, then try 1 - 9
        // if it is not empty, then validate if it is ok
        let n = self.states.get(i).unwrap().get(j).unwrap();
        print!("{}", n);


    }
}

/*
 * how I solve Soduku puzzle?
 *   1. validate if Soduku state, if invalid pop from stack
 *   2. scan the the empty fields, get all the possibility
 *   3. find the lowest possibility field, list the possibility and push all into a stack
 *   4. loop 1,2,3 until all state match, print it out
 */
