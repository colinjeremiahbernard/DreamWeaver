use crate::services::{
    writer_agent,
    designer_agent,
    researcher_agent,
    critic_agent,
};

// 1. Added the return type here: (String, String, String, String)
pub async fn run_workflow(project: String) -> (String, String, String, String) {

    let story = writer_agent::generate(&project).await;
    let visuals = designer_agent::generate(&project).await;
    let research = researcher_agent::generate(&project).await;
    let review = critic_agent::evaluate(&project).await;

    println!("Story: {}", story);
    println!("Visuals: {}", visuals);
    println!("Research: {}", research);
    println!("Review: {}", review);

    // 2. Return the values at the end of the function
    (story, visuals, research, review)
}