# Overview
Learn operating system by building, following [Philipp Oppermann's blog](https://os.phil-opp.com/)

[系列视频](https://www.bilibili.com/video/BV1Ez42117Hq)

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

# Async/Await 并发与协程
## Single-core CPUs vs Multi-core CPUs
## Preemptive vs Cooperative
### Preemptive
- 操作系统通过中断（例如定时器）获取CPU，进行任务的调度
- 操作系统需要保存原进程的“上下文”，包括调用堆栈和CPU寄存器信息等
- 为减小上下文切换开销，引入“线程”概念，本质上为“独立管理”的调用栈，这样在上下文切换时操作系统只需要保存并恢复寄存器的状态，而不用每次都重新保存整个堆栈
- 好处：操作系统可以完全掌握每一个任务（线程）的运行时间，不需要依赖任务自己不“流氓”
- 缺点：每个任务/线程需要自己的调用栈空间，所需的内存空间更大（Goroutine 轻量级线程）；另外保存/恢复全量CPU寄存器的开销也不容小视
### Cooperative
- 任务间通过“协作”，主动“让出”和“接管”CPU，而不是由操作系统强制剥离，狭义上的“协程”
- 协程可以自己控制让出CPU的时间，比如需要IO的时候，一般是语言级/应用级的实现，"yield"之类的关键字，有显式的也有隐式的
- 常常与“异步”操作结合使用
- 好处：更小的切换成本，任务可以按需保存需要的状态信息，而不是全部的（典型的有状态机实现），这样可以使用一个调用栈，资源开销要小得多，这也是为什么普遍都说协程可以在相同内存容量下，创建比线程多得多的数量
- 缺点：避免不了有意或无意的“流氓”任务，一个坏任务可能导致整体挂死，作为一个操作系统，假设所有任务都是正常任务不现实

## Async/Await in Rust
### trait Future
提供 `poll` 接口，进行异步化操作
#### naive: loop + 无限轮询
#### future combinator
- 粗略解读，本质是为一个Future增加另一个Future包装，内部Future自己执行自己的，外部Future在真正需要获取相关性质时，再poll，有点像lazy load（类似iterator的设计）
- 好处：能实现真正的异步，并且可以利用一切编译器优化能力，可以具有出色的性能；
- 缺点：写起来会比较复杂，特别是配合Rust的类型系统以及基于闭包的接口设计（一旦涉及到闭包+ownership的问题，和Rust编译器斗争起来会非常痛苦）
### Async/Await Pattern
- “用同步的方式写异步代码”，由编译器完成这层转换，例如状态机实现
- 某种自动生成Future的语法糖
- Pinning 略
- Executors and Wakers ，类比CPU核心/线程/GMP模型等，这也是我们要实现的部分
## 实现我们自己的任务调度器
