### rust 版本的shm spmc

### 要求
- 需要spmc库的静态链接库
- c中发布的消息要在repr(C)中声明
- rust中要实现MesssageHandler特征

### 代码
```rust
trait MesssageHandler {
    fn on_message(&self, msg: &Test);
}
struct Strategy {
}

impl MesssageHandler for Strategy {
    fn on_message(&self, data: &Test) {
        println!("msg recv {} {} {}", data.a, data.b, data.c);
    }
}

fn main() {
    let s = Strategy{};
    SPMCVarQ::start("/hq_test", &s);
}
  

```
