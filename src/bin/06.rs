fn is_unique(input: &[u8]) -> bool {
  let mut unique_flag = true;
  for i in 1..input.len() {
      if input[i..].contains(&input[i - 1]) {
          unique_flag = false
      }
  }
  unique_flag
}

pub fn part_one(input: &str) -> Option<usize> {
    let code_length = 4;
    let bytes = input.as_bytes();
    (code_length..bytes.len()).find(|&i| is_unique(&bytes[i-code_length..i]))
}

pub fn part_two(input: &str) -> Option<usize> {
    let code_length = 14;
    let bytes = input.as_bytes();
    (code_length..bytes.len()).find(|&i| is_unique(&bytes[i-code_length..i]))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
