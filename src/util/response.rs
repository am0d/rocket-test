//! Provides an implementation of a custom response type for rocket handlers.

use std::borrow::Cow;
use serde::Serialize;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Redirect, Responder, Response as RocketResponse};
use rocket_contrib::Template;
use util::errors::{error_page, AppError};

/// The types of application responses available
pub enum Response {
    /// A view to be rendered on the client
    View { template: Template },
    /// A resource was successfully saved
    Saved {
        redirect_location: Cow<'static, str>,
    },
    /// An application error occurred
    Error { error: AppError },
}

/// Returns a view response, with a template and a context
pub fn view<S, C>(template_name: S, context: &C) -> Response
where
    S: Into<Cow<'static, str>>,
    C: Serialize,
{
    Response::View {
        template: Template::render(template_name, context),
    }
}

/// Returns a response informing the user an application error has occurred
pub fn error(error: AppError) -> Response {
    Response::Error { error }
}

/// Returns a response indicating that a save was successful, with a redirect
/// to a new page
pub fn saved<S>(redirect_location: S) -> Response
where
    S: Into<Cow<'static, str>>,
{
    Response::Saved {
        redirect_location: redirect_location.into(),
    }
}

impl<'r> Responder<'r> for Response {
    fn respond_to(self, request: &Request) -> Result<RocketResponse<'r>, Status> {
        match self {
            Response::View { template } => template.respond_to(request),
            Response::Saved { redirect_location } => {
                Redirect::to(&redirect_location).respond_to(request)
            }
            Response::Error { error } => error_page(error).respond_to(request),
        }
    }
}
