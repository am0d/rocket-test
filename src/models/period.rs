use schema::period;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use chrono::prelude::*;
use util::errors::AppResult;
use util::time::date_from_str;

#[derive(Insertable, Debug, Clone, Serialize)]
#[table_name = "period"]
struct NewPeriod {
    name: String,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
}

#[derive(Identifiable, Insertable, Debug, Clone, AsChangeset, Queryable, Serialize)]
#[table_name = "period"]
pub struct Period {
    pub id: i32,
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub previous_period_id: Option<i32>,
}

#[derive(FromForm)]
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

    pub fn get(id: i32, conn: &PgConnection) -> AppResult<Period> {
        period::table
            .filter(period::id.eq(id))
            .first::<Period>(conn)
            .map_err(|e| app_error!(DatabaseError, e))
    }

    pub fn list(conn: &PgConnection) -> AppResult<Vec<Period>> {
        period::table
            .order(period::id)
            .load::<Period>(conn)
            .map_err(|e| app_error!(DatabaseError, e))
    }

    pub fn save(&self, conn: &PgConnection) -> AppResult<Period> {
        if self.is_new() {
            NewPeriod::from(self).insert(conn)
        } else {
            self.update(conn)
        }
    }

    pub fn is_new(&self) -> bool {
        self.id == 0
    }

    fn update(&self, conn: &PgConnection) -> AppResult<Period> {
        use diesel::SaveChangesDsl;
        self.save_changes::<Period>(conn).map_err(|e| {
            app_error!(DatabaseError, e)
        })
    }

    pub fn get_previous_period(&self, conn: &PgConnection) -> AppResult<Option<Period>> {
        match self.previous_period_id {
            None => Ok(None),
            Some(id) => Ok(Some(Self::get(id, conn)?)),
        }
    }
}

impl NewPeriod {
    fn from(period: &Period) -> NewPeriod {
        NewPeriod {
            name: period.name.clone(),
            start_date: period.start_date,
            end_date: period.end_date,
        }
    }
    fn insert(&self, conn: &PgConnection) -> AppResult<Period> {
        let period = diesel::insert(self)
            .into(period::table)
            .get_result(conn)
            .map_err(|e| app_error!(DatabaseError, e))?;
        Ok(period)
    }
}

impl PeriodForm {
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