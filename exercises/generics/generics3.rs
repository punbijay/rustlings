pub struct Student <T: std::fmt::Display> {
    pub grade: T,
    pub name: String,
    pub age: u8,
}

impl <T: std::fmt::Display> Student<T> {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - is on grade {}",
            &self.name, self.age, self.grade
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn student_details() {
        let this_student = Student {
            grade: 1,
            name: "John Wick".to_string(),
            age: 52,
        };
        assert_eq!(
            student_details.print(),
            "John Wick (52) - is on grade 1",
        );
    }
}
