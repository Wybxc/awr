#![feature(let_chains)]
//! libawr 的示例代码。

use anyhow::Result;
use libawr::{login, msg};
use ricq::msg::elem::Face;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let account = std::env::var("AWR_ACCOUNT")?.parse()?;
    let password = std::env::var("AWR_PASSWORD")?;
    let test_account = std::env::var("AWR_TEST_ACCOUNT")
        .ok()
        .map(|s| s.parse())
        .transpose()?;

    let (client, alive) = login!(
        account,
        password = &password,
        protocol = ricq::Protocol::IPad,
    )
    .await?;

    tracing::info!("登录成功！");

    tracing::info!("账号: {}", client.uin);
    tracing::info!("在线状态: {}", client.is_online());

    tracing::info!("################################");

    let friend_list = client.get_friend_list().await?;
    tracing::info!("好友数量: {}", friend_list.total_count);
    tracing::info!("在线好友数量: {}", friend_list.online_count);
    let friend_count = friend_list.friends().len();
    tracing::info!("好友总数: {}", friend_count);
    tracing::info!(
        "好友分组: {}",
        friend_list
            .friend_groups()
            .values()
            .map(|group| group.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );

    tracing::info!("################################");

    if let Some(test_account) = test_account
        && let Some(friend) = client.get_friend(test_account).await? {
        tracing::debug!("好友信息: {:?}", friend);
        tracing::info!("好友账号: {}", friend.uin);
        tracing::info!("好友账号昵称: {}", friend.nickname);
        tracing::info!("好友账号备注: {}", friend.remark);
        tracing::info!("好友账号分组: {}", friend.group_id);

        tracing::info!("发送好友戳一戳");
        friend.poke().await?;

        tracing::info!("发送好友消息");
        let receipt = friend.send(msg!("在吗", Face::new(0))).await?;
        tokio::time::sleep(Duration::from_secs(2)).await;
        tracing::info!("撤回好友消息");
        receipt.recall().await?;
    }

    tokio::select! {
        _ = count_down(5) => {
            tracing::info!("等待结束，自动退出");
        },
        _ = alive.auto_reconnect() => {
            tracing::error!("自动重连失败");
        }
    }

    Ok(())
}

async fn count_down(max: u64) {
    for i in 0..max {
        tracing::info!("保持连接中，剩余: {}", max - i);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
