    pub fn build_proverb(list: &[&str]) -> String {
        let last_verb = format!("And all for the want of a {}.",
                                list.first().unwrap_or_else(|| &""));
        if list.len() == 1{
            return last_verb;
        }
        list.iter()
            .zip(list.iter().skip(1))
            .map(|(first, last)|{
                let mut text = format!("For want of a {first} the {last} was lost.\n");
                if last == list.last().unwrap(){
                    text += last_verb.as_str();
                }
                text
            })
            .collect()
    }