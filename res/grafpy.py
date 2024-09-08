from itertools import compress, chain
from collections import namedtuple
import matplotlib.pyplot as plt

Node = namedtuple('Node', ['i','j','connects'])

# Flatten array
def flatten(l, ltypes=(list, tuple)):
    ltype = type(l)
    l = list(l)
    i = 0
    while i < len(l):
        while isinstance(l[i], ltypes):
            if not l[i]:
                l.pop(i)
                i -= 1
                break
            else:
                l[i:i + 1] = l[i]
        i += 1
    return ltype(l)

def remove(graph, removal):
    """
    

    Parameters
    ----------
    graph : Graph.
    removal : list of keys for removal

    Returns
    -------
    graph : Graph.

    """
    
    for rkey in removal: 
        
        for key in graph[rkey].connects:
            graph[key].connects.remove(rkey)
     
    for rkey in removal:       
        del graph[rkey]
        
    return graph

def insert(graph, node, key):
    """
    Adds a node into graph and ensures connections are added to connected nodes

    Parameters
    ----------
    graph : Graph.
    node: node to be inserted

    Returns
    -------
    graph : Graph.

    """
    
    if key in graph: raise Exception("key already in graph")
    
    for connection in node.connects:
        if key not in graph[connection].connects:
            graph[connection].connects.append(key)
    
    graph[key] = node
        
    return graph

def prune(graph, condition):
    """
    

    Parameters
    ----------
    graph : Graph.
    condition : Function takes a Node and returns a boolean.

    Returns
    -------
    graph : The pruned graph.

    """
    
    # Items to be deleted
    targets = [key for key in graph.keys() if condition(graph[key])]
    
    for key in targets: graph = remove(graph, targets)
    
    return graph

def prune_recursive(graph, condition):
    """
    

    Parameters
    ----------
    graph : Graph.
    condition : Function takes a Node and returns a boolean.

    Returns
    -------
    graph : The pruned graph.

    """
     
    # Items to be deleted
    targets = [key for key in graph.keys() if condition(graph[key])]
    
    graph = remove(graph, targets)
    
    while(len(targets) != 0):
        
        targets = [key for key in graph.keys() if condition(graph[key])]
        
        graph = remove(graph, targets)

    return graph

def breadth_first_search(graph, start_index, success_cond, fail_cond):
    """
    

    Parameters
    ----------
    graph : Graph.
    index: Graph index to start search.
    success_cond : Function takes a Node and list of int and returns a boolean.
    fail_cond : Function takes a Node and list of int and returns a boolean.

    Returns
    -------
    path: list of indices taken to reach success condition.

    """
    
    def march_forward(crawler):
        
        options = graph[crawler[-1]].connects.copy()
        options.remove(crawler[-2])
        return [crawler.copy() + [i] for i in options]
    
    crawlers = [[start_index, i] for i in graph[start_index].connects]
    
    while(len(crawlers) != 0):
        
        # Check for success condition
        success = [success_cond(graph[crawler[-1]], crawler) for crawler in crawlers]
        
        if any(success): 
            return [crawler for crawler in compress(crawlers, success)]
        
        # Prune away options with the failure condition
        crawlers = [crawler for crawler in crawlers if not fail_cond(graph[crawler[-1]], crawler)]
        
        # March crawlers forward
        crawlers = list(chain.from_iterable([march_forward(crawler) for crawler in crawlers]))
    
    return []
    
def merge_neighbours(graph, condition, merge):
    """
    

    Parameters
    ----------
    graph : Graph.
    condition : Function takes a Node and returns a boolean.
    merge : takes a list of nodes and returns a tuple of new i,j

    Returns
    -------
    graph : The pruned graph.

    """
    
    keys = [key for key in graph.keys()]
        
    for key in keys:
        
        # already been removed/merged
        if not key in graph: continue
    
        to_merge = [neighbour for neighbour in graph[key].connects if condition(graph[key], graph[neighbour])]
        to_merge.append(key)
        
        # Create new data
        (new_i, new_j) = merge([graph[neighbour] for neighbour in to_merge])
        
        # Create new connects list
        connects = list(set(flatten([graph[node].connects for node in to_merge])))
        
        # Remove connections that are being removed
        connects = [item for item in connects if item not in to_merge]

        new_node = Node(i=new_i, j=new_j, connects=connects)
        
        # Remove all to be merged        
        graph = remove(graph, to_merge)
        
        # Insert the new node
        graph = insert(graph, new_node, to_merge[0])

    return graph

def merge_all(graph, condition, merge):
    """
    

    Parameters
    ----------
    graph : Graph.
    condition : Function takes a Node and returns a boolean.
    merge : takes a list of nodes and returns a tuple of new i,j

    Returns
    -------
    graph : The pruned graph.

    """
    
    keys = [key for key in graph.keys()]
        
    for key in keys:
        
        # already been removed/merged
        if not key in graph: continue
    
        to_check = [k for k in graph.keys()if k != key]
    
        to_merge = [item for item in to_check if condition(graph[key], graph[item])]
        
        if len(to_merge) == 0: continue
        to_merge.append(key)

        # Create new data
        (new_i, new_j) = merge([graph[neighbour] for neighbour in to_merge])
        
        # Create new connects list
        connects = list(set(flatten([graph[node].connects for node in to_merge])))
        
        # Remove connections that are being removed
        connects = [item for item in connects if item not in to_merge]

        new_node = Node(i=new_i, j=new_j, connects=connects)
                
        # Remove all to be merged        
        graph = remove(graph, to_merge)
        
        # Insert the new node
        graph = insert(graph, new_node, to_merge[0])

    return graph

def mutate(graph, mutator):
    
    new_graph = {}
    
    for key in graph.keys():
        (i,j) = mutator(graph[key].i, graph[key].j)
        new_graph[key] = Node(i, j, graph[key].connects.copy())
        
    return new_graph

def diagnose_graph(graph):
    
    fail = False
    
    for key in graph.keys():
        
        # Check connections are in graph
        for neighbour in graph[key].connects:
            if neighbour not in graph:
                print("{A} connects to {B}, but {B} is not in graph".format(A=key, B=neighbour))
                return

        
        # Check neighbours connect to each other
        for neighbour in graph[key].connects:
            if key not in graph[neighbour].connects:
                print("{A} connects to {B}, but {B} does not connect to {A}".format(A=key, B=neighbour))
                fail = True
                


    
    if not fail: print("graph success")
    
def plot_graph(graph, color='r', annotate=False):
    
    for key in graph.keys():
        
        node = graph[key]
        
        plt.plot(node.i, node.j, 'x')
        if annotate: plt.annotate(key, (node.i, node.j))
        
        
        if len(node.connects) > 1:
            
            for index in node.connects:
                
                if index not in graph:
                    print("missing index {} from graph".format(index))
                    continue
                
                plt.plot(
                    [node.i, graph[index].i],
                    [node.j, graph[index].j], 
                    color)      
        