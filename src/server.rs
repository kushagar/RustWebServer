use std::net::TcpListener;
use std::io::{Read,Write};
use crate::http::{request, Request, Response, StatusCode,ParseError};

pub struct Server{
    addr: String,
  }
  pub trait Handler {
      fn handle_request(&mut self, request: &Request)-> Response;
      fn handle_bad_request(&mut self, err: &ParseError) -> Response {
        println!("Error parsing the response");
        Response::new(StatusCode::BadRequest, None)
      }
  }
impl Server{
    pub  fn new(addr :String) -> Self {
            Self{
                addr: addr
            }
        }
    pub fn run(self,mut handler: impl Handler){
       println!("listening on {}",self.addr);
       let listener = TcpListener::bind(&self.addr).unwrap();
       loop{
        match listener.accept(){
            Ok((mut stream,addr)) => {
                let mut buffer: [u8;1024] = [0;1024];
                match stream.read(&mut buffer){
                    Ok(_)=>{
                        let response=match Request::try_from(&buffer[..]){
                            Ok(request)=>{
                                dbg!(&request);
                               handler.handle_request(&request)

                            },
                            Err(err)=>{
                                handler.handle_bad_request(&err)
                            } };
                        if let Err(e)=response.send(&mut stream){
                            println!("Failed to send response :{}",e);
                        };
                       

                    }
                    Err(e)=> println!("Failed to establish a connection {}",e)
                }

            }
            Err(err)=>{
                println!("Error creating connection {}",err);
                continue;

            }
        }
       }
       
    }
}