pub mod error;

pub trait FromAxumResponse<Inner, FromError, ToError: Send + Sync> {
    type TFrom;
    type TTo: From<FromError> + From<Inner>;

    fn as_axum(&self) -> Result<Self::TTo, ToError>;
}
