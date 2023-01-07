use crate::models::TaskGroup;


impl TaskGroup {
    pub fn new(group_name: String) -> Self {
        Self {
            group_name,
            tasks: vec![],
        }
    }
}
