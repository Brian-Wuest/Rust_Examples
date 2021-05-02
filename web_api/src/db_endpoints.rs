use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, post, web::{self, Data}};
use async_std::task;
use core::ops::Add;
use futures::executor;
use futures::poll;
use futures::{FutureExt, TryStreamExt};
use sqlx::Connection;
use sqlx::Error;
use sqlx::MssqlConnection;
use sqlx::{
    mssql::{MssqlConnectOptions, MssqlRow},
    Row,
};
use std::{borrow::{Borrow, BorrowMut}, ops::{Deref, DerefMut}, time::Duration};

#[derive(Debug)]
pub struct DBEndPoints {
    dbConnection: MssqlConnection,
}

impl DBEndPoints {
    pub fn new() -> Self {
        let mut connection_options = MssqlConnectOptions::new();

        // TCP connections are used to connect to the MSSQL server.
        // Make sure to turn this on in the MS SQL Server Configuration Manager
        connection_options = connection_options.database("Northwind");
        connection_options = connection_options.host("localhost");
        connection_options = connection_options.username("sa");
        connection_options = connection_options.password("sql");

        let database_connection: Result<MssqlConnection, Error> =
            executor::block_on(MssqlConnection::connect_with(&connection_options));

        let connection = database_connection.unwrap();
        DBEndPoints {
            dbConnection: connection,
        }
    }

    pub fn config(cfg: &mut web::ServiceConfig) {
        let endpoints = DBEndPoints::new();

        cfg.app_data(endpoints);

        cfg.service(web::resource("/app").route(web::get().to(DBEndPoints::somethingCool)));
    }

    pub async fn somethingCool(request: HttpRequest) -> impl Responder {
        let connectionInfo = request.app_data::<Data<DBEndPoints>>();
        let mut someResults: Vec<String> = Vec::new();

        if (connectionInfo.is_some()) {
            let mut unwrapped_connection_info = connectionInfo.unwrap();
            let sql_results = unwrapped_connection_info.sql_stuff().await.unwrap();

            if !sql_results.is_empty() {
                for result in sql_results {
                    someResults.push(result.to_owned());
                }
            }
        }

        HttpResponse::Ok().body("Sweet")
    }

    async fn manual_hello(&mut self) -> impl Responder {
        let result = self.sql_stuff().await;

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

    async fn sql_stuff(&mut self) -> Result<Vec<String>, sqlx::Error> {
        // Note: Does not support "Image" types. These would have to be converted to VarBinary(Max) during selection.
        // Assuming the same thing would have to be done for "Text" types as they would be converted to VarChar(Max).
        let rows = sqlx::query("SELECT EmployeeID, LastName, FirstName FROM Employees")
            .fetch_all(&mut self.dbConnection)
            .await?;

        let mut return_value: Vec<String> = Vec::new();

        for row1 in rows {
            let result: String = row1.try_get(1)?;

            return_value.push(result);
        }

        /*  while let Some(row) = rows.try_next().await? as Option<MssqlRow> {
            let result: String = row.try_get(1)?;

            return_value.push(result);
        } */

        Ok(return_value)
    }
}
