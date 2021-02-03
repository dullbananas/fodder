use crate::ast::{self, source};
use crate::error::syntax as error;
use crate::project::Repo;
use super::IResult;


fn module<'a>(
    repo: Option<Repo>
) -> impl Fn(
    &'a [u8]
) -> IResult<'a, source::Module, error::Module> {
    
}
