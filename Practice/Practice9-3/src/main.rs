use rand::{thread_rng, Rng};
use std::io::{self, Write};

const width: usize = 10;
const height: usize = 5;

struct point {
    x: i8,
    y: i8,
}

const offsets: [point; 4] = [
    point { x: -2, y: 0 },
    point { x: 2, y: 0 },
    point { x: 0, y: -2 },
    point { x: 0, y: 2 },
];

fn initMaze(maze: &mut Vec<Vec<String>>) {
    for (indexRow, row) in maze.iter_mut().enumerate() {
        for (indexColumn, column) in row.iter_mut().enumerate() {
            if indexRow % 2 == 0 {
                *column = "#".to_string();
            } else {
                if indexColumn % 2 == 0 {
                    *column = "#".to_string();
                } else {
                    *column = "?".to_string();
                }
            }
        }
    }
    maze[1][0] = " ".to_string();
    maze[1][1] = " ".to_string();
    maze[2 * height - 1][2 * width] = " ".to_string();
}

fn createMaze(x: i8, y: i8, maze: &mut Vec<Vec<String>>) {
    loop {
        let mut questionMarkCount = 0;
        for k in offsets {
            if x + k.x < 0
                || x + k.x > 2 * height as i8
                || y + k.y < 0
                || y + k.y > 2 * width as i8
                || !(maze[(x + k.x) as usize][(y + k.y) as usize] == "?")
            {
                continue;
            } else {
                questionMarkCount += 1;
            }
        }
        if questionMarkCount == 0 {
            return;
        }
        let mut chose = thread_rng().gen_range::<usize, _>(0..4);
        while x + offsets[chose].x < 0
            || x + offsets[chose].x > 2 * height as i8
            || y + offsets[chose].y < 0
            || y + offsets[chose].y > 2 * width as i8
            || !(maze[(x + offsets[chose].x) as usize][(y + offsets[chose].y) as usize] == "?")
        {
            chose = thread_rng().gen_range::<usize, _>(0..4);
        }
        let choseX = x + offsets[chose].x;
        let choseY = y + offsets[chose].y;
        let choseWallX = x + offsets[chose].x / 2;
        let choseWallY = y + offsets[chose].y / 2;
        maze[choseX as usize][choseY as usize] = " ".to_string();
        maze[choseWallX as usize][choseWallY as usize] = " ".to_string();
        createMaze(choseX, choseY, maze);
    }
}

fn printMaze(maze: &Vec<Vec<String>>){
    for row in maze{
        for column in row{
            print!("{}",*column);
        }
        print!("\n");
    }
}

fn main() {
    let mut maze: Vec<Vec<String>> = vec![vec!["".to_string(); 2 * width + 1]; 2 * height + 1];
    initMaze(&mut maze);
    createMaze(1, 1, &mut maze);
    printMaze(&maze);
}
