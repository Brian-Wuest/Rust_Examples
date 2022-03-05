use actix_web::{HttpRequest, HttpResponse, Responder, web::{self}};

use crate::DATA_CONTEXT;
use crate::models::system::Customer;
use futures::StreamExt;
use futures::future;

pub struct CustomerController {}

impl CustomerController {
    pub fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("/api/sales/customer").route(web::get().to(CustomerController::get_customers)));
    }

    pub async fn get_customers(_request: HttpRequest) -> impl Responder {
        println!("Getting customer information");

        let result = CustomerController::get_database_results().await;

        println!("Got Customer Data, serializing it!");

        let serialized = serde_json::to_string(&result).unwrap();

        println!("Serialization complete, sending response");

        return HttpResponse::Ok().body(serialized);
    }

    pub async fn get_database_results() -> Vec<Customer> {
        let mut final_result: Vec<Customer> = Vec::new();

        println!("Getting connection from connection pool");
        let initial_connection = &mut DATA_CONTEXT.connection_pool.get().await;

        match initial_connection {
            Ok(connection) => {
                println!("Running query against database");
                let initial_result = connection.simple_query("Select CustomerID,
                CustomerName,
                BillToCustomerID,
                CustomerCategoryID,
                BuyingGroupID,
                PrimaryContactPersonID,
                AlternateContactPersonID,
                DeliveryMethodID,
                DeliveryCityID,
                PostalCityID,
                CreditLimit,
                AccountOpenedDate From Sales.Customers").await;

                match initial_result {
                    Ok(stream) => {
                        println!("Processing data rows");

                        let row_stream = stream.into_row_stream();

                        let row_item = row_stream.for_each(|row| {
                            final_result.push(Customer::load_from_row(row.unwrap()));
                            future::ready(())
                        });

                        row_item.await;
                    }
                    _ => {
                        println!("No data found");
                    }
                }
            }

            Err(message) => {

                dbg!(message);
            }
        }

        final_result
    }
}