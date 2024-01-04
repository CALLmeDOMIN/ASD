fn macierz_sasiadow(graph: &Vec<Vec<(i32, i32)>>) -> Vec<Vec<i32>> {
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; graph.len()]; graph.len()];

    for vertices in graph {
        let vertex: usize = vertices[0].0 as usize;

        for i in 1..vertices.len() {
            let edge = vertices[i].0 as usize;
            matrix[vertex - 1][edge - 1] = 1 * vertices[i].1;
        }
    }

    matrix
}

fn macierz_indydencji(graph: &Vec<Vec<(i32, i32)>>) -> Vec<Vec<i32>> {
    let mut edges = 0;

    for v in graph {
        for _ in v[1..].iter() {
            edges += 1;
        }
    }

    let mut matrix: Vec<Vec<i32>> = vec![vec![0; edges]; graph.len()];

    let mut edge_counter: usize = 0 as usize;

    for v in graph {
        let vertex: usize = v[0].0 as usize;

        for i in 1..v.len() {
            let dest_v = v[i].0 as usize;
            matrix[vertex - 1][edge_counter] = 1 * v[i].1;
            matrix[dest_v - 1][edge_counter] = -1 * v[i].1;
            edge_counter += 1;
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
            print!("{} ", value);
        }
        println!("");
    }

    println!("");

    let matrix = macierz_indydencji(&graph);

    for line in matrix {
        for value in line {
            if value >= 0 {
                print!(" {} ", value);
            } else {
                print!("{} ", value);
            }
        }
        println!("");
    }
}
