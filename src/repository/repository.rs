use actix_web::{HttpRequest, HttpResponse, Responder};
use crate::storage::models::Storage;
use crate::repository::models::Repository;
use crate::repository::repo_error::RepositoryError;
use actix_web::web::Bytes;
use crate::site_response::SiteResponse;
use std::path::{Path, PathBuf};
use crate::error::request_error::RequestError;
use diesel::MysqlConnection;

pub enum RepoResponse {
    FileList(Vec<String>),
    FileResponse(PathBuf),
    Ok,
    NotFound,
    NotAuthorized,

}

pub type RepoResult = Result<RepoResponse, RequestError>;

pub struct RepositoryRequest {
    pub request: HttpRequest,
    pub storage: Storage,
    pub repository: Repository,
    pub value: String,
}

pub trait RepositoryType {
    fn handle_get(request: RepositoryRequest, conn: &MysqlConnection) -> RepoResult;
    fn handle_post(request: RepositoryRequest, conn: &MysqlConnection, bytes: Bytes) -> RepoResult;
    fn handle_put(request: RepositoryRequest, conn: &MysqlConnection, bytes: Bytes) -> RepoResult;
    fn handle_patch(request: RepositoryRequest, conn: &MysqlConnection, bytes: Bytes) -> RepoResult;
    fn handle_head(request: RepositoryRequest, conn: &MysqlConnection) -> RepoResult;
}