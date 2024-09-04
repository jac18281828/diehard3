import heapq
import sys
import itertools
from collections import deque

class State:
    def __init__(self, containers, steps=0):
        self.containers = tuple(containers)
        self.steps = steps

    def __hash__(self):
        return hash(self.containers)

    def __eq__(self, other):
        return self.containers == other.containers

    def __repr__(self):
        return f"State({self.containers})"

    def heuristic(self, target):
        return min(abs(x - target) for x in self.containers)

    def __lt__(self, other):
        return self.steps < other.steps

def a_star_container_solution(capacities, target):
    initial_state = State([0] * len(capacities))
    open_set = []
    heapq.heappush(open_set, (initial_state.heuristic(target), 0, initial_state))
    visited = set()
    visited.add(initial_state)

    while open_set:
        _, cost, state = heapq.heappop(open_set)

        if target in state.containers:
            return state.steps

        for i in range(len(capacities)):
            # Fill container i
            new_containers = list(state.containers)
            new_containers[i] = capacities[i]
            new_state = State(new_containers, state.steps + 1)
            if new_state not in visited:
                visited.add(new_state)
                heapq.heappush(open_set, (new_state.steps + new_state.heuristic(target), new_state.steps, new_state))

            # Empty container i
            new_containers = list(state.containers)
            new_containers[i] = 0
            new_state = State(new_containers, state.steps + 1)
            if new_state not in visited:
                visited.add(new_state)
                heapq.heappush(open_set, (new_state.steps + new_state.heuristic(target), new_state.steps, new_state))

            # Pour from container i to container j
            for j in range(len(capacities)):
                if i != j:
                    new_containers = list(state.containers)
                    pour_amount = min(new_containers[i], capacities[j] - new_containers[j])
                    new_containers[i] -= pour_amount
                    new_containers[j] += pour_amount
                    new_state = State(new_containers, state.steps + 1)
                    if new_state not in visited:
                        visited.add(new_state)
                        heapq.heappush(open_set, (new_state.steps + new_state.heuristic(target), new_state.steps, new_state))

    return None

def generate_puzzle(min_steps):
    for num_containers in range(3, 10):  # Try with 2 to 4 containers
        for capacities in itertools.combinations(range(3, 30), num_containers):
            for target in range(1, max(capacities)):
                solution_steps = a_star_container_solution(capacities, target)
                if solution_steps and solution_steps == min_steps:
                    return capacities, target, solution_steps

    return None

def main():
    if len(sys.argv) > 1:
        min_steps = int(sys.argv[1])
    else:
        min_steps = 5
    puzzle = generate_puzzle(min_steps)

    if puzzle:
        capacities, target, solution_steps = puzzle
        print(f"Found puzzle with capacities: {capacities} and target: {target}")
        print(f"Solution takes {solution_steps} steps")
    else:
        print("No puzzle found with the given number of steps.")

if __name__ == "__main__":
    main()