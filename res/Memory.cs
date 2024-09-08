using System.Collections;

namespace BufferGen.Optimisers.Pathing
{
    /// <summary>
    /// Holds the memory of the travelled route. Primes are guaranteeed to be unique.
    /// </summary>
    /// <typeparam name="T"></typeparam>
    internal class Memory<T> : IEnumerable<MemoryItem<T>>
    {
        public IEnumerable<Prime<T>> TravelledPrimes => Data.Select(e => e.Route).ToList();

        private List<MemoryItem<T>> Data { get; } = [];

        public Node<T> Head => Data.Last().Node.TraverseConnection(Data.Last().Route);

        public Memory(Node<T> startingNode)
        {
            Data.Add(MemoryItem<T>.Create(startingNode, TravelledPrimes));
        }

        public IEnumerator<MemoryItem<T>> GetEnumerator() => Data.GetEnumerator();

        IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();

        public MemoryItem<T> this[int index] => Data[index];

        /// <summary>
        /// Revert back to a junction with a decision
        /// </summary>
        /// <returns></returns>
        public bool Revert()
        {
            while (true)
            {
                // Remove the last location
                Data.RemoveAt(Data.Count - 1);

                // Fail if no more options to takes
                if (!Data.Any()) return false;

                // Iterate on the previous location route choice if possible, otherwise
                // revert again
                if (Data.Last().Options.Any())
                {
                    Data.Last().Iterate();
                    break;
                }
            }
            return true;
        }

        /// <summary>
        /// Travel to the next node and record the route taken
        /// </summary>
        public bool Onwards()
        {
            Node<T> nextNode = Data.Last().Node.TraverseConnection(Data.Last().Route);

            List<Prime<T>> options = [];
            foreach (Prime<T> option in nextNode.Connections)
            {
                // Can't go through excluded primes
                if (!TravelledPrimes.Contains(option)) options.Add(option);
            }

            if (!options.Any()) return false;

            Prime<T> optionTaken = options.First();
            options = options.Skip(1).ToList();

            Data.Add(new(nextNode, optionTaken, options));

            return true;
        }
    }

    internal class MemoryItem<T>(Node<T> node, Prime<T> route, IEnumerable<Prime<T>> options)
    {
        public Node<T> Node { get; private set; } = node;
        public Prime<T> Route { get; private set; } = route;
        public IEnumerable<Prime<T>> Options { get; private set; } = options;

        public static MemoryItem<T> Create(Node<T> node, IEnumerable<Prime<T>> excludePrimes)
        {
            List<Prime<T>> options = [];

            foreach (Prime<T> option in node.Connections)
            {
                // Can't go through excluded primes
                if (!excludePrimes.Contains(option)) options.Add(option);
            }

            Prime<T> optionTaken = options.First();
            options = options.Skip(1).ToList();

            return new(node, optionTaken, options);
        }

        public void Iterate()
        {
            Route = Options.First();
            Options = Options.TakeLast(Options.Count() - 1);
        }

    }
}
