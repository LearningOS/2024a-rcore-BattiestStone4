# lab2

本次完成的工作：

在task/mod.rs里添加两个函数set_task_info和increase_syscall_times，并在taskmanager里分别实现，以做到设置task_info与给syscall计数的作用。

syscall计数是在trap/mod.rs里实现的，保证每一次调用都可以被计数。

在TaskControlBlock里设置syscall_times与start_time字段，用来记录syscall次数和运行时间。

由于采用了虚拟地址，可能出现当前任务被两个内存页分割的情况，为此需要根据当前任务的token(current_user_token)来寻找物理内存地址并写入内容。

这也就是translate_ptr的由来。改造之后，可以恢复ch3两个功能的使用。



对于mmap和munmap的实现，由于我们需要申请物理内存，所以我们考虑使用BTreeMap，来标记虚拟地址与物理页帧之间的对应关系（mmap_frames）。

对于port的判断，我们可以放在syscall/process.rs一层来判断，不符合要求就直接返回-1。符合要求的我们再继续到task/mod.rs这一层来操作。

当然，这两个功能的实现也要有memory_set的配合。具体就是mmap负责查找虚拟地址所在vpn对应的pte，并判断pte是否已经被分配，如果没有分配便可以分配页帧，在页表中标记vpn和ppn的关系后，再在mmap_frames里标记vpn与frame的关系。那么munmap基本就是这个操作的逆操作，这里不多赘述。