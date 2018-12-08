use std::num::ParseIntError;


const INPUT: &str = include_str!("../input.txt");
//const SAMPLE_INPUT: &str = include_str!("../sample_input.txt");


struct Node {
    children: Vec<Node>,
    meta: Vec<u32>,
}

impl Node {
    fn from_data(data: &Vec<u32>) -> Self {
        fn build_note(data: &[u32]) -> (Node, usize) {
            let child_count = data[0];
            let meta_count = data[1];
            let mut index = 2usize;

            let mut children: Vec<Node> = Vec::new();
            let mut meta: Vec<u32> = Vec::new();

            for _i in 0..child_count {
                let (child, len) = build_note(&data[index..]);
                children.push(child);
                index += len;
            }            
            for i in index..(index+(meta_count as usize)) {
                meta.push(data[i]);
            }
            index += meta_count as usize;

            (Node{ children, meta}, index)
        }
        build_note(data).0
    }

    fn hereditary_sum_meta (&self) -> u32 {
        self.meta.iter().sum::<u32>() + self.children.iter().map(|c| c.hereditary_sum_meta()).sum::<u32>()
    }

    fn value(&self) -> u32 {
        if self.children.is_empty() {
            return self.meta.iter().sum::<u32>()
        }
        else {
            let mut partial_value = 0u32;
            for m in &self.meta {
                let n: usize = (m-1) as usize;
                if n < self.children.len() {
                    partial_value += self.children[n].value();
                }
            }
            partial_value
        }
            
    }
}


fn solve_part_1(node: &Node) -> u32 {
    node.hereditary_sum_meta()
}

fn solve_part_2(node: &Node) -> u32 {
    node.value()
}

fn format_input(input_str: &str) -> Result<Node, ParseIntError> {
    let data = input_str.trim()
        .split_whitespace()
        .map(|d| d.parse())
        .collect::<Result<Vec<_>, ParseIntError>>()?;
    Ok(Node::from_data(&data))
}



fn main() -> Result<(), ParseIntError> {
    let input = format_input(INPUT)?;
    println!("Answer part 1: {}", solve_part_1(&input));

    println!("Answer part 2: {}", solve_part_2(&input));

    Ok(())
}
