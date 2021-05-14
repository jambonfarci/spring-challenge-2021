use std::io;
use rand::Rng;
use std::cmp;
use std::time::{Duration, Instant};

const POPULATION_SIZE: usize = 30;
const ACTIONS_COUNT: usize = 3;
const MAX_DEPTH: usize = 10;
const ACTION_TYPES: [&str;3] = ["GROW", "SEED", "COMPLETE"];

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Debug)]
struct Player {
    id: i32,
    sun: i32,
    rollback_sun: i32,
    score: i32,
    rollback_score: i32,
    waiting: i32,
    rollback_waiting: i32,
    number_of_trees_0 : i32,
    rollback_number_of_trees_0: i32,
    number_of_trees_1 : i32,
    rollback_number_of_trees_1: i32,
    number_of_trees_2 : i32,
    rollback_number_of_trees_2: i32,
    number_of_trees_3 : i32,
    rollback_number_of_trees_3: i32,
}

#[derive(Debug)]
struct Cell {
    index: i32,
    x: i32,
    y: i32,
    richness: i32,
    rollback_richness: i32,
    tree: Option<Tree>,
    rollback_tree: Option<Tree>,
    neighbours: [Option<i32>;6],
    shadow_size: i32,
    rollback_shadow_size: i32
}

#[derive(Debug)]
struct Game {
    day: i32,
    rollback_day: i32,
    nutrients: i32,
    rollback_nutrients: i32,
    number_of_trees: i32,
    cells: Vec<Cell>
}

#[derive(Debug)]
struct Tree {
    size: i32,
    is_mine: i32,
    is_dormant: i32
}

#[derive(Debug)]
struct Action {
    name: String,
    index1: i32,
    index2: i32
}

#[derive(Debug)]
struct Individual {
    player_id: i32,
    actions: Vec<Vec<Action>>,
    fitness: f64
}

impl Player {
    fn gather_sun(&mut self, game: &Game) {
        for c in &game.cells {
            match &c.tree {
                Some(t) => {
                    match t.is_mine {
                        1 => {
                            if c.shadow_size < t.size {
                                self.sun += t.size;
                            }
                        },
                        0 => (),
                        _ => panic!("")
                    }
                },
                None => ()
            }
        }
    }

    // Compléter le cycle de vie d'un arbre coûte 4 points de soleil.
    fn complete(&mut self, index: usize, game: &mut Game) -> i32 {
        match &game.cells[index].tree {
            None => return -10,
            Some(t) => {
                match t.is_mine {
                    0 => -10,
                    1 => {
                        match t.size {
                            3 => {
                                if self.sun >= 4 {
                                    self.sun -= 4;
                                    self.number_of_trees_3 -= 1;

                                    self.score += game.nutrients + match game.cells[index].richness {
                                        1 | 0 => 0,
                                        2 => 2,
                                        3 => 4,
                                        _ => panic!("")
                                    };
                        
                                    game.nutrients -= 1;
                                    game.cells[index].tree = None;
                                    return 0
                                }

                                return -10
                            },
                            0 | 1 | 2 => return -1,
                            _ => panic!("")
                        }
                    },
                    _ => panic!("")
                }
            }
        }
    }

    fn grow(&mut self, index: usize, game: &mut Game) -> i32 {
        match &mut game.cells[index].tree {
            Some(t) => {
                match t.is_mine {
                    1 => {
                        if t.size == 3 || t.is_dormant == 1 {
                            return -10
                        }

                        let cost = match t.size {
                            0 => 1 + self.number_of_trees_1,
                            1 => 3 + self.number_of_trees_2,
                            2 => 7 + self.number_of_trees_3,
                            _ => panic!("")
                        };

                        if cost <= self.sun {
                            self.sun -= cost;
                            t.size += 1;
                            t.is_dormant = 1;

                            match t.size {
                                1 => {
                                    self.number_of_trees_0 -= 1;
                                    self.number_of_trees_1 += 1;
                                    return 1
                                },
                                2 => {
                                    self.number_of_trees_1 -= 1;
                                    self.number_of_trees_2 += 1;
                                    return 2
                                },
                                3 => {
                                    self.number_of_trees_2 -= 1;
                                    self.number_of_trees_3 += 1;
                                    return 3
                                },
                                _ => panic!("")
                            }
                        }

                        return -10
                    },
                    0 => return -10,
                    _ => panic!("")
                }
            },
            None => return -1
        }
    }

    fn seed(&mut self, index1: usize, index2: usize, game: &mut Game) -> i32 {
        match &game.cells[index1].tree {
            None => -1,
            Some(t) => {
                match t.is_mine {
                    0 => return -1,
                    1 => {
                        match t.is_dormant {
                            1 => return -1,
                            0 => {
                                match &game.cells[index2].richness {
                                    0 => return -1,
                                    1 | 2 | 3 => {
                                        match &game.cells[index2].tree {
                                            Some(_t) => return -1,
                                            None => {
                                                if self.sun < self.number_of_trees_0 {
                                                    return -1;
                                                }

                                                if &game.cells[index1].distance(&game.cells[index2]) > &t.size {
                                                    return -1;
                                                }

                                                self.sun -= self.number_of_trees_0;
                                                self.number_of_trees_0 += 1;

                                                game.cells[index1].tree = Some(Tree {
                                                    size: t.size,
                                                    is_mine: t.is_mine,
                                                    is_dormant: 1
                                                });

                                                game.cells[index2].tree = Some(Tree {
                                                    size: 0,
                                                    is_mine: match self.id {
                                                        0 => 1,
                                                        1 => 0,
                                                        _ => panic!("")
                                                    },
                                                    is_dormant: 1
                                                });
                                                
                                                return 1
                                            }
                                        }
                                    },
                                    _ => panic!("")
                                }
                            },
                            _ => panic!("")
                        }
                    },
                    _ => panic!("")
                }
            }
        }
    }

    fn get_possible_actions(&self, game: &Game) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();

        actions.push(Action {
            name: String::from("WAIT"),
            index1: 0,
            index2: 0
        });

        for c in &game.cells {
            match &c.tree {
                None => (),
                Some(t) => {
                    match t.is_mine {
                        0 => (),
                        1 => {
                            match t.is_dormant {
                                1 => (),
                                0 => {
                                    if t.size < 3 {
                                        let cost = match t.size {
                                            0 => 1 + self.number_of_trees_1,
                                            1 => 3 + self.number_of_trees_2,
                                            2 => 7 + self.number_of_trees_3,
                                            _ => panic!("")
                                        };

                                        if cost <= self.sun {
                                            actions.push(Action {
                                                name: String::from("GROW"),
                                                index1: c.index,
                                                index2: 0
                                            });
                                        }
                                    } else {
                                        if self.sun >= 4 {
                                            actions.push(Action {
                                                name: String::from("COMPLETE"),
                                                index1: c.index,
                                                index2: 0
                                            });
                                        }
                                    }
                                    
                                    if self.sun >= self.number_of_trees_0 && t.size > 0 {
                                        for c2 in &game.cells {
                                            match &c2.tree {
                                                Some(_t2) => (),
                                                None => {
                                                    match &game.cells[c2.index as usize].richness {
                                                        0 => (),
                                                        1 | 2 | 3 => {
                                                            if c.distance(c2) <= t.size {
                                                                actions.push(Action {
                                                                    name: String::from("SEED"),
                                                                    index1: c.index,
                                                                    index2: c2.index
                                                                });
                                                            }
                                                        },
                                                        _ => panic!("")
                                                    }
                                                } 
                                            }
                                        }
                                    }
                                },
                                _ => panic!("")
                            }
                        },
                        _ => panic!("")
                    }
                }
            }
        }

        actions
    }

    fn rollback(&mut self) {
        self.sun = self.rollback_sun;
        self.score = self.rollback_score;
        self.waiting = self.rollback_waiting;
        self.number_of_trees_0 = self.rollback_number_of_trees_0;
        self.number_of_trees_1 = self.rollback_number_of_trees_1;
        self.number_of_trees_2 = self.rollback_number_of_trees_2;
        self.number_of_trees_3 = self.rollback_number_of_trees_3;
    }
}

impl Cell {
    fn new(index: i32, x: i32, y: i32) -> Cell {
        Cell {
            index,
            x,
            y,
            richness: 0,
            rollback_richness: 0,
            tree: None,
            rollback_tree: None,
            neighbours: [None, None, None, None, None, None],
            shadow_size: 0,
            rollback_shadow_size: 0
        }
    }

    fn distance(&self, other: &Cell) -> i32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        if (dx >= 0 && dy >= 0) || (dx < 0 && dy < 0) {
            (dx + dy).abs()
        } else {
            cmp::max(dx.abs(), dy.abs())
        }
    }

    fn rollback(&mut self) {
        self.richness = self.rollback_richness;

        match &self.rollback_tree {
            None => {
                self.tree = None;
            },
            Some(t) => {
                self.tree = Some(Tree {
                    size: t.size,
                    is_mine: t.is_mine,
                    is_dormant: t.is_dormant
                });
            }
        }

        self.shadow_size = self.rollback_shadow_size;
    }
}

impl Game {
    fn set_shadows(&mut self) {
        let sun_direction = self.day % 6;
        let mut shadows: Vec<(usize, i32)> = Vec::new();

        for c in &self.cells {
            match &c.tree {
                Some(t) => {
                    let mut index = c.index;
                    let mut tree_size = t.size;

                    while tree_size > 0 {
                        match self.cells[index as usize].neighbours[sun_direction as usize] {
                            None => break,
                            Some(i) => {
                                shadows.push((i as usize, t.size));
                                index = i;
                            }
                        }

                        tree_size -= 1;
                    }

                    ()
                },
                None => ()
            }
        }

        for s in shadows {
            self.cells[s.0].shadow_size = s.1;
        }
    }

    fn rollback(&mut self) {
        self.day = self.rollback_day;
        self.nutrients = self.rollback_nutrients;

        for c in &mut self.cells {
            c.rollback();
        }
    }
}

impl Individual {
    fn crossover(&self, other: &Self) -> Self {
        let mut rng = rand::thread_rng();

        let mut child = Individual {
            player_id: self.player_id,
            actions: vec!(),
            fitness: 0.0
        };

        for d in 0..MAX_DEPTH {
            match rng.gen_range(0..=1) {
                0 => {
                    let count = self.actions[d].iter().count();
                    let mut actions: Vec<Action> = Vec::new();

                    for i in 0..count {
                        actions.push(Action {
                            name: self.actions[d][i].name.clone(),
                            index1: self.actions[d][i].index1,
                            index2: self.actions[d][i].index2
                        });
                    }

                    child.actions.push(actions);
                },
                1 => {
                    let count = other.actions[d].iter().count();
                    let mut actions: Vec<Action> = Vec::new();

                    for i in 0..count {
                        actions.push(Action {
                            name: other.actions[d][i].name.clone(),
                            index1: other.actions[d][i].index1,
                            index2: other.actions[d][i].index2
                        });
                    }

                    child.actions.push(actions);
                },
                _ => panic!("")
            }
        }

        child
    }

    fn randomize(&mut self, game: &Game, player: &Player) {
        let mut rng = rand::thread_rng();
        let possible_actions = player.get_possible_actions(game);
        let count = possible_actions.iter().count();

        for _i in 0..MAX_DEPTH {
            let action_counts = rng.gen_range(1..ACTIONS_COUNT);
            let mut actions: Vec<Action> = Vec::new();

            for _i in 0..action_counts {
                let i = rng.gen_range(0..count);

                actions.push(Action {
                    name: possible_actions[i].name.clone(),
                    index1: possible_actions[i].index1,
                    index2: possible_actions[i].index2
                });
            }

            self.actions.push(actions);
        }
    }

    fn simulate(&mut self, game: &mut Game, player: &mut Player, opponent: &mut Player) {
        for i in 0..MAX_DEPTH {
            let count = self.actions[i].iter().count();

            for n in 0..count {
                match self.actions[i][n].name.as_str() {
                    "GROW" => {
                        self.fitness += player.grow(self.actions[i][n].index1 as usize, game) as f64;
                    },
                    "SEED" => {
                        self.fitness += player.seed(self.actions[i][n].index1 as usize, self.actions[i][n].index2 as usize, game) as f64;
                    },
                    "COMPLETE" => {
                        self.fitness += player.complete(self.actions[i][n].index1 as usize, game) as f64;
                    },
                    "WAIT" => (),
                    _ => panic!("")
                }
            }

            game.day += 1;
            player.gather_sun(game);
            opponent.gather_sun(game);
            game.set_shadows();
        }

        self.fitness += self.fitness(player, opponent);
        game.rollback();
        player.rollback();
        opponent.rollback();
    }

    fn print(&self) {
        if self.actions.iter().count() == 0 {
            println!("WAIT");
            return;
        }

        match self.actions[0][0].name.as_str() {
            "GROW" => println!("GROW {}", self.actions[0][0].index1),
            "SEED" => println!("SEED {} {}", self.actions[0][0].index1, self.actions[0][0].index2),
            "COMPLETE" => println!("COMPLETE {}", self.actions[0][0].index1),
            "WAIT" => println!("WAIT"),
            _ => panic!("")
        }
    }

    fn fitness(&self, player: &Player, _opponent: &Player) -> f64 {
        (player.score * 100 + player.number_of_trees_0 + player.number_of_trees_1 * 2 + player.number_of_trees_2 * 3 + player.number_of_trees_3 * 4).into()
        //(player.score + player.sun / 3).into()
        //((player.score + player.sun / 3) - (opponent.score + opponent.sun / 3)).into()
    }
}

// Write an action using println!("message...");
// To debug: eprintln!("Debug message...");
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let number_of_cells = parse_input!(input_line, i32); // 37
    let mut cells: Vec<Cell> = Vec::new();

    cells.push(Cell::new(0, 0, 0));
    cells.push(Cell::new(1, 1, 0));
    cells.push(Cell::new(2, 0, 1));
    cells.push(Cell::new(3, -1, 1));
    cells.push(Cell::new(4, -1, 0));
    cells.push(Cell::new(5, 0, -1));
    cells.push(Cell::new(6, 1, -1));
    cells.push(Cell::new(7, 2, 0));
    cells.push(Cell::new(8, 1, 1));
    cells.push(Cell::new(9, 0, 2));
    cells.push(Cell::new(10, -1, 2));
    cells.push(Cell::new(11, -2, 2));
    cells.push(Cell::new(12, -2, 1));
    cells.push(Cell::new(13, -2, 0));
    cells.push(Cell::new(14, -1, -1));
    cells.push(Cell::new(15, 0, -2));
    cells.push(Cell::new(16, 1, -2));
    cells.push(Cell::new(17, 2, -2));
    cells.push(Cell::new(18, 2, -1));
    cells.push(Cell::new(19, 3, 0));
    cells.push(Cell::new(20, 2, 1));
    cells.push(Cell::new(21, 1, 2));
    cells.push(Cell::new(22, 0, 3));
    cells.push(Cell::new(23, -1, 3));
    cells.push(Cell::new(24, -2, 3));
    cells.push(Cell::new(25, -3, 3));
    cells.push(Cell::new(26, -3, 2));
    cells.push(Cell::new(27, -3, 1));
    cells.push(Cell::new(28, -3, 0));
    cells.push(Cell::new(29, -2, -1));
    cells.push(Cell::new(30, -1, -2));
    cells.push(Cell::new(31, 0, -3));
    cells.push(Cell::new(32, 1, -3));
    cells.push(Cell::new(33, 2, -3));
    cells.push(Cell::new(34, 3, -3));
    cells.push(Cell::new(35, 3, -2));
    cells.push(Cell::new(36, 3, -1));

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

        cells[index as usize].richness = richness;

        cells[index as usize].neighbours = [
            match neigh_0 {
                -1 => None,
                _ => Some(neigh_0)
            }, 
            match neigh_1 {
                -1 => None,
                _ => Some(neigh_1)
            }, 
            match neigh_2 {
                -1 => None,
                _ => Some(neigh_2)
            }, 
            match neigh_3 {
                -1 => None,
                _ => Some(neigh_3)
            }, 
            match neigh_4 {
                -1 => None,
                _ => Some(neigh_4)
            }, 
            match neigh_5 {
                -1 => None,
                _ => Some(neigh_5)
            }
        ];
    }

    let mut game = Game {
        day: 0,
        rollback_day: 0,
        nutrients: 20,
        rollback_nutrients: 20,
        number_of_trees: 0,
        cells
    };

    let mut player = Player {
        id: 0,
        sun: 0,
        rollback_sun: 0,
        score: 0,
        rollback_score: 0,
        waiting: 0,
        rollback_waiting: 0,
        number_of_trees_0: 0,
        rollback_number_of_trees_0: 0,
        number_of_trees_1: 0,
        rollback_number_of_trees_1: 0,
        number_of_trees_2: 0,
        rollback_number_of_trees_2: 0,
        number_of_trees_3: 0,
        rollback_number_of_trees_3: 0,
    };

    let mut opponent = Player {
        id: 1,
        sun: 0,
        rollback_sun: 0,
        score: 0,
        rollback_score: 0,
        waiting: 0,
        rollback_waiting: 0,
        number_of_trees_0: 0,
        rollback_number_of_trees_0: 0,
        number_of_trees_1: 0,
        rollback_number_of_trees_1: 0,
        number_of_trees_2: 0,
        rollback_number_of_trees_2: 0,
        number_of_trees_3: 0,
        rollback_number_of_trees_3: 0,
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
        player.rollback_sun = sun;

        let score = parse_input!(inputs[1], i32); // your current score
        player.score = score;
        player.rollback_score = score;

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();

        let opp_sun = parse_input!(inputs[0], i32); // opponent's sun points
        opponent.sun = opp_sun;
        opponent.rollback_sun = opp_sun;

        let opp_score = parse_input!(inputs[1], i32); // opponent's score
        opponent.score = opp_score;
        opponent.rollback_score = opp_score;

        let opp_is_waiting = parse_input!(inputs[2], i32); // whether your opponent is asleep until the next day
        opponent.waiting = opp_is_waiting;
        opponent.rollback_waiting = opp_is_waiting;

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();

        let number_of_trees = parse_input!(input_line, i32); // the current amount of trees
        game.number_of_trees = number_of_trees;

        for i in 0..number_of_cells {
            game.cells[i as usize].tree = None;
        }

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

            match is_mine {
                1 => match size {
                    0 => player.number_of_trees_0 += 1,
                    1 => player.number_of_trees_1 += 1,
                    2 => player.number_of_trees_2 += 1,
                    3 => player.number_of_trees_3 += 1,
                    _ => panic!("")
                },
                0 => match size {
                    0 => opponent.number_of_trees_0 += 1,
                    1 => opponent.number_of_trees_1 += 1,
                    2 => opponent.number_of_trees_2 += 1,
                    3 => opponent.number_of_trees_3 += 1,
                    _ => panic!("")
                },
                _ => panic!("")
            }

            player.rollback_number_of_trees_0 = player.number_of_trees_0;
            player.rollback_number_of_trees_1 = player.number_of_trees_1;
            player.rollback_number_of_trees_2 = player.number_of_trees_2;
            player.rollback_number_of_trees_3 = player.number_of_trees_3;
            opponent.rollback_number_of_trees_0 = opponent.number_of_trees_0;
            opponent.rollback_number_of_trees_1 = opponent.number_of_trees_1;
            opponent.rollback_number_of_trees_2 = opponent.number_of_trees_2;
            opponent.rollback_number_of_trees_3 = opponent.number_of_trees_3;

            for (pos, c) in game.cells.iter().enumerate() {
                if c.index == cell_index {
                    game.cells[pos].tree = Some(tree);
                    break;
                }
            }
        }

        game.set_shadows();

        // eprintln!("{:?}", game);
        // eprintln!("{:?}", player);
        // eprintln!("{:?}", opponent);

        // for c in &game.cells {
        //     eprintln!("{:?}", c);
        // }

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let number_of_possible_moves = parse_input!(input_line, i32);
        // let mut possible_moves: Vec<String> = Vec::new();

        for _i in 0..number_of_possible_moves as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let _possible_move = input_line.trim_matches('\n').to_string();
            // possible_moves.push(possible_move);
        }

        // let index = rand::thread_rng().gen_range(0..number_of_possible_moves);
        // println!("{}", possible_moves[index as usize]);

        let mut best = Individual {
            player_id: 0,
            actions: vec!(),
            fitness: -100.0
        };

        let mut population: Vec<Individual> = Vec::new();

        for _i in 0..POPULATION_SIZE {
            let mut individual = Individual {
                player_id: 0,
                actions: vec!(),
                fitness: 0.0
            };
            
            individual.randomize(&game, &player);
            individual.simulate(&mut game, &mut player, &mut opponent);
            
            if individual.fitness > best.fitness {
                best.player_id = individual.player_id;
                best.actions.clear();

                for j in 0..MAX_DEPTH {
                    let count = individual.actions[j].iter().count();
                    let mut actions: Vec<Action> = Vec::new();

                    for n in 0..count {
                        actions.push(Action {
                            name: individual.actions[j][n].name.clone(),
                            index1: individual.actions[j][n].index1,
                            index2: individual.actions[j][n].index2
                        });
                    }

                    best.actions.push(actions);
                }

                best.fitness = individual.fitness;
            }

            population.push(individual);
        }

        // eprintln!("{:?}", population);
        let mut rng = rand::thread_rng();

        /********************************************* */
        //                  TIME                    
        /********************************************* */
        let start = Instant::now();
        let limit = Duration::from_millis(90);

        while start.elapsed() < limit {
            let mut population2: Vec<Individual> = Vec::new();

            let mut temp_best = Individual {
                player_id: 0,
                actions: vec!(),
                fitness: 0.0
            };

            temp_best.player_id = best.player_id;

            for j in 0..MAX_DEPTH {
                let count = best.actions[j].iter().count();
                let mut actions: Vec<Action> = Vec::new();

                for n in 0..count {
                    actions.push(Action {
                        name: best.actions[j][n].name.clone(),
                        index1: best.actions[j][n].index1,
                        index2: best.actions[j][n].index2
                    });
                }

                temp_best.actions.push(actions);
            }

            temp_best.fitness = best.fitness;

            // Perform elitism
            population2.push(temp_best);

            for i in 1..POPULATION_SIZE {
                let mut index1 = rng.gen_range(0..POPULATION_SIZE);
                let mut index2 = rng.gen_range(0..POPULATION_SIZE);
                let first_index: usize;

                if population[index1].fitness > population[index2].fitness {
                    first_index = index1;
                } else {
                    first_index = index2;
                }

                index1 = rng.gen_range(0..POPULATION_SIZE);
                index2 = rng.gen_range(0..POPULATION_SIZE);
                let second_index: usize;

                if population[index1].fitness > population[index2].fitness {
                    second_index = index1;
                } else {
                    second_index = index2;
                }

                let mut child = population[first_index].crossover(&population[second_index]);
                child.simulate(&mut game, &mut player, &mut opponent);

                if child.fitness > best.fitness {
                    best.player_id = child.player_id;
                    best.actions.clear();
    
                    for j in 0..MAX_DEPTH {
                        let count = child.actions[j].iter().count();
                        let mut actions: Vec<Action> = Vec::new();
    
                        for n in 0..count {
                            actions.push(Action {
                                name: child.actions[j][n].name.clone(),
                                index1: child.actions[j][n].index1,
                                index2: child.actions[j][n].index2
                            });
                        }

                        best.actions.push(actions);
                    }
    
                    best.fitness = population[i].fitness;
                }

                population2.push(child);
            }

            population = population2;
        }

        eprintln!("{:?}", best);
        best.print();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_shadows() {
        let mut cells: Vec<Cell> = Vec::new();

        for i in 0..37 {
            cells.push(Cell {
                index: i,
                x: 0,
                y: 0,
                richness: 3,
                rollback_richness: 3,
                tree: None,
                rollback_tree: None,
                neighbours: [None, None, None, None, None, None],
                shadow_size: 0,
                rollback_shadow_size: 0
            });
        }

        cells[0].tree = Some(Tree{
            size: 3,
            is_mine: 1,
            is_dormant: 0
        });

        cells[0].neighbours = [Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)];
        cells[1].neighbours = [Some(7), Some(8), Some(2), Some(8), Some(6), Some(18)];
        cells[7].neighbours = [Some(19), Some(20), Some(8), Some(1), Some(18), Some(36)];

        let mut game = Game {
            day: 0,
            rollback_day: 0,
            nutrients: 20,
            rollback_nutrients: 20,
            number_of_trees: 1,
            cells
        };

        game.set_shadows();
        assert_eq!(game.cells[1].shadow_size, 3);
        assert_eq!(game.cells[7].shadow_size, 3);
        assert_eq!(game.cells[19].shadow_size, 3);
    }

    #[test]
    fn test_grow() {
        let mut cells: Vec<Cell> = Vec::new();

        for i in 0..37 {
            cells.push(Cell {
                index: i,
                x: 0,
                y: 0,
                richness: 3,
                rollback_richness: 3,
                tree: None,
                rollback_tree: None,
                neighbours: [None, None, None, None, None, None],
                shadow_size: 0,
                rollback_shadow_size: 0
            });
        }

        cells[0].tree = Some(Tree{
            size: 0,
            is_mine: 1,
            is_dormant: 0
        });

        let mut player = Player {
            id: 0,
            sun: 2,
            rollback_sun: 0,
            score: 0,
            rollback_score: 0,
            waiting: 0,
            rollback_waiting: 0,
            number_of_trees_0: 1,
            rollback_number_of_trees_0: 1,
            number_of_trees_1: 0,
            rollback_number_of_trees_1: 0,
            number_of_trees_2: 0,
            rollback_number_of_trees_2: 0,
            number_of_trees_3: 0,
            rollback_number_of_trees_3: 0,
        };

        let mut game = Game {
            day: 0,
            rollback_day: 0,
            nutrients: 20,
            rollback_nutrients: 20,
            number_of_trees: 1,
            cells
        };

        player.grow(0, &mut game);

        match &game.cells[0].tree {
            None => panic!(""),
            Some(t) => {
                assert_eq!(t.size, 1);
                assert_eq!(t.is_dormant, 1);
            }
        }

        assert_eq!(player.sun, 1);
        assert_eq!(player.number_of_trees_0, 0);
        assert_eq!(player.number_of_trees_1, 1);
    }

    #[test]
    fn test_gather_sun() {
        let mut cells: Vec<Cell> = Vec::new();

        for i in 0..37 {
            cells.push(Cell {
                index: i,
                x: 0,
                y: 0,
                richness: 3,
                rollback_richness: 3,
                tree: None,
                rollback_tree: None,
                neighbours: [None, None, None, None, None, None],
                shadow_size: 0,
                rollback_shadow_size: 0
            });
        }

        cells[0].tree = Some(Tree{
            size: 0,
            is_mine: 1,
            is_dormant: 0
        });

        cells[1].tree = Some(Tree{
            size: 1,
            is_mine: 1,
            is_dormant: 0
        });

        cells[2].tree = Some(Tree{
            size: 2,
            is_mine: 1,
            is_dormant: 0
        });

        cells[2].shadow_size = 3;

        cells[3].tree = Some(Tree{
            size: 3,
            is_mine: 1,
            is_dormant: 0
        });

        let mut player = Player {
            id: 0,
            sun: 0,
            rollback_sun: 0,
            score: 0,
            rollback_score: 0,
            waiting: 0,
            rollback_waiting: 0,
            number_of_trees_0: 0,
            rollback_number_of_trees_0: 0,
            number_of_trees_1: 0,
            rollback_number_of_trees_1: 0,
            number_of_trees_2: 0,
            rollback_number_of_trees_2: 0,
            number_of_trees_3: 0,
            rollback_number_of_trees_3: 0,
        };

        let game = Game {
            day: 0,
            rollback_day: 0,
            nutrients: 20,
            rollback_nutrients: 20,
            number_of_trees: 6,
            cells
        };

        player.gather_sun(&game);
        assert_eq!(player.sun, 4);
    }

    #[test]
    fn test_complete() {
        let mut cells: Vec<Cell> = Vec::new();

        for i in 0..37 {
            cells.push(Cell {
                index: i,
                x: 0,
                y: 0,
                richness: 3,
                rollback_richness: 3,
                tree: None,
                rollback_tree: None,
                neighbours: [None, None, None, None, None, None],
                shadow_size: 0,
                rollback_shadow_size: 0
            });
        }

        cells[0].tree = Some(Tree{
            size: 3,
            is_mine: 1,
            is_dormant: 0
        });

        let mut player = Player {
            id: 0,
            sun: 4,
            rollback_sun: 0,
            score: 0,
            rollback_score: 0,
            waiting: 0,
            rollback_waiting: 0,
            number_of_trees_0: 0,
            rollback_number_of_trees_0: 0,
            number_of_trees_1: 0,
            rollback_number_of_trees_1: 0,
            number_of_trees_2: 0,
            rollback_number_of_trees_2: 0,
            number_of_trees_3: 1,
            rollback_number_of_trees_3: 1,
        };

        let mut game = Game {
            day: 0,
            rollback_day: 0,
            nutrients: 20,
            rollback_nutrients: 20,
            number_of_trees: 1,
            cells
        };

        player.complete(0, &mut game);
        assert_eq!(player.sun, 0);
        assert_eq!(player.score, 24);
        assert_eq!(player.number_of_trees_3, 0);
        assert_eq!(game.nutrients, 19);
        assert_eq!(game.cells[0].tree.is_none(), true);
    }

    #[test]
    fn test_distance() {
        let mut cells: Vec<Cell> = Vec::new();
        cells.push(Cell::new(0, 0, 0));
        cells.push(Cell::new(1, 3, -3));
        assert_eq!(cells[0].distance(&cells[1]), 3);
    }

    #[test]
    fn test_seed() {
        let mut cells: Vec<Cell> = Vec::new();
        cells.push(Cell::new(0, 0, 0));
        cells.push(Cell::new(1, 3, 0));

        cells[0].tree = Some(Tree{
            size: 3,
            is_mine: 1,
            is_dormant: 0
        });

        cells[1].richness = 1;

        let mut player = Player {
            id: 0,
            sun: 1,
            rollback_sun: 0,
            score: 0,
            rollback_score: 0,
            waiting: 0,
            rollback_waiting: 0,
            number_of_trees_0: 1,
            rollback_number_of_trees_0: 1,
            number_of_trees_1: 0,
            rollback_number_of_trees_1: 0,
            number_of_trees_2: 0,
            rollback_number_of_trees_2: 0,
            number_of_trees_3: 1,
            rollback_number_of_trees_3: 1,
        };

        let mut game = Game {
            day: 0,
            rollback_day: 0,
            nutrients: 20,
            rollback_nutrients: 20,
            number_of_trees: 2,
            cells
        };

        player.seed(0, 1, &mut game);
        assert_eq!(player.sun, 0);
        assert_eq!(player.number_of_trees_0, 2);
        
        match &game.cells[0].tree {
            None => panic!(""),
            Some(t) => assert_eq!(t.is_dormant, 1)
        };

        match &game.cells[1].tree {
            None => panic!(""),
            Some(t) => {
                assert_eq!(t.size, 0);
                assert_eq!(t.is_dormant, 1);
            }
        };
    }

    #[test]
    fn test_player_rollback() {
        let mut cells: Vec<Cell> = Vec::new();

        for i in 0..37 {
            cells.push(Cell {
                index: i,
                x: 0,
                y: 0,
                richness: 3,
                rollback_richness: 3,
                tree: None,
                rollback_tree: None,
                neighbours: [None, None, None, None, None, None],
                shadow_size: 0,
                rollback_shadow_size: 0
            });
        }

        cells[0].tree = Some(Tree{
            size: 3,
            is_mine: 1,
            is_dormant: 0
        });

        let mut player = Player {
            id: 0,
            sun: 4,
            rollback_sun: 4,
            score: 0,
            rollback_score: 0,
            waiting: 0,
            rollback_waiting: 0,
            number_of_trees_0: 0,
            rollback_number_of_trees_0: 0,
            number_of_trees_1: 0,
            rollback_number_of_trees_1: 0,
            number_of_trees_2: 0,
            rollback_number_of_trees_2: 0,
            number_of_trees_3: 1,
            rollback_number_of_trees_3: 1,
        };

        let mut game = Game {
            day: 0,
            rollback_day: 0,
            nutrients: 20,
            rollback_nutrients: 20,
            number_of_trees: 1,
            cells
        };

        player.complete(0, &mut game);
        assert_eq!(player.sun, 0);
        assert_eq!(player.score, 24);
        assert_eq!(player.number_of_trees_3, 0);
        assert_eq!(game.nutrients, 19);
        assert_eq!(game.cells[0].tree.is_none(), true);

        player.rollback();
        assert_eq!(player.sun, 4);
        assert_eq!(player.score, 0);
        assert_eq!(player.number_of_trees_3, 1);
    }

    #[test]
    fn test_cell_rollback() {
        let mut cells: Vec<Cell> = Vec::new();
        cells.push(Cell::new(0, 0, 0));
        cells.push(Cell::new(1, 3, 0));

        cells[0].tree = Some(Tree{
            size: 3,
            is_mine: 1,
            is_dormant: 0
        });

        cells[0].rollback_tree = Some(Tree{
            size: 3,
            is_mine: 1,
            is_dormant: 0
        });

        cells[1].richness = 1;
        cells[1].rollback_richness = 1;

        let mut player = Player {
            id: 0,
            sun: 1,
            rollback_sun: 0,
            score: 0,
            rollback_score: 0,
            waiting: 0,
            rollback_waiting: 0,
            number_of_trees_0: 1,
            rollback_number_of_trees_0: 1,
            number_of_trees_1: 0,
            rollback_number_of_trees_1: 0,
            number_of_trees_2: 0,
            rollback_number_of_trees_2: 0,
            number_of_trees_3: 1,
            rollback_number_of_trees_3: 1,
        };

        let mut game = Game {
            day: 0,
            rollback_day: 0,
            nutrients: 20,
            rollback_nutrients: 20,
            number_of_trees: 2,
            cells
        };

        player.seed(0, 1, &mut game);
        assert_eq!(player.sun, 0);
        assert_eq!(player.number_of_trees_0, 2);
        
        match &game.cells[0].tree {
            None => panic!(""),
            Some(t) => assert_eq!(t.is_dormant, 1)
        };

        match &game.cells[1].tree {
            None => panic!(""),
            Some(t) => {
                assert_eq!(t.size, 0);
                assert_eq!(t.is_dormant, 1);
            }
        };

        game.cells[0].rollback();
        game.cells[1].rollback();

        match &game.cells[0].tree {
            None => panic!(""),
            Some(t) => assert_eq!(t.is_dormant, 0)
        };

        assert_eq!(game.cells[1].tree.is_none(), true);
    }

    #[test]
    fn test_game_rollback() {
        let mut cells: Vec<Cell> = Vec::new();

        for i in 0..37 {
            cells.push(Cell {
                index: i,
                x: 0,
                y: 0,
                richness: 3,
                rollback_richness: 3,
                tree: None,
                rollback_tree: None,
                neighbours: [None, None, None, None, None, None],
                shadow_size: 0,
                rollback_shadow_size: 0
            });
        }

        cells[0].tree = Some(Tree{
            size: 3,
            is_mine: 1,
            is_dormant: 0
        });

        cells[0].rollback_tree = Some(Tree{
            size: 3,
            is_mine: 1,
            is_dormant: 0
        });

        let mut player = Player {
            id: 0,
            sun: 4,
            rollback_sun: 4,
            score: 0,
            rollback_score: 0,
            waiting: 0,
            rollback_waiting: 0,
            number_of_trees_0: 0,
            rollback_number_of_trees_0: 0,
            number_of_trees_1: 0,
            rollback_number_of_trees_1: 0,
            number_of_trees_2: 0,
            rollback_number_of_trees_2: 0,
            number_of_trees_3: 1,
            rollback_number_of_trees_3: 1,
        };

        let mut game = Game {
            day: 0,
            rollback_day: 0,
            nutrients: 20,
            rollback_nutrients: 20,
            number_of_trees: 1,
            cells
        };

        player.complete(0, &mut game);
        assert_eq!(game.cells[0].tree.is_none(), true);
        game.rollback();

        match &game.cells[0].tree {
            None => panic!(""),
            Some(t) => {
                assert_eq!(t.size, 3);
                assert_eq!(t.is_mine, 1);
                assert_eq!(t.is_dormant, 0);
            }
        };
    }

    #[test]
    fn test_initialize_population() {
        let mut cells: Vec<Cell> = Vec::new();

        for i in 0..37 {
            cells.push(Cell {
                index: i,
                x: 0,
                y: 0,
                richness: 3,
                rollback_richness: 3,
                tree: None,
                rollback_tree: None,
                neighbours: [None, None, None, None, None, None],
                shadow_size: 0,
                rollback_shadow_size: 0
            });
        }

        let player = Player {
            id: 0,
            sun: 4,
            rollback_sun: 4,
            score: 0,
            rollback_score: 0,
            waiting: 0,
            rollback_waiting: 0,
            number_of_trees_0: 0,
            rollback_number_of_trees_0: 0,
            number_of_trees_1: 0,
            rollback_number_of_trees_1: 0,
            number_of_trees_2: 0,
            rollback_number_of_trees_2: 0,
            number_of_trees_3: 1,
            rollback_number_of_trees_3: 1,
        };

        let game = Game {
            day: 0,
            rollback_day: 0,
            nutrients: 20,
            rollback_nutrients: 20,
            number_of_trees: 1,
            cells
        };

        let mut population: Vec<Individual> = Vec::new();

        for _i in 0..POPULATION_SIZE {
            let mut individual = Individual {
                player_id: 0,
                actions: vec!(),
                fitness: 0.0
            };

            individual.randomize(&game, &player);
            population.push(individual);
        }

        eprintln!("{:?}", population);
    }

    #[test]
    fn test_get_possible_actions() {
        let mut cells: Vec<Cell> = Vec::new();

        cells.push(Cell::new(0, 0, 0));
        cells.push(Cell::new(1, 1, 0));
        cells.push(Cell::new(2, 0, 1));
        cells.push(Cell::new(3, -1, 1));
        cells.push(Cell::new(4, -1, 0));
        cells.push(Cell::new(5, 0, -1));
        cells.push(Cell::new(6, 1, -1));
        cells.push(Cell::new(7, 2, 0));
        cells.push(Cell::new(8, 1, 1));
        cells.push(Cell::new(9, 0, 2));
        cells.push(Cell::new(10, -1, 2));
        cells.push(Cell::new(11, -2, 2));
        cells.push(Cell::new(12, -2, 1));
        cells.push(Cell::new(13, -2, 0));
        cells.push(Cell::new(14, -1, -1));
        cells.push(Cell::new(15, 0, -2));
        cells.push(Cell::new(16, 1, -2));
        cells.push(Cell::new(17, 2, -2));
        cells.push(Cell::new(18, 2, -1));
        cells.push(Cell::new(19, 3, 0));
        cells.push(Cell::new(20, 2, 1));
        cells.push(Cell::new(21, 1, 2));
        cells.push(Cell::new(22, 0, 3));
        cells.push(Cell::new(23, -1, 3));
        cells.push(Cell::new(24, -2, 3));
        cells.push(Cell::new(25, -3, 3));
        cells.push(Cell::new(26, -3, 2));
        cells.push(Cell::new(27, -3, 1));
        cells.push(Cell::new(28, -3, 0));
        cells.push(Cell::new(29, -2, -1));
        cells.push(Cell::new(30, -1, -2));
        cells.push(Cell::new(31, 0, -3));
        cells.push(Cell::new(32, 1, -3));
        cells.push(Cell::new(33, 2, -3));
        cells.push(Cell::new(34, 3, -3));
        cells.push(Cell::new(35, 3, -2));
        cells.push(Cell::new(36, 3, -1));

        cells[0].neighbours = [Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)];
        cells[1].neighbours = [Some(7), Some(8), Some(2), Some(0), Some(6), Some(18)];
        cells[2].neighbours = [Some(8), Some(9), Some(10), Some(3), Some(0), Some(1)];
        cells[3].neighbours = [Some(2), Some(10), Some(11), Some(12), Some(4), Some(0)];
        cells[4].neighbours = [Some(0), Some(3), Some(12), Some(13), Some(14), Some(5)];
        cells[5].neighbours = [Some(6), Some(0), Some(4), Some(14), Some(15), Some(16)];
        cells[6].neighbours = [Some(18), Some(1), Some(0), Some(5), Some(16), Some(17)];
        cells[7].neighbours = [Some(19), Some(20), Some(8), Some(1), Some(18), Some(36)];
        cells[8].neighbours = [Some(20), Some(21), Some(9), Some(2), Some(1), Some(7)];
        cells[9].neighbours = [Some(21), Some(22), Some(23), Some(10), Some(2), Some(8)];
        cells[10].neighbours = [Some(9), Some(23), Some(24), Some(11), Some(3), Some(2)];
        cells[11].neighbours = [Some(10), Some(24), Some(25), Some(26), Some(12), Some(3)];
        cells[12].neighbours = [Some(3), Some(11), Some(26), Some(27), Some(13), Some(4)];
        cells[13].neighbours = [Some(4), Some(12), Some(27), Some(28), Some(29), Some(14)];
        cells[14].neighbours = [Some(5), Some(4), Some(13), Some(29), Some(30), Some(15)];
        cells[15].neighbours = [Some(16), Some(5), Some(14), Some(30), Some(31), Some(32)];
        cells[16].neighbours = [Some(17), Some(6), Some(5), Some(15), Some(32), Some(33)];
        cells[17].neighbours = [Some(35), Some(18), Some(6), Some(16), Some(33), Some(34)];
        cells[18].neighbours = [Some(36), Some(7), Some(1), Some(6), Some(17), Some(35)];
        cells[19].neighbours = [None, None, Some(20), Some(7), Some(36), None];
        cells[20].neighbours = [None, None, Some(21), Some(8), Some(7), Some(19)];
        cells[21].neighbours = [None, None, Some(22), Some(9), Some(8), Some(20)];
        cells[22].neighbours = [None, None, None, Some(23), Some(9), Some(21)];
        cells[23].neighbours = [Some(22), None, None, Some(24), Some(10), Some(9)];
        cells[24].neighbours = [Some(23), None, None, Some(25), Some(11), Some(10)];
        cells[25].neighbours = [Some(24), None, None, None, Some(26), Some(11)];
        cells[26].neighbours = [Some(11), Some(25), None, None, Some(27), Some(12)];
        cells[27].neighbours = [Some(12), Some(26), None, None, Some(28), Some(13)];
        cells[28].neighbours = [Some(13), Some(27), None, None, None, Some(29)];
        cells[29].neighbours = [Some(14), Some(13), Some(28), None, None, Some(30)];
        cells[30].neighbours = [Some(15), Some(14), Some(29), None, None, Some(31)];
        cells[31].neighbours = [Some(32), Some(15), Some(30), None, None, None];
        cells[32].neighbours = [Some(33), Some(16), Some(15), Some(31), None, None];
        cells[33].neighbours = [Some(34), Some(17), Some(16), Some(32), None, None];
        cells[34].neighbours = [None, Some(35), Some(17), Some(33), None, None];
        cells[35].neighbours = [None, Some(36), Some(18), Some(17), Some(34), None];
        cells[36].neighbours = [None, Some(19), Some(7), Some(18), Some(35), None];

        cells[31].richness = 3;
        cells[33].richness = 2;

        cells[32].tree = Some(Tree{
            size: 1,
            is_mine: 1,
            is_dormant: 0
        });

        cells[32].rollback_tree = Some(Tree{
            size: 1,
            is_mine: 1,
            is_dormant: 0
        });

        cells[35].tree = Some(Tree{
            size: 1,
            is_mine: 1,
            is_dormant: 0
        });

        cells[35].rollback_tree = Some(Tree{
            size: 1,
            is_mine: 1,
            is_dormant: 0
        });

        cells[23].tree = Some(Tree{
            size: 1,
            is_mine: 0,
            is_dormant: 0
        });

        cells[23].rollback_tree = Some(Tree{
            size: 1,
            is_mine: 0,
            is_dormant: 0
        });

        cells[26].tree = Some(Tree{
            size: 1,
            is_mine: 0,
            is_dormant: 0
        });

        cells[26].rollback_tree = Some(Tree{
            size: 1,
            is_mine: 0,
            is_dormant: 0
        });

        let player = Player {
            id: 0,
            sun: 2,
            rollback_sun: 2,
            score: 0,
            rollback_score: 0,
            waiting: 0,
            rollback_waiting: 0,
            number_of_trees_0: 0,
            rollback_number_of_trees_0: 0,
            number_of_trees_1: 2,
            rollback_number_of_trees_1: 2,
            number_of_trees_2: 0,
            rollback_number_of_trees_2: 0,
            number_of_trees_3: 0,
            rollback_number_of_trees_3: 0,
        };

        let game = Game {
            day: 0,
            rollback_day: 0,
            nutrients: 20,
            rollback_nutrients: 20,
            number_of_trees: 4,
            cells
        };

        let possible_actions = player.get_possible_actions(&game);
        eprintln!("{:?}", possible_actions);
    }

    #[test]
    fn test_individual_simulate() {
        let mut cells: Vec<Cell> = Vec::new();

        cells.push(Cell::new(0, 0, 0));
        cells.push(Cell::new(1, 1, 0));
        cells.push(Cell::new(2, 0, 1));
        cells.push(Cell::new(3, -1, 1));
        cells.push(Cell::new(4, -1, 0));
        cells.push(Cell::new(5, 0, -1));
        cells.push(Cell::new(6, 1, -1));
        cells.push(Cell::new(7, 2, 0));
        cells.push(Cell::new(8, 1, 1));
        cells.push(Cell::new(9, 0, 2));
        cells.push(Cell::new(10, -1, 2));
        cells.push(Cell::new(11, -2, 2));
        cells.push(Cell::new(12, -2, 1));
        cells.push(Cell::new(13, -2, 0));
        cells.push(Cell::new(14, -1, -1));
        cells.push(Cell::new(15, 0, -2));
        cells.push(Cell::new(16, 1, -2));
        cells.push(Cell::new(17, 2, -2));
        cells.push(Cell::new(18, 2, -1));
        cells.push(Cell::new(19, 3, 0));
        cells.push(Cell::new(20, 2, 1));
        cells.push(Cell::new(21, 1, 2));
        cells.push(Cell::new(22, 0, 3));
        cells.push(Cell::new(23, -1, 3));
        cells.push(Cell::new(24, -2, 3));
        cells.push(Cell::new(25, -3, 3));
        cells.push(Cell::new(26, -3, 2));
        cells.push(Cell::new(27, -3, 1));
        cells.push(Cell::new(28, -3, 0));
        cells.push(Cell::new(29, -2, -1));
        cells.push(Cell::new(30, -1, -2));
        cells.push(Cell::new(31, 0, -3));
        cells.push(Cell::new(32, 1, -3));
        cells.push(Cell::new(33, 2, -3));
        cells.push(Cell::new(34, 3, -3));
        cells.push(Cell::new(35, 3, -2));
        cells.push(Cell::new(36, 3, -1));

        cells[0].neighbours = [Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)];
        cells[1].neighbours = [Some(7), Some(8), Some(2), Some(0), Some(6), Some(18)];
        cells[2].neighbours = [Some(8), Some(9), Some(10), Some(3), Some(0), Some(1)];
        cells[3].neighbours = [Some(2), Some(10), Some(11), Some(12), Some(4), Some(0)];
        cells[4].neighbours = [Some(0), Some(3), Some(12), Some(13), Some(14), Some(5)];
        cells[5].neighbours = [Some(6), Some(0), Some(4), Some(14), Some(15), Some(16)];
        cells[6].neighbours = [Some(18), Some(1), Some(0), Some(5), Some(16), Some(17)];
        cells[7].neighbours = [Some(19), Some(20), Some(8), Some(1), Some(18), Some(36)];
        cells[8].neighbours = [Some(20), Some(21), Some(9), Some(2), Some(1), Some(7)];
        cells[9].neighbours = [Some(21), Some(22), Some(23), Some(10), Some(2), Some(8)];
        cells[10].neighbours = [Some(9), Some(23), Some(24), Some(11), Some(3), Some(2)];
        cells[11].neighbours = [Some(10), Some(24), Some(25), Some(26), Some(12), Some(3)];
        cells[12].neighbours = [Some(3), Some(11), Some(26), Some(27), Some(13), Some(4)];
        cells[13].neighbours = [Some(4), Some(12), Some(27), Some(28), Some(29), Some(14)];
        cells[14].neighbours = [Some(5), Some(4), Some(13), Some(29), Some(30), Some(15)];
        cells[15].neighbours = [Some(16), Some(5), Some(14), Some(30), Some(31), Some(32)];
        cells[16].neighbours = [Some(17), Some(6), Some(5), Some(15), Some(32), Some(33)];
        cells[17].neighbours = [Some(35), Some(18), Some(6), Some(16), Some(33), Some(34)];
        cells[18].neighbours = [Some(36), Some(7), Some(1), Some(6), Some(17), Some(35)];
        cells[19].neighbours = [None, None, Some(20), Some(7), Some(36), None];
        cells[20].neighbours = [None, None, Some(21), Some(8), Some(7), Some(19)];
        cells[21].neighbours = [None, None, Some(22), Some(9), Some(8), Some(20)];
        cells[22].neighbours = [None, None, None, Some(23), Some(9), Some(21)];
        cells[23].neighbours = [Some(22), None, None, Some(24), Some(10), Some(9)];
        cells[24].neighbours = [Some(23), None, None, Some(25), Some(11), Some(10)];
        cells[25].neighbours = [Some(24), None, None, None, Some(26), Some(11)];
        cells[26].neighbours = [Some(11), Some(25), None, None, Some(27), Some(12)];
        cells[27].neighbours = [Some(12), Some(26), None, None, Some(28), Some(13)];
        cells[28].neighbours = [Some(13), Some(27), None, None, None, Some(29)];
        cells[29].neighbours = [Some(14), Some(13), Some(28), None, None, Some(30)];
        cells[30].neighbours = [Some(15), Some(14), Some(29), None, None, Some(31)];
        cells[31].neighbours = [Some(32), Some(15), Some(30), None, None, None];
        cells[32].neighbours = [Some(33), Some(16), Some(15), Some(31), None, None];
        cells[33].neighbours = [Some(34), Some(17), Some(16), Some(32), None, None];
        cells[34].neighbours = [None, Some(35), Some(17), Some(33), None, None];
        cells[35].neighbours = [None, Some(36), Some(18), Some(17), Some(34), None];
        cells[36].neighbours = [None, Some(19), Some(7), Some(18), Some(35), None];

        cells[31].richness = 3;
        cells[33].richness = 2;

        cells[32].tree = Some(Tree{
            size: 1,
            is_mine: 1,
            is_dormant: 0
        });

        cells[32].rollback_tree = Some(Tree{
            size: 1,
            is_mine: 1,
            is_dormant: 0
        });

        cells[35].tree = Some(Tree{
            size: 1,
            is_mine: 1,
            is_dormant: 0
        });

        cells[35].rollback_tree = Some(Tree{
            size: 1,
            is_mine: 1,
            is_dormant: 0
        });

        cells[23].tree = Some(Tree{
            size: 1,
            is_mine: 0,
            is_dormant: 0
        });

        cells[23].rollback_tree = Some(Tree{
            size: 1,
            is_mine: 0,
            is_dormant: 0
        });

        cells[26].tree = Some(Tree{
            size: 1,
            is_mine: 0,
            is_dormant: 0
        });

        cells[26].rollback_tree = Some(Tree{
            size: 1,
            is_mine: 0,
            is_dormant: 0
        });

        let mut player = Player {
            id: 0,
            sun: 0,
            rollback_sun: 0,
            score: 0,
            rollback_score: 0,
            waiting: 0,
            rollback_waiting: 0,
            number_of_trees_0: 0,
            rollback_number_of_trees_0: 0,
            number_of_trees_1: 2,
            rollback_number_of_trees_1: 2,
            number_of_trees_2: 0,
            rollback_number_of_trees_2: 0,
            number_of_trees_3: 0,
            rollback_number_of_trees_3: 0,
        };

        let mut opponent = Player {
            id: 1,
            sun: 0,
            rollback_sun: 0,
            score: 0,
            rollback_score: 0,
            waiting: 0,
            rollback_waiting: 0,
            number_of_trees_0: 0,
            rollback_number_of_trees_0: 0,
            number_of_trees_1: 2,
            rollback_number_of_trees_1: 2,
            number_of_trees_2: 0,
            rollback_number_of_trees_2: 0,
            number_of_trees_3: 0,
            rollback_number_of_trees_3: 0,
        };

        let mut game = Game {
            day: 0,
            rollback_day: 0,
            nutrients: 20,
            rollback_nutrients: 20,
            number_of_trees: 4,
            cells
        };

        player.gather_sun(&game);
        game.set_shadows();

        let mut best = Individual {
            player_id: 0,
            actions: vec!(),
            fitness: -100.0
        };

        let mut population: Vec<Individual> = Vec::new();

        for _i in 0..POPULATION_SIZE {
            let mut individual = Individual {
                player_id: 0,
                actions: vec!(),
                fitness: 0.0
            };
            
            individual.randomize(&game, &player);
            eprintln!("{:?}", individual);
            individual.simulate(&mut game, &mut player, &mut opponent);
            
            if individual.fitness > best.fitness {
                best.player_id = individual.player_id;
                best.actions.clear();

                for j in 0..MAX_DEPTH {
                    let count = individual.actions[j].iter().count();
                    let mut actions: Vec<Action> = Vec::new();

                    for n in 0..count {
                        actions.push(Action {
                            name: individual.actions[j][n].name.clone(),
                            index1: individual.actions[j][n].index1,
                            index2: individual.actions[j][n].index2
                        });
                    }

                    best.actions.push(actions);
                }

                best.fitness = individual.fitness;
            }

            population.push(individual);
        }

        // eprintln!("{:?}", population);
        let mut rng = rand::thread_rng();

        /********************************************* */
        //                  TIME                    
        /********************************************* */
        let start = Instant::now();
        let limit = Duration::from_millis(90);

        while start.elapsed() < limit {
            let mut population2: Vec<Individual> = Vec::new();

            let mut temp_best = Individual {
                player_id: 0,
                actions: vec!(),
                fitness: 0.0
            };

            temp_best.player_id = best.player_id;

            for j in 0..MAX_DEPTH {
                let count = best.actions[j].iter().count();
                let mut actions: Vec<Action> = Vec::new();

                for n in 0..count {
                    actions.push(Action {
                        name: best.actions[j][n].name.clone(),
                        index1: best.actions[j][n].index1,
                        index2: best.actions[j][n].index2
                    });
                }

                temp_best.actions.push(actions);
            }

            temp_best.fitness = best.fitness;

            // Perform elitism
            population2.push(temp_best);

            for i in 1..POPULATION_SIZE {
                let mut index1 = rng.gen_range(0..POPULATION_SIZE);
                let mut index2 = rng.gen_range(0..POPULATION_SIZE);
                let first_index: usize;

                if population[index1].fitness > population[index2].fitness {
                    first_index = index1;
                } else {
                    first_index = index2;
                }

                index1 = rng.gen_range(0..POPULATION_SIZE);
                index2 = rng.gen_range(0..POPULATION_SIZE);
                let second_index: usize;

                if population[index1].fitness > population[index2].fitness {
                    second_index = index1;
                } else {
                    second_index = index2;
                }

                let mut child = population[first_index].crossover(&population[second_index]);
                child.simulate(&mut game, &mut player, &mut opponent);

                if child.fitness > best.fitness {
                    best.player_id = child.player_id;
                    best.actions.clear();
    
                    for j in 0..MAX_DEPTH {
                        let count = child.actions[j].iter().count();
                        let mut actions: Vec<Action> = Vec::new();
    
                        for n in 0..count {
                            actions.push(Action {
                                name: child.actions[j][n].name.clone(),
                                index1: child.actions[j][n].index1,
                                index2: child.actions[j][n].index2
                            });
                        }

                        best.actions.push(actions);
                    }
    
                    best.fitness = population[i].fitness;
                }

                population2.push(child);
            }

            population = population2;

            eprintln!("{:?}", best);
            eprintln!("{:?}", start.elapsed());
        }
    }
}