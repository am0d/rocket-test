/// Budget periods.

use schema::period;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use chrono::prelude::*;
use util::*;
use models::prelude::*;
use models::transaction::Transaction;

#[derive(Identifiable, Insertable, Debug, Clone, AsChangeset, Queryable, Serialize)]
#[table_name = "period"]
pub struct Period {
    pub id: i32,
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub previous_period_id: Option<i32>,
}

#[derive(Insertable, Debug)]
#[table_name = "period"]
pub struct NewPeriod {
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub previous_period_id: Option<i32>,
}

#[derive(FromForm, Serialize, Debug)]
pub struct PeriodForm {
    pub id: i32,
    pub name: String,
    pub start_date: String,
    pub end_date: Option<String>,
}

impl Period {
    pub fn new() -> Period {
        Period {
            id: 0,
            name: "".to_string(),
            start_date: Utc::now().naive_utc().date(),
            end_date: None,
            previous_period_id: None,
        }
    }

    pub fn get_previous_period(&self, conn: &PgConnection) -> AppResult<Option<Period>> {
        match self.previous_period_id {
            None => Ok(None),
            Some(id) => Ok(Some(Self::get(id, conn)?)),
        }
    }

    pub fn get_transactions(&self, conn: &PgConnection) -> AppResult<Vec<Transaction>> {
        Transaction::get_by_period(self.id, conn)
    }
}

impl_crud!(Period, NewPeriod, period);

impl NewPeriod {
    fn insert(&self, conn: &PgConnection) -> AppResult<Period>
    where
        Self: Sized,
    {
        diesel::insert_into(period::table)
            .values(self)
            .get_result(conn)
            .map_err(|e| app_error!(DatabaseError, e))
    }
}

impl<'a> From<&'a Period> for NewPeriod {
    fn from(period: &'a Period) -> NewPeriod {
        NewPeriod {
            name: period.name.clone(),
            start_date: period.start_date,
            end_date: period.end_date,
            previous_period_id: period.previous_period_id
        }
    }
}

impl Default for Period {
    fn default() -> Self {
        Self::new()
    }
}

impl PeriodForm {
    pub fn is_new(&self) -> bool {
        self.id == 0
    }

    pub fn save(&self, conn: &PgConnection) -> AppResult<Period> {
        let start_date = date_from_str(&*self.start_date)?;
        let end_date = match self.end_date {
            Some(ref ed) => Some(
                DateTime::<FixedOffset>::parse_from_str(&*ed, "%Y-%m-%d")
                    .map_err(|e| app_error!(TimeParseError, e))?
                    .naive_utc()
                    .date(),
            ),
            None => None,
        };
        let period = Period {
            id: self.id,
            name: self.name.clone(),
            start_date: start_date,
            end_date: end_date,
            previous_period_id: None,
        };
        period.save(conn)
    }
}

impl From<Period> for PeriodForm {
    fn from(period: Period) -> PeriodForm {
        PeriodForm {
            id: period.id,
            name: period.name,
            start_date: period.start_date.to_string(),
            end_date: period.end_date.map(|d| d.to_string()),
        }
    }
}

impl Validate for PeriodForm {
    fn is_valid(&self) -> ValidateResult {
        let mut errors = vec![];
        if self.name.is_empty() {
            errors.push(String::from("Name cannot be empty"));
        }
        if date_from_str(&self.start_date).is_err() {
            errors.push(String::from("Start date must be valid"));
        }
        match errors.len() {
            0 => ValidateResult::Valid,
            _ => ValidateResult::Invalid(errors),
        }
    }
}
