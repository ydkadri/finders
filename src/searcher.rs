use regex::Regex;

// Struct for search results
#[derive(Debug, PartialEq)]
pub struct SearchResult {
    pub rownum: usize,
    pub line: String,
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
    fn search<'a>(&'a self, contents: &'a str) -> Vec<SearchResult>;

    // New method for streaming line-by-line search
    fn search_line(&self, line: &str, rownum: usize) -> Option<SearchResult>;
}

impl SearchResult {
    fn new(rownum: usize, line: String) -> SearchResult {
        SearchResult { rownum, line }
    }
}

impl Searcher<'_> {
    pub fn new(query: &str, case_insensitive: bool) -> Searcher<'_> {
        Searcher {
            query,
            case_insensitive,
        }
    }

    fn _sensitive_search<'a>(&'a self, contents: &'a str) -> Vec<SearchResult> {
        let mut results = Vec::new();
        let mut rownum = 1;

        for line in contents.lines() {
            if line.contains(self.query) {
                let result = SearchResult::new(rownum, line.to_string());
                results.push(result)
            }
            rownum += 1;
        }

        results
    }

    fn _insensitive_search<'a>(&'a self, contents: &'a str) -> Vec<SearchResult> {
        let mut results = Vec::new();
        let query = self.query.to_lowercase();
        let mut rownum = 1;

        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                let result = SearchResult::new(rownum, line.to_string());
                results.push(result)
            }
            rownum += 1;
        }

        results
    }
}

impl ReSearcher {
    pub fn new(pattern: &str) -> ReSearcher {
        ReSearcher {
            pattern: Regex::new(pattern).unwrap(),
        }
    }
}

impl Searches for Searcher<'_> {
    fn search<'a>(&'a self, contents: &'a str) -> Vec<SearchResult> {
        if self.case_insensitive {
            self._insensitive_search(contents)
        } else {
            self._sensitive_search(contents)
        }
    }

    fn search_line(&self, line: &str, rownum: usize) -> Option<SearchResult> {
        let matches = if self.case_insensitive {
            line.to_lowercase().contains(&self.query.to_lowercase())
        } else {
            line.contains(self.query)
        };

        if matches {
            Some(SearchResult::new(rownum, line.to_string()))
        } else {
            None
        }
    }
}

impl Searches for ReSearcher {
    fn search<'a>(&'a self, contents: &'a str) -> Vec<SearchResult> {
        let mut results = Vec::new();
        let mut rownum = 1;

        for line in contents.lines() {
            if self.pattern.is_match(line) {
                let result = SearchResult::new(rownum, line.to_string());
                results.push(result)
            }
            rownum += 1;
        }

        results
    }

    fn search_line(&self, line: &str, rownum: usize) -> Option<SearchResult> {
        if self.pattern.is_match(line) {
            Some(SearchResult::new(rownum, line.to_string()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Error;
    static CONTENTS: &str = "line one\nLINE TWO";

    #[test]
    fn test_case_sensitive() -> Result<(), Error> {
        let searcher = Searcher::new("line", false);

        let observed_result = searcher.search(CONTENTS);
        let expected_result = vec![SearchResult::new(1, "line one".to_string())];

        assert_eq!(observed_result, expected_result);

        Ok(())
    }

    #[test]
    fn test_case_insensitive() -> Result<(), Error> {
        let searcher = Searcher::new("line", true);

        let observed_result = searcher.search(CONTENTS);
        let expected_result = vec![
            SearchResult::new(1, "line one".to_string()),
            SearchResult::new(2, "LINE TWO".to_string()),
        ];

        assert_eq!(observed_result, expected_result);

        Ok(())
    }

    #[test]
    fn test_regex_match() -> Result<(), Error> {
        let re_searcher = ReSearcher::new("[a-z]+");

        let observed_result = re_searcher.search(CONTENTS);
        let expected_result = vec![SearchResult::new(1, "line one".to_string())];

        assert_eq!(observed_result, expected_result);

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
        let re_searcher = ReSearcher::new("[a-z]+");

        let result1 = re_searcher.search_line("line one", 1);
        assert!(result1.is_some());

        let result2 = re_searcher.search_line("LINE TWO", 2);
        assert!(result2.is_none());

        Ok(())
    }
}
