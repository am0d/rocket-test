use schema::category;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use util::*;
use models::prelude::*;

#[derive(Identifiable, Insertable, Debug, Clone, AsChangeset, Queryable, Serialize, Default)]
#[table_name = "category"]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(FromForm, Debug, Clone, Serialize)]
pub struct CategoryForm {
    pub id: i32,
    pub name: String,
}

impl Category {
    pub fn new() -> Category {
        Category {
            id: 0,
            name: "".to_string(),
        }
    }
}

impl_crud!(Category, category);

impl CategoryForm {
    pub fn is_new(&self) -> bool {
        self.id == 0
    }
    
    pub fn save(&self, conn: &PgConnection) -> AppResult<Category> {
        match self.is_valid() {
            ValidateResult::Valid => {
                let category = Category {
                    id: self.id,
                    name: self.name.clone(),
                };
                category.save(conn)
            }
            errors @ ValidateResult::Invalid(_) => {
                Err(errors::AppError::ValidationError { errors })
            }
        }
    }
}

impl Validate for CategoryForm {
    fn is_valid(&self) -> ValidateResult {
        let mut errors = vec![];
        if self.name.is_empty() {
            errors.push(String::from("Name is required"));
        }
        match errors.len() {
            0 => ValidateResult::Valid,
            _ => ValidateResult::Invalid(errors),
        }
    }
}

impl From<Category> for CategoryForm {
    fn from(category: Category) -> CategoryForm {
        CategoryForm {
            id: category.id,
            name: category.name,
        }
    }
}
