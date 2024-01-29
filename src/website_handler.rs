use std::fmt::format;
use std::fs;
use crate::http::{Response,Request,Method,StatusCode};

use super::server::Handler;
pub struct  WebsiteHandler{
    public_path: String,
}
impl WebsiteHandler {
    pub fn new(path: String) -> Self{
         WebsiteHandler{public_path:path }
    }
    pub fn read_file(&self , file_path: &str) ->Option<String>{
        let path = format!("{}\\{}",self.public_path,file_path);
        match fs::canonicalize(&path){
            Ok(canpath)=>{
                if canpath.starts_with(&self.public_path){
                    
                    fs::read_to_string(path).ok()
                }else{
                    Some("Nice Try Hacking Blub".to_string())
                } }
            Err(_)=> None
        }
        
    }
}
impl Handler for WebsiteHandler{
    fn handle_request(&mut self, request: &Request)-> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(crate::http::StatusCode::OK, self.read_file("index.html")),
                "/Hello" => Response::new(crate::http::StatusCode::OK, self.read_file("hello.html")),
                path  => Response::new(crate::http::StatusCode::OK, self.read_file(path)),
                
                
            },
            _  => Response::new(crate::http::StatusCode::NotFound, None)
            
        }
        
    }
}