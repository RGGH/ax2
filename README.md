[![Rust](https://github.com/RGGH/ax2/actions/workflows/rust.yml/badge.svg)](https://github.com/RGGH/ax2/actions/workflows/rust.yml)

# ax2 - Rust

Axum API with Swagger UI

### Learning project

- create swagger UI
- add POST and GET

## 

todo:<br>

- take 2 params : i32, i32 and return result in JSON<br>
- eg
  ```params
  pub async fn query_params(Query(query): Query<QueryParams>) -> Json<QueryParams> {
      Json(query)
  }
  ```
- link with SQLX
