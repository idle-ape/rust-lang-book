// 在 mod front_of_house 后使用分号，而不是代码块 {}，表示在另一个与模块同名的文件中加载模块的内容
/* 
在 Rust 中，**内联模块定义（使用 `mod {}` 代码块）**和**文件系统模块定义（使用 `mod name;` 分号声明）**有不同的适用场景。以下是内联模块定义的典型使用情况：

---

## 1. **模块内容较少，不值得单独一个文件**
   - 当模块代码很少（比如只有几个简单的函数或类型），单独创建一个文件反而会增加项目复杂度时。
   - **示例**：
     ```rust
     mod utils {
         pub fn helper() {
             println!("This is a helper function");
         }
     }
     ```

---

## 2. **模块是私密的，不需要外部访问**
   - 如果模块只用于当前文件内部，不需要被其他文件引用，内联定义更合适。
   - **示例**：
     ```rust
     mod internal_math {
         fn square(x: i32) -> i32 {
             x * x
         }
         pub fn calculate(x: i32) -> i32 {
             square(x) + 1
         }
     }
     ```

---

## 3. **测试模块（`#[cfg(test)]`）**
   - Rust 的单元测试通常直接写在源码文件中，使用 `#[cfg(test)] mod tests {}`。
   - **示例**：
     ```rust
     #[cfg(test)]
     mod tests {
         use super::*; // 导入父模块的内容
         #[test]
         fn test_something() {
             assert_eq!(2 + 2, 4);
         }
     }
     ```

---

## 4. **条件编译模块（`#[cfg(...)]`）**
   - 当某些代码只在特定平台或条件下编译时，内联定义更清晰。
   - **示例**：
     ```rust
     #[cfg(target_os = "linux")]
     mod linux_specific {
         pub fn run_command() {
             println!("Running Linux command");
         }
     }
     ```

---

## 5. **快速原型设计**
   - 在早期开发阶段，可能先内联定义模块，后续再拆分到单独文件。

---

## **对比：什么时候用 `mod name;`（文件系统模块）？**
✅ **适用场景**：
- 模块代码较多，单独文件更清晰。
- 模块需要被多个地方引用。
- 项目较大，需要更好的代码组织。

**示例**：
```rust
mod front_of_house; // 从 `front_of_house.rs` 或 `front_of_house/mod.rs` 加载
```

---

## **总结**
| **内联 `mod {}`** | **文件系统 `mod name;`** |
|------------------|------------------------|
| 少量代码 | 大量代码 |
| 私有/内部使用 | 公开/共享模块 |
| 测试模块 (`#[cfg(test)]`) | 主业务逻辑 |
| 条件编译 (`#[cfg(...)]`) | 稳定代码 |
| 快速原型 | 正式项目结构 |

**一般建议**：
- **优先考虑 `mod name;`（文件系统模块）**，因为 Rust 鼓励模块化设计。
- **仅在内联更清晰时才用 `mod {}`**，比如测试、条件编译或小工具函数。

这样你的代码结构会更清晰、更易于维护！
*/
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
