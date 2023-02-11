use actix_web::web::Json;

pub struct ErrorHelper {
    pub name: String,
    pub title: String,
    pub message: String,
}

pub async fn ReturnError(new_error: Json<ErrorHelper>) {
    let data = ErrorHelper {
        name: new_error.name.to_owned(),
        title: new_error.title.to_owned(),
        message: new_error.message.to_owned()
    };
}