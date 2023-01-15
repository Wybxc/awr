//! 缓存控制。
//!
//! awr 对从服务器获取的数据进行缓存，以减少网络请求次数。
//! 如好友列表、群列表、群成员列表等，均会被缓存。
//!
//! # 缓存 API
//! 大多数情况下，缓存的使用是无感的，awr 会每隔一定时间自动更新缓存（见 [`consts`] 中定义的缓存过期时间）。
//! 当某些 API 调用使得远程值发生变化时，awr 会自行使缓存失效。
//!
//! 但是，如果需要手动更新缓存，awr 也提供了相应的 API。
//!
//! ## 配置项
//! [`consts`] 中定义了一些缓存的配置项，如缓存过期时间等。
//! 这些配置项是编译时可配置的常量，在运行时无法修改。它们作为缓存配置的默认值使用。
//!
//! 要想在运行时修改缓存配置，可以使用 [`Client`] 提供的方法，如 [`Client::set_friend_list_cache_time`] 等。
//!
//! ## [`Client`] 的缓存 API
//!
//! [`Client`] 提供了一些缓存相关 API，用于强制使缓存失效，如 [`Client::flush_friend_list`] 等。
//!
//! ## 选择器 API
//!
//! awr 的缓存机制与[选择器]机制高度协作，对缓存的操作需要获取对应的选择器。
//!
//! [`Selector::flush`] 方法使缓存失效，下次获取时会重新从服务器获取。
//!
//! 大部分远程实体对象都实现了到选择器的隐式转换，因此 `flush` 方法也可以直接在对象上调用。
//!
//! [`consts`]: mod@crate::consts
//! [选择器]: crate::meta::selector
//! [`Selector::flush`]: crate::meta::selector::Selector::flush

use std::{
    collections::HashMap,
    hash::Hash,
    ops::Deref,
    sync::Arc,
    time::{Duration, Instant},
};

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::Client;

type ValueWithLastUpdate<T> = (Arc<T>, Instant);

/// 缓存映射。
pub(crate) struct CachedMap<T: MapCacheable> {
    cached_value: RwLock<HashMap<T::Key, ValueWithLastUpdate<T>>>,
    duration: RwLock<Duration>,
}

/// 可缓存的值。
#[async_trait]
pub(crate) trait MapCacheable: Clone {
    /// 键类型。
    type Key: Eq + Hash + Clone;
    /// 错误类型。
    type Error;

    /// 从远程获取值。
    async fn fetch_uncached(
        client: &Arc<Client>,
        key: &Self::Key,
    ) -> Result<Option<Self>, Self::Error>;
}

impl<T: MapCacheable> CachedMap<T> {
    /// 创建一个新的缓存。
    ///
    /// # Arguments
    /// * `duration` - 缓存时长。
    pub(crate) fn new(duration: Duration) -> Self {
        Self {
            cached_value: RwLock::new(HashMap::new()),
            duration: RwLock::new(duration),
        }
    }

    /// 设置缓存时长。
    pub(crate) async fn set_cache_time(&self, duration: Duration) {
        *self.duration.write().await = duration;
    }

    /// 获取缓存，如果缓存过期或不存在则更新缓存。
    pub(crate) async fn get(
        &self,
        client: &Arc<Client>,
        key: &T::Key,
    ) -> Result<Option<Arc<T>>, T::Error> {
        let map = self.cached_value.read().await;
        // 缓存存在
        if let Some((cached, last_update)) = map.get(key) {
            // 且未过期
            if last_update.elapsed() < *self.duration.read().await {
                return Ok(Some(cached.clone()));
            }
        }
        drop(map);
        self.refresh(client, key).await
    }

    /// 标记缓存为过期。
    pub(crate) async fn make_dirty(&self, key: &T::Key) {
        let mut map = self.cached_value.write().await;
        map.remove(key);
    }

    /// 强制更新缓存。
    pub(crate) async fn refresh(
        &self,
        client: &Arc<Client>,
        key: &T::Key,
    ) -> Result<Option<Arc<T>>, T::Error> {
        if let Some(value) = T::fetch_uncached(client, key).await? {
            let value = Arc::new(value);
            let mut map = self.cached_value.write().await;
            map.insert(key.clone(), (value.clone(), Instant::now()));
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

/// 可批量获取的缓存值。
#[async_trait]
pub(crate) trait BatchCacheable: MapCacheable {
    /// 从远程批量获取值。
    async fn fetch_uncached_batch(
        client: &Arc<Client>,
        keys: &[Self::Key],
    ) -> Result<Vec<(Self::Key, Self)>, Self::Error>;
}

impl<T: BatchCacheable> CachedMap<T> {
    /// 批量获取缓存，如果缓存过期或不存在则更新缓存。
    pub(crate) async fn get_batch(
        &self,
        client: &Arc<Client>,
        keys: &[T::Key],
    ) -> Result<HashMap<T::Key, Arc<T>>, T::Error> {
        let map = self.cached_value.read().await;
        let mut result = HashMap::new();
        let mut uncached_keys = Vec::new();
        for key in keys {
            // 缓存存在
            if let Some((cached, last_update)) = map.get(key) {
                // 且未过期
                if last_update.elapsed() < *self.duration.read().await {
                    result.insert(key.clone(), cached.clone());
                    continue;
                }
            }
            uncached_keys.push(key.clone());
        }
        if !uncached_keys.is_empty() {
            drop(map);
            result.extend(self.refresh_batch(client, &uncached_keys).await?);
        }
        Ok(result)
    }

    /// 标记缓存为过期。
    pub(crate) async fn make_dirty_batch(&self, keys: &[T::Key]) {
        let mut map = self.cached_value.write().await;
        for key in keys {
            map.remove(key);
        }
    }

    /// 批量强制更新缓存。
    pub(crate) async fn refresh_batch(
        &self,
        client: &Arc<Client>,
        keys: &[T::Key],
    ) -> Result<HashMap<T::Key, Arc<T>>, T::Error> {
        let result = T::fetch_uncached_batch(client, keys).await?;
        let result: HashMap<_, _> = result
            .into_iter()
            .map(|(key, value)| {
                let value = Arc::new(value);
                (key, value)
            })
            .collect();
        let mut map = self.cached_value.write().await;
        map.extend(
            result
                .iter()
                .map(|(key, value)| (key.clone(), (value.clone(), Instant::now()))),
        );
        Ok(result)
    }
}

/// 可全部获取的缓存值。
#[async_trait]
pub(crate) trait AllCacheable: MapCacheable {
    /// 从远程获取所有值。
    async fn fetch_uncached_all(
        client: &Arc<Client>,
    ) -> Result<Vec<(Self::Key, Self)>, Self::Error>;
}

impl<T: AllCacheable> CachedMap<T> {
    /// 刷新所有缓存。
    pub(crate) async fn refresh_all(
        &self,
        client: &Arc<Client>,
    ) -> Result<HashMap<T::Key, Arc<T>>, T::Error> {
        let result = T::fetch_uncached_all(client).await?;
        let result: HashMap<_, _> = result
            .into_iter()
            .map(|(key, value)| {
                let value = Arc::new(value);
                (key, value)
            })
            .collect();
        let mut map = self.cached_value.write().await;
        *map = result
            .iter()
            .map(|(key, value)| (key.clone(), (value.clone(), Instant::now())))
            .collect();
        Ok(result)
    }

    /// 标记所有缓存为过期。
    pub(crate) async fn make_dirty_all(&self) {
        let mut map = self.cached_value.write().await;
        *map = HashMap::new();
    }
}

/// 单对象缓存。
pub(crate) struct Cached<T: Cacheable> {
    cached_value: RwLock<Option<ValueWithLastUpdate<T>>>,
    duration: RwLock<Duration>,
}

/// 可缓存的值。
#[async_trait]
pub(crate) trait Cacheable: Clone {
    /// 错误类型。
    type Error;

    /// 从远程获取值。
    async fn fetch_uncached(client: &Arc<Client>) -> Result<Self, Self::Error>;
}

impl<T: Cacheable> Cached<T> {
    /// 创建一个新的缓存。
    ///
    /// # Arguments
    /// * `duration` - 缓存时长。
    pub(crate) fn new(duration: Duration) -> Self {
        Self {
            cached_value: RwLock::new(None),
            duration: RwLock::new(duration),
        }
    }

    /// 设置缓存时长。
    pub(crate) async fn set_cache_time(&self, duration: Duration) {
        *self.duration.write().await = duration;
    }

    /// 获取缓存，如果缓存过期或不存在则更新缓存。
    pub(crate) async fn get(&self, client: &Arc<Client>) -> Result<Arc<T>, T::Error> {
        let locked = self.cached_value.read().await;
        // 缓存存在
        if let Some((cached, last_update)) = locked.deref() {
            // 且未过期
            if last_update.elapsed() < *self.duration.read().await {
                return Ok(cached.clone());
            }
        }
        drop(locked);
        self.refresh(client).await
    }

    /// 标记缓存为过期。
    pub(crate) async fn make_dirty(&self) {
        let mut locked = self.cached_value.write().await;
        *locked = None;
    }

    /// 强制更新缓存。
    pub(crate) async fn refresh(&self, client: &Arc<Client>) -> Result<Arc<T>, T::Error> {
        let value = Arc::new(T::fetch_uncached(client).await?);
        let mut locked = self.cached_value.write().await;
        *locked = Some((value.clone(), Instant::now()));
        Ok(value)
    }
}
