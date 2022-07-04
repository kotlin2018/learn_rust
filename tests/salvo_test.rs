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