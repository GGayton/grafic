using BufferGen.Optimisers;
using MathNet.Numerics.LinearAlgebra;

namespace BufferGen.Optimisers.Pathing
{
    /// <summary>
    /// A path is collection of primes that are executed in order.
    /// </summary>
    internal class Path<T>(IEnumerable<Prime<T>> primes)
    {
        public IEnumerable<Node<T>> Junctions => Nodes.Where(e => e.IsJunction);
        public IEnumerable<Node<T>> EndPoints => Nodes.Where(e => e.IsEndPoint);

        public bool IsCyclical => !Junctions.Any();

        public Vector<double> StartPoint => Primes.First().StartPoint;
        public Vector<double> EndPoint => Primes.Last().EndPoint;
        /// <summary>
        /// Path has two junctions only
        /// </summary>
        public bool IsDirect => Junctions.Count() == 2;

        public List<Prime<T>> Primes = primes.Distinct().ToList();

        public IEnumerable<Node<T>> Nodes
        {
            get
            {
                List<Node<T>> nodes = [];

                foreach (Prime<T> prime in Primes)
                {
                    nodes.Add(prime.StartNode);
                    nodes.Add(prime.EndNode);
                }

                return nodes.Distinct();
            }
        }

        /// <summary>
        /// Connect two paths by merging <see cref="Node{T}"/> nodeB into <see cref="Node{T}"/> nodeA
        /// </summary>
        /// <param name="nodeA"></param>
        /// <param name="nodeB"></param>
        /// <returns></returns>
        public void Merge(Node<T> nodeA, Node<T> nodeB, IEnumerable<Prime<T>> primes)
        {
            nodeA.MergeWith(ref nodeB);
            Primes.AddRange(primes);
            Primes = Primes.Distinct().ToList();
        }

        /// <summary>
        /// Subtract the path by severing all node connections and removing the overlapping primes.
        /// 
        /// Performs no action on the argument [path].
        /// </summary>
        /// <param name="path"></param>
        /// <returns></returns>
        public void Subtract(Path<T> path)
        {
            // Remove all primes from this path
            Primes.RemoveAll(path.Primes.Contains);

            // Separate out nodes by severing the connection so the new node
            IEnumerable<Node<T>> nodes = path.Nodes;
            foreach (Node<T> node in nodes)
            {
                node.SeverFrom(path.Primes);
            }
        }

        /// <summary>
        /// Assuming the path is already ordered from start to finish, reorder the path according to the prime.
        /// </summary>
        /// <param name="prime"></param>
        /// <returns></returns>
        public Path<T> ReorderFrom(Prime<T> prime)
        {
            int index = Primes.IndexOf(prime);
            Primes = Primes.Skip(index).Concat(Primes.Take(index)).ToList();
            return this;
        }
        /// <summary>
        /// Reverses the direction of every prime in this path
        /// </summary>
        /// <returns>this</returns>
        public Path<T> Reverse()
        {
            Primes.Reverse();
            Primes.ForEach(p => p.Reverse());
            return this;
        }

        /// <summary>
        /// Checks the path is direct and ordered.
        /// </summary>
        /// <returns></returns>
        public bool IsWriteable()
        {
            if (IsDirect && !Primes.First().StartNode.IsJunction) return false;
            if (IsDirect && !Primes.Last().EndNode.IsJunction) return false;

            for (int i = 0; i < Primes.Count - 1; i++)
            {
                if (!Primes[i].EndNode.Connections.Contains(Primes[i + 1])) return false;
            }

            return true;
        }
    }
}
