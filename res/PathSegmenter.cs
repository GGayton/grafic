using BufferGen.Optimisers;
using BufferGen.Primitives;
using MathNet.Numerics.LinearAlgebra;

namespace BufferGen.Optimisers.Pathing
{
    /// <summary>
    /// Segments a list of <see cref="Primitive"/> into 
    /// </summary>
    /// <param name="limit">Limit that decides if two primitives are connected</param>
    internal class PathSegmenter<T>(double limit, Func<T, Vector<double>> getStartPoint, Func<T, Vector<double>> getEndPoint)
    {
        public double[,] Cost { get; private set; } = new double[0, 0];
        public bool[,] Connects { get; private set; } = new bool[0, 0];
        private double Limit { get; } = limit;

        public IEnumerable<Path<T>> CyclicalPaths { get; private set; } = [];
        public IEnumerable<Path<T>> DirectPaths { get; private set; } = [];
        public IEnumerable<Path<T>> Paths => CyclicalPaths.Concat(DirectPaths);

        Func<T, Vector<double>> GetStartPoint { get; } = getStartPoint;
        Func<T, Vector<double>> GetEndPoint { get; } = getEndPoint;
        PathingFactory<T> Factory { get; } = new(getStartPoint, getEndPoint);

        /// <summary>
        /// 
        /// </summary>
        /// <param name="primitives">A list of primitives to create a path from.</param>
        /// <param name="limit">Specifies the upper limit to which two primitives can be considered connected.</param>
        /// <exception cref="Exception"></exception>
        public IEnumerable<Path<T>> Segment(IEnumerable<T> primitives)
        {
            ConstructMatrices(primitives);
            IEnumerable<Path<T>> connectedPaths = ConstructConnectedPaths(primitives);

            // Check validity of paths
            foreach (var path in connectedPaths)
            {
                if (path.Junctions.Count() % 2 == 1) throw new PathSegmenterException("Invalid path");
            }

            CyclicalPaths = connectedPaths.Where(e => e.IsCyclical).Select(OrderCyclicalPath);
            DirectPaths = connectedPaths.Where(e => !e.IsCyclical).SelectMany(OrderBranchingPath);

            return CyclicalPaths.Concat(DirectPaths);
        }

        /// <summary>
        /// Connects all primitives that are connected into a single path which may contain branches 
        /// </summary>
        /// <param name="primitives"></param>
        /// <returns></returns>
        public IEnumerable<Path<T>> ConstructConnectedPaths(IEnumerable<T> primitives)
        {
            int size = primitives.Count();

            List<Path<T>> paths = new(size);
            List<Prime<T>> primes = new(size);

            for (int n = 0; n < size; n++)
            {
                paths.Add(Factory.GeneratePath(primitives.ElementAt(n)));
                primes.Add(paths[n].Primes.First());
            }

            void Merge(int i, Node<T> nodeA, int j, Node<T> nodeB)
            {
                paths[i].Merge(nodeA, nodeB, paths[j].Primes);
                paths = paths.Select(e => e == paths[j] ? paths[i] : e).ToList();
            }

            // Connect all nodes
            for (int i = 0; i < size; i++)
            {
                for (int j = i + 1; j < size; j++)
                {
                    if (Connects[2 * i + 0, 2 * j + 0] && Connects[2 * i + 0, 2 * j + 1])
                        throw new Exception($"Invalid path: under the limit {Limit}, the supplied primitives form an invalid path by connecting to themselves. Make the limit smaller or remove the invalid primitive");
                    if (Connects[2 * i + 1, 2 * j + 0] && Connects[2 * i + 1, 2 * j + 1])
                        throw new Exception($"Invalid path: under the limit {Limit}, the supplied primitives form an invalid path by connecting to themselves. Make the limit smaller or remove the invalid primitive");

                    if (Connects[2 * i + 0, 2 * j + 0]) Merge(i, primes[i].StartNode, j, primes[j].StartNode);
                    if (Connects[2 * i + 1, 2 * j + 0]) Merge(i, primes[i].EndNode, j, primes[j].StartNode);
                    if (Connects[2 * i + 0, 2 * j + 1]) Merge(i, primes[i].StartNode, j, primes[j].EndNode);
                    if (Connects[2 * i + 1, 2 * j + 1]) Merge(i, primes[i].EndNode, j, primes[j].EndNode);
                }
            }
            return paths.Distinct();
        }

        /// <summary>
        /// Define the distance of each node to each other node and determine if they connect.
        /// </summary>
        /// <param name="primitives"></param>
        private void ConstructMatrices(IEnumerable<T> primitives)
        {
            // Construct cost matrix
            Cost = new double[primitives.Count() * 2, primitives.Count() * 2];
            Connects = new bool[primitives.Count() * 2, primitives.Count() * 2];

            for (int i = 0; i < primitives.Count(); i++)
            {
                T A = primitives.ElementAt(i);

                Cost[2 * i + 0, 2 * i + 0] = double.MaxValue;
                Cost[2 * i + 1, 2 * i + 0] = double.MaxValue;
                Cost[2 * i + 0, 2 * i + 1] = double.MaxValue;
                Cost[2 * i + 1, 2 * i + 1] = double.MaxValue;

                for (int j = i + 1; j < primitives.Count(); j++)
                {
                    T B = primitives.ElementAt(j);

                    Cost[2 * i + 0, 2 * j + 0] = Cost[2 * j + 0, 2 * i + 0] = (GetStartPoint(A) - GetStartPoint(B)).L2Norm();
                    Cost[2 * i + 1, 2 * j + 0] = Cost[2 * j + 0, 2 * i + 1] = (GetEndPoint(A) - GetStartPoint(B)).L2Norm();
                    Cost[2 * i + 0, 2 * j + 1] = Cost[2 * j + 1, 2 * i + 0] = (GetStartPoint(A) - GetEndPoint(B)).L2Norm();
                    Cost[2 * i + 1, 2 * j + 1] = Cost[2 * j + 1, 2 * j + 1] = (GetEndPoint(A) - GetEndPoint(B)).L2Norm();

                    Connects[2 * i + 0, 2 * j + 0] = Connects[2 * j + 0, 2 * i + 0] = Cost[2 * i + 0, 2 * j + 0] < Limit;
                    Connects[2 * i + 1, 2 * j + 0] = Connects[2 * j + 0, 2 * i + 1] = Cost[2 * i + 1, 2 * j + 0] < Limit;
                    Connects[2 * i + 0, 2 * j + 1] = Connects[2 * j + 1, 2 * i + 0] = Cost[2 * i + 0, 2 * j + 1] < Limit;
                    Connects[2 * i + 1, 2 * j + 1] = Connects[2 * j + 1, 2 * i + 1] = Cost[2 * i + 1, 2 * j + 1] < Limit;
                }
            }
        }

        /// <summary>
        /// Get the physical locations of the junctions. Note - Junctions are defined by a limit distance, so point may vary by limit.
        /// </summary>
        /// <param name="path"></param>
        /// <returns></returns>
        public IEnumerable<Vector<double>> GetJunctionLocations(Path<T> path)
        {
            return path.Junctions.Select(GetJunctionLocation);
        }

        private Vector<double> GetJunctionLocation(Node<T> node)
        {
            Prime<T> prime = node.Connections.First();
            if (node == prime.StartNode) return prime.StartPoint;
            else return prime.EndPoint;
        }

        #region Path Management
        /// <summary>
        /// Take a cyclical path and order it.
        /// </summary>
        /// <typeparam name="T"></typeparam>
        /// <param name="paths"></param>
        /// <returns></returns>
        /// <exception cref="PathSegmenterException"></exception>
        private static Path<T> OrderCyclicalPath(Path<T> path)
        {
            Node<T> startNode = path.Nodes.First();

            // Travelled the same length as path & included all nodes within path
            bool successPredicate(Memory<T> memory) => memory.Count() == path.Primes.Count && memory.Head == startNode;

            // Travelled longer than the path length
            bool failurePredicate(Memory<T> memory) => memory.Count() > path.Primes.Count;

            PathTraveller<T> traveller = new(startNode, successPredicate, failurePredicate);

            bool success = traveller.TravelToTarget();

            if (!success) throw new PathSegmenterException("Path cannot be travelled");

            return traveller.GetPath();
        }

        /// <summary>
        /// Take a direct path and order it. Path must have ONLY 2 junctions.
        /// </summary>
        /// <param name="paths"></param>
        /// <returns></returns>
        /// <exception cref="PathSegmenterException"></exception>
        private static Path<T> OrderDirectPath(Path<T> path)
        {
            if (!path.IsDirect) throw new PathSegmenterException("Path is not direct");

            // Have reached the last node & included all nodes within path
            bool successPredicate(Memory<T> memory) =>
                memory.Head == path.Junctions.Last() &&
                memory.Count() == path.Primes.Count;

            //Travelled longer than the path length
            bool failurePredicate(Memory<T> memory) => memory.Count() > path.Primes.Count;

            PathTraveller<T> traveller = new(path.Junctions.First(), successPredicate, failurePredicate);

            bool success = traveller.TravelToTarget();

            if (!success) throw new PathSegmenterException("Path cannot be travelled");

            return traveller.GetPath();
        }

        /// <summary>
        /// 
        /// </summary>
        /// <param name="path"></param>
        /// <returns></returns>
        /// <exception cref="PathSegmenterException"></exception>
        private static List<Path<T>> OrderBranchingPath(Path<T> path)
        {
            int num = path.Junctions.Count() / 2;

            List<Path<T>> directs = new(num);

            for (int i = 0; i < num - 1; i++)
            {
                bool successPredicate(Memory<T> memory) => memory.Head.IsJunction;
                bool failurePredicate(Memory<T> memory) => memory.Count() > path.Primes.Count;

                PathTraveller<T> traveller = new(path.EndPoints.First(), successPredicate, failurePredicate);

                bool success = traveller.TravelToTarget();

                if (!success) throw new PathSegmenterException("Path with odd number of junctions exists");

                Path<T> travelledPath = traveller.GetPath();
                path.Subtract(travelledPath);
                directs.Add(travelledPath);
            }

            if (path.IsCyclical) directs.Add(OrderCyclicalPath(path));
            else directs.Add(OrderDirectPath(path));

            return directs;
        }

        #endregion

        /// <summary>
        /// Converts a path into a segment.
        /// 
        /// Paths must be non-branching and ordered.
        /// </summary>
        /// <param name="path"></param>
        /// <returns></returns>
        public static IEnumerable<Primitive> PathToPrimitives(Path<Primitive> path)
        {
            foreach (Prime<Primitive> prime in path.Primes)
            {
                if (prime.IsReversed)
                {
                    yield return prime.Value.Reverse();
                }
                else
                {
                    yield return prime.Value;
                }
            }
        }

        public IEnumerable<Primitive> PathsToPrimitives(IEnumerable<Path<Primitive>> paths)
        {
            Vector<double> position = paths.First().StartPoint;
            foreach(Path<Primitive> path in paths)
            {
                if ((path.StartPoint - position).L2Norm() > Limit)
                {
                    yield return new Jump(position, path.StartPoint);
                }
                foreach(Primitive p in PathToPrimitives(path))
                {
                    yield return p;
                }
                position = path.EndPoint;
            }
        }

    }

    public class PathSegmenterException : Exception
    {
        public PathSegmenterException() { }

        public PathSegmenterException(string message) : base(message) { }
    }
}
