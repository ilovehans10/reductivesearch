pub mod reductivesearch {
    ///This is a searcher struct, it is the main interface for the reductive search module. It can
    ///be added to
    pub struct Searcher {
        tobesearched: Vec<String>,
        searchstring: String,
        searchcache: Vec<String>,
    }

    impl Searcher {
        /// Creates a new Searcher
        ///
        /// # Examples
        ///
        /// ```
        /// use reductivesearch::reductivesearch::Searcher;
        ///
        /// let mut greetingsearch = Searcher::new(vec![String::from("hi"), String::from("hello")]);
        /// ```
        pub fn new(tobesearched: Vec<String>) -> Searcher {
            Searcher {
                searchcache: tobesearched.clone(),
                tobesearched,
                searchstring: String::new(),
            }
        }

        /// Returns the search results stored in the search wrapped in a result
        ///
        /// # Examples
        ///
        /// ```
        /// use reductivesearch::reductivesearch::Searcher;
        ///
        /// let mut greetingsearch = Searcher::new(vec![String::from("hi"), String::from("hello")]);
        /// greetingsearch.add_character('h').unwrap();
        /// greetingsearch.add_character('e').unwrap();
        ///
        /// assert_eq!(vec![String::from("hello")], greetingsearch.search_results().unwrap());
        /// ```
        pub fn search_results(&self) -> Result<Vec<String>, String> {
            if !self.searchcache.is_empty() {
                Ok(self.searchcache.clone())
            } else {
                Err(String::from("String not found"))
            }
        }

        /// Adds a character to the search string and updates the search cache
        ///
        /// # Examples
        ///
        /// ```
        /// use reductivesearch::reductivesearch::Searcher;
        ///
        /// let mut greetingsearch = Searcher::new(vec![String::from("hi"), String::from("hello")]);
        /// greetingsearch.add_character('h').unwrap();
        /// greetingsearch.add_character('e').unwrap();
        ///
        /// assert_eq!(vec![String::from("hello")], greetingsearch.search_results().unwrap());
        /// ```
        pub fn add_character(&mut self, character: char) -> Result<String, String> {
            let mut searchstring: String = self.searchstring.clone();
            searchstring.push(character);
            if !self.substring_search(searchstring.as_str()).is_empty() {
                self.searchstring.push(character);
                self.update_cache();
                return Ok(self.searchstring.clone())
            }
            Err(String::from("Adding character {character} caused search to return invalid"))
        }

        /// Adds a character to the search string and updates the search cache
        ///
        /// # Examples
        ///
        /// ```
        /// use reductivesearch::reductivesearch::Searcher;
        ///
        /// let mut greetingsearch = Searcher::new(vec![String::from("hi"), String::from("hello")]);
        /// greetingsearch.add_character('h').unwrap();
        /// greetingsearch.add_character('e').unwrap();
        /// greetingsearch.remove_character();
        /// greetingsearch.add_character('i').unwrap();
        ///
        /// assert_eq!(vec![String::from("hi")], greetingsearch.search_results().unwrap());
        /// ```
        pub fn remove_character(&mut self) {
            self.searchstring.pop();
            self.searchcache = self.tobesearched.clone();
            self.update_cache();
        }

        /// Clears the search and resets the search cache
        ///
        /// # Examples
        ///
        /// ```
        /// use reductivesearch::reductivesearch::Searcher;
        ///
        /// let mut greetingsearch = Searcher::new(vec![String::from("hi"), String::from("hello")]);
        /// greetingsearch.add_character('h').unwrap();
        /// greetingsearch.add_character('e').unwrap();
        /// greetingsearch.reset_search();
        ///
        /// assert_eq!(vec![String::from("hi"), String::from("hello")], greetingsearch.search_results().unwrap());
        /// ```
        pub fn reset_search(&mut self) {
            self.searchstring.clear();
            self.searchcache = self.tobesearched.clone();
        }

        // This searches each element of searchcache for searchstring, and returns a vector of all
        // of the results
        fn substring_search(&self, searchstring: &str) -> Vec<String> {
            self.searchcache
                .clone()
                .into_iter()
                .filter(|element| element.contains(searchstring))
                .collect()
        }

        // This method updates the searchcache variable based on a new search and the current
        // searchcache
        fn update_cache(&mut self) {
            self.searchcache = self.substring_search(&self.searchstring)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::reductivesearch::*;

    #[test]
    fn two_word_test() {
        let mut testsearcher = Searcher::new(vec![String::from("hi"), String::from("hill")]);
        testsearcher.add_character('h').unwrap();
        testsearcher.add_character('i').unwrap();
        assert_eq!(testsearcher.search_results().unwrap().len(), 2);
    }

    #[test]
    fn three_word_test() {
        let mut testsearcher = Searcher::new(vec![
            String::from("hi"),
            String::from("hill"),
            String::from("hello"),
        ]);
        testsearcher.add_character('h').unwrap();
        testsearcher.add_character('i').unwrap();
        assert_eq!(testsearcher.search_results().unwrap().len(), 2);
    }

    #[test]
    fn bad_add_test() {
        let mut testsearcher = Searcher::new(vec![
            String::from("hi"),
            String::from("hill"),
            String::from("hello"),
        ]);
        testsearcher.add_character('h').unwrap();
        testsearcher.add_character('a').unwrap_err();
        testsearcher.add_character('i').unwrap();
        assert_eq!(testsearcher.search_results().unwrap().len(), 2);
    }

    #[test]
    fn remove_test() {
        let mut testsearcher = Searcher::new(vec![
            String::from("hi"),
            String::from("hill"),
            String::from("hello"),
        ]);
        testsearcher.add_character('h').unwrap();
        testsearcher.add_character('i').unwrap();
        testsearcher.remove_character();
        testsearcher.add_character('e').unwrap();
        assert_eq!(testsearcher.search_results().unwrap().len(), 1);
    }

    #[test]
    fn reset_test() {
        let mut testsearcher = Searcher::new(vec![
            String::from("hi"),
            String::from("hill"),
            String::from("hello"),
        ]);
        testsearcher.add_character('h').unwrap();
        testsearcher.add_character('i').unwrap();
        testsearcher.reset_search();
        assert_eq!(testsearcher.search_results().unwrap().len(), 3);
    }
}
