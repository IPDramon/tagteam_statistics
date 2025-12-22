// This file was generated with `clorinde`. Do not modify.

#[derive(Clone, Copy, Debug)]
pub struct CreateTeamParams {
    pub left_partner_id: uuid::Uuid,
    pub right_partner_id: uuid::Uuid,
    pub player_id: uuid::Uuid,
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Team {
    pub id: uuid::Uuid,
    pub left_partner_id: uuid::Uuid,
    pub right_partner_id: uuid::Uuid,
    pub player_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct TeamQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<Team, tokio_postgres::Error>,
    mapper: fn(Team) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> TeamQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(Team) -> R) -> TeamQuery<'c, 'a, 's, C, R, N> {
        TeamQuery {
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
pub struct GetTeamsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_teams() -> GetTeamsStmt {
    GetTeamsStmt(
        "SELECT id, left_partner_id, right_partner_id, player_id, created_at FROM tagteam.team",
        None,
    )
}
impl GetTeamsStmt {
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
    ) -> TeamQuery<'c, 'a, 's, C, Team, 0> {
        TeamQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<Team, tokio_postgres::Error> {
                Ok(Team {
                    id: row.try_get(0)?,
                    left_partner_id: row.try_get(1)?,
                    right_partner_id: row.try_get(2)?,
                    player_id: row.try_get(3)?,
                    created_at: row.try_get(4)?,
                })
            },
            mapper: |it| Team::from(it),
        }
    }
}
pub struct GetTeamByIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_team_by_id() -> GetTeamByIdStmt {
    GetTeamByIdStmt(
        "SELECT id, left_partner_id, right_partner_id, player_id, created_at FROM tagteam.team WHERE id = $1",
        None,
    )
}
impl GetTeamByIdStmt {
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
    ) -> TeamQuery<'c, 'a, 's, C, Team, 1> {
        TeamQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<Team, tokio_postgres::Error> {
                Ok(Team {
                    id: row.try_get(0)?,
                    left_partner_id: row.try_get(1)?,
                    right_partner_id: row.try_get(2)?,
                    player_id: row.try_get(3)?,
                    created_at: row.try_get(4)?,
                })
            },
            mapper: |it| Team::from(it),
        }
    }
}
pub struct CreateTeamStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_team() -> CreateTeamStmt {
    CreateTeamStmt(
        "INSERT INTO tagteam.team (id, left_partner_id, right_partner_id, player_id) VALUES (gen_random_uuid(), $1, $2, $3) RETURNING id, left_partner_id, right_partner_id, player_id, created_at",
        None,
    )
}
impl CreateTeamStmt {
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
        left_partner_id: &'a uuid::Uuid,
        right_partner_id: &'a uuid::Uuid,
        player_id: &'a uuid::Uuid,
    ) -> TeamQuery<'c, 'a, 's, C, Team, 3> {
        TeamQuery {
            client,
            params: [left_partner_id, right_partner_id, player_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<Team, tokio_postgres::Error> {
                Ok(Team {
                    id: row.try_get(0)?,
                    left_partner_id: row.try_get(1)?,
                    right_partner_id: row.try_get(2)?,
                    player_id: row.try_get(3)?,
                    created_at: row.try_get(4)?,
                })
            },
            mapper: |it| Team::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateTeamParams,
        TeamQuery<'c, 'a, 's, C, Team, 3>,
        C,
    > for CreateTeamStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateTeamParams,
    ) -> TeamQuery<'c, 'a, 's, C, Team, 3> {
        self.bind(
            client,
            &params.left_partner_id,
            &params.right_partner_id,
            &params.player_id,
        )
    }
}
