using MathNet.Numerics.LinearAlgebra;

namespace BufferGen.Optimisers.Pathing
{
    internal class PathingFactory<T>(Func<T, Vector<double>> getStartPoint, Func<T, Vector<double>> getEndPoint)
    {
        Func<T, Vector<double>> GetStartPoint { get; } = getStartPoint;
        Func<T, Vector<double>> GetEndPoint { get; } = getEndPoint;

        public Prime<T> GeneratePrime(T value) => new(value) { GetStartPoint = GetStartPoint, GetEndPoint = GetEndPoint };
        public Path<T> GeneratePath(T value) => new([new Prime<T>(value) { GetStartPoint = GetStartPoint, GetEndPoint = GetEndPoint }]);
    }
}
