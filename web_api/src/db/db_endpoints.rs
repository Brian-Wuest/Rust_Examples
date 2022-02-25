use actix_web::{HttpRequest, HttpResponse, Responder, web::{self, Data}};
use core::ops::Add;
use futures::executor;
use sqlx::{MssqlPool};
use sqlx::{
    mssql::{MssqlConnectOptions},
    Row,
};
use std::{time::{Instant}};

#[derive(Debug)]
pub struct DBEndPoints {
    connection_pool: MssqlPool
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

        //let database_connection: Result<MssqlConnection, Error> =
        //    executor::block_on(MssqlConnection::connect_with(&connection_options));

        //let connection = database_connection.unwrap();
        let pool_connection = executor::block_on(MssqlPool::connect_with(connection_options));//MssqlPool::connect("mssql://sa:sql@localhost/Northwind"));
        let pool = pool_connection.unwrap();

        DBEndPoints {
            connection_pool: pool
        }
    }

    pub fn config(cfg: &mut web::ServiceConfig) {
        let endpoints = Data::new(DBEndPoints::new());

        cfg.app_data(Data::clone(&endpoints));

        cfg.service(web::resource("/app").route(web::get().to(DBEndPoints::something_cool)));
    }

    pub async fn something_cool(request: HttpRequest) -> impl Responder {
        let start = Instant::now();
        let connection_info = request.app_data::<Data<DBEndPoints>>();

        dbg!("Getting connection information");

        if connection_info.is_some() {
            dbg!("Found connection information");
            let unwrapped_connection_info = connection_info.unwrap();

            dbg!("getting sql results through manual_hello");

            let result = unwrapped_connection_info.manual_hello().await;

            dbg!("sql connection information found");

            if !result.is_empty() {
                let duration = start.elapsed();
                println!("Time elapsed in something_cool() is: {:?}", duration);
                return HttpResponse::Ok().body(result);
            }
        }

        HttpResponse::Ok().body("No value")
    }

    async fn manual_hello(&self) -> String {
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

                return result_string;
            }
            Err(error) => {
                dbg!(error);
                return "Not So Great!".to_string();
            }
        }
    }

    async fn sql_stuff(&self) -> Result<Vec<String>, sqlx::Error> {
        let connection= &self.connection_pool;
        /* 
            Note:
                Does not support "Image" types. These would have to be converted to VarBinary(Max) during selection.
                Does not support "money" types. These would have to be converted to decimal during selection.
            */
        // Assuming the same thing would have to be done for "Text" types as they would be converted to VarChar(Max).
        //let rows = sqlx::query("SELECT EmployeeID, LastName, FirstName FROM Employees")
        let rows = sqlx::query("SELECT CustomerID, ShipName, ShipAddress FROM Orders")
            .fetch_all(connection)
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
