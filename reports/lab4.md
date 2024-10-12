# lab4

本次完成的工作：

在task/mod.rs里添加两个函数set_task_info和increase_syscall_times，并在taskmanager里分别实现，以做到设置task_info与给syscall计数的作用。

syscall计数是在trap/mod.rs里实现的，保证每一次调用都可以被计数。

在TaskControlBlock里设置syscall_times与start_time字段，用来记录syscall次数和运行时间。

由于采用了虚拟地址，可能出现当前任务被两个内存页分割的情况，为此需要根据当前任务的token(current_user_token)来寻找物理内存地址并写入内容。

这也就是translate_ptr的由来。改造之后，可以恢复ch3两个功能的使用。



对于mmap和munmap的实现，由于我们需要申请物理内存，所以我们考虑使用BTreeMap，来标记虚拟地址与物理页帧之间的对应关系（mmap_frames）。

对于port的判断，我们可以放在syscall/process.rs一层来判断，不符合要求就直接返回-1。符合要求的我们再继续到task/mod.rs这一层来操作。

当然，这两个功能的实现也要有memory_set的配合。具体就是mmap负责查找虚拟地址所在vpn对应的pte，并判断pte是否已经被分配，如果没有分配便可以分配页帧，在页表中标记vpn和ppn的关系后，再在mmap_frames里标记vpn与frame的关系。那么munmap基本就是这个操作的逆操作，这里不多赘述。



由于代码框架实现了一个分离，所以我们先要将之前所有的代码都在task/processor.rs里实现。比如在run_tasks()里记录进程开始时间与设置stride值。同时set_task_info()也放在了里面。

sys_spawn主要就是将path所在字符串的页表中的地址先找到，然后根据这个名字去寻找程序的数据。之后我们创建一个新的tcb，并将tcb的内容复制进去，再设置好最后的父子关系，最后加入到调度队列中，返回pid。

而stride的调度处理是在task/processor.rs设置的，在task/manager.rs里实现的。task/manager.rs里会遍历队列找到stride最小值的进程，并将其移除队列交给处理器来执行。至于set_prio，则可以直接对tcb_inner中的相关字段进行处理即可。



由于文件系统大改，sys_spawn需要适配上新的文件系统。这里我原本的写法是将这些操作全部放置于syscall/process.rs下，但是问题在于data会莫名其妙的数据置零。所以只能将这个任务扔进task中了。

link的话首先就是在Inode里加入link_num字段，然后写两个方法对这个link_num进行维护。之后在easy-fs中vfs.rs写两个link和unlink方法来实现他们在文件系统上的link。具体就是先根据old_name找到它的disk_inode，再对root_inode进行修改，把新的目录项写入。unlink是逆操作，不多赘述。

fstat是由get_stat来实现的，把每个字段都给出去就好了。