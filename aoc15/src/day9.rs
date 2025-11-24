use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;
use nom::{
    IResult,
    Parser,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, usize as nom_usize},
    combinator::{map, map_res}, // Import map and map_res
    multi::many1,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rustc_hash::FxHashMap;

const INPUT: &str = include_str!("../inputs/day9.txt");

struct Graph {
    distances: Vec<Vec<usize>>, // Adjacency matrix
    num_cities: usize,
}

impl Graph {
    fn new<'a>(routes_data: &[(&'a str, &'a str, usize)]) -> Self {
        let mut city_to_id = FxHashMap::default();
        let mut id_to_city = Vec::new();
        let mut next_id = 0;

        let mut get_id = |city: &'a str| {
            *city_to_id.entry(city).or_insert_with(|| {
                id_to_city.push(city);
                let id = next_id;
                next_id += 1;
                id
            })
        };

        // First pass: collect all unique city names and assign IDs
        for &(from, to, _) in routes_data {
            get_id(from);
            get_id(to);
        }

        let num_cities = next_id;
        let mut distances = vec![vec![usize::MAX; num_cities]; num_cities]; // Initialize with MAX for no path

        // Distance to self is 0 (though not needed for TSP, as we visit each city once)
        (0..num_cities).for_each(|i| {
            distances[i][i] = 0;
        });

        // Second pass: fill adjacency matrix with distances
        for &(from, to, dist) in routes_data {
            let from_id = city_to_id[from];
            let to_id = city_to_id[to];
            distances[from_id][to_id] = dist;
            distances[to_id][from_id] = dist; // Graph is undirected
        }

        Graph {
            distances,
            num_cities,
        }
    }

    // Recursive helper for finding the shortest path that visits all cities
    fn find_min_path_recursive(
        &self,
        current_city_id: usize,
        visited_mask: usize, // Bitmask to track visited cities
        current_distance: usize,
    ) -> usize {
        // Base case: If all cities have been visited (mask has all bits set)
        if visited_mask == (1 << self.num_cities) - 1 {
            return current_distance;
        }

        let mut min_total_distance = usize::MAX;

        for next_city_id in 0..self.num_cities {
            // Check if next_city is not visited and there's a path
            if (visited_mask >> next_city_id) & 1 == 0
                && self.distances[current_city_id][next_city_id] != usize::MAX
            {
                let distance_to_next = self.distances[current_city_id][next_city_id];
                let total_path = self.find_min_path_recursive(
                    next_city_id,
                    visited_mask | (1 << next_city_id), // Mark next_city as visited
                    current_distance + distance_to_next,
                );
                min_total_distance = min_total_distance.min(total_path);
            }
        }
        min_total_distance
    }

    // Recursive helper for finding the longest path that visits all cities
    fn find_max_path_recursive(
        &self,
        current_city_id: usize,
        visited_mask: usize, // Bitmask to track visited cities
        current_distance: usize,
    ) -> usize {
        // Base case: If all cities have been visited (mask has all bits set)
        if visited_mask == (1 << self.num_cities) - 1 {
            return current_distance;
        }

        let mut max_total_distance = 0;

        let mut found_next_step = false;
        for next_city_id in 0..self.num_cities {
            // Check if next_city is not visited and there's a path
            if (visited_mask >> next_city_id) & 1 == 0
                && self.distances[current_city_id][next_city_id] != usize::MAX
            {
                found_next_step = true;
                let distance_to_next = self.distances[current_city_id][next_city_id];
                let total_path = self.find_max_path_recursive(
                    next_city_id,
                    visited_mask | (1 << next_city_id), // Mark next_city as visited
                    current_distance + distance_to_next,
                );
                max_total_distance = max_total_distance.max(total_path);
            }
        }

        if !found_next_step && visited_mask != (1 << self.num_cities) - 1 {
            return 0; // This branch didn't lead to a full path.
        }

        max_total_distance
    }
}

// Parses a single line of input into (from_city, to_city, distance)
fn parse_route_line(i: &str) -> IResult<&str, (&str, &str, usize)> {
    map(
        (alpha1, tag(" to "), alpha1, tag(" = "), nom_usize),
        |(from, _, to, _, distance)| (from, to, distance),
    )
    .parse(i)
}

// Parses the entire input string into a Vec of routes
fn parse_all_routes(i: &str) -> Vec<(&str, &str, usize)> {
    many1(map_res((parse_route_line, line_ending), |(route, _)| {
        Ok::<_, nom::error::Error<&str>>(route)
    }))
    .parse(i)
    .unwrap()
    .1
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let routes_data = parse_all_routes(INPUT); // Parse once
    let graph = Graph::new(&routes_data);

    let min_total_distance = (0..graph.num_cities) // Iterate over all possible starting cities
        .into_par_iter()
        .map(|start_city_id| {
            // The initial call has 0 current_distance and only the start_city_id visited
            graph.find_min_path_recursive(start_city_id, 1 << start_city_id, 0)
        })
        .min()
        .unwrap(); // Unwrap because there's always at least one path

    format_result!(min_total_distance)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let routes_data = parse_all_routes(INPUT); // Parse once
    let graph = Graph::new(&routes_data);

    let max_total_distance = (0..graph.num_cities) // Iterate over all possible starting cities
        .into_par_iter()
        .map(|start_city_id| {
            // The initial call has 0 current_distance and only the start_city_id visited
            graph.find_max_path_recursive(start_city_id, 1 << start_city_id, 0)
        })
        .max()
        .unwrap(); // Unwrap because there's always at least one path

    format_result!(max_total_distance)
}
