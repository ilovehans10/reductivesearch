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
        if self.tobesearched.contains(&self.searchstring) {
            Ok(
                self.tobesearched
                    .clone()
                    .into_iter()
                    .filter(|element| element.contains(&self.searchstring))
                    .collect(),
            )
        } else {
            Err(String::from("String not found"))
        }
    }
    pub fn addsearch(&mut self, character: char) {
        self.searchstring.push(character)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut testsearcher = Searcher::new(vec![String::from("hi")]);
        testsearcher.addsearch('h');
        testsearcher.addsearch('i');
        assert_eq!(testsearcher.searchresults().unwrap().len(), 1);
    }
}
