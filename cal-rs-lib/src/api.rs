use super::*;

#[derive(utoipa::ToSchema)]
struct Item {
}

/// Get item
#[utoipa::path(
    get,
    path = "/item",
    responses(
        (status = 200, description = "Pet found successfully", body = Item),
        (status = NOT_FOUND, description = "Pet was not found")
    ),
)]
async fn get_item() -> Item {
    Item {}
}

#[derive(utoipa::OpenApi)]
#[openapi(paths(get_item))]
pub struct ApiDoc;

impl ApiDoc {
  pub fn aws_apigateway_integrated_openapi() -> utoipa::openapi::OpenApi {
    use utoipa::OpenApi;

    let mut openapi = ApiDoc::openapi();

    { // Handle component
      let aws_apigateway_integration = utoipa::openapi::extensions::ExtensionsBuilder::new()
        // See parameters at https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions-integration.html
        .add("x-amazon-apigateway-integration", 
          serde_json::json!({
            "type": "mock",
            "passthroughBehavior": "when_no_match",
            "httpMethod": "POST",
            "contentHandling": "CONVERT_TO_TEXT",
          })
        )
        .build();
      if let Some(components) = &mut openapi.components {
        if let Some(extensions) = &mut components.extensions {
          extensions.merge(aws_apigateway_integration);
        } else {
          components.extensions = Some(aws_apigateway_integration);
        }
      } else {
        openapi.components = Some(
          utoipa::openapi::schema::ComponentsBuilder::new()
          .extensions(Some(aws_apigateway_integration))
          .build()
        );
      }
    }

    { // Handle paths
      let aws_apigateway_integration = utoipa::openapi::extensions::ExtensionsBuilder::new()
        // See parameters at https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions-integration.html
        .add("x-amazon-apigateway-integration", 
          serde_json::json!({ "$ref": "#/components/x-amazon-apigateway-integration", })
        ).build();
      let extend = |operation: &mut utoipa::openapi::path::Operation| {
        if let Some(extensions) = &mut operation.extensions {
          extensions.merge(aws_apigateway_integration.clone());
        } else {
          operation.extensions = Some(aws_apigateway_integration.clone());
        }
      };

      // Add custom x-...
      for (k, path_item) in openapi.paths.paths.iter_mut() {
        if let Some(operation) = &mut path_item.get    { extend(operation); }
        if let Some(operation) = &mut path_item.put    { extend(operation); }
        if let Some(operation) = &mut path_item.post   { extend(operation); }
        if let Some(operation) = &mut path_item.delete { extend(operation); }
        if let Some(operation) = &mut path_item.options { extend(operation); }
        if let Some(operation) = &mut path_item.head   { extend(operation); }
        if let Some(operation) = &mut path_item.patch  { extend(operation); }
        if let Some(operation) = &mut path_item.trace  { extend(operation); }
      }
    }

    openapi
  }
}
