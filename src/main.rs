use rand::seq::SliceRandom;
use std::io::{stdout, BufWriter, Write};

const HEIGHT: usize = 101;
const WIDTH: usize = 101;

struct Pole {
    x: usize,
    y: usize,
}

fn init_maze(maze: &mut Vec<Vec<usize>>) -> Vec<Pole> {
    // Wall
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if i == 0 || j == 0 || i == HEIGHT - 1 || j == WIDTH - 1 {
                maze[i][j] = 1;
            } else {
                maze[i][j] = 0;
            }
        }
    }

    let mut pole_data: Vec<Pole> = Vec::new();
    // Pole
    for i in 2..HEIGHT - 1 {
        for j in 2..WIDTH - 1 {
            if i % 2 == 0 && j % 2 == 0 {
                maze[i][j] = 1;
                pole_data.push(Pole { x: i, y: j });
            }
        }
    }

    return pole_data;
}

fn gen_maze(maze: &mut Vec<Vec<usize>>, pole_data: &Vec<Pole>) {
    let mut rng = rand::thread_rng();

    // 0 -> up
    // 1 -> left
    // 2 -> down
    // 3 -> right

    for i in pole_data {
        if i.x == 2 {
            let mut can_put_pole: Vec<usize> = Vec::new();

            if maze[i.x - 1][i.y] == 0 {
                can_put_pole.push(0);
            }
            if maze[i.x][i.y + 1] == 0 {
                can_put_pole.push(1);
            }
            if maze[i.x + 1][i.y] == 0 {
                can_put_pole.push(2);
            }
            if maze[i.x][i.y - 1] == 0 {
                can_put_pole.push(3);
            }

            if can_put_pole.len() == 0 {
                continue;
            } else {
                can_put_pole.shuffle(&mut rng);

                if can_put_pole[0] == 0 {
                    maze[i.x - 1][i.y] = 1;
                } else if can_put_pole[0] == 1 {
                    maze[i.x][i.y + 1] = 1;
                } else if can_put_pole[0] == 2 {
                    maze[i.x + 1][i.y] = 1;
                } else if can_put_pole[0] == 3 {
                    maze[i.x][i.y - 1] = 1;
                }
            }
        } else {
            let mut can_put_pole: Vec<usize> = Vec::new();
            if maze[i.x][i.y + 1] == 0 {
                can_put_pole.push(1);
            }
            if maze[i.x + 1][i.y] == 0 {
                can_put_pole.push(2);
            }
            if maze[i.x][i.y - 1] == 0 {
                can_put_pole.push(3);
            }
            if can_put_pole.len() == 0 {
                continue;
            } else {
                can_put_pole.shuffle(&mut rng);

                if can_put_pole[0] == 1 {
                    maze[i.x][i.y + 1] = 1;
                } else if can_put_pole[0] == 2 {
                    maze[i.x + 1][i.y] = 1;
                } else if can_put_pole[0] == 3 {
                    maze[i.x][i.y - 1] = 1;
                }
            }
        }
    }
}

fn print_maze(maze: &Vec<Vec<usize>>) {
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if maze[i][j] == 0 {
                write!(out, "⬛").unwrap();
            } else if maze[i][j] == 1 {
                write!(out, "⬜").unwrap();
            }
        }
        write!(out, "\n").unwrap();
    }
}

fn main() {
    let mut maze = vec![vec![0; WIDTH]; HEIGHT];
    let pole_data: Vec<Pole> = init_maze(&mut maze);

    gen_maze(&mut maze, &pole_data);
    print_maze(&maze);
}
