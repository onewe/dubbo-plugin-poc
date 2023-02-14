# dubbo-plugin-poc
## 关于在 Dubbo-rs 中实现插件机制的一种构想
rust 是一种静态语言,想要实现 java 中 spi 的效果是很难的, 但是并不是没有办法, 之前讨论过使用 wasm 方式做但本人觉得这种方式会稍微复杂点,而且还要集成一个 runtime 感觉有点划不来.那么
就还剩下一种方法 使用 FFI 的当时调用动态库, 这样以来就有点 SPI 的味道了. 但是这种方法是 unsafe 的, 有 crash 的风险, 与其被 unsafe 束手束脚不如放手的做, 控制好 safe 和 unsafe的
边界就行.

## 如何使用 FFI 实现 extension-loader?
这块儿的代码已经写了个 demo ,实现起来还算是方便, 思路就是查找路径下的 动态库文件 然后加载进去 
https://github.com/onewe/dubbo-plugin-poc/blob/main/extension-loader/src/lib.rs

## 如何写一个扩展库？
如果在 rust 的框架下 写一个 扩展库是非常方便的, 只要按照约定写一个扩展, 后续可以使用宏来固定这种写法
https://github.com/onewe/dubbo-plugin-poc/blob/main/http-protocol-extension/src/lib.rs
