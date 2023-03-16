use super::super::DbPool;

use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use shared::models::tasks::*;
use crate::models::tasks::*;

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

//create task group
#[post("/tickets/{ticket_id}/taskgroups")]
async fn create_taskgroup(pool: web::Data<DbPool>, ticket: web::Path<i32>, payload: web::Json<TaskGroupPayload>) -> Result<HttpResponse, Error> {
    let taskgroup = web::block(move || {
        let mut conn = pool.get()?;
        add_taskgroup(payload.into_inner(), ticket.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(taskgroup))
}

//create task
#[post("/taskgroups/{group_id}/tasks")]
async fn create_task(pool: web::Data<DbPool>, group: web::Path<Uuid>, payload: web::Json<TaskPayload>) -> Result<HttpResponse, Error> {
    let task = web::block(move || {
        let mut conn = pool.get()?;
        add_task(payload.into_inner(), group.into_inner(), &mut conn)
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
async fn delete_task(pool: web::Data<DbPool>, task: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    let task = web::block(move || {
        let mut conn = pool.get()?;
        db_delete_task(task.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(task))
}

//delete task group
#[delete("/taskgroups/{group_id}")]
async fn delete_taskgroup(pool: web::Data<DbPool>, group: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    let taskgroup = web::block(move || {
        let mut conn = pool.get()?;
        db_delete_taskgroup(group.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(taskgroup))
}


fn get_ticket_tasks(ticket: i32, conn: &mut PgConnection) -> Result<Tasklist, DbError> {
    use crate::schema::task_groups::dsl::*;

    let groups = task_groups
        .filter(ticket_id.eq(ticket))
        .order(order_index)
        .load::<TaskGroup>(conn)?;

    let mut task_groups_with_tasks = Vec::new();

    for group in groups {
        let group_tasks = get_group_tasks(group.group_id, conn)?;
        let group_with_tasks = TaskGroupRepresentation {
            group_id: group.group_id,
            label: group.label,
            order_index: group.order_index,
            tasks: group_tasks,
        };
        task_groups_with_tasks.push(group_with_tasks);
    }

    //put in tasklist
    let tasklist = Tasklist {
        ticket_id: ticket,
        task_groups: task_groups_with_tasks,
    };

    Ok(tasklist)
}

//Get tasks for a group (representation)
fn get_group_tasks(group: Uuid, conn: &mut PgConnection) -> Result<Vec<TaskRepresentation>, diesel::result::Error> {
    use crate::schema::tasks::dsl::*;

    let group_tasks = tasks
        .filter(group_id.eq(group))
        .order(order_index)
        .load::<Task>(conn)?;

    let mut task_representations = Vec::new();

    for t in group_tasks {
        let task_representation = TaskRepresentation {
            task_id: t.task_id,
            label: t.label,
            is_done: t.is_done,
            order_index: t.order_index,
        };
        task_representations.push(task_representation);
    }

    Ok(task_representations)
}

fn add_taskgroup(payload: TaskGroupPayload, ticket: i32, conn: &mut PgConnection) -> Result<TaskGroupRepresentation, DbError> {
    use crate::schema::task_groups::dsl::*;

    let new_group = NewTaskGroup {
        group_id: Uuid::new_v4(),
        label: payload.label,
        order_index: payload.order_index,
        ticket_id: ticket,
    };

    let group = diesel::insert_into(task_groups)
        .values(&new_group)
        .get_result::<TaskGroup>(conn)?;

    let group_representation = TaskGroupRepresentation {
        group_id: group.group_id,
        label: group.label,
        order_index: group.order_index,
        tasks: Vec::new(),
    };

    Ok(group_representation)
}

fn add_task(payload: TaskPayload, group: Uuid, conn: &mut PgConnection) -> Result<TaskRepresentation, DbError> {
    use crate::schema::tasks::dsl::*;

    let new_task = NewTask {
        label: payload.label,
        order_index: payload.order_index,
        is_done: payload.is_done,
        group_id: group,
    };

    let task = diesel::insert_into(tasks)
        .values(&new_task)
        .get_result::<Task>(conn)?;

    let task_representation = TaskRepresentation {
        task_id: task.task_id,
        label: task.label,
        is_done: task.is_done,
        order_index: task.order_index,
    };

    Ok(task_representation)
}

fn update_task(payload: TaskUpdatePayload, id: Uuid, conn: &mut PgConnection) -> Result<TaskRepresentation, DbError> {
    use crate::schema::tasks::dsl::*;

    //
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
        diesel::update(tasks.filter(task_id.eq(id)))
            .set(order_index.eq(order_index_value))
            .execute(conn)?;
    }

    //get updated task
    let task = tasks
        .filter(task_id.eq(id))
        .first::<Task>(conn)?;

    let task_representation = TaskRepresentation {
        task_id: task.task_id,
        label: task.label,
        is_done: task.is_done,
        order_index: task.order_index,
    };

    Ok(task_representation)
}

fn db_delete_task(id: Uuid, conn: &mut PgConnection) -> Result<TaskRepresentation, DbError> {
    use crate::schema::tasks::dsl::*;

    let task = tasks
        .filter(task_id.eq(id))
        .first::<Task>(conn)?;

    diesel::delete(tasks.filter(task_id.eq(task_id)))
        .execute(conn)?;

    let task_representation = TaskRepresentation {
        task_id: task.task_id,
        label: task.label,
        is_done: task.is_done,
        order_index: task.order_index,
    };

    Ok(task_representation)
}

fn db_delete_taskgroup(id: Uuid, conn: &mut PgConnection) -> Result<TaskGroupRepresentation, DbError> {
    use crate::schema::task_groups::dsl::*;

    let group = task_groups
        .filter(group_id.eq(id))
        .first::<TaskGroup>(conn)?;

    diesel::delete(task_groups.filter(group_id.eq(group_id)))
        .execute(conn)?;

    let group_representation = TaskGroupRepresentation {
        group_id: group.group_id,
        label: group.label,
        order_index: group.order_index,
        tasks: Vec::new(),
    };

    Ok(group_representation)
}