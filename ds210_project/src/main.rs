// Justin Zheng, U67912393
// Collaborators: None

use std::fs::read_to_string;
use std::collections::VecDeque;

//function for reading the data into vectors 
fn open_file(file: &str) -> (Vec<Vec<String>>, Vec<String>) {
    let data = read_to_string(file); //reading data as string

    //checking data is okay
    let information = match data {
        Ok(content) => content,
        Err(error) => {panic!("Could not open or find file: {}", error);}
    };

    //splits data into points by their id number
    let mut split_info: Vec<&str> = information.trim().split("Id:").collect();

    //removes blank first node 
    split_info.remove(0);

    //variable for storing id number, ASIN, title and group
    let mut items: Vec<Vec<String>> = vec![vec!["".into()];split_info.len()]; 

    //variable fore storing all the edges of each item
    let mut edges: Vec<String> = vec!["".into();split_info.len()];

    //iterate through each item 
    for i in 0..split_info.len() {

        //splits the data further by new lines and stores each component in a vector
        let split_item: Vec<&str> = split_info[i].trim().split("\n").collect(); 

        //checks if item is discontinued, discontinued items will have a length of 3(id, ASIN,  "discontinued product")
        if split_item.len() == 3 {
            continue;
        }
        
        let id = split_item[0].to_owned(); //gets the ID number of the item
        let asin = split_item[1].to_owned(); //gets the ASIN of the item
        let title = split_item[2].to_owned(); //gets the title of the item
        let group = split_item[3].to_owned(); //gets the group of the item
        let edge = split_item[5].trim().to_owned(); //gets the list of similar edges of the item

        //stores the id number, ASIN, title, and group as one vector
        let info = vec![id, asin.into(), title.into(), group.into()];

        //inserts all vectors containing the information into another vector 
        items[i] = info;

        //inserts all edges into another vector
        edges[i] = edge;
    };
    //returns two vectors: items, which contains the items id number, ASIN, group, and title, 
    //and edges, which contains list of similar edges
    return (items, edges);
}

//function that further cleans the data for use
fn clean_data(data: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    //stores the cleaned data
    let mut cleaned: Vec<Vec<String>> = vec![vec!["".into();4];data.len()];

    //iterate through all the items 
    for i in 0..data.len() {
        //if data has length of 1 then it is a discontinued item
        if data[i].len() == 1 {
            continue;
        }

        let cut = data[i][0].len() - 1; //length of id number - 1
        cleaned[i][0] = data[i][0][..cut].into(); //removes last character and stores the id number
        cleaned[i][1] = data[i][1][6..(data[i][1].len() - 1)].into(); //slices the string, removes last character, and stores the ASIN
        cleaned[i][2] = data[i][2][9.. (data[i][2].len() - 1)].into(); //slices the string, removes last character, and stores the title
        cleaned[i][3] = data[i][3][9..(data[i][3].len() - 1)].into(); //slices the string, removes last character, and stores the group
    }
    return cleaned;
}

//function that assigns the edge number to the ASIN (see why in report)
fn assign_edges(edges: &String, file: &Vec<Vec<String>>) -> Vec<i32> {
    //stores the reassigned edge
    let mut edge: Vec<i32> = vec![];

    //checks that edges exists
    if let x = Some(edges) {
        //checks if it is a empty string
        if x.unwrap() == "" {
            return vec![0];
        }

        //slices the string of edges, splits by "  ", and then collect
        let edge_split: Vec<&str> = x.unwrap()[9..].split("  ").collect();

        //declaring size of new edge
        edge = vec![0;edge_split.len()];

        //iterates through each edge in the list of edges
        for i in 0..edge_split.len() {

            let mut id = 0; //stores number id

            //checks it is an ASIN (ASIN always have a length of 10)
            if edge_split[i].len() == 10 {
                //iterates through all the items
                for j in 0..file.len() {
                    //checks if item is discontinued
                    if file[j].len() > 1 {
                        let asin = &file[j][1]; //stores the ASIN value at j

                        //compares ASIN value at j to current edge, if its equal then assign id to j
                        if edge_split[i] == asin {
                            id = j;
                        }
                    }
                }
            }
            else {
                //this is the case where the edge is the id number and not the ASIN
                //takes string of integer and unwraps, then assign it to id
                id = edge_split[i].parse().unwrap();
            }
            //store id in edge at i
            edge[i] = id as i32;
        }
    }
    return edge;
}

//module containing functions for calculating distance using bfs, average distance, and the most and least suggested items
pub mod calculations {
    use std::collections::VecDeque;

    //fucntion that calculates the distance from one point to all other points using breadth-first search
    pub fn compute_distance_bfs(start: usize, graph: &Vec<Vec<i32>>) -> Vec<Option<u32>> {

        let mut distance: Vec<Option<u32>> = vec![None;graph.len()]; //stores the distances of each point
        distance[start] = Some(0); //initialize distance for start point

        let mut queue: VecDeque<usize> = VecDeque::new();
        //insert start into queue
        queue.push_back(start); 

        //while there remains unprocessed vertices
        while let Some(v) = queue.pop_front() {
            for u in graph[v].iter() {

                //consider all unprocessed neighbors of current point
                if let None = distance[*u as usize] { 
                    distance[*u as usize] = Some(distance[v].unwrap() + 1);
                    queue.push_back(*u as usize);
                }
            }
        } 
        return distance;
    }

    //function that computes the average distance of one point to all other points
    pub fn compute_average_distance(distances: &Vec<Option<u32>>) -> f32 {
        let mut avg_dist = 0.0; //stores the average distance
        let mut sum = 0.0; //stores sum of distances

        //iterate through all distances in the vector
        for dist in distances {
            //checks distance exists
            if let Some(x) = dist {
                sum += *x as f32; //add up all distances
                //avg_dist = sum / distances.len() as f32;
            }
        }
        avg_dist = sum / distances.len() as f32;
        return avg_dist as f32;
    }

    //function that computes the item with the most suggested items
    pub fn most_suggested(edges: &Vec<Vec<i32>>) -> Vec<i32> {
        let mut id = 0; //stores id of item
        let mut max = edges[0].len(); //stores the length for most suggested items

        //iterate through all edges
        for i in 1..edges.len() {
            //check if length of next edge is greater than current max
            if max < edges[i].len() {
                id = i; //assigns i if new max is found
                max = edges[i].len(); //assigns new length if new max is found
            }
        }
        return vec![id as i32, max as i32];
    }

    //function that computes the item with the least suggested items
    pub fn least_suggested(edges: &Vec<Vec<i32>>) -> Vec<i32> {
        let mut id = 0; //stores id of item
        let mut min = edges[0].len(); //stores the length for least suggested items

        //iterate through all edges
        for i in 1..edges.len() {
            //check if length of next edge is less than current least
            if min > edges[i].len() {
                id = i; //assigns i if new least is found
                min = edges[i].len() //assigns new length if new least is found
            }
        }
        return vec![id as i32, min as i32];
    }
}

//test for computing the distances at a point using breadth-first search
#[test]
fn test_compute_distance_bfs() {
    let graph = vec![vec![1,2], vec![0,2,9], vec![0,1,3,4], vec![2,4], vec![2,3,5,6], vec![4,6], vec![4,5,7,8], vec![6,8], vec![6,7], vec![1]];
    let mut distances = vec![vec![];graph.len()];
    for i in 0..graph.len() {
        let distance = calculations::compute_distance_bfs(i, &graph);
        distances[i] = distance;
    }
    let answer_at_zero = vec![Some(0), Some(1), Some(1), Some(2), Some(2), Some(3), Some(3), Some(4), Some(4), Some(2)];
    assert_eq!(distances[0], answer_at_zero);
    let answer_at_one = vec![Some(1), Some(0), Some(1), Some(2), Some(2), Some(3), Some(3), Some(4), Some(4), Some(1)];
    assert_eq!(distances[1], answer_at_one);
    let asnswer_at_two = vec![Some(1), Some(1), Some(0), Some(1), Some(1), Some(2), Some(2), Some(3), Some(3), Some(2)];
    assert_eq!(distances[2], asnswer_at_two);
}

//test for computing the average distance of each point
#[test]
fn test_average_distance() {
    let graph = vec![vec![1,2], vec![0,2,9], vec![0,1,3,4], vec![2,4], vec![2,3,5,6], vec![4,6], vec![4,5,7,8], vec![6,8], vec![6,7], vec![1]];
    let answer = vec![2.2, 2.1, 1.6, 1.9, 1.5, 2.0, 1.8, 2.5, 2.5, 2.9];
    let mut all_distances = vec![vec![];graph.len()];
    for i in 0..graph.len() {
        all_distances[i] = calculations::compute_distance_bfs(i, &graph);
    }

    let mut avg_distances = vec![0.0;graph.len()];
    for i in 0..graph.len() {
        let dist = calculations::compute_average_distance(&all_distances[i]);
        avg_distances[i] = dist;
    }
    assert_eq!(avg_distances, answer);
}

//test for finding the item with the most suggested items
#[test]
fn test_most_suggested() {
    let graph = vec![vec![1,2], vec![0,2,9], vec![0,1,3,4], vec![2,4], vec![2,3,5,6], vec![4,6], vec![4,5,7,8], vec![6,8], vec![6,7], vec![1]];
    let answer = vec![2,4];
    let most = calculations::most_suggested(&graph);
    assert_eq!(most, answer);

}

//test for finding the item with the least suggested items
#[test]
fn test_least_suggested() {
    let graph = vec![vec![1,2], vec![0,2,9], vec![0,1,3,4], vec![2,4], vec![2,3,5,6], vec![4,6], vec![4,5,7,8], vec![6,8], vec![6,7], vec![1]];
    let answer = vec![9,1];
    let least = calculations::least_suggested(&graph);
    assert_eq!(least, answer);
}

fn main() {
    //calls open_file() and stores the results in file and edges
    let (mut file, mut edges) = open_file("amazon-meta.txt"); 
    //cleans the file using clean_data()
    let mut clean_file = clean_data(&file);
    
    let mut graph: Vec<Vec<i32>> = vec![vec![];edges.len()]; //stores the graph of edges
    let mut all_distances = vec![vec![];edges.len()]; //stores the distances at each point
    let mut avg_distances = vec![0.0;graph.len()]; //stores the average distances at each point

    let n = 100000;
    //iterates through all the edges and calls assign_edges() 
    for i in 0..150000 {
        graph[i] = assign_edges(&edges[i], &clean_file);
    }
    //iterates through and calculates the distance using compute_distance_bfs()
    for i in 0..9000 {
        all_distances[i] = calculations::compute_distance_bfs(i, &graph);
    }

    //iterates through and calculates the average distance
    let mut max_id = 0;
    let mut max_avg = 0.0;
    let mut min_id = 0;
    let mut min_avg = 0.0;
    for i in 0..5000 {
        avg_distances[i] = calculations::compute_average_distance(&all_distances[i]);
        if avg_distances[i] > max_avg {
            max_id = i;
            max_avg = avg_distances[i];
        }
        if avg_distances[i] < min_avg {
            min_id = i;
            min_avg = avg_distances[i];
        }
    }
    println!("max id: {:?} max average distance: {:?} min id: {:?} min average distance: {:?}", max_id, max_avg, min_id, min_avg);

    let max = calculations::most_suggested(&graph);
    let min = calculations::least_suggested(&graph);
    println!("most suggested: {:?} least suggested: {:?}", max, min);
}
