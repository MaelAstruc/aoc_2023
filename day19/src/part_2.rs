
pub(crate) fn part_2(input: &str) {
    let (workflows, ratings) = parse_input(input);

    let mut total: usize = 0;

    let mut name = "in".to_string();

    for rating in ratings {
        let mut name = "in".to_string();
        //println!("{rating:?}");
        loop {
            match workflows.get(&name).unwrap().includes(&rating) {
                WorkflowResult::Accepted => {
                    total += rating.sum();
                    break;
                },
                WorkflowResult::Rejected => break,
                WorkflowResult::Workflow(new_name) => name = new_name,
                WorkflowResult::Pass => unreachable!("Shouldn't pass here"),
            }
        }
        //println!("{total}");
    }
    
    println!("{total}");
}
