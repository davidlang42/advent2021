use std::env;
use std::fs;
use point::Point;
use std::collections::HashSet;
use crate::frame::Orientation;
use crate::scanner::Scanner;
use crate::frame::FrameOfReference;

mod scanner;
mod point;
mod frame;

const MINIMUM_OVERLAP: usize = 12;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut scanners = text.split("\r\n\r\n").map(|s| s.parse().unwrap());
        let mut reference_scanner: Scanner = scanners.next().unwrap();
        reference_scanner.frame = Some(FrameOfReference::BASE);
        let mut found: Vec<Scanner> = vec![reference_scanner];
        let mut remaining: Vec<Scanner> = scanners.collect();
        while remaining.len() > 0 {
            let mut matched = None;
            for source in &found {
                for i in 0..remaining.len() {
                    if let Some(frame) = remaining[i].find_frame(source, MINIMUM_OVERLAP) {
                        let mut newly_found = remaining.remove(i);
                        newly_found.frame = Some(frame);
                        println!("Matched {} with {}", newly_found, source);
                        matched = Some(newly_found);
                        break;
                    }
                }
                if matched.is_some() {
                    break;
                }
            }
            if let Some(newly_found) = matched {
                found.push(newly_found);
            } else {
                panic!("No matches found with {} remaining", remaining.len());
            }
        }
        let absolute_beacons: HashSet<Point> = found.iter().flat_map(|s| s.absolute_beacons()).collect();
        println!("Total beacons: {}", absolute_beacons.len());
        let mut max_distance = 0;
        for a in 0..found.len() {
            for b in 0..found.len() {
                if a != b {
                    let distance = found[a].frame.as_ref().unwrap().position.manhatten_distance(&found[b].frame.as_ref().unwrap().position);
                    if distance > max_distance {
                        max_distance = distance;
                    }
                }
            }
        }
        println!("Max distance: {}", max_distance);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}