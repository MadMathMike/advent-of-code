use std::iter::repeat;

pub fn count_valid_arrangements(mask: &str, damaged_segment_lengths: &Vec<usize>) -> usize {
    let num_of_segment_gaps = damaged_segment_lengths.len() - 1;
    let num_of_damaged_springs:usize = damaged_segment_lengths.iter().sum();
    let num_of_unassigned_working_springs = 
        mask.len() 
        - num_of_segment_gaps // Each segment gap must have at least one working spring
        - num_of_damaged_springs;

    if num_of_unassigned_working_springs == 0 {
        return 1;
    }

    let mut damaged_segments = Vec::<String>::new();
    for i in 0..damaged_segment_lengths.len() {
        let segment_length = damaged_segment_lengths[i];

        let mut damaged_segment = repeat('#').take(segment_length).collect::<Vec<char>>();

        if i < damaged_segment_lengths.len() - 1 {
            damaged_segment.push('.');
        }

        let segment = damaged_segment.iter().collect::<String>();

        damaged_segments.push(segment);
    }

    count_valid_arrangements_recursive(
        mask, 
        "", 
        num_of_unassigned_working_springs,
        &damaged_segments, 
        0
    )
}

// TODO: probably change springs_arrangment type to be String
fn count_valid_arrangements_recursive(
    mask: &str, 
    springs_arrangement: &str, 
    num_of_unassigned_working_springs: usize,
    damaged_segments: &Vec<String>, 
    depth: usize) 
    -> usize 
{
    if !matches(mask, springs_arrangement) {
        return 0;
    }        

    let mut next_springs_arrangement = String::from(springs_arrangement);

    if depth == damaged_segments.len() {
        next_springs_arrangement.push_str(&repeat('.').take(num_of_unassigned_working_springs).collect::<String>());

        return if matches(mask, &next_springs_arrangement) {
            1
        } else {
            0
        }
    }

    let mut valid_arrangement_count = 0;
    for i in 0..(num_of_unassigned_working_springs + 1) {
        let mut next_springs_arrangement = String::from(&next_springs_arrangement);
        next_springs_arrangement.push_str(&repeat('.').take(i).collect::<String>());
        
        if depth < damaged_segments.len() { 
            next_springs_arrangement.push_str(&damaged_segments[depth]);
        }

        valid_arrangement_count += count_valid_arrangements_recursive(
            mask, 
            &next_springs_arrangement, 
            num_of_unassigned_working_springs - i, 
            damaged_segments, 
            depth + 1
        );
    }

    valid_arrangement_count
}

fn matches(mask: &str, arrangement: &str) -> bool {
    arrangement.is_empty() 
    || arrangement.char_indices()
        .all(|(i, char)| 
            mask.chars().nth(i) == Some('?') 
            || char.eq(&mask.chars().nth(i).unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_valid_arrangements_returns_1() {
        let record = ("???.###", vec![1,1,3]);
        let arrangement_count = count_valid_arrangements(record.0, &record.1);
        assert_eq!(arrangement_count, 1);

        let record = ("????.#..", vec![4,1,1]);
        let arrangement_count = count_valid_arrangements(record.0, &record.1);
        assert_eq!(arrangement_count, 1);
    }

    #[test]
    fn count_valid_arrangements_works_on_sample_records() {
        let record = ("???.###", vec![1,1,3]);
        let arrangement_count = count_valid_arrangements(record.0, &record.1);
        assert_eq!(arrangement_count, 1);

        let record = (".??..??...?##.", vec![1,1,3]);
        let arrangement_count = count_valid_arrangements(record.0, &record.1);
        assert_eq!(arrangement_count, 4);

        let record = ("?#?#?#?#?#?#?#?", vec![1,3,1,6]);
        let arrangement_count = count_valid_arrangements(record.0, &record.1);
        assert_eq!(arrangement_count, 1);

        let record = ("????.#...#...", vec![4,1,1]);
        let arrangement_count = count_valid_arrangements(record.0, &record.1);
        assert_eq!(arrangement_count, 1);

        let record = ("????.######..#####.", vec![1,6,5]);
        let arrangement_count = count_valid_arrangements(record.0, &record.1);
        assert_eq!(arrangement_count, 4);

        let record = ("?###????????", vec![3,2,1]);
        let arrangement_count = count_valid_arrangements(record.0, &record.1);
        assert_eq!(arrangement_count, 10);

        // "unfolded" records
        let record = (
            "????.#...#...?????.#...#...?????.#...#...?????.#...#...?????.#...#...", 
            vec![4,1,1,4,1,1,4,1,1,4,1,1,4,1,1]
        );
        let arrangement_count = count_valid_arrangements(record.0, &record.1);
        assert_eq!(arrangement_count, 16);

        let record = (
            "????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####.", 
            vec![1,6,5,1,6,5,1,6,5,1,6,5,1,6,5]
        );
        let arrangement_count = count_valid_arrangements(record.0, &record.1);
        assert_eq!(arrangement_count, 2500);
    }

    #[test]
    fn matches_returns_true() {
        let mask = "?###????????";
        
        let arrangement = ".###.##.#...";
        assert!(matches(mask, arrangement));
        
        let arrangement = ".###..##...#";
        assert!(matches(mask, arrangement));
    }
}