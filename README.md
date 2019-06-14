SDK builder for various API, like retrofit

#### HttpService

```rust
// define service interface
#[macro_use] extern crate interfacer_http;
use interfacer_http::Result;
use interfacer::http::content_type;

#[http_service]
trait MyServiceInterface {
    #[get("/api/user?limit={limit}&offset={offset}", content_type::JSON)]
    #[except(200, content_type::JSON)]
    fn get_users_info(limit: u64, offset: u64) -> Result<Vec<User>>;
    
    #[put("/api/user/{uid}", content_type::JSON)]
    #[expect(200)]
    fn put_user_info(uid: u64, user: &User) -> Result<()>;
}
```

```rust
// define service
use interfacer_http::HttpService;
use http::{Request, Response};

struct MyService {
    base_url: String,
}

impl MyService {
    pub fn new(base_url: Into<String>) -> Self {
        Self {base_url: base_url.into()}
    }
}

impl HttpService for MyService {
    fn get_base_url(&self) -> &str {
        &self.base_url
    }
}
```

```rust
// use them
use crate::{MyServiceInterface, MyService};
use interfacer_http::Result;

fn main() -> Result<()> {
    let service = MyService::new("https://www.host.com");
    let users = service.get_users_info(0, 0)?;
    // async version
    // service.put_user_info_async(users[0].uid, &users[0]).await?
    Ok(())
}
```

#### SQLService

```rust
// define service interface
#[macro_use] extern crate interfacer_sql;
use interfacer_sql::Result;

#[sql_service]
trait MyServiceInterface {
    #[sql("SELECT * FROM `user` WHERE `uid`={uid}")]
    fn get_user(uid: u64) -> Result<User>;
    
    // parse by method name
    #[sql]
    fn get_user_by_uid(uid: u64) -> Result<User>;
}
```

```rust
// define service
use interfacer_sql::{SQLService, Result};

struct MyService {
    pool: ConnectionPool,
}

impl MyService {
    pub fn connect(db_url: Into<String>) -> Result<Self> {
        Ok(Self {pool: ConnectionPool::connect(base_url.into())?})
    }
}

impl SQLService for MyService {
    fn get_pool(&self) -> &ConnectionPool {
        &self.pool
    }
}
```

```rust
// use them
use crate::{MyServiceInterface, MyService};
use interfacer_sql::Result;

fn main() -> Result<()> {
    let service = MyService::connect("https://www.host.com")?;
    let users = service.get_user(0)?;
    // async version
    // service.get_user_async(0).await?
    Ok(())
}
```


