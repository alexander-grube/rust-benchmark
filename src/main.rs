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
    use tokio_postgres::Row;

    #[derive(Debug, Serialize, Deserialize, PostgresMapper)]
    #[pg_mapper(table = "person")]
    pub struct Person {
        pub id: i32,
        pub name: String,
        pub job: String,
        pub is_adult: bool,
        pub favorite_number: i16,
    }

    impl From<&Row> for Person {
        fn from(row: &Row) -> Self {
            Self {
                id: row.get("p_id"),
                name: row.get("p_name"),
                job: row.get("p_job"),
                is_adult: row.get("p_is_adult"),
                favorite_number: row.get("p_favorite_number"),
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PersonDOD {
        pub names: Vec<String>,
        pub jobs: Vec<String>,
        pub is_adults: Vec<bool>,
        pub favorite_numbers: Vec<i16>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct NewPerson {
        pub name: String,
        pub job: String,
        pub is_adult: bool,
        pub favorite_number: i16,
    }

    #[derive(Debug, Serialize, Deserialize, PostgresMapper)]
    #[pg_mapper(table = "organization")]
    pub struct Organization {
        pub id: i32,
        pub name: String,
        pub address: String,
        pub phone: String,
        pub ceo_id: i32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct NewOrganization {
        pub name: String,
        pub address: String,
        pub phone: String,
        pub ceo_id: i32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct OrganizationWithCeo {
        pub id: i32,
        pub name: String,
        pub address: String,
        pub phone: String,
        pub ceo: Person,
    }
}

mod db {
    use deadpool_postgres::Client;
    use tokio_pg_mapper::FromTokioPostgresRow;

    use crate::models::{NewOrganization, NewPerson, Organization, Person};

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

    pub async fn select_all_organizations(client: &Client) -> Vec<Organization> {
        let _stmt = "SELECT * FROM organization ORDER BY ID ASC";
        let stmt = client.prepare(&_stmt).await.unwrap();

        return client
            .query(&stmt, &[])
            .await
            .unwrap()
            .iter()
            .map(|row| Organization::from_row_ref(row).unwrap())
            .collect::<Vec<Organization>>();
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
        let _stmt = "CALL get_person_by_id($1, $2, $3, $4, $5)";
        let stmt = client.prepare(&_stmt).await.unwrap();

        return client
            .query(
                &stmt,
                &[
                    &id,
                    &None::<String>,
                    &None::<String>,
                    &None::<bool>,
                    &None::<i16>,
                ],
            )
            .await
            .unwrap()
            .iter()
            .map(|row| Person::from(row))
            .collect::<Vec<Person>>()
            .pop()
            .unwrap();
    }

    pub async fn select_ceo_of_organisation(client: &Client, id: i32) -> Person {
        let _stmt =
            "SELECT * FROM person WHERE id = (SELECT ceo_id FROM organization WHERE id = $1)";
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
        let _stmt = "CALL insert_person($1, $2, $3, $4, $5)";
        let stmt = client.prepare(&_stmt).await.unwrap();

        return client
            .query(
                &stmt,
                &[
                    &person.name,
                    &person.job,
                    &person.is_adult,
                    &person.favorite_number,
                    &None::<i32>,
                ],
            )
            .await
            .unwrap()
            .iter()
            .map(|row| Person::from(row))
            .collect::<Vec<Person>>()
            .pop()
            .unwrap();
    }

    pub async fn insert_organization(
        client: &Client,
        organization: &NewOrganization,
    ) -> Organization {
        let _stmt = "INSERT INTO organization (name, address, phone, ceo_id) VALUES ($1, $2, $3, $4) RETURNING id, name, address, phone, ceo_id";
        let stmt = client.prepare(&_stmt).await.unwrap();

        return client
            .query(
                &stmt,
                &[
                    &organization.name,
                    &organization.address,
                    &organization.phone,
                    &organization.ceo_id,
                ],
            )
            .await
            .unwrap()
            .iter()
            .map(|row| Organization::from_row_ref(row).unwrap())
            .collect::<Vec<Organization>>()
            .pop()
            .unwrap();
    }
}

mod handlers {
    use actix_web::{get, post, web, Error, HttpResponse};
    use deadpool_postgres::{Client, Pool};

    use crate::{db, models::OrganizationWithCeo};

    #[get("/benchmark/person")]
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
        return Ok(HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(json));
    }

    #[get("/benchmark/oop/person")]
    pub async fn oop_person_benchmark() -> Result<HttpResponse, Error> {
        let timer_start = std::time::Instant::now();
        let mut persons = Vec::<crate::models::Person>::new();
        for i in 0..100_000_000 {
            persons.push(crate::models::Person {
                id: i,
                name: "John".to_string(),
                job: "Programmer".to_string(),
                is_adult: true,
                favorite_number: 27,
            });
        }
        // increment favorite number
        for person in &mut persons {
            person.favorite_number += 1;
        }
        let timer_end = std::time::Instant::now();
        println!("Elapsed time OOP: {:?}", timer_end - timer_start);
        return Ok(HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body("OOP benchmark done!"));
    }

    #[get("/benchmark/dod/person")]
    pub async fn dod_person_benchmark() -> Result<HttpResponse, Error> {
        let timer_start = std::time::Instant::now();
        let mut names = Vec::<String>::new();
        let mut jobs = Vec::<String>::new();
        let mut is_adults = Vec::<bool>::new();
        let mut favorite_numbers = Vec::<i16>::new();
        for _i in 0..100_000_000 {
            names.push("John".to_string());
            jobs.push("Programmer".to_string());
            is_adults.push(true);
            favorite_numbers.push(27);
        }
        // increment favorite number
        for i in 0..100_000_000 {
            favorite_numbers[i] += 1;
        }
        let timer_end = std::time::Instant::now();
        println!("Elapsed time DOD: {:?}", timer_end - timer_start);
        return Ok(HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body("DOD benchmark done!"));    
    }

    #[get("/benchmark/organization")]
    pub async fn get_all_organizations(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
        let client: Client = db_pool.get().await.unwrap();
        let timer_start = std::time::Instant::now();
        let mut orgs_with_ceo = Vec::<OrganizationWithCeo>::new();
        let orgs = db::select_all_organizations(&client).await;
        for org in &orgs {
            let ceo = db::select_ceo_of_organisation(&client, org.id).await;
            println!("CEO of {}: {}", org.name, ceo.name);
            orgs_with_ceo.push(OrganizationWithCeo {
                id: org.id,
                name: org.name.clone(),
                address: org.address.clone(),
                phone: org.phone.clone(),
                ceo: ceo,
            });
        }
        let timer_end = std::time::Instant::now();
        println!("Elapsed time DB: {:?}", timer_end - timer_start);
        let json_timer_start = std::time::Instant::now();
        let json = serde_json::to_string(&orgs_with_ceo).unwrap();
        let json_timer_end = std::time::Instant::now();
        println!("Elapsed time JSON: {:?}", json_timer_end - json_timer_start);
        return Ok(HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(json));
    }

    #[get("/benchmark/person/limit/{limit}")]
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
        return Ok(HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(json));
    }

    #[get("/benchmark/person/{id}")]
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
        return Ok(HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(json));
    }

    #[post("/benchmark/person")]
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
        return Ok(HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(json));
    }

    #[post("/benchmark/organization")]
    pub async fn post_organization(
        db_pool: web::Data<Pool>,
        json: web::Json<crate::models::NewOrganization>,
    ) -> Result<HttpResponse, Error> {
        let client: Client = db_pool.get().await.unwrap();
        let timer_start = std::time::Instant::now();
        let goods = db::insert_organization(&client, &json).await;
        let timer_end = std::time::Instant::now();
        println!("Elapsed time DB: {:?}", timer_end - timer_start);
        let json_timer_start = std::time::Instant::now();
        let json = serde_json::to_string(&goods).unwrap();
        let json_timer_end = std::time::Instant::now();
        println!("Elapsed time JSON: {:?}", json_timer_end - json_timer_start);
        return Ok(HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(json));
    }
}

use ::config::Config;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use handlers::{
    get_all_organizations, get_all_persons, get_person_by_id, get_persons_limit, post_organization,
    post_person, dod_person_benchmark, oop_person_benchmark
};
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
            .service(get_all_organizations)
            .service(get_persons_limit)
            .service(get_person_by_id)
            .service(post_person)
            .service(post_organization)
            .service(oop_person_benchmark)
            .service(dod_person_benchmark)
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, web, App};
    use serde_json::json;

    #[actix_rt::test]
    async fn test_post_person() {
        dotenv().ok();
        env_logger::init();

        let config_ = Config::builder()
            .add_source(::config::Environment::default())
            .build()
            .unwrap();

        let config: ExampleConfig = config_.try_deserialize().unwrap();

        let pool = config.pg.create_pool(None, NoTls).unwrap();
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(post_person),
        )
        .await;

        let payload = json!({
            "name": "John",
            "job": "Programmer",
            "is_adult": true,
            "favorite_number": 27
        });

        let req = test::TestRequest::post()
            .uri("/person")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
