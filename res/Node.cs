namespace BufferGen.Optimisers.Pathing
{
    /// <summary>
    /// Node represents the end of a <see cref="Prime{T}"/> - the entrance or exit from the <see cref="Prime{T}"/>.
    /// </summary>
    internal class Node<T>(IEnumerable<Prime<T>> connections)
    {
        public List<Prime<T>> Connections { get; private set; } = connections.ToList();
        public int Count => Connections.Count;
        /// <summary>
        /// Only has one connection and therefore must terminate the path.s
        /// </summary>
        public bool IsEndPoint => Connections.Count == 1;

        /// <summary>
        /// Has an odd number of connections and therefore will at some point terminate the path.
        /// </summary>
        public bool IsJunction => Connections.Count % 2 == 1;

        public bool HasConnections => Connections.Count > 0;

        public Prime<T> this[int index]
        {
            get
            {
                return Connections[index];
            }
        }

        /// <summary>
        /// Add a connection to a <see cref="Prime{T}"/>.
        /// </summary>
        /// <param name="prime"></param>
        public void AddConnection(Prime<T> prime) => Connections.Add(prime);

        /// <summary>
        /// Merges two nodes together.
        /// </summary>
        /// <param name="node"></param>
        public void MergeWith(ref Node<T> node)
        {
            Connections.AddRange(node.Connections);
            Connections = Connections.Distinct().ToList();
            foreach (Prime<T> prime in node.Connections)
            {
                prime.ReplaceNode(node, this);
            }
            node = this;
        }

        /// <summary>
        /// Sever this node, creating a new node in the process. This node severs connections
        /// to the argument primes. The new node replaces this node and is connected to all the argument primes.
        /// 
        /// Dont have to pass the perfect intersection.
        /// 
        /// Can pass list of length 0
        /// 
        /// </summary>
        /// <param name="primes"></param>
        public Node<T> SeverFrom(IEnumerable<Prime<T>> primes)
        {
            IEnumerable<Prime<T>> intersection = Connections.Where(primes.Contains).Distinct();
            Node<T> newNode = new(intersection);

            foreach (Prime<T> prime in intersection)
            {
                prime.ReplaceNode(this, newNode);
            }
            Connections.RemoveAll(intersection.Contains);
            return newNode;
        }

        /// <summary>
        /// Return the opposite side <see cref="Node{T}"/> of the ith <see cref="Prime{T}"/> connected to this <see cref="Node{T}"/>.
        /// </summary>
        /// <param name="i"></param>
        /// <returns></returns>
        public Node<T> TraverseConnection(int i) => Connections[i].TraverseFrom(this);

        public Node<T> TraverseConnection(Prime<T> prime) => Connections.First(e => e == prime).TraverseFrom(this);

        public Node<T> Clone() =>  new(Connections);
    }
}
