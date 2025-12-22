// This file was generated with `clorinde`. Do not modify.

#[derive(Clone, Copy, Debug)]
pub struct CreatePartnerParams {
    pub hero_id: uuid::Uuid,
    pub final_power: i32,
    pub final_health: i32,
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Partner {
    pub id: uuid::Uuid,
    pub hero_id: uuid::Uuid,
    pub final_power: i32,
    pub final_health: i32,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct PartnerQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<Partner, tokio_postgres::Error>,
    mapper: fn(Partner) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> PartnerQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(Partner) -> R) -> PartnerQuery<'c, 'a, 's, C, R, N> {
        PartnerQuery {
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
pub struct GetPartnersStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_partners() -> GetPartnersStmt {
    GetPartnersStmt(
        "SELECT id, hero_id, final_power, final_health, created_at FROM tagteam.partner",
        None,
    )
}
impl GetPartnersStmt {
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
    ) -> PartnerQuery<'c, 'a, 's, C, Partner, 0> {
        PartnerQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<Partner, tokio_postgres::Error> {
                Ok(Partner {
                    id: row.try_get(0)?,
                    hero_id: row.try_get(1)?,
                    final_power: row.try_get(2)?,
                    final_health: row.try_get(3)?,
                    created_at: row.try_get(4)?,
                })
            },
            mapper: |it| Partner::from(it),
        }
    }
}
pub struct GetPartnerByIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_partner_by_id() -> GetPartnerByIdStmt {
    GetPartnerByIdStmt(
        "SELECT id, hero_id, final_power, final_health, created_at FROM tagteam.partner WHERE id = $1",
        None,
    )
}
impl GetPartnerByIdStmt {
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
    ) -> PartnerQuery<'c, 'a, 's, C, Partner, 1> {
        PartnerQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<Partner, tokio_postgres::Error> {
                Ok(Partner {
                    id: row.try_get(0)?,
                    hero_id: row.try_get(1)?,
                    final_power: row.try_get(2)?,
                    final_health: row.try_get(3)?,
                    created_at: row.try_get(4)?,
                })
            },
            mapper: |it| Partner::from(it),
        }
    }
}
pub struct CreatePartnerStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_partner() -> CreatePartnerStmt {
    CreatePartnerStmt(
        "INSERT INTO tagteam.partner (id, hero_id, final_power, final_health) VALUES (gen_random_uuid(), $1, $2, $3) RETURNING id, hero_id, final_power, final_health, created_at",
        None,
    )
}
impl CreatePartnerStmt {
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
        hero_id: &'a uuid::Uuid,
        final_power: &'a i32,
        final_health: &'a i32,
    ) -> PartnerQuery<'c, 'a, 's, C, Partner, 3> {
        PartnerQuery {
            client,
            params: [hero_id, final_power, final_health],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<Partner, tokio_postgres::Error> {
                Ok(Partner {
                    id: row.try_get(0)?,
                    hero_id: row.try_get(1)?,
                    final_power: row.try_get(2)?,
                    final_health: row.try_get(3)?,
                    created_at: row.try_get(4)?,
                })
            },
            mapper: |it| Partner::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreatePartnerParams,
        PartnerQuery<'c, 'a, 's, C, Partner, 3>,
        C,
    > for CreatePartnerStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreatePartnerParams,
    ) -> PartnerQuery<'c, 'a, 's, C, Partner, 3> {
        self.bind(
            client,
            &params.hero_id,
            &params.final_power,
            &params.final_health,
        )
    }
}
