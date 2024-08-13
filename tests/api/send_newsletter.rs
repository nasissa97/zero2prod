use crate::helpers::{spawn_app, assert_is_redirect_to};
use uuid::Uuid;

#[tokio::test]
async fn get_send_email_page() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let login_body = serde_json::json!({
        "username": app.test_user.username,
        "password": app.test_user.password,
    });
    let reponse = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");

    // Act - Part 2 - Go to Send Newsletter form
    

}