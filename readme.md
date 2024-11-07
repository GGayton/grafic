
# Api

## Constructors

### from_sparse

### unidirectional_from_matrix

### bidirectional_from_matrix

## Iterators

### filter

### iter

### into_iter

### iter_mut

## Mutators

### add_graph

### connect_nodes

### destroy_node

### filter

### recursive_filter

### neighbour_recursive_filter

### merge

### split

## Iterators

### breadth_first_search
Always returns the starting node as the start of the search
### breadth_first_search_cost
### breadth_first_search_n

### depth_first_search
### depth_first_search_cost
### depth_first_search_n

## Generators

### Optimise?

how to handle n connections of node A to B.
If seperate edges, then would have to call .unique()
if a specific enum, have to implement the nGO


# PLANS
1. prune_nodes
2. form skeleton image
3. split?
4. optimise path


