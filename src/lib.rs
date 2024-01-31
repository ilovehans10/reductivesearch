pub mod reductivesearch {
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
        /// greetingsearch.add_character('h');
        /// greetingsearch.add_character('e');
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
        /// greetingsearch.add_character('h');
        /// greetingsearch.add_character('e');
        ///
        /// assert_eq!(vec![String::from("hello")], greetingsearch.search_results().unwrap());
        /// ```
        pub fn add_character(&mut self, character: char) {
            let mut searchstring: String = self.searchstring.clone();
            searchstring.push(character);
            if !self.substring_search(searchstring.as_str()).is_empty() {
                self.searchstring.push(character);
                self.update_cache();
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
        /// greetingsearch.add_character('h');
        /// greetingsearch.add_character('e');
        /// greetingsearch.remove_character();
        /// greetingsearch.add_character('i');
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
        /// greetingsearch.add_character('h');
        /// greetingsearch.add_character('e');
        /// greetingsearch.reset_search();
        ///
        /// assert_eq!(vec![String::from("hi"), String::from("hello")], greetingsearch.search_results().unwrap());
        /// ```
        pub fn reset_search(&mut self) {
            self.searchstring.clear();
            self.searchcache = self.tobesearched.clone();
        }

        fn substring_search(&self, searchstring: &str) -> Vec<String> {
            self.searchcache
                .clone()
                .into_iter()
                .filter(|element| element.contains(searchstring))
                .collect()
        }

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
        testsearcher.add_character('h');
        testsearcher.add_character('i');
        assert_eq!(testsearcher.search_results().unwrap().len(), 2);
    }

    #[test]
    fn three_word_test() {
        let mut testsearcher = Searcher::new(vec![
            String::from("hi"),
            String::from("hill"),
            String::from("hello"),
        ]);
        testsearcher.add_character('h');
        testsearcher.add_character('i');
        assert_eq!(testsearcher.search_results().unwrap().len(), 2);
    }

    #[test]
    fn bad_add_test() {
        let mut testsearcher = Searcher::new(vec![
            String::from("hi"),
            String::from("hill"),
            String::from("hello"),
        ]);
        testsearcher.add_character('h');
        testsearcher.add_character('a');
        testsearcher.add_character('i');
        assert_eq!(testsearcher.search_results().unwrap().len(), 2);
    }

    #[test]
    fn remove_test() {
        let mut testsearcher = Searcher::new(vec![
            String::from("hi"),
            String::from("hill"),
            String::from("hello"),
        ]);
        testsearcher.add_character('h');
        testsearcher.add_character('i');
        testsearcher.remove_character();
        testsearcher.add_character('e');
        assert_eq!(testsearcher.search_results().unwrap().len(), 1);
    }

    #[test]
    fn reset_test() {
        let mut testsearcher = Searcher::new(vec![
            String::from("hi"),
            String::from("hill"),
            String::from("hello"),
        ]);
        testsearcher.add_character('h');
        testsearcher.add_character('i');
        testsearcher.reset_search();
        assert_eq!(testsearcher.search_results().unwrap().len(), 3);
    }
}
