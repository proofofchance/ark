use ark_db::schema::ark_total_paid_out_reports;
use diesel::prelude::{Insertable, Queryable};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = ark_total_paid_out_reports)]
pub struct TotalPaidOutReport {
    pub id: i64,
    pub amount: String,
}

impl TotalPaidOutReport {
    pub fn derive_new(&self, amount: f64) -> UnsavedTotalPaidOutReport {
        let old_amount: f64 = self.amount.parse().unwrap();
        let new_amount = old_amount + amount;

        UnsavedTotalPaidOutReport {
            amount: new_amount.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = ark_total_paid_out_reports)]
pub struct UnsavedTotalPaidOutReport {
    amount: String,
}

impl UnsavedTotalPaidOutReport {
    pub fn new(amount: f64) -> UnsavedTotalPaidOutReport {
        UnsavedTotalPaidOutReport {
            amount: amount.to_string(),
        }
    }
}
