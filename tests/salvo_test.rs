use salvo::prelude::*;
use serde::{Deserialize,Serialize};

#[cfg(test)]
pub mod salvo_test{
    use crate::{Request, Response, Router, Server, TcpListener, Text,Serialize,Deserialize};

    #[derive(Debug,Serialize,Deserialize)]
    pub struct User{
        name: Option<String>,
        age: Option<u32>,
        gender: Option<bool>,
    }

    pub async fn add_user(req: &mut Request,res: &mut Response){
        let user = req.parse_json::<User>().await.unwrap();
        res.render(Text::Json(user))
    }

    // 将 json 格式的参数解析成结构体
    #[test]
    pub async fn parse_request(){
        tracing_subscriber::fmt().init();
        tracing::info!("Listening on http://127.0.0.1:7878");
        let router = Router::with_path("/addUser").post(add_user);
        Server::new(TcpListener::bind("127.0.0.1:7878")).serve(router).await;
    }

}

#[cfg(test)]
pub mod salvo_test02{
    use salvo::prelude::*;
    use salvo::prelude::{Text::Json,Text::Plain};

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
            res.render(Json("Error"));
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

    #[tokio::main]
    async fn main(){
        tracing_subscriber::fmt::init();
        // 对 /index 上的 url 使用 jwt_auth 这个中间件
        let router = Router::new().path("/").
            hoop(jwt_auth).get(hello_world)
            .push(Router::with_path("<id:num>").get(get_path_variable))
            .push(Router::with_path("login").get(get_query_param));

        tracing::info!("Listening on http://127.0.0.1:7878");
        Server::new(TcpListener::bind("127.0.0.1:7878")).serve(router).await;
    }
}