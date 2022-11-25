use dash_rs::{
    request::{
        account::LoginRequest,
        comment::{UploadCommentRequest, DeleteCommentRequest, CommentHistoryRequest, LevelCommentsRequest, ProfileCommentsRequest, SortMode},
    },
};

#[tokio::test]
async fn get_level_comments() {
    let request = LevelCommentsRequest::new(76298358)
        .page(0)
        .limit(5)
        .most_recent();

    let request_body = request.get_response_body()
        .await
        .unwrap();

    let level_comments = request.into_robtop(&request_body)
        .await
        .unwrap();

    assert_eq!(level_comments.len(), 5);
}


#[tokio::test]
async fn get_profile_comments() {
    let request = ProfileCommentsRequest::new(57903)
        .page(0);

    let response_body = request.get_response_body()
        .await
        .unwrap();

    let profile_comments = request.into_robtop(&response_body)
        .await
        .unwrap();

    assert_eq!(profile_comments.len(), 2);
}

#[tokio::test]
async fn upload_comment() {
    dotenv::from_filename("test_env.env").expect("test_env.env file not found");

    let user_name = dotenv::var("GJ_ACCOUNT_USERNAME").unwrap();
    let password = dotenv::var("GJ_ACCOUNT_PASSWORD").unwrap();

    let request = LoginRequest::default()
        .user_name(&user_name)
        .password(&password);

    let login_response = request.to_authenticated_user()
        .await
        .unwrap();

    let mut comment_upload_request = UploadCommentRequest::new(login_response, 76298358)
        .comment("More tests still ignore me")
        .percent(69);
    let comment_upload_response = comment_upload_request
        .get_response_body()
        .await
        .unwrap();

    assert!(!comment_upload_response.eq("-1"))
}

#[tokio::test]
async fn delete_comment() {
    dotenv::from_filename("test_env.env").expect("test_env.env file not found");

    let user_name = dotenv::var("GJ_ACCOUNT_USERNAME").unwrap();
    let password = dotenv::var("GJ_ACCOUNT_PASSWORD").unwrap();

    let request = LoginRequest::default()
        .user_name(&user_name)
        .password(&password);

    let login_response = request.to_authenticated_user()
        .await
        .unwrap();


    let comment_history_request = CommentHistoryRequest::new(3713125)
        .sort_mode(SortMode::Recent)
        .limit(1)
        .page(0);

    let comment_history_response = comment_history_request.get_response_body()
        .await
        .unwrap();

    let comment = comment_history_request.into_robtop(&comment_history_response)
        .await
        .unwrap();

    let comment_id = comment.get(0).unwrap().comment_id;

    let comment_delete_request = DeleteCommentRequest::new(login_response, 76298358, comment_id);

    let comment_delete_response = comment_delete_request.get_response_body()
        .await
        .unwrap();

    assert!(!comment_delete_response.eq("-1"))
}