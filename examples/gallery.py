import os
import awr
import asyncio

# 登录账号
account = os.environ.get("QQ_ACCOUNT")
if not account:
    print("QQ_ACCOUNT not set")
    exit(1)

# 测试用好友账号
test_account = os.environ.get("QQ_TEST_ACCOUNT")

async def main():
    client = await awr.Dynamic().login(int(account), "./bots")
    
    uin = client.uin
    print(f"账号: {uin}")

    is_online = client.is_online()
    print(f"在线状态: {is_online}")

    info = await client.get_account_info()
    print("昵称:", info.nickname)
    print("年龄:", info.age)
    print("性别:", info.gender)

    print("################################")

    friend_list = await client.get_friend_list()
    print("好友数量:", friend_list.total_count)
    print("在线好友数量:", friend_list.online_count)
    friend_count = len(list(friend_list.friends()))
    print("遍历好友列表, 总数:", friend_count)    
    print("好友分组:", [friend_group.name for friend_group in friend_list.friend_groups()])

    print("################################")

    if test_account and (friend := friend_list.find_friend(int(test_account))):             
        print("好友账号:", friend.uin)
        print("好友昵称:", friend.nickname)
        print("好友备注:", friend.remark)
        print("好友分组:", friend.group_id)

        selector = friend.as_selector()
        print("发送戳一戳")
        await selector.poke()
        await asyncio.sleep(0.5)

        print("发送消息")
        receipt = await client.friend(1779724477).send(["在吗", awr.message.face(0)])
        await asyncio.sleep(2)
        print("撤回消息")
        await receipt.recall()        

    print("################################")

    group_list = await client.get_group_list()
    print("群数量:", len(group_list))
    print("群列表:", [group.name for group in group_list])

    await client.alive()
    
try:
    asyncio.run(main())
except KeyboardInterrupt:
    import sys
    sys.exit(0)
