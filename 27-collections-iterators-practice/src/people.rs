#[derive(Debug, PartialEq, Eq)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    pub fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}

pub fn sort_by_age_descending(persons: Vec<Person>) -> Vec<Person> {
    let mut sorted_persons = persons;
    sorted_persons.sort_by(|a, b| b.age.cmp(&a.age));
    sorted_persons
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_people_from_oldest_to_youngest() {
        let persons = vec![
            Person::new("Alice", 30),
            Person::new("Bob", 25),
            Person::new("Charlie", 35),
        ];

        let sorted = sort_by_age_descending(persons);
        let names = sorted.iter().map(|person| person.name.as_str()).collect::<Vec<_>>();

        assert_eq!(names, vec!["Charlie", "Alice", "Bob"]);
    }
}
