use std::env;
use std::fs;
use point::Point;
use crate::frame::Orientation;
use crate::scanner::Scanner;
use crate::frame::FrameOfReference;

mod scanner;
mod point;
mod frame;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut scanners = text.split("\r\n\r\n").map(|s| s.parse().unwrap());
        //TODO
        let s: Scanner = scanners.next().unwrap();
        for o in Orientation::all() {
            let frame = FrameOfReference {
                position: Point { x: 0, y: 0, z: 0 },
                orientation: o
            };
            println!("{}", frame.orientation);
            for p in &s.beacons {
                println!("{}", p.absolute(&frame));
            }
            println!("");
        }

        //TODO
        // let minimum_overlap = 12;
        // let mut reference_scanner: Scanner = scanners.next().unwrap();
        // reference_scanner.position = Some(Point { x: 0, y: 0, z: 0 });
        // reference_scanner.orientation = Some(Orientation::BASE);
        // let mut found: Vec<Scanner> = vec![reference_scanner];
        // let mut remaining: Vec<Scanner> = scanners.collect();
        // while remaining.len() > 0 {
        //     let mut matched = None;
        //     for source in &found {
        //         for i in 0..remaining.len() {
        //             if let Some((position, orientation)) = find_matching_scanners(source, &remaining[i], minimum_overlap) {
        //                 let mut newly_found = remaining.remove(i);
        //                 newly_found.position = Some(position);
        //                 newly_found.orientation = Some(orientation);
        //                 println!("Matched {} with {}", newly_found, source);
        //                 matched = Some(newly_found);
        //                 break;
        //             }
        //         }
        //         if matched.is_some() {
        //             break;
        //         }
        //     }
        //     if let Some(newly_found) = matched {
        //         found.push(newly_found);
        //     } else {
        //         panic!("No matches found with {} remaining", remaining.len());
        //     }
        // }
        // let absolute_beacons: HashSet<Point> = found.iter().flat_map(|s| s.absolute_beacons()).collect();
        // println!("Total beacons: {}", absolute_beacons.len());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}