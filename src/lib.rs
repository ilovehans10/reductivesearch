pub mod reductivesearch {
    ///This is a searcher struct, it is the main interface for the reductive search module. It can
    ///be added to
    pub struct Searcher {
        queried_strings: Vec<String>,
        search_string: String,
        search_cache: Vec<String>,
    }

    impl Searcher {
        /// Creates a new Searcher
        ///
        /// # Examples
        ///
        /// ```
        /// use reductivesearch::reductivesearch::Searcher;
        ///
        /// let mut greeting_search = Searcher::new(vec![String::from("hi"), String::from("hello")]);
        /// ```
        pub fn new(queried_strings: Vec<String>) -> Searcher {
            Searcher {
                search_cache: queried_strings.clone(),
                queried_strings,
                search_string: String::new(),
            }
        }

        /// Returns the search results stored in the search wrapped in a result
        ///
        /// # Examples
        ///
        /// ```
        /// use reductivesearch::reductivesearch::Searcher;
        ///
        /// let mut greeting_search = Searcher::new(vec![String::from("hi"), String::from("hello")]);
        /// greeting_search.add_character('h').unwrap();
        /// greeting_search.add_character('e').unwrap();
        ///
        /// assert_eq!(vec![String::from("hello")], greeting_search.search_results());
        /// ```
        pub fn search_results(&self) -> Vec<String> {
            self.search_cache.clone()
        }

        /// Adds a character to the search string and updates the search cache
        ///
        /// # Examples
        ///
        /// ```
        /// use reductivesearch::reductivesearch::Searcher;
        ///
        /// let mut greeting_search = Searcher::new(vec![String::from("hi"), String::from("hello")]);
        /// greeting_search.add_character('h').unwrap();
        /// greeting_search.add_character('e').unwrap();
        ///
        /// assert_eq!(vec![String::from("hello")], greeting_search.search_results());
        /// ```
        pub fn add_character(&mut self, character: char) -> Result<String, String> {
            let mut search_string: String = self.search_string.clone();
            search_string.push(character);
            if !self.substring_search(search_string.as_str()).is_empty() {
                self.search_string.push(character);
                self.update_cache();
                return Ok(self.search_string.clone())
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
        /// let mut greeting_search = Searcher::new(vec![String::from("hi"), String::from("hello")]);
        /// greeting_search.add_character('h').unwrap();
        /// greeting_search.add_character('e').unwrap();
        /// greeting_search.remove_character();
        /// greeting_search.add_character('i').unwrap();
        ///
        /// assert_eq!(vec![String::from("hi")], greeting_search.search_results());
        /// ```
        pub fn remove_character(&mut self) {
            self.search_string.pop();
            self.search_cache = self.queried_strings.clone();
            self.update_cache();
        }

        /// Clears the search and resets the search cache
        ///
        /// # Examples
        ///
        /// ```
        /// use reductivesearch::reductivesearch::Searcher;
        ///
        /// let mut greeting_search = Searcher::new(vec![String::from("hi"), String::from("hello")]);
        /// greeting_search.add_character('h').unwrap();
        /// greeting_search.add_character('e').unwrap();
        /// greeting_search.reset_search();
        ///
        /// assert_eq!(vec![String::from("hi"), String::from("hello")], greeting_search.search_results());
        /// ```
        pub fn reset_search(&mut self) {
            self.search_string.clear();
            self.search_cache = self.queried_strings.clone();
        }

        /// Add to the vec of strings to be searched through. This will also hard reset the cache.
        ///
        /// # Examples
        ///
        /// ```
        /// use reductivesearch::reductivesearch::Searcher;
        ///
        /// let mut greeting_search = Searcher::new(vec![String::from("hi"), String::from("hello")]);
        /// greeting_search.add_character('h').unwrap();
        /// greeting_search.add_character('e').unwrap();
        /// greeting_search.add_to_vec(String::from("hev suit"));
        ///
        /// assert_eq!(vec![String::from("hello"), String::from("hev suit")], greeting_search.search_results());
        /// ```
        pub fn add_to_vec(&mut self, string_to_add: String) {
            self.queried_strings.push(string_to_add);
            self.search_cache = self.queried_strings.clone();
            self.update_cache();
        }

        // This searches each element of searchcache for searchstring, and returns a vector of all
        // of the results
        fn substring_search(&self, search_string: &str) -> Vec<String> {
            self.search_cache
                .clone()
                .into_iter()
                .filter(|element| element.contains(search_string))
                .collect()
        }

        // This method updates the searchcache variable based on a new search and the current
        // searchcache
        fn update_cache(&mut self) {
            self.search_cache = self.substring_search(&self.search_string)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::reductivesearch::*;

    #[test]
    fn two_word_test() {
        let mut test_searcher = Searcher::new(vec![String::from("hi"), String::from("hill")]);
        dbg!(test_searcher.add_character('h').unwrap());
        dbg!(test_searcher.add_character('i').unwrap());
        assert_eq!(dbg!(test_searcher.search_results()).len(), 2);
    }

    #[test]
    fn three_word_test() {
        let mut test_searcher = Searcher::new(vec![
            String::from("hi"),
            String::from("hill"),
            String::from("hello"),
        ]);
        dbg!(test_searcher.add_character('h').unwrap());
        dbg!(test_searcher.add_character('i').unwrap());
        assert_eq!(dbg!(test_searcher.search_results()).len(), 2);
    }

    #[test]
    fn bad_add_test() {
        let mut test_searcher = Searcher::new(vec![
            String::from("hi"),
            String::from("hill"),
            String::from("hello"),
        ]);
        dbg!(test_searcher.add_character('h').unwrap());
        dbg!(test_searcher.add_character('a').unwrap_err());
        dbg!(test_searcher.add_character('i').unwrap());
        assert_eq!(dbg!(test_searcher.search_results()).len(), 2);
    }

    #[test]
    fn remove_test() {
        let mut test_searcher = Searcher::new(vec![
            String::from("hi"),
            String::from("hill"),
            String::from("hello"),
        ]);
        dbg!(test_searcher.add_character('h').unwrap());
        dbg!(test_searcher.add_character('i').unwrap());
        dbg!(test_searcher.remove_character());
        dbg!(test_searcher.add_character('e').unwrap());
        assert_eq!(dbg!(test_searcher.search_results()).len(), 1);
    }

    #[test]
    fn reset_test() {
        let mut test_searcher = Searcher::new(vec![
            String::from("hi"),
            String::from("hill"),
            String::from("hello"),
        ]);
        dbg!(test_searcher.add_character('h').unwrap());
        dbg!(test_searcher.add_character('i').unwrap());
        test_searcher.reset_search();
        assert_eq!(dbg!(test_searcher.search_results()).len(), 3);
    }

    #[test]
    fn add_to_vec_test() {
        let mut test_searcher = Searcher::new(vec![
            String::from("hi"),
            String::from("hill"),
            String::from("hello"),
        ]);
        dbg!(test_searcher.add_character('h').unwrap());
        dbg!(test_searcher.add_character('e').unwrap());
        test_searcher.add_to_vec(String::from("hev suit"));
        assert_eq!(dbg!(test_searcher.search_results()).len(), 2);
    }
}
