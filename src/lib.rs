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
    pub fn searchresults(&self) -> Result<Vec<String>, String> {
        let filteredlist: Vec<String> = self.substringsearch(&self.searchstring);
        if filteredlist.is_empty() {
            Err(String::from("String not found"))
        } else {
            Ok(filteredlist)
        }
    }
    pub fn addsearch(&mut self, character: char) {
        self.searchstring.push(character)
    }
    fn substringsearch(&self, searchstring: &str) -> Vec<String> {
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
    fn it_works() {
        let mut testsearcher = Searcher::new(vec![String::from("hi"), String::from("hill")]);
        testsearcher.addsearch('h');
        testsearcher.addsearch('i');
        assert_eq!(testsearcher.searchresults().unwrap().len(), 2);
    }
}
