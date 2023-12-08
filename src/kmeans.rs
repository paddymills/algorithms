
use super::Point;

#[derive(Debug)]
pub struct KMeans<T, N> {
    k: usize,
    clusters: Vec<Cluster<T, N>>
}


#[derive(Debug)]
pub struct Cluster<T, N> {
    centroid: Point<N>,
    entities: Vec<T>
}
