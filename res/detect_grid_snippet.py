#%% define graphim

"""
Define highly connected pixels (more than 2 neighbouring pixels) as nodes in
the graph. Lowly connected pixels are just connections between nodes.
"""

t = process_time()

graphim = np.full_like(skel, 0, dtype = np.uint8)

graphim += np.roll(skel, 1, axis=0)
graphim += np.roll(skel, -1, axis=0)
graphim += np.roll(skel, 1, axis=1)
graphim += np.roll(skel, -1, axis=1)

graphim += np.roll(np.roll(skel, 1, axis=0), 1, axis=1)
graphim += np.roll(np.roll(skel, -1, axis=0), 1, axis=1)
graphim += np.roll(np.roll(skel, 1, axis=0), -1, axis=1)
graphim += np.roll(np.roll(skel, -1, axis=0), -1, axis=1)

graphim[:,0] = False
graphim[:,-1] = False
graphim[0,:] = False
graphim[-1,:] = False

edges = np.logical_and(graphim > 0, skel)
edges = np.logical_and(edges, graphim<3)

nodes = np.logical_and(graphim > 2, skel)

graphplot = np.copy(graphim)
graphplot[edges] = 127
graphplot[nodes] = 255

print("Define graph image process time: {time}".format(time=process_time() - t))

plt.close('all')
plt.figure()
plt.imshow(nodes)

plt.figure()
plt.imshow(edges)

plt.figure()
plt.imshow(graphplot)

#%% label edges

"""
Blob labelling for edges of the graph
"""

from morphology import label, edge_label

t = process_time()

edge_labels = label(edges)

print("Edge blob detection time: {time}".format(time=process_time() - t))

plt.close('all')
plt.figure()
plt.imshow(edge_labels.T)

#%% label nodes with node edges
"""
blob labelling for the nodes of the graph

As we label the blobs, also label the neighbouring edge pixels to the blobs - 
those that hit edge blobs detected above means that node blob connects to 
that edge blob. This will be used below to construct a graph.
"""

t = process_time()

node_label = label(nodes)

node_edges = edge_label(node_label)

print("Centroid and centroid edge detection time: {time}".format(time=process_time() - t))

plt.close('all')
plt.figure()
plt.imshow(node_label)

q = np.zeros_like(nodes, dtype=bool)
for item in node_edges.values():
    for (i,j) in item:
        
        if (i < q.shape[0]) & (j < q.shape[0]) & (i >= 0) & (j >= 0):
            q[i,j] = True
    
plt.figure()
plt.title("Node edges")
plt.imshow(q.T)


#%% construct graph

"""
Generate graph using blob information above.
"""

from grafpy import Node

t = process_time()
    
edge_connections = {n:[] for n in range(1, edge_labels.max() + 1)}
graph = {}

# Create nodes in graph and connect via edge pixels
for i in range(1, node_label.max()):
    
    centroid = np.mean(np.stack(np.nonzero(node_label == i)), axis=1)
    
    node = Node(i=centroid[0], j=centroid[1], connects=[])
    
    graph[i] = node
    
    for (x,y) in node_edges[i]:
        
        edge_hit = edge_labels[x,y]
        
        if (edge_hit != 0): edge_connections[edge_hit].append(i)

# Connect
for indices in edge_connections.values():
    
    if len(indices) != 2: continue

    if indices[0] not in graph[indices[1]].connects : graph[indices[1]].connects.append(indices[0])
    if indices[1] not in graph[indices[0]].connects : graph[indices[0]].connects.append(indices[1])

               
print("Graph construction time: {time}".format(time=process_time() - t))

plt.close('all')
plt.figure()
plt.title("Graph")
plot_graph(graph, color='r')
plt.imshow(im_original.T, cmap='gray')

plt.figure()
plt.title("Image with skeleton")
arr = im_original.T.copy()
arr[skel.T] = arr.max()
plt.imshow(arr, cmap='gray')


#%% graph filtering - merging

"""
Next set of cells are about filtering the graph based on some precondition.

This cell merges nodes that within 30 pixels of each other.
"""

t = process_time()

def condition(A, B): return ((A.i - B.i)**2 + (A.j - B.j)**2)**0.5 < 30

def merge(to_merge):
    
    # Find the centroid of the nodes to merge
    mean_i = sum([node.i for node in to_merge]) / len(to_merge)
    mean_j = sum([node.j for node in to_merge]) / len(to_merge)

    return (mean_i, mean_j)

graph = merge_all(graph, condition, merge)

print("Graph filtering - merge close conections: {time}".format(time=process_time() - t))

# plt.close('all')
plt.figure()
plt.imshow(im.T)
plot_graph(graph, color='r')
#%% graph filtering - remove single connected nodes

"""
Remove nodes that connected to only on other node recursively.

These are likely false positives and is relatively cheap.
"""

def condition(node): return len(node.connects) < 2

t = process_time()

graph = prune_recursive(graph, condition)

print("Graph filtering - single nodes time: {time}".format(time=process_time() - t))

plt.close('all')
plt.figure()
plt.imshow(im.T)
plot_graph(graph, color='r')



#%% graph filtering - quad pass filter

"""
Allow only nodes that are connected to themselves via 3 other nodes 
(have 3 nodes of seperation). Only allowing nodes that are part of a 
square (at least an object that has 4 corners)
"""
                 
t = process_time()

removal = []
for key in graph.keys():
    def success_cond(node, path): return graph[key] == node
    def fail_cond(node, path): return len(path) == 5
    
    path = breadth_first_search(graph, key, success_cond, fail_cond)

    # remove erroneous or absent paths        
    if (any([len(p) != 5 for p in path])) | (not any(path)): removal.append(key)

for key in removal:
    
    plt.plot(graph[key].i, graph[key].j, 'D')
    
graph = remove(graph, removal)

print("Graph filtering - quad pass filter time: {time}".format(time=process_time() - t))

plt.figure()
plt.imshow(im.T)
plot_graph(graph, color='r')

#%% graph filtering - remove single connected nodes

"""
Once again remove single connected nodes in case we created any above
"""

def condition(node): return len(node.connects) < 2

t = process_time()

graph = prune_recursive(graph, condition)

print("Graph filtering - single nodes time: {time}".format(time=process_time() - t))

plt.close('all')
plt.figure()
plt.imshow(im.T)
plot_graph(graph)
    
#%% graph filtering - find the largest connected graph
"""
Return the largest connected graph - have no way to deal with multiple graphs.
"""

t = process_time()

keys = set(graph.keys())

sub_graphs = []

while(any(keys)):
    
    front = [next(iter(keys))]
    sub_graph_keys = []
    
    while(True):
        
        # iterate the front
        front = flatten([graph[i].connects for i in front])
        
        # remove those already found
        new_front = []
        for i in front:
            if i in keys:
                new_front.append(i)
                keys.remove(i)
                sub_graph_keys.append(i)
        
        front = new_front
        
        if not any(front): break
    
    sub_graph = {key: graph[key] for key in sub_graph_keys}
    sub_graphs.append(sub_graph)


plt.close('all')
plt.figure()

for sub_graph in sub_graphs:
    plot_graph(sub_graph)
    
maxN = max(map( lambda x : len(x), sub_graphs))

for sub_graph in sub_graphs:
    if len(sub_graph) == maxN:
        graph = sub_graph
        break
    
print("Graph filtering - largest graph filter time: {time}".format(time=process_time() - t))
        
plt.figure()
plot_graph(graph)

#%% graph filtering - check graph
"""
Check we didnt make a "compile-time" mistake above.
"""

t = process_time()

diagnose_graph(graph)

print("Graph filtering - check graph: {time}".format(time=process_time() - t))
