import asyncio
import os

from loguru import logger

import awr

# 登录账号
account = os.environ.get("AWR_ACCOUNT")
if not account:
    logger.error("AWR_ACCOUNT not set")
    exit(1)
account = int(account)

password = os.environ.get("AWR_PASSWORD")
if not password:
    logger.error("AWR_PASSWORD not set")
    exit(1)

# 测试用好友账号
test_account = os.environ.get("QQ_TEST_ACCOUNT")

async def main():
    client = await awr.Dynamic().login(int(account), "./bots")

    uin = client.uin
    logger.info(f"账号: {uin}")

    is_online = client.is_online()
    logger.info(f"在线状态: {is_online}")

    info = await client.get_account_info()
    logger.info(f"昵称: {info.nickname}")
    logger.info(f"年龄: {info.age}")
    logger.info(f"性别: {info.gender}")

    logger.info("################################")

    friend_list = await client.get_friend_list()
    logger.info(f"好友数量: {friend_list.total_count}")
    logger.info(f"在线好友数量: {friend_list.online_count}")
    friend_count = len(list(friend_list.friends()))
    logger.info(f"遍历好友列表, 总数: {friend_count}")
    logger.info(
        f"好友分组: {[friend_group.name for friend_group in friend_list.friend_groups()]}"
    )

    logger.info("################################")

    if test_account and (friend := friend_list.find_friend(int(test_account))):
        logger.info(f"好友账号: {friend.uin}")
        logger.info(f"好友昵称: {friend.nickname}")
        logger.info(f"好友备注: {friend.remark}")
        logger.info(f"好友分组: {friend.group_id}")

        selector = friend.as_selector()
        logger.info("发送戳一戳")
        await selector.poke()
        await asyncio.sleep(0.5)

        logger.info("发送消息")
        receipt = await client.friend(1779724477).send("在吗", awr.Face(0))
        await asyncio.sleep(2)
        logger.info("撤回消息")
        await receipt.recall()

        logger.info("构造消息链")
        message = await awr.MessageContent.build_friend_message("您好")
        await friend.send(message)

    logger.info("################################")

    group_list = await client.get_group_list()
    logger.info(f"群数量: {len(group_list)}")
    logger.info(f"群列表: {[group.name for group in group_list]}")

    logger.info("测试完毕")
    await client.alive()


try:
    asyncio.run(main())
except KeyboardInterrupt:
    import sys

    sys.exit(0)
