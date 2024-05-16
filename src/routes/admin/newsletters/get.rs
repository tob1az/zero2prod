use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn publish_newsletter_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    let idempotency_key = uuid::Uuid::new_v4();
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Submit a new issue</title>
</head>
<body>
    {msg_html}
    <form action="/admin/newsletters" method="post">
        <label>Title
        <input
            type="text"
            placeholder="Enter the issue title"
            name="title"
        >
        </label>
        <br>
        <label>Text</label>
        <textarea name="text_content" rows="10" columns="30">
        </textarea>
        <br>
        <label>HTML</label>
        <textarea name="html_content" rows="10" columns="30">
        </textarea>
        <br>
        <button type="submit">Submit issue</button>
        <input hidden type="text" name="idempotency_key" value="{idempotency_key}">
    </form>
    <p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html>"#,
        )))
}
