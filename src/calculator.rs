#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Token, Operator};

    #[test]
    fn test_duration_as_minutes() {
        let duration = Duration::new(7200);
        assert_eq!(duration.as_minutes(), 120);
    }

    #[test]
    fn test_duration_as_hours() {
        let duration = Duration::new(7200);
        assert_eq!(duration.as_hours(), 2.0);
        
        let duration = Duration::new(9000);
        assert_eq!(duration.as_hours(), 2.5);
    }

    #[test]
    fn test_duration_human_readable() {
        let duration = Duration::new(9000);
        assert_eq!(duration.to_human_readable(), "2h 30m");
        
        let duration = Duration::new(90061);
        assert_eq!(duration.to_human_readable(), "1d 1h 1m 1s");
        
        let duration = Duration::new(0);
        assert_eq!(duration.to_human_readable(), "0s");
    }

    #[test]
    fn test_duration_human_readable_negative() {
        let duration = Duration::new(-7200);
        assert_eq!(duration.to_human_readable(), "-2h");
    }

    #[test]
    fn test_duration_iso8601() {
        let duration = Duration::new(9000);
        assert_eq!(duration.to_iso8601(), "PT2H30M0S");
        
        let duration = Duration::new(3661);
        assert_eq!(duration.to_iso8601(), "PT1H1M1S");
    }

    #[test]
    fn test_duration_iso8601_negative() {
        let duration = Duration::new(-7200);
        assert_eq!(duration.to_iso8601(), "-PT2H0M0S");
    }

    #[test]
    fn test_evaluate_simple_addition() {
        let calculator = Calculator::new();
        let tokens = vec![
            Token::Duration(7200),
            Token::Operator(Operator::Add),
            Token::Duration(1800),
        ];
        let result = calculator.evaluate(tokens).unwrap();
        assert_eq!(result.seconds, 9000);
    }

    #[test]
    fn test_evaluate_subtraction() {
        let calculator = Calculator::new();
        let tokens = vec![
            Token::Duration(7200),
            Token::Operator(Operator::Subtract),
            Token::Duration(1800),
        ];
        let result = calculator.evaluate(tokens).unwrap();
        assert_eq!(result.seconds, 5400);
    }

    #[test]
    fn test_evaluate_multiplication() {
        let calculator = Calculator::new();
        let tokens = vec![
            Token::Duration(3600),
            Token::Operator(Operator::Multiply),
            Token::Duration(2),
        ];
        let result = calculator.evaluate(tokens).unwrap();
        assert_eq!(result.seconds, 7200);
    }

    #[test]
    fn test_evaluate_division() {
        let calculator = Calculator::new();
        let tokens = vec![
            Token::Duration(7200),
            Token::Operator(Operator::Divide),
            Token::Duration(2),
        ];
        let result = calculator.evaluate(tokens).unwrap();
        assert_eq!(result.seconds, 3600);
    }

    #[test]
    fn test_evaluate_division_by_zero() {
        let calculator = Calculator::new();
        let tokens = vec![
            Token::Duration(7200),
            Token::Operator(Operator::Divide),
            Token::Duration(0),
        ];
        let result = calculator.evaluate(tokens);
        assert!(result.is_err());
    }

    #[test]
    fn test_evaluate_chain_operations() {
        let calculator = Calculator::new();
        let tokens = vec![
            Token::Duration(28800),
            Token::Operator(Operator::Subtract),
            Token::Duration(1800),
            Token::Operator(Operator::Add),
            Token::Duration(3600),
        ];
        let result = calculator.evaluate(tokens).unwrap();
        assert_eq!(result.seconds, 30600);
    }

    #[test]
    fn test_evaluate_empty_tokens() {
        let calculator = Calculator::new();
        let result = calculator.evaluate(vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_evaluate_incomplete_expression() {
        let calculator = Calculator::new();
        let tokens = vec![
            Token::Duration(7200),
            Token::Operator(Operator::Add),
        ];
        let result = calculator.evaluate(tokens);
        assert!(result.is_err());
    }
}