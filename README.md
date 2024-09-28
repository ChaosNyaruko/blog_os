# Overview
Learn operating system by building, following [Philipp Oppermann's blog](https://os.phil-opp.com/)

# Heap Allocation
- 局部变量与全局变量
- 动态内存
  - 堆和栈
  - 堆内存管理与常见错误
    - 内存泄漏：需要"手动"释放
    - Use After Free (https://cwe.mitre.org/data/definitions/416.html?)
    - Double Free
  - Rust的内存管理和Ownership机制
  - 为什么我们需要堆内存分配
    - 性能问题
    - 面对具有真正生命周期和有不同大小的变量不友好
- 实现
  - Rust的Allocator接口
  - 类比C++ std库，例如Vector，但更底层，需要一些额外的设置，因为我们没有操作系统提供的new/delete/malloc/free接口
  - GlobalAlloc Trait
  - 初始化可用的动态内存/堆
  - 嵌入一个可用的Allocator

# Allocator Designs
## 基本原则
- alloc: 返回一片"未使用"的内存
- dealloc: 释放一些被使用的内存，使其能被再次使用
## 其他可能需要考虑的因素
- 充分利用可用空间，减少"碎片"和"浪费"
- 并发/多核的扩展能力
- 性能
- 好的分配器像jemalloc之类的实现都很复杂，但设计内核用的分配器时一般都不希望太复杂

## Bump Allocator
- "线性"、"递进"式分配
- 很少直接使用 arena allocation
- 优点：性能高
- 缺点：在「所有」的分配都释放后，才能再次使用 -> 有理论上足够的空间也使用不了 -> external "fragmentation"

## Linked List Allocator
- 用链表管理可用内存，不受"连续性"的限制
- 优点：可以直接"复用"内存，所以更通用
- 缺点：
  - 没有及时"合并"可用内存，因"碎片"问题(external fragmentation)，导致分配失败（主要是实现上的缺陷，并不是链表本身的缺陷）
  - 性能不可控，多数情况下是变差很多（是链表这一选型根本导致的）

## Fixed-Size Block Allocator
- 在碎片和分配性能间做取舍，引入一些固定大小的内存"基本单元"，以及多个链表
- 分配时找离目标最近的更大的内存单元块
- 针对大片内存的fallback
- 某种类型单元分配完后，新建一些（用fallback分配器或者拆分大单元为小单元）
- 优点：
  - 比单纯的链表分配快得多
- 缺点：
  - 浪费一些内存(internal fragmentation)，可以通过定义更多级的内存单元来缓解（最坏1/2浪费，平均1/4浪费）
- 变种：Slab Allocator 和 Buddy Allocator

## It’s also important to remember that each kernel implementation has a unique workload, so there is no “best” allocator design that fits all cases.

