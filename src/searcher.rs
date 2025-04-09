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
}

impl SearchResult {
    fn new(rownum: usize, line: String) -> SearchResult {
        SearchResult { rownum, line }
    }
}

impl Searcher<'_> {
    pub fn new<'a>(query: &'a str, case_insensitive: bool) -> Searcher<'a> {
        Searcher {
            query: query,
            case_insensitive: case_insensitive,
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
        let mut expected_result = Vec::new();

        expected_result.push(SearchResult::new(1, "line one".to_string()));

        assert_eq!(observed_result, expected_result);

        Ok(())
    }

    #[test]
    fn test_case_insensitive() -> Result<(), Error> {
        let searcher = Searcher::new("line", true);

        let observed_result = searcher.search(CONTENTS);
        let mut expected_result = Vec::new();

        expected_result.push(SearchResult::new(1, "line one".to_string()));
        expected_result.push(SearchResult::new(2, "LINE TWO".to_string()));

        assert_eq!(observed_result, expected_result);

        Ok(())
    }

    #[test]
    fn test_regex_match() -> Result<(), Error> {
        let re_searcher = ReSearcher::new("[a-z]+");

        let observed_result = re_searcher.search(CONTENTS);
        let mut expected_result = Vec::new();

        expected_result.push(SearchResult::new(1, "line one".to_string()));

        assert_eq!(observed_result, expected_result);

        Ok(())
    }
}
