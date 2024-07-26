[Crafting Interpreters](https://craftinginterpreters.com/)里的 jLox 的 Rust 实现

做了以下改进：

- 4 章扫描，添加了对 C 样式`/ * ... * /`屏蔽注释的支持。

- 6 章解析表达式，添加了对逗号运算符的支持。

  语法表示规则修改

  ```
  expression   -> assignment ( "," assignment )* ;
  arguments    → assignment ( "," assignment )* ;
  ```
  
- 6 章添加了对 C 风格的条件操作符或 "三元 "操作符 `?:` 的支持

  语法表示规则修改

  ```
  assignment     → ( call "." )? IDENTIFIER "=" assignment
                 | conditional ;
  conditional    → logic_or ( "?" expression ":" conditional )? ; // 三元表达式是右结合，因此 : 后面还是 conditional
  ```

- 7 章支持字符串与数字相加

- 9 章支持 break 语句。与 return 一样用返回`Result`里的错误实现。

- 11 章语义分析，将解析信息存储到语法树节点本身里，代替原 Java 版使用的`Map<Expr, Integer> locals`存储信息。

  这是因为如果不覆盖 `hashCode` 和 `equals` 方法，Java 会使用 Object 的内存地址来生成哈希码和进行比较。因此，不同的 `Expr` 对象即使表示相同的表达式，它们的哈希码也是不同的。并且即使表示相同的表达式，Java 中的 `==` 操作符用于比较两个引用是否指向同一个对象，它们在 `==` 比较中也是不同的。

  而 Rust 的 Hash trait 跟结构体本身的地址没关系，即使自己实现 Hash trait，结构体本身并不直接持有或知道自己的地址。
  
  