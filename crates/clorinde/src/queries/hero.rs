// This file was generated with `clorinde`. Do not modify.

#[derive(Debug, Clone, PartialEq)]
pub struct Hero {
    pub id: uuid::Uuid,
    pub display_name: String,
    pub expansion_id: uuid::Uuid,
    pub base_power: i32,
    pub base_health: i32,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct HeroBorrowed<'a> {
    pub id: uuid::Uuid,
    pub display_name: &'a str,
    pub expansion_id: uuid::Uuid,
    pub base_power: i32,
    pub base_health: i32,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<HeroBorrowed<'a>> for Hero {
    fn from(
        HeroBorrowed {
            id,
            display_name,
            expansion_id,
            base_power,
            base_health,
            created_at,
        }: HeroBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            display_name: display_name.into(),
            expansion_id,
            base_power,
            base_health,
            created_at,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct HeroQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<HeroBorrowed, tokio_postgres::Error>,
    mapper: fn(HeroBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> HeroQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(HeroBorrowed) -> R) -> HeroQuery<'c, 'a, 's, C, R, N> {
        HeroQuery {
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
pub struct GetHeroesStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_heroes() -> GetHeroesStmt {
    GetHeroesStmt(
        "SELECT id, display_name, expansion_id, base_power, base_health, created_at FROM tagteam.hero",
        None,
    )
}
impl GetHeroesStmt {
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
    ) -> HeroQuery<'c, 'a, 's, C, Hero, 0> {
        HeroQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<HeroBorrowed, tokio_postgres::Error> {
                Ok(HeroBorrowed {
                    id: row.try_get(0)?,
                    display_name: row.try_get(1)?,
                    expansion_id: row.try_get(2)?,
                    base_power: row.try_get(3)?,
                    base_health: row.try_get(4)?,
                    created_at: row.try_get(5)?,
                })
            },
            mapper: |it| Hero::from(it),
        }
    }
}
