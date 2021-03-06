
use leetcode_rs::p00xx::p39::*;

#[derive(Debug, Clone)]
struct TestCase {
    input : Input,
    answer: Output,
}

#[test]
fn t39() {

    let solutions = [
        Box::new(Solution0) as Box<dyn Solution>,
        // Box::new(Solution1) as Box<dyn Solution>,
    ];
    
    let test_cases = [
        TestCase {
            input: Input {
                candidates: vec![2, 3, 6, 7],
                target: 7,
            },
            answer: vec![
                vec![2, 2, 3],
                vec![7],
            ],
        },
        TestCase {
            input: Input {
                candidates: vec![2, 3, 5],
                target: 8,
            },
            answer: vec![
                vec![2, 2, 2, 2],
                vec![2, 3, 3],
                vec![3, 5],
            ],
        },
    ];

    for (i, solution) in solutions.into_iter().enumerate() {

        for (j, test_case) in test_cases.iter().cloned().enumerate() {

            let test_answer = solution.combination_sum(test_case.input.candidates, test_case.input.target);

            assert_eq!(test_answer, test_case.answer,
                "Test failed on Solution {} TestCase {}.\n\
                Correct  answer is {:?}\n\
                TestCase answer is {:?}\n",
                i, j, &test_case.answer, &test_answer);
        }
    }
}

