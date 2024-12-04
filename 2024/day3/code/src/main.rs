use std::fs;
use regex::Regex;

fn main() {
    let file_path = "./input";

    println!("In file {file_path}");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let parseDisable = Regex::new(r"(?s)do\(\).*?don't\(\)").unwrap();
    //let parseDisable = Regex::new(r"(^.*?don't\(\))|(do\(\)(.*?)don't\(\))\s").unwrap();
    //let parseDisable = Regex::new(r"(^.*?(mul\(\d+\,\d+\).*?)+don't\(\))|(do\(\).*?(?:don't\(\)))|(do\(\).*?$)").unwrap();
    //let parseDisable = Regex::new(r"don\'t\(\)(.*?)do\(\)").unwrap();
    //let parseDisable = Regex::new(r"don\'t\(\).*?(don\'t\(\))*.*?do\(\)").unwrap();
    let parseMul = Regex::new(r"mul\(\d+\,\d+\)").unwrap();
    let all: Vec<_> = parseMul.find_iter(&contents).map(|m| m.as_str()).collect();
    let mut toDisableStr: Vec<_> = parseDisable.find_iter(&contents).map(|m| m.as_str()).collect();
    toDisableStr.push("do()when()mul(865,453)%{{[?+] mul(813,642)/$*who()mul(817,115)mul(430,489)why()-from()where();%'select():from()mul(291,912)what()$@~*+(,)&mul(86,762)mul(695,56)who()%]@;&;:why()-mul(893,928)who()#how()mul(50,956)select()[/who()<");
    for s in toDisableStr.iter() {
        println!("{s}");
        println!("-------------------------");
    }
    let mut toSub: Vec<_> = vec![];
    for t in toDisableStr.iter() {
        for p in parseMul.find_iter(t).map(|m| m.as_str()) {
            toSub.push(p);
        }
    }
    //let toSub: Vec<_> = toDisableStr.iter()
    //                                .map(|x| parseMul.find_iter(x).map(|m| m.as_str()).collect::<Vec<_>>())
    //                                .flat_map(|a| a)
    //                                .collect();
    //println!("sub are {:?}", toSub);

    let mut acc = 0;
    let parseNumber = Regex::new(r"\d+").unwrap();
    for capture in all.iter() {
        let local = capture.split(",")
                         .map(|x| parseNumber.find(x).unwrap().as_str().parse::<i32>().unwrap())
                         .fold(1, |a, b| a*b);
        acc += local;
    }
    let mut sub = 0;
    println!("total before is {acc}");
    for capture in toSub.iter() {
        let local = capture.split(",")
                         .map(|x| parseNumber.find(x).unwrap().as_str().parse::<i32>().unwrap())
                         .fold(1, |a, b| a*b);
        sub += local;
    }

    let count = all.len();
    println!("captures count is {count}");
    println!("total sub is {sub}");
    let diff = acc - sub;
    println!("diff is {diff}");

    //let mut safes = 0;
    //for line in lines {
    //    println!("line {line}");
    //    let elts = line.split(" ")
    //                   .map(|x| x.parse::<i32>().unwrap())
    //                   .collect::<Vec<_>>();
    //    let mut safe = isSafe(&elts);
    //    let mut toRemove = 0..elts.len();
    //    let mut i = toRemove.next();
    //    while !safe && i.is_some() {
    //        let j = i.unwrap();
    //        let mut newElts = elts.to_vec();
    //        newElts.remove(j);
    //        println!("blbl is for {j}: {:?}", newElts);
    //        safe = isSafe(&newElts);
    //        i = toRemove.next();
    //    }

    //    if safe {
    //       safes += 1;
    //    }
    //}
    //println!("safes {safes}")
}
