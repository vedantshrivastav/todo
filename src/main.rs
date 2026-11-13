#[derive(Debug)]
enum Status {
    Pending,
    Done,
}
struct Task {
    id: u32,
    title: String,
    status: Status,
}

//functions
fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let new_task = add_task(&mut tasks, String::from("Start learning rust"));
}

fn add_task(tasks: &mut Vec<Task>, title: String) -> u32 {
    // compute the id
    let mut new_task_id: u32;
    if tasks.len() == 0 {
        new_task_id = 1;
    } else {
        new_task_id = tasks.iter().map(|t| t.id).max().unwrap() + 1;
    }
    // Build the task
    let task = Task {
        id: new_task_id,
        title: String::from(title),
        status: Status::Pending,
    };
    tasks.push(task);
    return new_task_id;
}

fn complete_task(tasks: &mut Vec<Task>, id: u32) -> Result<(), String> {
    match tasks.iter_mut().find(|t| t.id == id) {
        Some(task) => {
            task.status = Status::Done;
            Ok(())
        }
        None => Err(format!("Task {} not found", id)),
    }
}

fn delete_task(tasks: &mut Vec<Task>, id: u32) -> Result<Task, String> {
    match tasks.iter().position(|t| t.id == id) {
        Some(index) => Ok(tasks.remove(index)),
        None => Err(format!("Task {} not found", id)),
    }
}
fn list_tasks(tasks: &[Task]) {
    for task in tasks {
        println!("{}. {} [{:?}]", task.id, task.title, task.status);
    }
}
