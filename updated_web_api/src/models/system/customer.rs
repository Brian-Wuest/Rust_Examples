use serde::Serialize;
use tiberius::Row;
use tiberius::time::chrono::{NaiveDate};
use tiberius::numeric::{Decimal};
use num_traits::cast::ToPrimitive;

#[derive(Debug, Serialize, Clone)]
pub struct Customer {
    id: i32,
    name: String,
    bill_to_customer_id: i32,
    customer_category_id: i32,
    buying_group_id: i32,
    primary_contact_person_id: i32,
    alternate_contact_person_id: i32,
    delivery_method_id: i32,
    delivery_city_id: i32,
    postal_city_id: i32,
    credit_limit: f32,
    account_opened_date: NaiveDate
}

impl Customer {
    pub fn load_from_row(row: Row) -> Customer {
        let name: &str = row.get(1).unwrap();
        let account_date: NaiveDate = row.get(11).unwrap();
        let credit_limit: Option<Decimal> = row.get(10);

        Customer {
            id: row.get(0).unwrap(),
            name: name.to_string(),
            bill_to_customer_id: row.get(2).unwrap_or_default(),
            customer_category_id: row.get(3).unwrap_or_default(),
            buying_group_id: row.get(4).unwrap_or_default(),
            primary_contact_person_id: row.get(5).unwrap_or_default(),
            alternate_contact_person_id: row.get(6).unwrap_or_default(),
            delivery_method_id: row.get(7).unwrap_or_default(),
            delivery_city_id: row.get(8).unwrap_or_default(),
            postal_city_id: row.get(9).unwrap_or_default(),
            credit_limit: credit_limit.unwrap_or(Decimal::new(0, 0)).to_f32().unwrap(),
            account_opened_date: account_date,
        }
    }
}