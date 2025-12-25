struct Student {
   math : u8,
   science : u8,
   english : u8
}
struct Class {
    students : Vec<Student>
}
const MAX_MARKS : u8 = 100;

impl Class {
    fn new()->Self{
        Self {
            students : Vec::new()
        }
    }

    fn add_marks(&mut self,mm : u8 , sm :u8 , em : u8) {
        let  marks :Student = Student { math:mm , science: sm, english: em };
        self.students.push(marks);
    }
    
    fn class_average(&self) -> u8 {
        self.students.iter().map(|x| x.english )
    }
}