use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
    containers: Vec<i64>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Operation {
    Fill(usize),        // Fill container i
    Empty(usize),       // Empty container i
    Pour(usize, usize), // Pour from container i to container j
}

impl State {
    fn new(containers: Vec<i64>) -> Self {
        State { containers }
    }
}

fn bfs_container_puzzle_solver(capacities: Vec<i64>, target: i64) -> Option<Vec<Operation>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    let initial_state = State::new(vec![0; capacities.len()]);
    queue.push_back((initial_state.clone(), vec![]));
    visited.insert(initial_state.clone());

    while let Some((state, path)) = queue.pop_front() {
        // Check if any container has the target volume
        if state.containers.iter().any(|&x| x == target) {
            return Some(path);
        }

        for i in 0..capacities.len() {
            // Fill container i
            let mut new_state = state.clone();
            new_state.containers[i] = capacities[i];
            if visited.insert(new_state.clone()) {
                let mut new_path = path.clone();
                new_path.push(Operation::Fill(i));
                queue.push_back((new_state, new_path));
            }

            // Empty container i
            let mut new_state = state.clone();
            new_state.containers[i] = 0;
            if visited.insert(new_state.clone()) {
                let mut new_path = path.clone();
                new_path.push(Operation::Empty(i));
                queue.push_back((new_state, new_path));
            }

            // Pour from container i to container j
            #[allow(clippy::needless_range_loop)]
            for j in 0..capacities.len() {
                if i != j {
                    let mut new_state = state.clone();
                    let pour_amount =
                        (state.containers[i]).min(capacities[j] - state.containers[j]);
                    new_state.containers[i] -= pour_amount;
                    new_state.containers[j] += pour_amount;
                    if visited.insert(new_state.clone()) {
                        let mut new_path = path.clone();
                        new_path.push(Operation::Pour(i, j));
                        queue.push_back((new_state, new_path));
                    }
                }
            }
        }
    }

    None // No solution found
}

fn main() {
    let capacities = vec![3, 6, 26]; // container capacities
    let target = 7; // target volume

    capacities.iter().enumerate().for_each(|(i, &capacity)| {
        println!("Container {}: capacity = {}", i, capacity);
    });
    println!("Target volume: {}", target);

    if let Some(solution) = bfs_container_puzzle_solver(capacities.clone(), target) {
        let mut state = State::new(vec![0; capacities.len()]);
        println!("Solution found with {} steps:", solution.len());
        for (n, operation) in solution.iter().enumerate() {
            match *operation {
                Operation::Fill(index) => {
                    println!("{}: Fill container {}", n + 1, index);
                    state.containers[index] = capacities[index]
                }
                Operation::Empty(index) => {
                    println!("{}: Empty container {}", n + 1, index);
                    state.containers[index] = 0
                }
                Operation::Pour(index1, index2) => {
                    println!(
                        "{}: Pour from container {} to container {}",
                        n + 1,
                        index1,
                        index2
                    );
                    let pour_amount = (state.containers[index1])
                        .min(capacities[index2] - state.containers[index2]);
                    state.containers[index1] -= pour_amount;
                    state.containers[index2] += pour_amount;
                }
            }
            println!("Current state: {:?}", state);
        }
    } else {
        println!("No solution found.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diehard3() {
        let capacities = vec![3, 5];
        let target = 4;
        let solution = bfs_container_puzzle_solver(capacities, target);
        assert!(solution.is_some());
        let solution = solution.unwrap();
        assert_eq!(solution.len(), 6);
        assert_eq!(solution[0], Operation::Fill(1));
        assert_eq!(solution[1], Operation::Pour(1, 0));
        assert_eq!(solution[2], Operation::Empty(0));
        assert_eq!(solution[3], Operation::Pour(1, 0));
        assert_eq!(solution[4], Operation::Fill(1));
        assert_eq!(solution[5], Operation::Pour(1, 0));
    }

    #[test]
    fn test_bfs_container_puzzle_solver() {
        let capacities = vec![3, 5, 8];
        let target = 4;
        let solution = bfs_container_puzzle_solver(capacities, target);
        assert!(solution.is_some());
        let solution = solution.unwrap();
        assert_eq!(solution.len(), 6);
        assert_eq!(solution[0], Operation::Fill(1));
        assert_eq!(solution[1], Operation::Pour(1, 0));
        assert_eq!(solution[2], Operation::Empty(0));
        assert_eq!(solution[3], Operation::Pour(1, 0));
        assert_eq!(solution[4], Operation::Fill(1));
        assert_eq!(solution[5], Operation::Pour(1, 0));
    }

    #[test]
    fn test_solution_2() {
        let capacities = vec![5, 11, 13, 24];
        let target = 7;
        let solution = bfs_container_puzzle_solver(capacities, target);
        assert!(solution.is_some());
        let solution = solution.unwrap();
        assert_eq!(solution.len(), 4);
        assert_eq!(solution[0], Operation::Fill(0));
        assert_eq!(solution[1], Operation::Pour(0, 1));
        assert_eq!(solution[2], Operation::Fill(2));
        assert_eq!(solution[3], Operation::Pour(2, 1));
    }

    #[test]
    fn test_solution_3() {
        let capacities = vec![3, 5, 8, 13, 18];
        let target = 7;
        let solution = bfs_container_puzzle_solver(capacities, target);
        assert!(solution.is_some());
        let solution = solution.unwrap();
        assert_eq!(solution.len(), 3);
        assert_eq!(solution[0], Operation::Fill(4));
        assert_eq!(solution[1], Operation::Pour(4, 0));
        assert_eq!(solution[2], Operation::Pour(4, 2));
    }
}
