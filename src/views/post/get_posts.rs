use crate::diesel;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use actix_web::{web, Responder, HttpResponse};

use crate::models::post::post::Post;
use crate::json_serialization::post::Post as Json_Response;
use crate::database::DB;
use crate::schema::post;
use crate::schema::like;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>
}

#[derive(Serialize)]
pub struct Pagination {
    total_pages: i64,
    current_page: i64,
    per_page: i64,
    total_items: i64
}

#[derive(Serialize)]
pub struct JsonResponse {
    data: Vec<Json_Response>,
    pagination: Pagination,
}

pub async fn get_posts(db: DB, query_params: web::Query<PaginationParams>) -> impl Responder {
    let query = r#"
        SELECT
            "post".*,
            reactions.positive_counter,
            reactions.negative_counter
        FROM
            (SELECT
                p.id,
                count(CASE WHEN reaction = true THEN reaction ELSE null END) as positive_counter,
                count(CASE WHEN reaction = false THEN reaction ELSE null END) as negative_counter
            FROM "post" p
            INNER JOIN "like" l ON p.id = l.post_id
            GROUP BY (p.id)) reactions
        RIGHT JOIN "post" ON "post".id = reactions.id
    "#;

    let page = query_params.page.unwrap_or(1);
    let per_page = query_params.per_page.unwrap_or(10);
    let posts = post::table
        .offset((page - 1) * per_page.clone())
        .limit(per_page)
        .load::<Post>(&db.connection);

    match posts {
        Ok(res) => {
            let all_posts = post::table
                .load::<Post>(&db.connection)
                .unwrap();

            let total_items = all_posts.len() as i64;
            let total_pages = (total_items as i64 + per_page - 1) / per_page;
            let pagination = Pagination {
                total_pages,
                total_items,
                current_page: page,
                per_page
            };

            let post_json_response: Vec<Json_Response> = res.into_iter()
                .map(|post| Json_Response {
                    id: Some(post.id),
                    title: post.title,
                    content: post.content,
                    date: Some(post.date),
                }).collect();

            let json_response = JsonResponse {
                data: post_json_response,
                pagination
            };
            return HttpResponse::Ok().json(json_response)
        }
        Err(_) => return HttpResponse::NotFound().into()
    }
}