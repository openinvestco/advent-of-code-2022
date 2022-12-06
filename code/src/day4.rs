pub mod day4 {
    pub mod part_1 {
        fn doRangesOverlap(ranges: Vec<&str>) -> bool{
            let elf1Range:&str = ranges.get(0).unwrap();
            let elf2Range:&str = ranges.get(1).unwrap();

            let elf1Split:Vec<&str> = elf1Range.split("-").collect();
            let elf2Split:Vec<&str> = elf2Range.split("-").collect();

            let elf1Start = elf1Split.get(0).unwrap().parse::<u32>().unwrap();
            let elf1End = elf1Split.get(1).unwrap().parse::<u32>().unwrap();

            let elf2Start = elf2Split.get(0).unwrap().parse::<u32>().unwrap();
            let elf2End = elf2Split.get(1).unwrap().parse::<u32>().unwrap();

            if elf1Start >= elf2Start && elf1Start <= elf2End {
                if elf1End >= elf2Start && elf1End <= elf2End {
                    return true;
                }
            }

            if elf2Start >= elf1Start && elf2Start <= elf1End {
                if elf2End >= elf1Start && elf2End <= elf1End {
                    return true;
                }
            }
            return false;

        }
        pub fn run() {
            let mut countOfOverlappingRanges: u32 = 0;
            if let Ok(lines) = crate::read_lines("data/day4/input2.txt") {
                for line in lines {
                    if let Ok(ip) = line {
                        if ip.len() > 0 {
                            let ranges = format!("{}", ip);
                            let elfRangeSplit:Vec<&str> = ranges.split(",").collect();

                            if doRangesOverlap(elfRangeSplit) {
                                countOfOverlappingRanges += 1;
                            }
                        }
                    }
                }
                println!("Number of overlapping ranges: {}", countOfOverlappingRanges);
            }
        }
    }
    pub mod part_2 {
        fn doRangesOverlap(ranges: Vec<&str>) -> bool{
            let elf1Range:&str = ranges.get(0).unwrap();
            let elf2Range:&str = ranges.get(1).unwrap();

            let elf1Split:Vec<&str> = elf1Range.split("-").collect();
            let elf2Split:Vec<&str> = elf2Range.split("-").collect();

            let elf1Start = elf1Split.get(0).unwrap().parse::<u32>().unwrap();
            let elf1End = elf1Split.get(1).unwrap().parse::<u32>().unwrap();

            let elf2Start = elf2Split.get(0).unwrap().parse::<u32>().unwrap();
            let elf2End = elf2Split.get(1).unwrap().parse::<u32>().unwrap();

            if elf1Start >= elf2Start && elf1Start <= elf2End {
                return true;
            }

            if elf2Start >= elf1Start && elf2Start <= elf1End {
                return true;
            }
            return false;

        }
        pub fn run() {
            let mut countOfOverlappingRanges: u32 = 0;
            if let Ok(lines) = crate::read_lines("data/day4/input2.txt") {
                for line in lines {
                    if let Ok(ip) = line {
                        if ip.len() > 0 {
                            let ranges = format!("{}", ip);
                            let elfRangeSplit:Vec<&str> = ranges.split(",").collect();

                            if doRangesOverlap(elfRangeSplit) {
                                countOfOverlappingRanges += 1;
                            }
                        }
                    }
                }
                println!("Number of overlapping ranges: {}", countOfOverlappingRanges);
            }
        }
    }
}