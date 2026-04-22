use anyhow::{Context, Result};
use regex::Regex;

// Struct for search results
#[derive(Debug, PartialEq)]
pub struct SearchResult {
    pub rownum: usize,
    pub line: String,
    pub match_positions: Vec<(usize, usize)>, // (start, end) byte positions of matches
}

// Structs for basic and regex searchers
pub struct Searcher<'a> {
    query: &'a str,
    case_insensitive: bool,
}

pub struct ReSearcher {
    pattern: Regex,
}

// Searches trait for things which can perform search functions
pub trait Searches {
    // Line-by-line search method (used by production code)
    fn search_line(&self, line: &str, rownum: usize) -> Option<SearchResult>;
}

impl SearchResult {
    fn new(rownum: usize, line: String, match_positions: Vec<(usize, usize)>) -> SearchResult {
        SearchResult {
            rownum,
            line,
            match_positions,
        }
    }
}

impl Searcher<'_> {
    pub fn new(query: &str, case_insensitive: bool) -> Searcher<'_> {
        Searcher {
            query,
            case_insensitive,
        }
    }

    fn find_match_positions(&self, line: &str, case_insensitive: bool) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        let search_line = if case_insensitive {
            line.to_lowercase()
        } else {
            line.to_string()
        };
        let search_query = if case_insensitive {
            self.query.to_lowercase()
        } else {
            self.query.to_string()
        };

        let mut start = 0;
        while let Some(pos) = search_line[start..].find(&search_query) {
            let match_start = start + pos;
            let match_end = match_start + self.query.len();
            positions.push((match_start, match_end));
            start = match_end;
        }

        positions
    }
}

impl ReSearcher {
    pub fn new(pattern: &str) -> Result<ReSearcher> {
        Ok(ReSearcher {
            pattern: Regex::new(pattern)
                .context(format!("compiling regex pattern '{}'", pattern))?,
        })
    }

    fn find_regex_match_positions(&self, line: &str) -> Vec<(usize, usize)> {
        self.pattern
            .find_iter(line)
            .map(|m| (m.start(), m.end()))
            .collect()
    }
}

impl Searches for Searcher<'_> {
    fn search_line(&self, line: &str, rownum: usize) -> Option<SearchResult> {
        let matches = if self.case_insensitive {
            line.to_lowercase().contains(&self.query.to_lowercase())
        } else {
            line.contains(self.query)
        };

        if matches {
            let match_positions = self.find_match_positions(line, self.case_insensitive);
            Some(SearchResult::new(rownum, line.to_string(), match_positions))
        } else {
            None
        }
    }
}

impl Searches for ReSearcher {
    fn search_line(&self, line: &str, rownum: usize) -> Option<SearchResult> {
        if self.pattern.is_match(line) {
            let match_positions = self.find_regex_match_positions(line);
            Some(SearchResult::new(rownum, line.to_string(), match_positions))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Error;

    #[test]
    fn test_case_sensitive() -> Result<(), Error> {
        let searcher = Searcher::new("line", false);

        // Test line-by-line matching (production path)
        let result1 = searcher.search_line("line one", 1);
        assert!(result1.is_some());
        let unwrapped = result1.unwrap();
        assert_eq!(unwrapped.rownum, 1);
        assert_eq!(unwrapped.line, "line one");
        assert_eq!(unwrapped.match_positions, vec![(0, 4)]);

        // Should not match uppercase
        let result2 = searcher.search_line("LINE TWO", 2);
        assert!(result2.is_none());

        Ok(())
    }

    #[test]
    fn test_case_insensitive() -> Result<(), Error> {
        let searcher = Searcher::new("line", true);

        // Should match lowercase
        let result1 = searcher.search_line("line one", 1);
        assert!(result1.is_some());
        let unwrapped1 = result1.unwrap();
        assert_eq!(unwrapped1.rownum, 1);
        assert_eq!(unwrapped1.line, "line one");
        assert_eq!(unwrapped1.match_positions, vec![(0, 4)]);

        // Should also match uppercase
        let result2 = searcher.search_line("LINE TWO", 2);
        assert!(result2.is_some());
        let unwrapped2 = result2.unwrap();
        assert_eq!(unwrapped2.rownum, 2);
        assert_eq!(unwrapped2.line, "LINE TWO");
        assert_eq!(unwrapped2.match_positions, vec![(0, 4)]);

        Ok(())
    }

    #[test]
    fn test_regex_match() -> Result<(), Error> {
        let re_searcher = ReSearcher::new("[a-z]+").expect("Valid regex pattern");

        // Test line-by-line matching (production path)
        let result = re_searcher.search_line("line one", 1);
        assert!(result.is_some());
        let unwrapped = result.unwrap();
        assert_eq!(unwrapped.rownum, 1);
        assert_eq!(unwrapped.line, "line one");
        assert_eq!(unwrapped.match_positions, vec![(0, 4), (5, 8)]);

        // Should not match uppercase
        let no_match = re_searcher.search_line("LINE TWO", 2);
        assert!(no_match.is_none());

        Ok(())
    }

    #[test]
    fn test_search_line_case_sensitive() -> Result<(), Error> {
        let searcher = Searcher::new("line", false);

        let result = searcher.search_line("line one", 1);
        assert!(result.is_some());
        assert_eq!(result.unwrap().rownum, 1);

        let no_match = searcher.search_line("LINE TWO", 2);
        assert!(no_match.is_none());

        Ok(())
    }

    #[test]
    fn test_search_line_case_insensitive() -> Result<(), Error> {
        let searcher = Searcher::new("line", true);

        let result1 = searcher.search_line("line one", 1);
        assert!(result1.is_some());

        let result2 = searcher.search_line("LINE TWO", 2);
        assert!(result2.is_some());

        Ok(())
    }

    #[test]
    fn test_regex_search_line() -> Result<(), Error> {
        let re_searcher = ReSearcher::new("[a-z]+").expect("Valid regex pattern");

        let result1 = re_searcher.search_line("line one", 1);
        assert!(result1.is_some());

        let result2 = re_searcher.search_line("LINE TWO", 2);
        assert!(result2.is_none());

        Ok(())
    }

    #[test]
    fn test_invalid_regex_pattern() {
        // Test that invalid regex patterns return an error instead of panicking
        let invalid_patterns = vec![
            "[invalid(",      // Unclosed character class
            "(unclosed",      // Unclosed group
            "(?P<incomplete", // Incomplete named group
            "*invalid",       // Invalid repetition
        ];

        for pattern in invalid_patterns {
            let result = ReSearcher::new(pattern);
            assert!(
                result.is_err(),
                "Expected error for invalid pattern: {}",
                pattern
            );
        }
    }

    #[test]
    fn test_valid_regex_patterns() {
        // Test that valid regex patterns work correctly
        let valid_patterns = vec![
            "[a-z]+",
            "(test)",
            "(?P<name>\\w+)",
            "test*",
            "^start",
            "end$",
        ];

        for pattern in valid_patterns {
            let result = ReSearcher::new(pattern);
            assert!(
                result.is_ok(),
                "Expected success for valid pattern: {}",
                pattern
            );
        }
    }
}
