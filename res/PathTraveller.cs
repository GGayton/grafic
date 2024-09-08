namespace BufferGen.Optimisers.Pathing
{
    /// <summary>
    /// Used to track paths from A -> B.
    /// 
    /// </summary>
    /// <typeparam name="T"></typeparam>
    /// <remarks>
    /// 
    /// </remarks>
    /// <param name="startNode"></param>
    /// <param name="successPredicate"></param>
    /// <param name="failurePredicate"></param>
    internal class PathTraveller<T>(Node<T> startNode, Func<Memory<T>, bool> successPredicate, Func<Memory<T>, bool> failurePredicate)
    {
        private Func<Memory<T>, bool> SuccessPredicate { get; } = successPredicate;
        private Func<Memory<T>, bool> FailurePredicate { get; } = failurePredicate;
        private Memory<T> Memory { get; } = new(startNode);

        public bool TravelToTarget()
        {
            while (true)
            {
                bool forward = Memory.Onwards();

                // Finished check
                if (SuccessPredicate(Memory)) break;

                // Backtrack if at an end point or failed to travel forwards
                if (!forward || Memory.Head.IsEndPoint || FailurePredicate(Memory))
                {
                    bool backward = Memory.Revert();
                    if (!backward) return false;
                }
            }
            return true;
        }

        /// <summary>
        /// Return the path travelled in order
        /// </summary>
        /// <returns></returns>
        public Path<T> GetPath()
        {
            List<Prime<T>> primes = new(Memory.Count());
            foreach (MemoryItem<T> item in Memory)
            {
                Prime<T> prime = item.Route;
                if (prime.EndNode == item.Node) prime.Reverse();
                primes.Add(prime);
            }
            return new(primes);
        }
    }
}
