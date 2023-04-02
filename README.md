# ax2 - Rust
[![Rust](https://github.com/RGGH/ax2/actions/workflows/rust.yml/badge.svg)](https://github.com/RGGH/ax2/actions/workflows/rust.yml)

Axum API with Swagger UI

### Learning project

- create swagger UI
- add POST and GET

todo:<br>

- take 2 params : i32, i32 and return result in JSON <br>
- eg.Currently this is returning JSON `<QueryParams>`
  ```params
  pub async fn query_params(Query(query): Query<QueryParams>) -> Json<QueryParams> {
      Json(query)
  }
  ```
- Add in a function to return i32 from a calculation of input param1 and input param2
- link with SQLX
