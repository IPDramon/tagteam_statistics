// This file was generated with `clorinde`. Do not modify.

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub id: uuid::Uuid,
    pub display_name: String,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct PlayerBorrowed<'a> {
    pub id: uuid::Uuid,
    pub display_name: &'a str,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<PlayerBorrowed<'a>> for Player {
    fn from(
        PlayerBorrowed {
            id,
            display_name,
            created_at,
        }: PlayerBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            display_name: display_name.into(),
            created_at,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct PlayerQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<PlayerBorrowed, tokio_postgres::Error>,
    mapper: fn(PlayerBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> PlayerQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(PlayerBorrowed) -> R) -> PlayerQuery<'c, 'a, 's, C, R, N> {
        PlayerQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct GetPlayersStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_players() -> GetPlayersStmt {
    GetPlayersStmt(
        "SELECT id, display_name, created_at FROM tagteam.player",
        None,
    )
}
impl GetPlayersStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
    ) -> PlayerQuery<'c, 'a, 's, C, Player, 0> {
        PlayerQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<PlayerBorrowed, tokio_postgres::Error> {
                    Ok(PlayerBorrowed {
                        id: row.try_get(0)?,
                        display_name: row.try_get(1)?,
                        created_at: row.try_get(2)?,
                    })
                },
            mapper: |it| Player::from(it),
        }
    }
}
pub struct GetPlayerByIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_player_by_id() -> GetPlayerByIdStmt {
    GetPlayerByIdStmt(
        "SELECT id, display_name, created_at FROM tagteam.player WHERE id = $1",
        None,
    )
}
impl GetPlayerByIdStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        id: &'a uuid::Uuid,
    ) -> PlayerQuery<'c, 'a, 's, C, Player, 1> {
        PlayerQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<PlayerBorrowed, tokio_postgres::Error> {
                    Ok(PlayerBorrowed {
                        id: row.try_get(0)?,
                        display_name: row.try_get(1)?,
                        created_at: row.try_get(2)?,
                    })
                },
            mapper: |it| Player::from(it),
        }
    }
}
pub struct CreatePlayerStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_player() -> CreatePlayerStmt {
    CreatePlayerStmt(
        "INSERT INTO tagteam.player (id, display_name) VALUES (gen_random_uuid(),$1) RETURNING id, display_name, created_at",
        None,
    )
}
impl CreatePlayerStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        display_name: &'a T1,
    ) -> PlayerQuery<'c, 'a, 's, C, Player, 1> {
        PlayerQuery {
            client,
            params: [display_name],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<PlayerBorrowed, tokio_postgres::Error> {
                    Ok(PlayerBorrowed {
                        id: row.try_get(0)?,
                        display_name: row.try_get(1)?,
                        created_at: row.try_get(2)?,
                    })
                },
            mapper: |it| Player::from(it),
        }
    }
}
pub struct DeletePlayerByIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete_player_by_id() -> DeletePlayerByIdStmt {
    DeletePlayerByIdStmt(
        "DELETE FROM tagteam.player WHERE id = $1 RETURNING id, display_name, created_at",
        None,
    )
}
impl DeletePlayerByIdStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        id: &'a uuid::Uuid,
    ) -> PlayerQuery<'c, 'a, 's, C, Player, 1> {
        PlayerQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<PlayerBorrowed, tokio_postgres::Error> {
                    Ok(PlayerBorrowed {
                        id: row.try_get(0)?,
                        display_name: row.try_get(1)?,
                        created_at: row.try_get(2)?,
                    })
                },
            mapper: |it| Player::from(it),
        }
    }
}
