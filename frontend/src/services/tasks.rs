use crate::types::{Error, SuccessResponse};
use shared::models::tasks::*;
use uuid::Uuid;

use super::{request_delete, request_get, request_post, request_put};

//get tasklist for ticket (/tickets/{ticket_id}/tasks
pub async fn get_tasklist(ticket_id: i32) -> Result<Tasklist, Error> {
    let tasklist: Tasklist =
        request_get::<Tasklist>(format!("/tickets/{}/tasks", ticket_id)).await?;

    Ok(tasklist)
}

//create task group (/tickets/{ticket_id}/taskgroups)
pub async fn create_taskgroup(
    ticket_id: i32,
    taskgroup: TaskGroupNewPayload,
) -> Result<TaskGroupRepresentation, Error> {
    let taskgroup: TaskGroupRepresentation =
        request_post::<TaskGroupNewPayload, TaskGroupRepresentation>(
            format!("/tickets/{}/taskgroups", ticket_id),
            taskgroup,
        )
        .await?;

    Ok(taskgroup)
}

//create task (/taskgroups/{group_id}/tasks)
pub async fn create_task(group_id: Uuid, task: TaskNewPayload) -> Result<TaskRepresentation, Error> {
    let task: TaskRepresentation = request_post::<TaskNewPayload, TaskRepresentation>(
        format!("/taskgroups/{}/tasks", group_id),
        task,
    )
    .await?;

    Ok(task)
}

//update task (use optional fields) (/tasks/{task_id})
pub async fn update_task(
    task_id: Uuid,
    task: TaskUpdatePayload,
) -> Result<TaskRepresentation, Error> {
    let task: TaskRepresentation =
        request_put::<TaskUpdatePayload, TaskRepresentation>(format!("/tasks/{}", task_id), task)
            .await?;

    Ok(task)
}

//delete task (/tasks/{task_id})
pub async fn delete_task(task_id: Uuid, group_id: Uuid) -> Result<SuccessResponse, Error> {
    let response: SuccessResponse =
        request_delete::<SuccessResponse>(format!("/taskgroups/{}/tasks/{}", group_id, task_id))
            .await?;

    Ok(response)
}

//delete taskgroup (/taskgroups/{group_id})
pub async fn delete_taskgroup(group_id: &Uuid) -> Result<SuccessResponse, Error> {
    let response: SuccessResponse =
        request_delete::<SuccessResponse>(format!("/taskgroups/{}", group_id)).await?;

    Ok(response)
}

//get the tasks of a taskgroup
pub async fn get_group_tasks(group_id: Uuid) -> Result<Vec<TaskRepresentation>, Error> {
    let tasks: Vec<TaskRepresentation> =
        request_get::<Vec<TaskRepresentation>>(format!("/taskgroups/{}/tasks", group_id)).await?;

    Ok(tasks)
}


//update taskgroup label
pub async fn update_taskgroup(
    group_id: Uuid,
    taskgroup: TaskGroupUpdatePayload,
) -> Result<TaskGroupRepresentation, Error> {
    let taskgroup: TaskGroupRepresentation =
        request_put::<TaskGroupUpdatePayload, TaskGroupRepresentation>(
            format!("/taskgroups/{}", group_id),
            taskgroup,
        )
        .await?;

    Ok(taskgroup)
}