// This file was generated with `clorinde`. Do not modify.

#[derive(Clone, Copy, Debug)]
pub struct UpdateBestOfMatchParams {
    pub winner_id: uuid::Uuid,
    pub loser_id: uuid::Uuid,
    pub id: uuid::Uuid,
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct BestOfMatch {
    pub id: uuid::Uuid,
    pub winner_id: uuid::Uuid,
    pub loser_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct BestOfMatchQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<BestOfMatch, tokio_postgres::Error>,
    mapper: fn(BestOfMatch) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> BestOfMatchQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(BestOfMatch) -> R) -> BestOfMatchQuery<'c, 'a, 's, C, R, N> {
        BestOfMatchQuery {
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
pub struct GetBestOfMatchesStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_best_of_matches() -> GetBestOfMatchesStmt {
    GetBestOfMatchesStmt(
        "SELECT id, winner_id, loser_id, created_at FROM tagteam.best_of_match",
        None,
    )
}
impl GetBestOfMatchesStmt {
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
    ) -> BestOfMatchQuery<'c, 'a, 's, C, BestOfMatch, 0> {
        BestOfMatchQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<BestOfMatch, tokio_postgres::Error> {
                Ok(BestOfMatch {
                    id: row.try_get(0)?,
                    winner_id: row.try_get(1)?,
                    loser_id: row.try_get(2)?,
                    created_at: row.try_get(3)?,
                })
            },
            mapper: |it| BestOfMatch::from(it),
        }
    }
}
pub struct GetBestOfMatchByIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_best_of_match_by_id() -> GetBestOfMatchByIdStmt {
    GetBestOfMatchByIdStmt(
        "SELECT id, winner_id, loser_id, created_at FROM tagteam.best_of_match WHERE id = $1",
        None,
    )
}
impl GetBestOfMatchByIdStmt {
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
    ) -> BestOfMatchQuery<'c, 'a, 's, C, BestOfMatch, 1> {
        BestOfMatchQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<BestOfMatch, tokio_postgres::Error> {
                Ok(BestOfMatch {
                    id: row.try_get(0)?,
                    winner_id: row.try_get(1)?,
                    loser_id: row.try_get(2)?,
                    created_at: row.try_get(3)?,
                })
            },
            mapper: |it| BestOfMatch::from(it),
        }
    }
}
pub struct CreateBestOfMatchStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_best_of_match() -> CreateBestOfMatchStmt {
    CreateBestOfMatchStmt(
        "INSERT INTO tagteam.best_of_match (id, winner_id, loser_id) VALUES (gen_random_uuid(), NULL, NULL) RETURNING id, winner_id, loser_id, created_at",
        None,
    )
}
impl CreateBestOfMatchStmt {
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
    ) -> BestOfMatchQuery<'c, 'a, 's, C, BestOfMatch, 0> {
        BestOfMatchQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<BestOfMatch, tokio_postgres::Error> {
                Ok(BestOfMatch {
                    id: row.try_get(0)?,
                    winner_id: row.try_get(1)?,
                    loser_id: row.try_get(2)?,
                    created_at: row.try_get(3)?,
                })
            },
            mapper: |it| BestOfMatch::from(it),
        }
    }
}
pub struct UpdateBestOfMatchStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_best_of_match() -> UpdateBestOfMatchStmt {
    UpdateBestOfMatchStmt(
        "UPDATE tagteam.best_of_match SET winner_id = $1, loser_id = $2 WHERE id = $3 RETURNING id, winner_id, loser_id, created_at",
        None,
    )
}
impl UpdateBestOfMatchStmt {
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
        winner_id: &'a uuid::Uuid,
        loser_id: &'a uuid::Uuid,
        id: &'a uuid::Uuid,
    ) -> BestOfMatchQuery<'c, 'a, 's, C, BestOfMatch, 3> {
        BestOfMatchQuery {
            client,
            params: [winner_id, loser_id, id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<BestOfMatch, tokio_postgres::Error> {
                Ok(BestOfMatch {
                    id: row.try_get(0)?,
                    winner_id: row.try_get(1)?,
                    loser_id: row.try_get(2)?,
                    created_at: row.try_get(3)?,
                })
            },
            mapper: |it| BestOfMatch::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        UpdateBestOfMatchParams,
        BestOfMatchQuery<'c, 'a, 's, C, BestOfMatch, 3>,
        C,
    > for UpdateBestOfMatchStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a UpdateBestOfMatchParams,
    ) -> BestOfMatchQuery<'c, 'a, 's, C, BestOfMatch, 3> {
        self.bind(client, &params.winner_id, &params.loser_id, &params.id)
    }
}
