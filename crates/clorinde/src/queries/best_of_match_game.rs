// This file was generated with `clorinde`. Do not modify.

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct BestOfMatchGame {
    pub match_id: uuid::Uuid,
    pub game_id: uuid::Uuid,
    pub game_order: i32,
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct BestOfMatchGameQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<BestOfMatchGame, tokio_postgres::Error>,
    mapper: fn(BestOfMatchGame) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> BestOfMatchGameQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(BestOfMatchGame) -> R,
    ) -> BestOfMatchGameQuery<'c, 'a, 's, C, R, N> {
        BestOfMatchGameQuery {
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
pub struct GetBestOfMatchGamesStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_best_of_match_games() -> GetBestOfMatchGamesStmt {
    GetBestOfMatchGamesStmt(
        "SELECT match_id, game_id, game_order FROM tagteam.best_of_match_game",
        None,
    )
}
impl GetBestOfMatchGamesStmt {
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
    ) -> BestOfMatchGameQuery<'c, 'a, 's, C, BestOfMatchGame, 0> {
        BestOfMatchGameQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<BestOfMatchGame, tokio_postgres::Error> {
                    Ok(BestOfMatchGame {
                        match_id: row.try_get(0)?,
                        game_id: row.try_get(1)?,
                        game_order: row.try_get(2)?,
                    })
                },
            mapper: |it| BestOfMatchGame::from(it),
        }
    }
}
