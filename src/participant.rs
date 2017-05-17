use util::str_to_bool;

pub struct Participant {
    pub full_name: String,
    pub is_judge: bool,
    pub category: String,
}

impl Participant {
    pub fn new(name: String,
               is_judge: String,
               category: String)
               -> Participant {
        Participant {
            full_name: name,
            is_judge: str_to_bool(is_judge),
            category: category,
        }
    }
    pub fn from_tuple((name,
                       is_judge,
                       category):
                      (String,
                       String,
                       String))
                      -> Participant {
        Participant {
            full_name: name,
            is_judge: str_to_bool(is_judge),
            category: category,
        }
    }
}
