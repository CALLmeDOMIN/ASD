fn macierz_sasiadow(graph: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut v: Vec<i32> = Vec::new();

    for vertices in graph {
        for vs in vertices {
            if v.contains(vs) {
                continue;
            }
            v.push(*vs);
        }
    }

    v.sort();

    let mut matrix: Vec<Vec<i32>> = vec![vec![0; v.len() + 1]; v.len() + 1];

    for i in 1..v.len() + 1 {
        matrix[0][i] = v[i - 1];
        matrix[i][0] = v[i - 1];
    }

    for vertices in graph.iter() {
        let vertex: i32 = vertices[0];
        for i in 1..vertices.len() {
            let edge = vertices[i];
            let index1: usize = matrix[0].iter().position(|&x| x == vertex).unwrap();
            let index2: usize = matrix[0].iter().position(|&x| x == edge).unwrap();
            matrix[index1][index2] = 1;
        }
    }

    matrix
}

fn macierz_indydencji(graph: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut v: Vec<i32> = Vec::new();

    for vertices in graph {
        for vs in vertices {
            if v.contains(vs) {
                continue;
            }
            v.push(*vs);
        }
    }

    v.sort();

    let mut matrix: Vec<Vec<i32>> = vec![vec![0; v.len() + 1]; graph.len() + 1];

    for i in 1..v.len() + 1 {
        matrix[0][i] = v[i - 1];
    }

    for (i, vertices) in graph.iter().enumerate() {
        let vertex: i32 = vertices[0];
        for j in 1..vertices.len() {
            let edge = vertices[j];
            let index: usize = matrix[0].iter().position(|&x| x == edge).unwrap();
            if vertex < edge {
                matrix[i + 1][index] = 1;
            } else {
                matrix[i + 1][index] = -1;
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

    let mut graph: Vec<Vec<i32>> = Vec::new();

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
            if vertices[0] == values[0] {
                vertices.push(values[1]);
                pushed = true;
                break;
            }
        }

        // if the graph wasnt directed
        // for vertices in graph.iter_mut() {
        //     if vertices[0] == values[1] {
        //         vertices.push(values[0]);
        //         pushed = true;
        //         break;
        //     }
        // }

        if !pushed {
            graph.push(vec![values[0], values[1]]);
        }
    }

    println!("Graph: {:?}", graph);

    println!("Matrix: ");
    let matrix = macierz_sasiadow(&graph);
    for line in matrix.iter() {
        for value in line.iter() {
            print!("{} ", value);
        }
        println!("");
    }

    println!("Identity matrix: ");
    let matrix = macierz_indydencji(&graph);
    for line in matrix.iter() {
        for value in line.iter() {
            print!("{} ", value);
        }
        println!("");
    }
}
