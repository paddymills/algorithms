
use geo::{Point, EuclideanDistance};
use std::ops::{Div, DerefMut, Deref};

// TODO: use generics (Cluster for KMeans and Point for Cluster)

#[derive(Debug)]
pub struct KMeans {
    k: usize,
    clusters: Vec<Cluster>
}

impl KMeans {
    pub fn new(k: usize) -> Self {
        Self { k, clusters: Vec::new() }
    }
    
    pub fn create_centroids(&mut self, points: Vec<Point>) {
        if points.len() < self.k {
            self.k = points.len();
        }

        // TODO: methods for selecting centroids (first-k, random, evenly-spaced, etc.)
        for i in 0..self.k {
            self.clusters.push(Cluster::new(*points.get(i).unwrap()))
        }
    }

    pub fn add_points(&mut self, points: Vec<Point>) {
        for point in points {
            let mut closest_cluster = 0;
            let mut closest_distance = f64::MAX;

            for (i, cluster) in self.clusters.iter().enumerate() {
                let distance = cluster.centroid.euclidean_distance(&point);

                if distance < closest_distance {
                    closest_cluster = i;
                    closest_distance = distance;
                }
            }

            // TODO: borrow cluster above and just use a reference
            self.clusters.get_mut(closest_cluster).unwrap().push(point);
        }
    }
}

#[derive(Debug)]
pub struct Cluster {
    centroid: Point,
    entities: Vec<Point>
}

impl Cluster {
    pub fn new(point: Point) -> Self {
        Self { centroid: point, entities: Vec::new() }
    }

    pub fn calculate_centroid(mut self) {
        let n = self.entities.len() as f64;

        self.centroid = self.entities.into_iter()
            .reduce(|acc, p| acc + p)
            .unwrap()
            .div( n )
    }
}

impl Deref for Cluster {
    type Target = Vec<Point>;

    fn deref(&self) -> &Self::Target {
        &self.entities
    }
}

impl DerefMut for Cluster {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entities
    }
}
