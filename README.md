# SysYust-again

上一世，我们手搓编译器，中道崩殂。这一世，我们携三十分钟静态分析功力重生归来，定要教那CPU流水线为我们倒转（这次真的Rust了。）

## 项目结构概述

项目组织为一个 Cargo Workspace，位于顶层的 package 是编译器的驱动程序，其余辅助库位于 `lib` 目录下。

- `util` 通用工具
- `ast` AST表示
- `parser` 解析器
- `interpreter` 解释器
- `ast_ir` AST2IR 转换
- `ir` IR表示
- `codegen` 代码生成器
