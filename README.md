## 学习进度表

预备阶段

1. 通过Tauri的文档了解了窗口磨砂的实现方式，目前windows的效果还没有测试
2. 找到了文件管理器的一个基本设计方案，并且用html和css画出了大该框架

正式阶段

1.  完善了sidebar的收折功能，并且搞清楚了文件管理器查找的基本思路
2. 把标签栏做好了，搞了一点简单的css样式
3. 对于rust后端进行初步的系统性设计，考虑把后端分为多个模块，用单一模块聚合与tauri js api进行对接
4. 找到了现成的bplustree发现无法使用，想到用数据库sled模块一步到位完成bplustree的序列化和反序列化
5. 测试， 找到了可以遍历文件系统和多路并行的插件walkdir ，rayon，使用chrnon实现了最近24小时使用的文件路径的查找
6. 对于整个项目都做了大致的测试集成，由于前端任务还没有完善，还没有做测试，测试了前后端的tauri接口是否可用，一切顺利，还在进行前端移动插件的挑食
7. 