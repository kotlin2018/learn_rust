#[cfg(test)]
mod salvo_test{
    use salvo::prelude::*;
    use salvo::prelude::{Text::Plain};
    use salvo::writer::Json;
    use serde::{Serialize,Deserialize};
    use std::fs::*;
    use std::path::Path;

    #[fn_handler]
    async fn hello_world() -> &'static str {
        "Hello World"
    }

    // jwt_token 中间件
    #[fn_handler]
    async fn jwt_auth(req: &mut Request,res: &mut Response){
        let token = req.header::<String>("jwt_token").unwrap();
        if token == "hello" {
            res.render(Plain(token));
        }else {
            res.render(Plain("Error"));
        }
    }

    // 获取 url 路径中的参数，例如: http://localhost:7878/33
    #[fn_handler]
    async fn get_path_variable(req: &mut Request,res: &mut Response){
        let id: String = req.param("id").unwrap();
        res.render(Plain(id));
    }

    // 获取 get 请求中的查询参数，例如: http://127.0.0.1:7878/login?username=李玲&password=123456
    #[fn_handler]
    async fn get_query_param(req: &mut Request,res: &mut Response){
        let username: String = req.query("username").unwrap();
        let password: String = req.query("password").unwrap();
        res.render(Plain(format!("username is {:?} and password is {:?}",username,password)))
    }

    #[derive(Debug,Serialize,Deserialize)]
    pub struct User{
        gender: Option<bool>,
        email: Option<String>,
    }

    // 通用的返回数据包装器
    #[derive(Debug,Serialize,Deserialize)]
    pub struct Result<T>{
        code: Option<u32>,
        msg: Option<String>,
        data: Option<T>
    }

    // 使用 form-data 提交请求参数
    #[fn_handler]
    async fn get_form_body(req: &mut Request) -> Json<User>{
        let user = req.parse_form::<User>().await.unwrap();
        Json(user)
    }

    // 使用 application/json 提交请求参数
    #[fn_handler]
    async fn get_json_body(req: &mut Request) -> Json<User>{
        let user = req.parse_json::<User>().await.unwrap();
        Json(user)
    }

    // 使用统一结构体返回数据
    #[fn_handler]
    async fn get_req_result(req: &mut Request) -> Json<Result<User>>{
        let user = req.parse_json::<User>().await.unwrap();
        Json(Result{code: Some(200),msg: Some("success".to_string()),data: Some(user)})
    }

    // 上传文件到本地
    #[fn_handler]
    async fn upload_file(req: &mut Request) -> String {
        let file = req.file("file").await.unwrap();
        let dest = file.name().unwrap();
        dest.into()
    }

// 这种响应数据的方式是错误的
// #[fn_handler]
// async fn get_json_body(req: &mut Request,res: &mut Response){
//     let user = req.parse_json::<User>().await.unwrap();
//     res.render(Text::Json(user));
// }

    #[tokio::test]
    async fn test(){
        tracing_subscriber::fmt::init();
        let router = Router::new().path("/").
            hoop(jwt_auth).get(hello_world)
            .push(Router::with_path("<id:num>").get(get_path_variable))
            .push(Router::with_path("query").get(get_query_param))
            .push(Router::with_path("form").post(get_form_body))
            .push(Router::with_path("json").post(get_json_body))
            .push(Router::with_path("result").post(get_req_result))
            .push(Router::with_path("upload").post(upload_file));
        tracing::info!("Listening on http://127.0.0.1:7878");
        Server::new(TcpListener::bind("127.0.0.1:7878")).serve(router).await;
    }
}
