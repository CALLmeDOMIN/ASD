fn macierz_sasiadow(graph: &Vec<Vec<(i32, i32)>>) -> Vec<Vec<i32>> {
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; graph.len()]; graph.len()];

    for vertices in graph {
        let vertex: usize = vertices[0].0 as usize;

        for i in 1..vertices.len() {
            let edge = vertices[i].0 as usize;
            matrix[vertex - 1][edge - 1] = 1;
        }
    }

    matrix
}

fn macierz_indydencji(graph: &Vec<Vec<(i32, i32)>>) -> Vec<Vec<i32>> {
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; graph.len()]; graph.len()];

    for (i, vertices) in graph.iter().enumerate() {
        let vertex: usize = vertices[0].0 as usize;

        for j in 1..vertices.len() {
            let edge = vertices[j].0 as usize;
            if vertex < edge {
                matrix[i][j] = 1;
            } else {
                matrix[i][j] = -1;
            }
        }
    }

    matrix
}

fn main() {
    let mut data: Vec<String> = Vec::new();

    for line in std::fs::read_to_string("file.txt")
        .expect("File not found!")
        .lines()
    {
        data.push(line.trim().to_string());
    }

    let mut graph: Vec<Vec<(i32, i32)>> = Vec::new();

    for (i, line) in data.iter().enumerate() {
        if i == 0 || i == 1 {
            continue;
        }
        let values: Vec<i32> = line
            .split(", ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let mut pushed = false;
        for vertices in graph.iter_mut() {
            if vertices[0].0 == values[0] {
                vertices.push((values[1], values[2]));
                pushed = true;
                break;
            }
        }

        if !pushed {
            graph.push(vec![(values[0], -1), (values[1], values[2])]);
        }
    }

    for vertices in graph.iter() {
        for vertex in vertices.iter() {
            if vertex.1 == -1 {
                print!("{}: ", vertex.0);
            } else {
                print!("({}, {}) ", vertex.0, vertex.1);
            }
            println!("");
        }
    }

    let matrix = macierz_sasiadow(&graph);

    println!("");

    for line in matrix {
        for value in line {
            print!("{}", value);
        }
        println!("");
    }

    println!("");

    let matrix = macierz_indydencji(&graph);

    for line in matrix {
        for value in line {
            print!("{}", value);
        }
        println!("");
    }
}
