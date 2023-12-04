use std::fs;
fn main() {
    const RED:i32 = 12;const BLUE:i32=14;const GREEN:i32=13;
    let file = fs::read_to_string("ex1Data").expect("UwU");
    let lines = file.split("\n").map(|x| x.split(":").nth(1).expect("No line").split(";"));
    let mut possibles = 0;
    let mut gameNo = 0;
    for line in lines {
        let mut possible = true;
        gameNo +=1;
        for round in line {
            let lineParts = round.split(",");
            for part in lineParts {
                let mut part_parts = part.split(" ");
                println!("{part}");
                let n1 = part_parts.nth(1).unwrap();
                println!("{n1}");
                let num = n1.parse::<i32>().unwrap();
                let typ = part_parts.nth(0).unwrap();
                if typ == "red" {
                    if num > RED {
                        possible = false;
                        break;
                    }
                } else if typ == "green" {
                    if num > GREEN {
                        possible = false;
                        break;
                    }
                } else if typ == "blue" {
                    if num > BLUE {
                        possible = false;
                        break;
                    }
                }
            }
            if !possible {
                break;
            }
        }
        if possible {
            possibles += gameNo;
        }
        println!("{possibles}");
    }


}
