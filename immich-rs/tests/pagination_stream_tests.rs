use immich_rs::PaginationStream;
use immich_rs::error::{Error, Result};

fn assert_unpin<T: Unpin>() {}

#[test]
fn pagination_stream_is_unpin() {
    assert_unpin::<PaginationStream<'static, String>>();
    assert_unpin::<PaginationStream<'static, Vec<i32>>>();
}

#[tokio::test]
async fn pagination_stream_next_works_without_stream_ext_import() {
    let items = vec![Ok(1), Ok(2), Ok(3)];
    let raw_stream = futures_util::stream::iter(items);
    let mut stream = PaginationStream::new(raw_stream);

    let mut collected = Vec::new();
    // Inherent .next().await without importing futures_util::StreamExt
    while let Some(res) = stream.next().await {
        collected.push(res.unwrap());
    }

    assert_eq!(collected, vec![1, 2, 3]);
}

#[tokio::test]
async fn pagination_stream_collect_gathers_all_items() -> Result<()> {
    let items = vec![Ok("alpha".to_string()), Ok("beta".to_string()), Ok("gamma".to_string())];
    let raw_stream = futures_util::stream::iter(items);
    let stream = PaginationStream::new(raw_stream);

    // Inherent .collect().await
    let collected = stream.collect().await?;
    assert_eq!(collected, vec!["alpha", "beta", "gamma"]);
    Ok(())
}

#[tokio::test]
async fn pagination_stream_collect_propagates_error() {
    let items: Vec<Result<i32>> = vec![
        Ok(10),
        Err(Error::Validation("simulated pagination failure".into())),
        Ok(20),
    ];
    let raw_stream = futures_util::stream::iter(items);
    let stream = PaginationStream::new(raw_stream);

    let err = stream.collect().await.unwrap_err();
    assert!(err.to_string().contains("simulated pagination failure"));
}

#[tokio::test]
async fn pagination_stream_try_next_and_try_collect_work() -> Result<()> {
    let items = vec![Ok(100), Ok(200), Ok(300)];
    let raw_stream = futures_util::stream::iter(items);
    let mut stream = PaginationStream::new(raw_stream);

    let mut collected = Vec::new();
    while let Some(item) = stream.try_next().await? {
        collected.push(item);
    }
    assert_eq!(collected, vec![100, 200, 300]);

    let items2 = vec![Ok(400), Ok(500)];
    let raw_stream2 = futures_util::stream::iter(items2);
    let stream2 = PaginationStream::new(raw_stream2);
    let collected2 = stream2.try_collect().await?;
    assert_eq!(collected2, vec![400, 500]);

    Ok(())
}

#[tokio::test]
async fn pagination_stream_empty_collects_empty_vec() -> Result<()> {
    let items: Vec<Result<i32>> = Vec::new();
    let raw_stream = futures_util::stream::iter(items);
    let stream = PaginationStream::new(raw_stream);

    let res = stream.collect().await?;
    assert_eq!(res, Vec::<i32>::new());
    Ok(())
}

#[tokio::test]
async fn pagination_stream_try_next_propagates_inner_error_immediately() {
    let items: Vec<Result<i32>> = vec![Ok(1), Err(Error::Validation("upstream batch failed".into())), Ok(2)];
    let raw_stream = futures_util::stream::iter(items);
    let mut stream = PaginationStream::new(raw_stream);

    let first = stream.try_next().await.unwrap();
    assert_eq!(first, Some(1));

    let second_err = stream.try_next().await.unwrap_err();
    assert!(second_err.to_string().contains("upstream batch failed"));
}

#[tokio::test]
async fn pagination_stream_into_inner_preserves_remaining_stream_items() -> Result<()> {
    use futures_util::StreamExt;

    let items = vec![Ok(10), Ok(20), Ok(30)];
    let raw_stream = futures_util::stream::iter(items);
    let mut stream = PaginationStream::new(raw_stream);

    let first = stream.try_next().await?.unwrap();
    assert_eq!(first, 10);

    let mut inner = stream.into_inner();
    let second = inner.next().await.unwrap()?;
    let third = inner.next().await.unwrap()?;
    assert_eq!(second, 20);
    assert_eq!(third, 30);
    assert!(inner.next().await.is_none());

    Ok(())
}
