
extern crate mysql;

pub fn pool() -> mysql::Pool {
    let pool = mysql::Pool::new("mysql://root@localhost:3306/spot_business").unwrap();
    pool
}