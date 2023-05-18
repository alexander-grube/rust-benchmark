mod config {
    use serde::Deserialize;
    #[derive(Debug, Default, Deserialize)]
    pub struct ExampleConfig {
        pub server_addr: String,
        pub pg: deadpool_postgres::Config,
    }
}

mod models {
    use serde::{Deserialize, Serialize};
    use tokio_pg_mapper_derive::PostgresMapper;

    #[derive(Debug, Serialize, Deserialize, PostgresMapper)]
    #[pg_mapper(table = "person")]
    pub struct Person {
        pub id: i32,
        pub name: String,
        pub job: String,
        pub is_adult: bool,
        pub favorite_number: i16,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct NewPerson {
        pub name: String,
        pub job: String,
        pub is_adult: bool,
        pub favorite_number: i16,
    }
}

mod db {
    use deadpool_postgres::Client;
    use tokio_pg_mapper::FromTokioPostgresRow;

    use crate::models::{Person, NewPerson};

    pub async fn select_all_persons(client: &Client) -> Vec<Person> {
        let _stmt = "SELECT * FROM person ORDER BY ID ASC";
        let stmt = client.prepare(&_stmt).await.unwrap();

        return client
            .query(&stmt, &[])
            .await
            .unwrap()
            .iter()
            .map(|row| Person::from_row_ref(row).unwrap())
            .collect::<Vec<Person>>();
    }

    pub async fn select_persons_limit(client: &Client, limit: i64) -> Vec<Person> {
        let _stmt = "SELECT * FROM person ORDER BY ID ASC LIMIT $1";
        let stmt = client.prepare(&_stmt).await.unwrap();

        return client
            .query(&stmt, &[&limit])
            .await
            .unwrap()
            .iter()
            .map(|row| Person::from_row_ref(row).unwrap())
            .collect::<Vec<Person>>();
    }

    pub async fn select_person_by_id(client: &Client, id: i32) -> Person {
        let _stmt = "SELECT * FROM person WHERE id = $1";
        let stmt = client.prepare(&_stmt).await.unwrap();

        return client
            .query(&stmt, &[&id])
            .await
            .unwrap()
            .iter()
            .map(|row| Person::from_row_ref(row).unwrap())
            .collect::<Vec<Person>>()
            .pop()
            .unwrap();
    }

    pub async fn insert_person(client: &Client, person: &NewPerson) -> Person {
        let _stmt = "INSERT INTO person (name, job, is_adult, favorite_number) VALUES ($1, $2, $3, $4) RETURNING id, name, job, is_adult, favorite_number";
        let stmt = client.prepare(&_stmt).await.unwrap();

        return client
            .query(&stmt, &[&person.name, &person.job, &person.is_adult, &person.favorite_number])
            .await
            .unwrap()
            .iter()
            .map(|row| Person::from_row_ref(row).unwrap())
            .collect::<Vec<Person>>()
            .pop()
            .unwrap();
    }
}

mod handlers {
    use actix_web::{web, Error, HttpResponse, get, post};
    use deadpool_postgres::{Client, Pool};

    use crate::db;

    #[get("/person")]
    pub async fn get_all_persons(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
        let client: Client = db_pool.get().await.unwrap();
        let timer_start = std::time::Instant::now();
        let goods = db::select_all_persons(&client).await;
        let timer_end = std::time::Instant::now();
        println!("Elapsed time DB: {:?}", timer_end - timer_start);
        let json_timer_start = std::time::Instant::now();
        let json = serde_json::to_string(&goods).unwrap();
        let json_timer_end = std::time::Instant::now();
        println!("Elapsed time JSON: {:?}", json_timer_end - json_timer_start);
        return Ok(HttpResponse::Ok().append_header(("Content-Type", "application/json")).body(json));
    }

    #[get("/person/limit/{limit}")]
    pub async fn get_persons_limit(
        db_pool: web::Data<Pool>,
        path: web::Path<(i64,)>,
    ) -> Result<HttpResponse, Error> {
        let client: Client = db_pool.get().await.unwrap();
        let timer_start = std::time::Instant::now();
        let goods = db::select_persons_limit(&client, path.0).await;
        let timer_end = std::time::Instant::now();
        println!("Elapsed time DB: {:?}", timer_end - timer_start);
        let json_timer_start = std::time::Instant::now();
        let json = serde_json::to_string(&goods).unwrap();
        let json_timer_end = std::time::Instant::now();
        println!("Elapsed time JSON: {:?}", json_timer_end - json_timer_start);
        return Ok(HttpResponse::Ok().append_header(("Content-Type", "application/json")).body(json));
    }

    #[get("/person/{id}")]
    pub async fn get_person_by_id(
        db_pool: web::Data<Pool>,
        path: web::Path<(i32,)>,
    ) -> Result<HttpResponse, Error> {
        let client: Client = db_pool.get().await.unwrap();
        let timer_start = std::time::Instant::now();
        let goods = db::select_person_by_id(&client, path.0).await;
        let timer_end = std::time::Instant::now();
        println!("Elapsed time DB: {:?}", timer_end - timer_start);
        let json_timer_start = std::time::Instant::now();
        let json = serde_json::to_string(&goods).unwrap();
        let json_timer_end = std::time::Instant::now();
        println!("Elapsed time JSON: {:?}", json_timer_end - json_timer_start);
        return Ok(HttpResponse::Ok().append_header(("Content-Type", "application/json")).body(json));
    }

    #[post("/person")]
    pub async fn post_person(
        db_pool: web::Data<Pool>,
        json: web::Json<crate::models::NewPerson>,
    ) -> Result<HttpResponse, Error> {
        let client: Client = db_pool.get().await.unwrap();
        let timer_start = std::time::Instant::now();
        let goods = db::insert_person(&client, &json).await;
        let timer_end = std::time::Instant::now();
        println!("Elapsed time DB: {:?}", timer_end - timer_start);
        let json_timer_start = std::time::Instant::now();
        let json = serde_json::to_string(&goods).unwrap();
        let json_timer_end = std::time::Instant::now();
        println!("Elapsed time JSON: {:?}", json_timer_end - json_timer_start);
        return Ok(HttpResponse::Ok().append_header(("Content-Type", "application/json")).body(json));
    }
}

use ::config::Config;
use actix_web::{web, App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use handlers::{get_all_persons, get_person_by_id, post_person, get_persons_limit};
use tokio_postgres::NoTls;

use crate::config::ExampleConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: ExampleConfig = config_.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(get_all_persons)
            .service(get_persons_limit)
            .service(get_person_by_id)
            .service(post_person)
    })
        .bind(config.server_addr.clone())?
        .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}