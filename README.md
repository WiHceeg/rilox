[Crafting Interpreters](https://craftinginterpreters.com/)的 Rust 实现

做了以下改进：

- 11 章语义分析，将解析信息存储到语法树节点本身里，代替原 Java 版使用的`Map<Expr, Integer> locals`存储信息。
  这是因为如果不覆盖 `hashCode` 和 `equals` 方法，Java 会使用 Object 的内存地址来生成哈希码和进行比较。因此，不同的 `Expr` 对象即使表示相同的表达式，它们的哈希码也是不同的。并且即使表示相同的表达式，Java 中的 `==` 操作符用于比较两个引用是否指向同一个对象，它们在 `==` 比较中也是不同的。
  而 Rust 的 Hash trait 跟结构体本身的地址没关系，即使自己实现 Hash trait，结构体本身并不直接持有或知道自己的地址。
- 
  