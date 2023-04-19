use super::super::DbPool;

use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use shared::models::tasks::*;
use crate::models::tasks::*;
use crate::models::SuccessResponse;

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[get("/tickets/{ticket_id}/tasks")]
async fn get_tasklist(pool: web::Data<DbPool>, ticket: web::Path<i32>) -> Result<HttpResponse, Error> {
    
    let tasklist = web::block(move || {
        let mut conn = pool.get()?;
        get_ticket_tasks(ticket.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(tasklist))
}

//create task
#[post("/taskgroups/{group_id}/tasks")]
async fn create_task(pool: web::Data<DbPool>, ticket: web::Path<i32>, payload: web::Json<TaskPayload>) -> Result<HttpResponse, Error> {
    let task = web::block(move || {
        let mut conn = pool.get()?;
        add_task(payload.into_inner(), ticket.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(task))
}

//update task (use optional fields)
#[put("/tasks/{task_id}")]
async fn put_task(pool: web::Data<DbPool>, task: web::Path<Uuid>, payload: web::Json<TaskUpdatePayload>) -> Result<HttpResponse, Error> {
    let task = web::block(move || {
        let mut conn = pool.get()?;
        update_task(payload.into_inner(), task.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(task))
}

//delete task
#[delete("/tasks/{task_id}")]
async fn delete_task(pool: web::Data<DbPool>, path: web::Path<Uuid>) -> Result<HttpResponse, Error> {

    let task = path.into_inner();
    let result = web::block(move || {
        let mut conn = pool.get()?;
        db_delete_task(task, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if result > 1 {
        let response = SuccessResponse {
            success: true,
            message: "Task deleted".to_string(),
        };
        Ok(HttpResponse::Ok().json(response))
    } else {
        let response = SuccessResponse {
            success: false,
            message: "Task not found".to_string(),
        };
        Ok(HttpResponse::Ok().json(response))
    }
}


fn get_ticket_tasks(ticket: i32, conn: &mut PgConnection) -> Result<Tasklist, DbError> {
    use crate::schema::tasks::dsl::*;

    let tasklist = tasks
        .filter(ticket_id.eq(ticket))
        .order(order_index.asc())
        .load::<Task>(conn)?
        .into_iter()
        .map(|task| TaskRepresentation {
            task_id: task.task_id,
            ticket_id: task.ticket_id,
            label: task.label,
            is_done: task.is_done,
            order_index: task.order_index,
        })
        .collect::<Vec<_>>();

    let tasklist = Tasklist {
        ticket_id: ticket,
        tasks: tasklist,
    };

    Ok(tasklist)
}

fn add_task(payload: TaskPayload, ticket: i32, conn: &mut PgConnection) -> Result<TaskRepresentation, DbError> {
    use crate::schema::tasks::dsl::*;

    //order index validation
    let index = set_task_order_index(payload.order_index, ticket, conn)?;

    let new_task = NewTask {
        task_id: Uuid::new_v4(),
        label: payload.label,
        order_index: index,
        is_done: payload.is_done,
        ticket_id: ticket,
    };

    let task = diesel::insert_into(tasks)
        .values(&new_task)
        .get_result::<Task>(conn)?;

    let task_representation = TaskRepresentation {
        task_id: task.task_id,
        ticket_id: task.ticket_id,
        label: task.label,
        is_done: task.is_done,
        order_index: task.order_index,
    };

    Ok(task_representation)
}


fn update_task(payload: TaskUpdatePayload, id: Uuid, conn: &mut PgConnection) -> Result<TaskRepresentation, DbError> {
    use crate::schema::tasks::dsl::*;

    if let Some(label_value) = payload.label {
        diesel::update(tasks.filter(task_id.eq(id)))
            .set(label.eq(label_value))
            .execute(conn)?;
    }

    if let Some(is_done_value) = payload.is_done {
        diesel::update(tasks.filter(task_id.eq(id)))
            .set(is_done.eq(is_done_value))
            .execute(conn)?;
    }

    if let Some(order_index_value) = payload.order_index {
        let final_order_index = set_task_order_index(order_index_value, payload.ticket_id , conn)?;
        diesel::update(tasks.filter(task_id.eq(id)))
            .set(order_index.eq(final_order_index))
            .execute(conn)?;
    }

    //get updated task
    let task = tasks
        .filter(task_id.eq(id))
        .first::<Task>(conn)?;

    let task_representation = TaskRepresentation {
        task_id: task.task_id,
        ticket_id: task.ticket_id,
        label: task.label,
        is_done: task.is_done,
        order_index: task.order_index,
    };

    Ok(task_representation)
}

fn db_delete_task(t_id: Uuid, conn: &mut PgConnection) -> Result<usize, DbError> {
    use crate::schema::tasks::dsl::*;

   //delete returning the ticket_id
    let ticket = diesel::delete(tasks.filter(task_id.eq(t_id)))
        .returning(ticket_id)
        .get_result::<i32>(conn);

    match ticket {
        Ok(ticket) => {
            //reindex tasks
            reindex_tasks(ticket, conn)?;
            Ok(1)
        }
        Err(_) => Ok(0),
    }
}

/// This function will check to see if the specified order_index or a task or taskgroup is already used, if so, we need to increment each existing task's order by 1.
fn set_task_order_index(requested_order_index: i32, ticket: i32, conn: &mut PgConnection) -> Result<i32, DbError> {
    use crate::schema::tasks::dsl::*;
    
    let mut retrieved_tasks = tasks
        .filter(ticket_id.eq(ticket))
        .order(order_index.asc())
        .load::<Task>(conn)?;

    //if the requested order_index is already in use, we need to increment each existing task's order by 1, starting from the existing task where order_index = requested_order_index
    if retrieved_tasks.iter().any(|task| task.order_index == requested_order_index) {
        for task in retrieved_tasks.iter_mut() {
            if task.order_index >= requested_order_index {
                task.order_index += 1;
            }
        }

        //now set values in db to new values in retrieved_tasks, matched by task_id
        for task in retrieved_tasks {
            diesel::update(tasks.filter(task_id.eq(task.task_id)))
                .set(order_index.eq(task.order_index))
                .execute(conn)?;
        }

    }


    Ok(requested_order_index)
}

//Re-index tasks so there are no integer gaps (used when deleting or adding in the middle of a list)
fn reindex_tasks(ticket: i32, conn: &mut PgConnection) -> Result<(), DbError> {
    use crate::schema::tasks::dsl::*;

    let mut index = 1;
    let mut retrieved_tasks = tasks
        .filter(ticket_id.eq(ticket))
        .order(order_index.asc())
        .load::<Task>(conn)?;

    for task in retrieved_tasks.iter_mut() {
        task.order_index = index;
        index += 1;
    }

    //now set values in db to new values in retrieved_tasks, matched by task_id
    for task in retrieved_tasks {
        diesel::update(tasks.filter(task_id.eq(task.task_id)))
            .set(order_index.eq(task.order_index))
            .execute(conn)?;
    }

    Ok(())
}
