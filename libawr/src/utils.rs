use futures_util::Future;

/// 自动重试直到得到 `Ok(..)`。
pub async fn retry<F, T, D, E>(
    mut max_count: usize,
    mut f: impl FnMut() -> F,
    mut on_retry: impl FnMut(E, usize) -> D,
) -> Result<T, E>
where
    F: Future<Output = Result<T, E>>,
    D: Future<Output = ()>,
{
    loop {
        match f().await {
            Ok(t) => return Ok(t),
            Err(e) => {
                if max_count == 0 {
                    return Err(e);
                }
                max_count -= 1;
                on_retry(e, max_count).await;
                tokio::task::yield_now().await;
            }
        }
    }
}

/// 包装 `Box<ErrorImpl>`.
macro_rules! box_error_impl {
    ($error: ident, $err_impl: ident, $doc: literal) => {
        #[doc = $doc]
        #[derive(Error, Debug)]
        #[error(transparent)]
        pub struct $error(Box<$err_impl>);

        impl<E> From<E> for $error
        where
            $err_impl: From<E>,
        {
            fn from(err: E) -> Self {
                $error(Box::new($err_impl::from(err)))
            }
        }
    };
}
