# 说明

使用`Voicemeeter`实现耳机与音响同时出声音，但是有些需要戴耳机的时候需要关闭音响的输出.

这时候一般有两种方案，一是关闭音响电源，二是在音响输出端清空设备选择。

方案一电源位置太远，方案二操作太繁琐，所以该软件为此场景出现。

`voicemeeter-quick-switch on` 启用设备
`voicemeeter-quick-switch off` 禁用设备

在快捷方式后添加`on`，`off`参数即可。

因为`https://docs.rs/voicemeeter/latest/voicemeeter/interface/parameters/bus/struct.BusDevice.html#method.name`方法获取到的值不一定准确，所以无法判断当前启用设备是什么，因此只能通过上述参数配置来实现。

# 配置

## bus

 - `OutputA1` // Output A1. Available on all Voicemeeter versions.

 - `OutputA2` // Output A2. Available on all Voicemeeter versions.

 - `OutputA3` // Output A3. Available on Voicemeeter Banana and Potato.

 - `OutputA4` // Output A4. Available on Voicemeeter Potato.

 - `OutputA5` // Output A5. Available on Voicemeeter Potato.

## device

需要操作设备的名字

## sleep

延迟时间, 如果操作无效尝试增加延迟时间