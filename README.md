Rust 嵌入式开发

这是一个 Rust 嵌入式开发的示例项目，旨在展示如何使用 Rust 进行嵌入式系统编程。

目前，Rust 在 ESP 平台上有两种主要开发方式：

1. **标准库（std）模式**：基于 ESP-IDF 框架，适合需要丰富功能（如网络、文件 I/O、多线程等）的开发者，入门门槛较低，适合初学者。
2. **裸机（no_std）模式**：不依赖操作系统，直接操作硬件，适合对性能和资源有极致要求的开发者。

## 硬件

ESP32-C3

项目使用的是[ESP32C3-CORE 开发板](https://wiki.luatos.com/chips/esp32c3/board.html)

![ESP32C3-CORE 开发板 引脚图](./imgs/ESP32-C3-CORE.png)

## 工具

- [Rust](https://www.rust-lang.org/): Rust 编程语言。

## 参考资料

- [《ESP32 团队写的 Rust SDK 开发参考文档》](https://esp32.implrust.com/index.html)
- [《The Rust on ESP Book 简体中文版》](https://narukara.github.io/rust-on-esp-book-zh-cn/introduction.html)

- [Embedded Rust (no_std) on Espressif](https://docs.espressif.com/projects/rust/no_std-training/)

- [Embedded Rust on Espressif](https://docs.esp-rs.org/std-training/)

- [Embedded Rust on Espressif 简体中文版](https://narukara.github.io/std-training-zh-cn/)

## 项目示例

### no_std 环境

- `hello-world`：最简单的 Rust 嵌入式程序，输出 "Hello, world!"。
- `active-buzzer`: 控制蜂鸣器。
- `led-pwm`：使用 PWM 控制 LED 呼吸灯效果。
- `blinky-embassy`：控制 LED 闪烁效果。
- `button`：使用按钮控制 LED 闪烁效果。(循环检测)
- `button-interrupt`：使用按钮中断控制 LED 闪烁效果。（中断触发）
- `http-client`：使用 HTTP 客户端发送请求。
- `oled-spi`：使用 SPI 接口驱动 OLED 屏幕。

### std 环境

- `hello-world-std`：最简单的 Rust 嵌入式程序，输出 "Hello, world!"。
- `http-client-std`：编写小型客户端，通过 HTTP，HTTPS 连接到互联网获取数据。
- `wifi`：使用 ESP32-C3 芯片的 Wi-Fi 功能。

## 使用到的电子元件

### 面包板

![](./imgs/Breadboard.png)

### LED

![](./imgs/LED.jpg)

### 四角按钮

![](./imgs/Button.jpg)

### OLED 屏幕

![](./imgs/OLED.png)

与 ESP32-C3 开发板连接
|SSD1306 引脚|ESP32-C3 引脚|引脚编号|引脚功能|
|:-:|:-:|:-:|:-:|
|SCL/SCK|GPIO2|19|SPI2 时钟 (Clock)|
|SDA/MOSI|GPIO3|20|SPI2 主机输出从机输入 (Data)|
|DC|GPIO8|29|数据/命令选择 (Data/Command)|
|RST|GPIO4|28|复位|
CS GPIO7 23 片选 (Chip Select)
