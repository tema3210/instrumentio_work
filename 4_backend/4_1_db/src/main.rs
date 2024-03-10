#![feature(if_let_guard)]
use std::path::PathBuf;
use clap::Parser;
use diesel::{dsl::sql, sql_types::{Array, Text}, ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl, SelectableHelper, TextExpressionMethods};
use diesel_async::{AsyncConnection,RunQueryDsl};
use uuid::Uuid;

mod schema;
mod models;

type Connection = diesel_async::AsyncPgConnection;

#[derive(clap::Parser,Debug)]
#[command(version, about, long_about = None)]
/// CRUD to a simple DB
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand,Debug)]
enum Commands {
    /// Creates a user, prints their ID
    CreateUser { 
        name: String, 
        age: u16 
    },
    /// Create a role
    CreateRole { 
        role: PathBuf,
        permit: u8,
        desc: Option<String> 
    },
    /// Assigns a role to user
    ModRelation {
        user: uuid::Uuid,
        role: PathBuf,
        has: bool
    },
    /// Lists roles of a user
    Data {
        user: Option<String>,
    },
    /// List all roles or all sub roles
    Roles {
        root: Option<PathBuf>
    },
    /// Lists all users who have a role
    WhoHas {
        role: PathBuf
    }
}

impl Commands {
    async fn exec(self, mut conn: Connection) -> Result<(),String> {
        use Commands::*;
        match self {
            CreateUser { name, age } => {
                let id = uuid::Uuid::new_v4();

                let user = models::User { name, age: age.into(), id};

                let res = diesel::insert_into(schema::users::table)
                    .values([user])
                    .execute(&mut conn)
                    .await;
                match res {
                    Ok(1) => {
                        println!("id: {}",&id);
                        Ok(())
                    },
                    _ => {
                        Err("Failed to insert".into())
                    }
                }
            },
            CreateRole { role, permit, desc } => {
                let role = models::Role {
                    slug: role.display().to_string(),
                    description: desc,
                    perms: permit.min(7).into() // unix style perm clamp
                };

                let res = diesel::insert_into(schema::roles::table)
                    .values([
                        role
                    ])
                    .execute(&mut conn)
                    .await;

                match res {
                    Ok(_) => Ok(()),
                    Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation,_)) => {
                        println!("error: such role already exists");
                        Ok(())
                    },
                    Err(e) => {
                        Err(format!("{:?}",&e))
                    }
                }
            },
            Roles { root } => {
                let res = if let Some(root) = root {
                    schema::roles::table.filter(
                        schema::roles::slug.like(&format!("{}%",root.display()))
                    )
                    .select(models::Role::as_select())
                    .load::<models::Role>(&mut conn)
                    .await
                } else {
                    schema::roles::table
                    .select(models::Role::as_select())
                    .load::<models::Role>(&mut conn)
                    .await
                };

                match res {
                    Ok(res) => {
                        println!("{:?}",res);
                        Ok(())
                    },
                    Err(e) => {
                        Err(format!("cannot load roles {:?}",e))
                    }
                }
                
            }
            ModRelation { user, role, has } => {
                if has {
                    let relation = models::UserRole {
                        user,
                        role: role.display().to_string()
                    };
    
                    let res = diesel::insert_into(schema::user_roles::table)
                        .values([
                            &relation
                        ])
                        .on_conflict(
                            schema::user_roles::all_columns
                        )
                        .do_update()
                        .set(&relation)
                        .execute(&mut conn)
                        .await;
                    
                    match res {
                        Ok(_) => Ok(()),
                        Err(e) => Err(format!("cannot mod rel: {:?}",e))
                    }
                } else {
                    diesel::delete(schema::user_roles::table)
                        .filter(
                            schema::user_roles::user.eq(user)
                        )
                        .filter(
                            schema::user_roles::role.eq(role.display().to_string())
                        )
                        .execute(&mut conn)
                        .await
                        .map_err(|e| format!("cannot mod rel: {:?}",e))?;
                    Ok(())
                }
            },
            Data { user } => {
                use schema::*;

                type Data = (uuid::Uuid, String, i32, Option<Vec<String>>);

                let q = users::table
                .inner_join(
                    user_roles::table
                        .on(user_roles::user.eq(users::id))
                )
                .inner_join(
                    roles::table
                        .on(roles::slug.eq(user_roles::role))
                )
                .group_by(
                    (users::id, users::name, users::age)
                )
                .select((
                    users::id,
                    users::name,
                    users::age,
                    // roles::slug
                    sql::<Array<Text>>("array_agg(roles.slug) AS role_slugs").nullable(),
                ));

                let data: Result<Vec<Data>,_> = if let Some(user) = user.clone() {
                    let Ok(user) = user.parse::<Uuid>() else {
                        return Err("bad argument format".into())
                    };
                    q
                        .filter(
                            users::id.eq::<Uuid>(user)
                        )
                        .load::<Data>(&mut conn).await
                } else {
                    q.load::<Data>(&mut conn).await
                };

                match data {
                    Ok(v) if v.is_empty() => {
                        println!("No such user found, id: {:?}",&user);
                        Ok(())
                   },
                    Ok(v) => {
                        for (id,name,age,roles) in v {
                            println!("User: \t {id} \t {name} \t {age}");
                            if let Some(roles) = roles {
                                println!("\t has roles:");
                                for role in roles {
                                    println!("\t {role};");
                                }
                            }
                        };
                        Ok(())
                    },
                    Err(e) => {
                        Err(format!("cannot load users {:?}",e))
                    }
                }
            },
            WhoHas { role } => {
                use schema::*;

                type Data = (uuid::Uuid, String, i32);

                let data = users::table
                .inner_join(
                    user_roles::table
                        .on(user_roles::user.eq(users::id))
                )
                .group_by(
                    (users::id, users::name, users::age)
                )
                .select((
                    users::id,
                    users::name,
                    users::age,
                ))
                .filter(
                    user_roles::role.eq(role.display().to_string())
                ).load::<Data>(&mut conn).await;

                match data {
                    Ok(v) if v.is_empty() => {
                        Ok(())
                    },
                    Ok(v) => {
                        for (id,name,age) in v {
                            println!("User: \t {id} \t {name} \t {age}");
                        };
                        Ok(())
                    },
                    Err(e) => {
                        Err(format!("cannot load users {:?}",e))
                    }
                }
            },
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    dotenv::dotenv().ok();

    let args = Args::parse();

    let db_url = std::env::var("DATABASE_URL").map_err(|e| format!("{e:?}"))?;

    let conn = Connection::establish(&db_url).await.map_err(|e| format!("{e:?}"))?;

    args.command.exec(conn).await?;
    Ok(())
}
