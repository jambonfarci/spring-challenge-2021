use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Debug)]
struct Player {
    sun: i32,
    score: i32,
    waiting: i32
}

#[derive(Debug)]
struct Cell {
    index: i32,
    richness: i32,
    tree: Option<Tree>
}

#[derive(Debug)]
struct Game {
    day: i32,
    nutrients: i32,
    number_of_trees: i32,
    cells: Vec<Cell>
}

#[derive(Debug)]
struct Tree {
    size: i32,
    is_mine: i32,
    is_dormant: i32
}

impl Player {
    fn sim_complete(&self, index: usize, game: &Game) -> i32 {
        let mut score = 0;

        if self.sun >= 4 {
            score = game.nutrients + match game.cells[index].richness {
                1 => 0,
                2 => 2,
                3 => 4,
                _ => panic!("")
            };
        }

        score
    }

    // Compléter le cycle de vie d'un arbre coûte 4 points de soleil.
    fn complete(&mut self, index: usize, game: &mut Game) {
        if self.sun >= 4 {
            self.score += game.nutrients + match game.cells[index].richness {
                1 => 0,
                2 => 2,
                3 => 4,
                _ => panic!("")
            };

            game.nutrients -= 1;
            game.cells[index].tree = None;
        }
    }
}

// Write an action using println!("message...");
// To debug: eprintln!("Debug message...");
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let number_of_cells = parse_input!(input_line, i32); // 37
    let mut cells: Vec<Cell> = Vec::new();

    for _i in 0..number_of_cells as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let index = parse_input!(inputs[0], i32); // 0 is the center cell, the next cells spiral outwards
        let richness = parse_input!(inputs[1], i32); // 0 if the cell is unusable, 1-3 for usable cells
        let neigh_0 = parse_input!(inputs[2], i32); // the index of the neighbouring cell for each direction
        let neigh_1 = parse_input!(inputs[3], i32);
        let neigh_2 = parse_input!(inputs[4], i32);
        let neigh_3 = parse_input!(inputs[5], i32);
        let neigh_4 = parse_input!(inputs[6], i32);
        let neigh_5 = parse_input!(inputs[7], i32);

        cells.push(Cell {
            index: index,
            richness: richness,
            tree: None
        });
    }

    let mut game = Game {
        day: 0,
        nutrients: 20,
        number_of_trees: 0,
        cells
    };

    let mut player = Player {
        sun: 0,
        score: 0,
        waiting: 0
    };

    let mut opponent = Player {
        sun: 0,
        score: 0,
        waiting: 0
    };

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();

        let day = parse_input!(input_line, i32); // the game lasts 24 days: 0-23
        game.day = day;

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();

        let nutrients = parse_input!(input_line, i32); // the base score you gain from the next COMPLETE action
        game.nutrients = nutrients;

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();

        let sun = parse_input!(inputs[0], i32); // your sun points
        player.sun = sun;

        let score = parse_input!(inputs[1], i32); // your current score
        player.score = score;

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();

        let opp_sun = parse_input!(inputs[0], i32); // opponent's sun points
        opponent.sun = opp_sun;

        let opp_score = parse_input!(inputs[1], i32); // opponent's score
        opponent.score = opp_score;

        let opp_is_waiting = parse_input!(inputs[2], i32); // whether your opponent is asleep until the next day
        opponent.waiting = opp_is_waiting;

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();

        let number_of_trees = parse_input!(input_line, i32); // the current amount of trees
        game.number_of_trees = number_of_trees;

        for _i in 0..number_of_trees as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();

            let cell_index = parse_input!(inputs[0], i32); // location of this tree
            let size = parse_input!(inputs[1], i32); // size of this tree: 0-3
            let is_mine = parse_input!(inputs[2], i32); // 1 if this is your tree
            let is_dormant = parse_input!(inputs[3], i32); // 1 if this tree is dormant

            let tree = Tree {
                size: size,
                is_mine: is_mine,
                is_dormant: is_dormant
            };

            for (pos, c) in game.cells.iter().enumerate() {
                if c.index == cell_index {
                    game.cells[pos].tree = Some(tree);
                    break;
                }
            }
        }

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let number_of_possible_moves = parse_input!(input_line, i32);

        for _i in 0..number_of_possible_moves as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let possible_move = input_line.trim_matches('\n').to_string();
        }

        eprintln!("{:?}", game);
        let mut complete_index = 0;
        let mut score = 0;

        for c in &game.cells {
            let new_score = match &c.tree {
                Some(t) => {
                    if t.is_mine == 1 {
                        player.sim_complete(c.index as usize, &game)
                    } else {
                        0
                    }
                },
                None => 0
            };

            if new_score > score {
                score = new_score;
                complete_index = c.index;
            }
        }

        player.complete(complete_index as usize, &mut game);

        // GROW cellIdx | SEED sourceIdx targetIdx | COMPLETE cellIdx | WAIT <message>
        println!("COMPLETE {}", complete_index);
    }
}
