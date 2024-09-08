using MathNet.Numerics.LinearAlgebra;

namespace BufferGen.Optimisers.Pathing
{
    /// <summary>
    /// A <see cref="Prime{T}"/> is a single, non-divisible unit of a path. A <see cref="Prime{T}"/> therefore has two entrance/exit points - given by its Nodes.
    /// </summary>
    internal class Prime<T>
    {
        public T Value { get; private set; }

        public Node<T> StartNode { get; private set; }
        public Node<T> EndNode { get; private set; }

        public bool HasConnections => StartNode.HasConnections || EndNode.HasConnections;
        public bool IsReversed { get; private set; }

        internal required Func<T, Vector<double>> GetStartPoint { get; init; }
        internal required Func<T, Vector<double>> GetEndPoint { get; init; }

        public Vector<double> StartPoint => IsReversed ? GetEndPoint(Value) : GetStartPoint(Value);
        public Vector<double> EndPoint => IsReversed ? GetStartPoint(Value) : GetEndPoint(Value);

        public Prime(T value)
        {
            IsReversed = false;
            Value = value;
            StartNode = new([this]);
            EndNode = new([this]);
        }

        public void ReplaceNode(Node<T> node, Node<T> replacement)
        {
            if (EndNode == node)
            {
                EndNode = replacement;
            }
            if (StartNode == node)
            {
                StartNode = replacement;
            }
        }

        public void ReplaceStartNode(Node<T> node)
        {
            StartNode = node;
        }

        public void ReplaceEndNode(Node<T> node)
        {
            EndNode = node;
        }

        /// <summary>
        /// Return the node connected to the other side of this <see cref="Prime{T}"/>.
        /// </summary>
        /// <param name="node"></param>
        /// <returns></returns>
        public Node<T> TraverseFrom(Node<T> node)
        {
            if (EndNode == node)
            {
                return StartNode;
            }
            else
            {
                return EndNode;
            }
        }

        public Prime<T> Reverse()
        {
            //Node<T> newStartNode = EndNode;
            //Node<T> newEndNode = StartNode;

            //EndNode = newEndNode;
            //StartNode = newStartNode;

            IsReversed = !IsReversed;
            return this;
        }
    }
}
