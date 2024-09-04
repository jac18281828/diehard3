### How the Code Works:

1. **State Struct**: 
   - `State` represents the current state of the containers. The `containers` vector holds the amount of water in each container.

2. **Operation Enum**: 
   - `Operation` represents the possible operations: filling a container, emptying a container, or pouring water from one container to another.

3. **BFS Solution**:
   - The BFS algorithm explores all possible states by applying the available operations. It starts from the initial state (where all containers are empty) and searches for a state where any container holds the target volume.
   - **Visited States**: The `visited` set keeps track of visited states to avoid processing the same state multiple times, which helps in optimizing the search.
   - For each state, the algorithm tries all possible operations (fill, empty, pour) on all containers.

4. **Main Function**:
   - The `main` function demonstrates the usage of the `bfs_solution` function. It sets up the container capacities and the target volume, then searches for a solution and prints the sequence of operations if one is found.

### Example:
- Given containers with capacities `[3, 5, 8]` and a target volume of `4`, the program will find a sequence of operations to achieve the target volume in any of the containers.

This approach can handle any number of containers and any target volume, making it a general solution to the Diehard container problem.

### Example Input

```bash
$  cargo run --bin diehard3 --release
```

### Example Output

```bash
Container 0: capacity = 3
Container 1: capacity = 5
Container 2: capacity = 8
Target volume: 4
Solution found with 6 steps:
1: Fill container 1
Current state: State { containers: [0, 5, 0] }
2: Pour from container 1 to container 0
Current state: State { containers: [3, 2, 0] }
3: Empty container 0
Current state: State { containers: [0, 2, 0] }
4: Pour from container 1 to container 0
Current state: State { containers: [2, 0, 0] }
5: Fill container 1
Current state: State { containers: [2, 5, 0] }
6: Pour from container 1 to container 0
Current state: State { containers: [3, 4, 0] }
```