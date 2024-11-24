
fn main() {
  use utoipa::OpenApi;

  let openapi = cal_rs_lib::ApiDoc::aws_apigateway_integrated_openapi();
  match openapi.to_yaml() {
    Ok(json) => println!("{json}"),
    Err(e) => println!("Error: {e:?}"),
  };
}

