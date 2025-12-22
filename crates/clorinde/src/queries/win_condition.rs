// This file was generated with `clorinde`. Do not modify.

#[derive(Debug, Clone, PartialEq)]
pub struct WinCondition {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct WinConditionBorrowed<'a> {
    pub id: uuid::Uuid,
    pub title: &'a str,
    pub description: &'a str,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<WinConditionBorrowed<'a>> for WinCondition {
    fn from(
        WinConditionBorrowed {
            id,
            title,
            description,
            created_at,
        }: WinConditionBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            description: description.into(),
            created_at,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct WinConditionQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<WinConditionBorrowed, tokio_postgres::Error>,
    mapper: fn(WinConditionBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> WinConditionQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(WinConditionBorrowed) -> R,
    ) -> WinConditionQuery<'c, 'a, 's, C, R, N> {
        WinConditionQuery {
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
pub struct GetWinConditionsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_win_conditions() -> GetWinConditionsStmt {
    GetWinConditionsStmt(
        "SELECT id, title, description, created_at FROM tagteam.win_condition",
        None,
    )
}
impl GetWinConditionsStmt {
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
    ) -> WinConditionQuery<'c, 'a, 's, C, WinCondition, 0> {
        WinConditionQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<WinConditionBorrowed, tokio_postgres::Error> {
                    Ok(WinConditionBorrowed {
                        id: row.try_get(0)?,
                        title: row.try_get(1)?,
                        description: row.try_get(2)?,
                        created_at: row.try_get(3)?,
                    })
                },
            mapper: |it| WinCondition::from(it),
        }
    }
}
pub struct GetWinConditionByIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_win_condition_by_id() -> GetWinConditionByIdStmt {
    GetWinConditionByIdStmt(
        "SELECT id, title, description, created_at FROM tagteam.win_condition WHERE id = $1",
        None,
    )
}
impl GetWinConditionByIdStmt {
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
    ) -> WinConditionQuery<'c, 'a, 's, C, WinCondition, 1> {
        WinConditionQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<WinConditionBorrowed, tokio_postgres::Error> {
                    Ok(WinConditionBorrowed {
                        id: row.try_get(0)?,
                        title: row.try_get(1)?,
                        description: row.try_get(2)?,
                        created_at: row.try_get(3)?,
                    })
                },
            mapper: |it| WinCondition::from(it),
        }
    }
}
