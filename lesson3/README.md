## 第三课作业  PoE 2

课程里会给出参考资料，大家一定要自己敲一遍**代码**！

注：

1. 提交源代码，运行`cargo test`的测试结果截图，前端UI的截图；
2. 测试应覆盖所有的业务逻辑，如不同的错误场景，以及一切正常的情况下，检查存储的数据是不是预期的那样。
3. 附加题不是必答的，但可以酌情加分。
4. 代码修改在本目录 substrate-node-template 和 substrate-front-end-template 的程序文件里。

第一题：编写存证模块的单元测试代码，包括：

* 创建存证的测试用例；
* 撤销存证的测试用例；
* 转移存证的测试用例；

![](./screenshots/PoE-Unit-Tests.png)

第二题：编写存证模块的UI，包括

* 创建存证的UI

1. Create Claim
![](./screenshots/Create-Claim-Start.png)

2. Create Claim InBlock
![](./screenshots/Create-Claim-InBlock.png)

3. Create Claim finalized
![](./screenshots/Create-Claim-Finalized.png)

* 删除存证的UI

1. Revoke Claim InBlock
![](./screenshots/Revoke-Claim-InBlock.png)

2. Revoke Claim finalized
![](./screenshots/Revoke-Claim-Finalized.png)

* 转移存证的UI

1. Transfer Claim Start
![](./screenshots/Transfer-Claim-Start.png)

2. Transfer Claim InBlock
![](./screenshots/Transfer-Claim-InBlock.png)

3. Transfer Claim Finalized
![](./screenshots/Transfer-Claim-Finalized.png)

第三题（附加题）：实现购买存证的功能代码：

* 用户A为自己的某个存证记录设置价格；
* 用户B可以以一定的价格购买某个存证，当出价高于用户A设置的价格时，则以用户A设定的价格将费用从用户B转移到用户A，再将该存证进行转移。如果出价低于用户A的价格时，则不进行转移，返回错误。

