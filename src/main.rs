use als_task::import_tasks;

fn main() -> Result<(), ()> {
    let mut task_groups = import_tasks("./tasks.json");
    if let Some(group) = task_groups.get_mut("homeless") {
        group.tasks[0].child_tasks.push(Task {
            content: "fsafadsfadsfasdfds".to_owned(),
            ..Default::default()
        })
    }
    println!("{:?}", task_groups);
    // task_groups.insert(
    //     String::from("homeless"),
    //     TaskGroup::new("homeless".to_owned()),
    // );
    // if let Some(group) = task_groups.get_mut("homeless") {
    //     group.add(Task {
    //         content: "fasfdsasf".to_owned(),
    //         ..Default::default()
    //     });
    //     group.add(Task {
    //         content: "content".to_owned(),
    //         ..Default::default()
    //     });
    //     group
    //         .change_state("content".to_owned(), TaskState::Abandon)
    //         .unwrap();
    //     group.delete("content".to_owned()).unwrap();
    // }
    //save_tasks("./tasks.json", task_groups);

    Ok(())
}
