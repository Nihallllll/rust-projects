#[derive(Debug)]
struct Todo {
    id: u8,
    name: String,
    description: String,
    status: Status,
}

#[derive(Debug)]
enum Status {
    Pending,
    Done,
}

enum Error {
    TaskNotRemoved,
    TaskNotUpdated,
}

struct TodoList {
    tasks: Vec<Todo>,
    next_id: u8,
}

impl TodoList {
    fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add(&mut self, name: &str, desc: &str) {
        let todo = Todo {
            id: self.next_id,
            name: name.to_string(),
            description: desc.to_string(),
            status: Status::Pending,
        };

        self.tasks.push(todo);
        self.next_id += 1;
    }

    fn completed(&mut self, id: u8) -> Result<(), Error> {
        match self.tasks.iter_mut().find(|t| t.id == id) {
            Some(task) => {
                task.status = Status::Done;
                Ok(())
            }
            None => Err(Error::TaskNotUpdated),
        }
    }

    fn remove_task(&mut self, id: u8) -> Result<(), Error> {
        if let Some(index) = self.tasks.iter().position(|x| x.id == id) {
            self.tasks.remove(index);
            Ok(())
        } else {
            Err(Error::TaskNotRemoved)
        }
    }

    fn get_all(&self) {
        for n in &self.tasks {
            println!(
                "Id: {} | Name: {} | Status: {:?} | Desc: {}",
                n.id, n.name, n.status, n.description
            );
        }
    }
}
