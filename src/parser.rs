#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_duration() {
        let parser = DurationParser::new();
        let tokens = parser.parse("2h").unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Duration(7200));
    }

    #[test]
    fn test_parse_compound_duration() {
        let parser = DurationParser::new();
        let tokens = parser.parse("2h 30m").unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Duration(9000));
    }

    #[test]
    fn test_parse_all_units() {
        let parser = DurationParser::new();
        let tokens = parser.parse("1w 2d 3h 4m 5s").unwrap();
        assert_eq!(tokens.len(), 1);
        let expected = 604800 + 172800 + 10800 + 240 + 5;
        assert_eq!(tokens[0], Token::Duration(expected));
    }

    #[test]
    fn test_parse_with_addition() {
        let parser = DurationParser::new();
        let tokens = parser.parse("2h + 30m").unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Duration(7200));
        assert_eq!(tokens[1], Token::Operator(Operator::Add));
        assert_eq!(tokens[2], Token::Duration(1800));
    }

    #[test]
    fn test_parse_all_operators() {
        let parser = DurationParser::new();
        let tokens = parser.parse("1h + 2h - 30m * 2 / 3").unwrap();
        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens[1], Token::Operator(Operator::Add));
        assert_eq!(tokens[3], Token::Operator(Operator::Subtract));
        assert_eq!(tokens[5], Token::Operator(Operator::Multiply));
        assert_eq!(tokens[7], Token::Operator(Operator::Divide));
    }

    #[test]
    fn test_parse_empty_string() {
        let parser = DurationParser::new();
        let result = parser.parse("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Empty expression");
    }

    #[test]
    fn test_parse_invalid_unit() {
        let parser = DurationParser::new();
        let result = parser.parse("2x");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_whitespace_handling() {
        let parser = DurationParser::new();
        let tokens = parser.parse("  2h   +   30m  ").unwrap();
        assert_eq!(tokens.len(), 3);
    }
}