pub mod module;

type IResult<'a, T, E> = nom::IResult<&'a [u8], T, E>;
