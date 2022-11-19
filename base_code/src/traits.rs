#[allow(dead_code)]
pub trait GetInformation {
  fn get_name(&self) -> &String;
  fn get_age(&self) -> u32;
}

#[derive(Debug)]
pub struct Student {
    pub name: String,
    pub age: u32,
}

impl GetInformation for Student {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}

#[derive(Debug)]
pub struct Teacher {
    pub name: String,
    pub age: u32,
    pub subject: String,
}

impl GetInformation for Teacher {

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}

/**********trait 作为参数使用***********/
#[allow(dead_code)]
fn print_information(items: impl GetInformation) {
    println!("name = {}", items.get_name());
    println!("age = {}", items.get_age())
}

/**********trait 默认实现***********/

#[cfg(test)]
mod test {
    use crate::traits::print_information;

    use super::{Student, Teacher};

    #[test]
    pub fn hello() {
       let st = Student {name: "xiaoming".to_string(), age: 23};
       let te = Teacher {name: "zhanglaoshi".to_string(), age: 35, subject: "Math".to_string()};
       
       print_information(st);
       print_information(te);
    //    println!("Teacher {:?}", te);
    //    println!("Studengt {:?}", st);
    }

}