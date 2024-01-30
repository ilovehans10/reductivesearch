pub struct Searcher {
    tobesearched: Vec<String>,
    searchstring: String,
}

impl Searcher {
    pub fn new(tobesearched: Vec<String>) -> Searcher {
        Searcher {
            tobesearched,
            searchstring: String::new(),
        }
    }

    pub fn search_results(&self) -> Result<Vec<String>, String> {
        let filteredlist: Vec<String> = self.substring_search(&self.searchstring);
        if filteredlist.is_empty() {
            Err(String::from("String not found"))
        } else {
            Ok(filteredlist)
        }
    }

    pub fn add_search(&mut self, character: char) {
        self.searchstring.push(character)
    }

    fn substring_search(&self, searchstring: &str) -> Vec<String> {
        self.tobesearched
            .clone()
            .into_iter()
            .filter(|element| element.contains(searchstring))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_word_test() {
        let mut testsearcher = Searcher::new(vec![String::from("hi"), String::from("hill")]);
        testsearcher.add_search('h');
        testsearcher.add_search('i');
        assert_eq!(testsearcher.search_results().unwrap().len(), 2);
    }
}
