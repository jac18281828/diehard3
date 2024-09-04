# diehard3

The Diehard 3 container problem is a type of puzzle where you need to measure out a specific amount of water using containers of different sizes and the operations of filling, draining, or pouring water between them. The problem is often presented with two containers, but the general approach can be extended to any number of containers.

### General Solution Approach:

1. **State Representation**: 
   - Represent the state of the problem as a tuple where each element corresponds to the amount of water in one of the containers. For example, if you have two containers with capacities of 3 and 5 liters, a state could be represented as \((x, y)\) where \(x\) is the amount of water in the 3-liter container and \(y\) is the amount of water in the 5-liter container.

2. **Operations**:
   - **Fill**: Fill any container to its maximum capacity.
   - **Drain**: Empty any container.
   - **Pour**: Pour water from one container to another until one is empty or the other is full.

3. **Graph Search**:
   - **Breadth-First Search (BFS)** or **Depth-First Search (DFS)**: Treat each possible state as a node in a graph. The edges between the nodes represent the valid operations (fill, drain, pour). Start from the initial state (where all containers might be empty) and explore all possible states by applying the operations. The goal is to find a state where the amount of water in any container matches the target volume.
   
   - **Visited States**: Keep track of visited states to avoid cycles and redundant calculations. This helps in optimizing the search.

4. **Target Check**:
   - As you explore the states, check if any state contains the target volume of water in any of the containers.

5. **Backtracking**:
   - If the target volume is found, you can backtrack to find the sequence of operations that led to the solution.

### Example Usage

### Example Input

```bash
$  cargo run --bin diehard3 --release
```

### Example:

Consider the classic problem presented in Diehard 3 with two containers: one with 3 liters capacity and one with 5 liters capacity, and the target is to measure out 4 liters.  

1. **Initial State**: \((0, 0)\)
2. **Operations**:
   - Fill the 5-liter container: \((0, 5)\)
   - Pour from 5-liter to 3-liter container: \((3, 2)\)
   - Empty the 3-liter container: \((0, 2)\)
   - Pour from 5-liter to 3-liter container: \((2, 0)\)
   - Fill the 5-liter container: \((2, 5)\)
   - Pour from 5-liter to 3-liter container: \((3, 4)\)

Here, we have found the target volume of 4 liters in the 5-liter container.

### Generalization to Multiple Containers:
For multiple containers, the BFS/DFS approach can be extended by considering the tuple representation for all containers, and similarly, exploring all possible states by applying the fill, drain, and pour operations to any of the containers.

### Summary:
- Represent the problem as a graph where states are nodes.
- Use BFS/DFS to explore possible states.
- Keep track of visited states.
- Identify when the target volume is reached.

This approach works for any number of containers and target volumes as long as the operations can be applied within the constraints of the containers' capacities.