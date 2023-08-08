pub use crate::utils::get_input;

// TODO: should I use Enum here?
fn parse_direction(direction: &char) -> i32 {
    match direction.to_string().as_str() {
        "(" => 1,
        ")" => -1,
        c => panic!("Character {} invalid!", c),
    }
}

fn validate_input(input: &str) {
    if input.len() <= 0 {
        panic!("Can`t parse an empty input!");
    }
}

pub fn part1(input: &str) -> i32 {
    let mut result: i32 = 0;

    validate_input(input);

    // TODO: try to use reduce here
    input.chars().for_each(|c| {
        result = result + parse_direction(&c);
    });

    result
}

pub fn part2(input: &str) -> i32 {
    let mut result: i32 = 0;
    let mut position: i32 = 0;

    validate_input(input);

    // TODO: try to use reduce here
    for (idx, c) in input.chars().enumerate() {
        result = result + parse_direction(&c);

        if result < 0 {
            println!("{} - {}", result, idx);
            position = (idx + 1) as i32;
            break;
        }
    }

    return position;
}

#[cfg(test)]
mod tests {
    use super::*;
    struct ObjTest {
        input: String,
        expected: i32,
    }

    impl ObjTest {
        fn create(input: &str, expect: i32) -> ObjTest {
            ObjTest {
                input: input.to_string(),
                expected: expect,
            }
        }
    }

    #[test]
    fn it_should_parse_mocked_part1() {
        let items: [ObjTest; 9] = [
            ObjTest::create("(())", 0),
            ObjTest::create("()()", 0),
            ObjTest::create("(((", 3),
            ObjTest::create("(()(()(", 3),
            ObjTest::create("))(((((", 3),
            ObjTest::create("())", -1),
            ObjTest::create("))(", -1),
            ObjTest::create(")))", -3),
            ObjTest::create(")())())", -3),
        ];

        items.iter().for_each(|obj| {
            let result = part1(&obj.input);

            assert_eq!(result, obj.expected);
        });
    }

    #[test]
    fn it_should_parse_mocked_part2() {
        let items: [ObjTest; 2] = [ObjTest::create(")", 1), ObjTest::create("()())", 5)];

        items.iter().for_each(|obj| {
            let result = part2(&obj.input);

            assert_eq!(result, obj.expected);
        });
    }

    const FILE_PATH: &str = "src/day01/input.txt";

    #[test]
    fn it_should_solve_part1() {
        let input = get_input(FILE_PATH);
        let result = part1(&input);

        assert_eq!(232, result);
    }

    #[test]
    fn it_should_solve_part2() {
        let input = get_input(FILE_PATH);
        let result = part2(&input);

        assert_eq!(1783, result);
    }
}
