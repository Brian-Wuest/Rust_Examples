use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use core::ops::Add;
use futures::prelude::stream::TryStreamExt;
use sqlx::mssql::MssqlConnectOptions;
use sqlx::mssql::MssqlRow;
use sqlx::Connection;
use sqlx::MssqlConnection;
use sqlx::Row;
use crate::db_endpoints::DBEndPoints;

mod db_endpoints;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    let result = sql_stuff().await;

    match result {
        Ok(result) => {
            dbg!(&result);

            let mut result_string: String = String::from("[");

            let last_item = String::from(result.last().unwrap());

            for current_result in result {
                result_string = result_string.add("\"");
                result_string = result_string.add(&current_result);
                result_string = result_string.add("\"");

                if last_item != current_result {
                    result_string = result_string.add(",");
                }
            }

            result_string = result_string.add("]");

            return HttpResponse::Ok().body(result_string);
        }
        Err(error) => {
            dbg!(error);
            return HttpResponse::Ok().body("Not So Great!");
        }
    }
}

async fn sql_stuff() -> Result<Vec<String>, sqlx::Error> {
    let mut connection_options = MssqlConnectOptions::new();

    // TCP connections are used to connect to the MSSQL server.
    // Make sure to turn this on in the MS SQL Server Configuration Manager
    connection_options = connection_options.database("Northwind");
    connection_options = connection_options.host("localhost");
    connection_options = connection_options.username("sa");
    connection_options = connection_options.password("sql");

    let mut conn: MssqlConnection = MssqlConnection::connect_with(&connection_options).await?;

    // Note: Does not support "Image" types. These would have to be converted to VarBinary(Max) during selection.
    // Assuming the same thing would have to be done for "Text" types as they would be converted to VarChar(Max).
    let mut rows =
        sqlx::query("SELECT EmployeeID, LastName, FirstName FROM Employees").fetch(&mut conn);

    let mut return_value: Vec<String> = Vec::new();

    while let Some(row) = rows.try_next().await? as Option<MssqlRow> {
        let result: String = row.try_get(1)?;

        return_value.push(result);
    }

    Ok(return_value)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    

    HttpServer::new(|| {
        App::new()
            .configure(DBEndPoints::config)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
