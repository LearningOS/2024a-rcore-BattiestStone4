# lab1

本次完成的工作：

在task/mod.rs里添加两个函数set_task_info和increase_syscall_times，并在taskmanager里分别实现，以做到设置task_info与给syscall计数的作用。

syscall计数是在trap/mod.rs里实现的，保证每一次调用都可以被计数。

在TaskControlBlock里设置syscall_times与start_time字段，用来记录syscall次数和运行时间。