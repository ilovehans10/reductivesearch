pub struct Searcher {
    tobesearched: Vec<String>,
    pub searchstring: String,
    pub searchcache: Vec<String>,
}

impl Searcher {
    pub fn new(tobesearched: Vec<String>) -> Searcher {
        Searcher {
            searchcache: tobesearched.clone(),
            tobesearched,
            searchstring: String::new(),
        }
    }

    pub fn search_results(&self) -> Result<Vec<String>, String> {
        if !self.searchcache.is_empty() {
            Ok(self.searchcache.clone())
        } else {
            Err(String::from("String not found"))
        }
    }

    pub fn add_character(&mut self, character: char) {
        let mut searchstring: String = self.searchstring.clone();
        searchstring.push(character);
        if !self.substring_search(searchstring.as_str()).is_empty(){
            self.searchstring.push(character);
            self.update_cache();
        }
    }

    pub fn remove_character(&mut self) {
        self.searchstring.pop();
        self.searchcache = self.tobesearched.clone();
    }

    fn substring_search(&self, searchstring: &str) -> Vec<String>{
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_word_test() {
        let mut testsearcher = Searcher::new(vec![String::from("hi"), String::from("hill")]);
        testsearcher.add_character('h');
        testsearcher.add_character('i');
        assert_eq!(testsearcher.search_results().unwrap().len(), 2);
    }

    #[test]
    fn three_word_test() {
        let mut testsearcher = Searcher::new(vec![String::from("hi"), String::from("hill"), String::from("hello")]);
        testsearcher.add_character('h');
        testsearcher.add_character('i');
        assert_eq!(testsearcher.search_results().unwrap().len(), 2);
    }

    #[test]
    fn bad_add_test() {
        let mut testsearcher = Searcher::new(vec![String::from("hi"), String::from("hill"), String::from("hello")]);
        testsearcher.add_character('h');
        testsearcher.add_character('a');
        testsearcher.add_character('i');
        assert_eq!(testsearcher.search_results().unwrap().len(), 2);
    }

    #[test]
    fn remove_test() {
        let mut testsearcher = Searcher::new(vec![String::from("hi"), String::from("hill"), String::from("hello")]);
        testsearcher.add_character('h');
        testsearcher.add_character('i');
        testsearcher.remove_character();
        testsearcher.add_character('e');
        assert_eq!(testsearcher.search_results().unwrap().len(), 1);
    }
}
