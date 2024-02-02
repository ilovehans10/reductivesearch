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
        #[must_use]
        pub fn new(queried_strings: Vec<String>) -> Self {
            Self {
                search_cache: queried_strings.clone(),
                queried_strings,
                search_string: String::new(),
            }
        }

        /// Returns the search results
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
        #[must_use]
        pub fn search_results(&self) -> Vec<String> {
            self.search_cache.clone()
        }

        /// Adds a character to the search string and updates the search cache
        ///
        /// # Errors
        ///
        /// Will return an error if the character can't be added to the search. This will happen if
        /// the search string + the argument character doesn't appear in any of the strings to be
        /// searched.
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
                return Ok(self.search_string.clone());
            }
            Err(format!(
                "Adding character '{character}' to the search returned no results"
            ))
        }

        /// Adds a character to the search string, resets the search cache, and updates the search cache
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
            self.reset_cache();
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
        /// greeting_search.add_to_vec(String::from("guten tag"));
        /// greeting_search.add_to_vec(String::from("hev suit"));
        ///
        /// assert_eq!(vec![String::from("hello"), String::from("hev suit")], greeting_search.search_results());
        /// ```
        pub fn add_to_vec(&mut self, string_to_add: String) {
            self.queried_strings.push(string_to_add);
            self.reset_cache();
        }

        /// Remove a string from the vector of strings to search
        ///
        /// # Errors
        ///
        /// This returns errors if the string is not found in the ``queried_strings`` or the string
        /// can't be removed without removing the last item from the search.
        ///
        /// # Examples
        ///
        /// ```
        /// use reductivesearch::reductivesearch::Searcher;
        ///
        /// let mut greeting_search = Searcher::new(vec![String::from("hi"), String::from("hello")]);
        /// greeting_search.remove_from_vec("hi");
        ///
        /// assert_eq!(vec![String::from("hello")], greeting_search.search_results());
        /// ```
        pub fn remove_from_vec(&mut self, string_to_remove: &str) -> Result<String, String> {
            if self.queried_strings.len() < 2 {
                return Err(String::from("Can't remove last item from search"));
            }
            let result: String;
            let possible_index = self
                .queried_strings
                .iter()
                .position(|element| *element == string_to_remove);
            if let Some(index) = possible_index {
                result = self.queried_strings.remove(index);
            } else {
                return Err(String::from("Couldn't find string in queried_strings"));
            }
            self.reset_cache();
            Ok(result)
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

        // This method updates the search_cache variable based on a new search and the current
        // searchcache
        fn update_cache(&mut self) {
            self.search_cache = self.substring_search(&self.search_string);
        }

        // This method resets the search_cache from queried_strings
        fn reset_cache(&mut self) {
            self.search_cache = self.queried_strings.clone();
            self.update_cache();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::reductivesearch::*;

    #[test]
    fn two_word_test() {
        let mut test_searcher = Searcher::new(vec![String::from("hi"), String::from("hill")]);
        dbg!(test_searcher
            .add_character('h')
            .expect("h should be able to be added to search string"));
        dbg!(test_searcher
            .add_character('i')
            .expect("i should be able to be added to search string"));
        assert_eq!(
            dbg!(test_searcher.search_results()),
            vec![String::from("hi"), String::from("hill")]
        );
    }

    #[test]
    fn three_word_test() {
        let mut test_searcher = Searcher::new(vec![
            String::from("hi"),
            String::from("hill"),
            String::from("hello"),
        ]);
        dbg!(test_searcher
            .add_character('h')
            .expect("h should be able to be added to search string"));
        dbg!(test_searcher
            .add_character('i')
            .expect("i should be able to be added to search string"));
        assert_eq!(
            dbg!(test_searcher.search_results()),
            vec![String::from("hi"), String::from("hill")]
        );
    }

    #[test]
    fn bad_add_test() {
        let mut test_searcher = Searcher::new(vec![
            String::from("hi"),
            String::from("hill"),
            String::from("hello"),
        ]);
        dbg!(test_searcher
            .add_character('h')
            .expect("h should be able to be added to search string"));
        dbg!(test_searcher
            .add_character('a')
            .expect_err("a shouldn't be able to be added to the search"));
        dbg!(test_searcher
            .add_character('i')
            .expect("i should be able to be added to search string"));
        assert_eq!(
            dbg!(test_searcher.search_results()),
            vec![String::from("hi"), String::from("hill")]
        );
    }

    #[test]
    fn remove_test() {
        let mut test_searcher = Searcher::new(vec![
            String::from("hi"),
            String::from("hill"),
            String::from("hello"),
        ]);
        dbg!(test_searcher
            .add_character('h')
            .expect("h should be able to be added to search string"));
        dbg!(test_searcher
            .add_character('i')
            .expect("i should be able to be added to search string"));
        dbg!(test_searcher.remove_character());
        dbg!(test_searcher
            .add_character('e')
            .expect("e should be able to be added to search string"));
        assert_eq!(
            dbg!(test_searcher.search_results()),
            vec![String::from("hello")]
        );
    }

    #[test]
    fn reset_test() {
        let mut test_searcher = Searcher::new(vec![
            String::from("hi"),
            String::from("hill"),
            String::from("hello"),
        ]);
        dbg!(test_searcher
            .add_character('h')
            .expect("h should be able to be added to search string"));
        dbg!(test_searcher
            .add_character('i')
            .expect("i should be able to be added to search string"));
        test_searcher.reset_search();
        assert_eq!(
            dbg!(test_searcher.search_results()),
            vec![
                String::from("hi"),
                String::from("hill"),
                String::from("hello")
            ]
        );
    }

    #[test]
    fn add_to_vec_test() {
        let mut test_searcher = Searcher::new(vec![
            String::from("hi"),
            String::from("hill"),
            String::from("hello"),
        ]);
        dbg!(test_searcher
            .add_character('h')
            .expect("h should be able to be added to search string"));
        dbg!(test_searcher
            .add_character('e')
            .expect("e should be able to be added to search string"));
        test_searcher.add_to_vec(String::from("hev suit"));
        assert_eq!(
            dbg!(test_searcher.search_results()),
            vec![String::from("hello"), String::from("hev suit")]
        );
    }

    #[test]
    fn good_remove_from_vec_test() {
        let mut test_searcher = Searcher::new(vec![
            String::from("hi"),
            String::from("hill"),
            String::from("hello"),
        ]);
        dbg!(test_searcher
            .remove_from_vec("hello")
            .expect("should be able to remove hello from queried_strings"));
        assert_eq!(
            dbg!(test_searcher.search_results()),
            vec![String::from("hi"), String::from("hill")]
        );
    }

    #[test]
    fn bad_remove_from_vec_test() {
        let mut test_searcher = Searcher::new(vec![String::from("hello")]);
        dbg!(test_searcher
            .remove_from_vec("hello")
            .expect_err("removing hello would empty the searcher which isn't allowed"));
        assert_eq!(
            dbg!(test_searcher.search_results()),
            vec![String::from("hello")]
        );
    }
}
