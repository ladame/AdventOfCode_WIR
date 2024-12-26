use std::{fs::read_to_string, io::Error, usize};
use itertools::Itertools;

pub struct Manual {
    rules: Vec<(usize, usize)>,
    updates: Vec<Vec<usize>>,
    sum_pages: usize,
}

impl Manual {
    pub fn new(file_path: &str) -> Result<Manual, Error> {
        let content = read_to_string(file_path).expect("Cannot open file, check the path!");
        // put it in a vector of lines
        let lines: Vec<_> = content.lines().collect();
        // split the lines into parts
        let data: Vec<_> = lines.split(|line| line.is_empty()).collect();

        let mut manual: Manual = Manual {
            rules: Manual::set_rules(&data[0]),
            updates: Manual::set_updates(&data[1]),
            sum_pages: 0
        };
        //println!("UPDATE {:?}", manual.updates);
        manual.sum_pages = Manual::sum_pages(manual.rules.clone(), manual.updates.clone());
        Ok(manual)
    }
    fn set_rules(rule: &[&str]) -> Vec<(usize, usize)> {
        // rule example: "97|29", "53|29", "61|53"
        return (*rule)
            .iter()
            .map(|line| {
                let mut page = line.split("|");
                (
                    page.next().unwrap().parse::<usize>().unwrap(),
                    page.next().unwrap().parse::<usize>().unwrap(),
                )
            })
            .collect::<Vec<(usize, usize)>>();
    }

    pub fn get_rules(&self) -> Vec<(usize, usize)> {
        self.rules.clone()
    }

    fn set_updates(update: &[&str]) -> Vec<Vec<usize>> {
        // update example: "1,2,3,4,5,6", "7,8,9,10,11,12"
        return update
            .iter()
            .map(|line| {
                //println!("LINE {:?}", line);
                line.split(",")
                    .map(|number| {
                        //println!("Number {:?}", number);
                        number.parse::<usize>().unwrap()
                    })
                    .collect() //Vec<usize>
            })
            .collect(); //Vec<Vec<usize>>
    }

    pub fn get_updates(&self) -> Vec<Vec<usize>> {
        self.updates.clone()
    }

    fn sum_pages(rules: Vec<(usize, usize)>, updates: Vec<Vec<usize>>) -> usize {
        // UPDATE [[75, 47, 61, 53, 29], [97, 29, 53, 29, 61], ...
        // Check that the update is not in the rules, so the inverse pair (r.1==x and r.2==y) shall always wrong
        return updates
            .iter()
            .filter(|update| -> bool {
                update
                    .iter()
                    .combinations(2)
                    .map(|pair| {
                        //println!("PAIR {:?}", pair);
                        (pair[0], pair[1])
                    })
                    .all(|(&x, &y)| 
                        rules.iter().any(|r|{ 
                    //println!("r1:{} Y: {}, r0:{} X: {}", r.1, y, r.0, x);
                    (r.1 == y && r.0 == x)
                    })
                )
            })
            .map(|update| 
                {
                    //println!("UPDATE {:?}", update);
                    update[update.len() / 2]
                }
        ).sum();
    }

    pub fn get_sum_pages(&self) -> usize {
        return self.sum_pages;
    }
}
