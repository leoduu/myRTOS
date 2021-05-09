/*
 * @Author: your name
 * @Date: 2021-03-16 20:58:22
 * @LastEditTime: 2021-05-09 01:03:20
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \MDK-ARMf:\project\myRTOS\nucleo-64\hello\RTOS\Src\ipc.c
 */
#include "ipc.h"

#ifdef OS_MUTEX
///////////////////////////////////////////////////////////////////////////////////////////////
//  mutex

list_t *mutex_queue=NULL;

static os_mutex_t *__os_mutex_find_id(uint32_t id)
{
    list_t *list = mutex_queue;
    if (list == NULL) return NULL;
    
    // tarverse the mutex queue
    do {        
        if (__CAST_LIST_TO_MUTEX(list)->id == id)
            return __CAST_LIST_TO_MUTEX(list);
                    
        list = list->next;
    // already traverse the queue once
    } while(list != mutex_queue);

    return NULL;
}

inline uint8_t os_mutex_id_check(uint32_t id)
{
    if (__os_mutex_find_id(id) == NULL) {
        return 0;
    }
    else return 1;
}

status_t os_mutex_create(uint32_t id)
{    
    os_mutex_t *mutex;

    // check if the mutex of this ID is existing
    os_assert(!__os_mutex_find_id(id));
    
    __disable_irq();
    
    mutex = (os_mutex_t *)os_malloc(sizeof(os_mutex_t));
    // initialize parameter
    mutex->id = id; 
    mutex->value = 1;
    mutex->suspend_queue = NULL;
    __os_list_init(&mutex->list);
    // add to mutex queue
    __os_list_add_end(&mutex_queue, &mutex->list);
    
    __enable_irq();

    DEBUG_LOG(("--mutex:%d carete\r\n", mutex->id));
    
    return os_ok;
}

status_t os_mutex_acquire(uint32_t id, int delay)
{
    os_mutex_t *mutex;
    TCB_t *thread;

    // find the mutex of this ID
    mutex = __os_mutex_find_id(id);

    // check if the delay is legal
    os_assert(delay>=NOWAIT);
    // check if thie mutex of this ID is inexistent
    os_assert(mutex);

    __disable_irq();
    
    // thread who want to acquire mutex
    thread = os_get_current_thread();

    if (mutex->own_thread == thread) {
        __enable_irq();
        return os_ok;
    }

    if (mutex->value == 0) {
        /* increse the priority of own_thread, to prevent that
         * the mutex cann't be released by low priority thread.
        */ 
        if (mutex->orignal_prio < thread->priority) {
            __os_thread_prio_set(thread, thread->priority);
        }
        
        __enable_irq();
        
        // do not wait
        if (delay == NOWAIT) return OS_error;

        // sleep and wait
        os_thread_sleep_ipc(thread, delay, &mutex->suspend_queue);
        /* Now the CPU is working with PSP. When we switch context, 
         * the location of code which will be save as PC into PSP
         * is next to where we trigger the PendSV. So, when the
         * thread gets the right to use CPU next time, the code 
         * will jump to the PC that is pulled from thread's PSP 
         * and go through here.
         */
        // if the thread is awakened because of mutex deleting;
        if (__os_mutex_find_id(id) == NULL) {
            return OS_error;
        }
        // if the thread is awakened because of timeout, return timeout
        if (thread->timer.status == Timer_Timeout) {
            DEBUG_LOG(("--mutex:%d %s timeout\r\n", mutex->id, thread->name));
            return os_timeout;
        }
    }

    // It shows that the mutex is vacant if the code comes here
    mutex->value = 0;
    mutex->own_thread = thread; 
    mutex->orignal_prio = thread->priority;
    
    __enable_irq();

    DEBUG_LOG(("--mutex:%d acquire by %s\r\n", mutex->id, thread->name));

    return os_ok;    
}


status_t os_mutex_release(uint32_t id)
{
    os_mutex_t *mutex; 
    list_t *list;

    mutex = __os_mutex_find_id(id);
    // check if the mutex of this ID is inexistent
    os_assert(mutex);

    __disable_irq();

    // release mutex     
    mutex->value = 1;
    mutex->own_thread = NULL;
    // regress priority
    if (mutex->own_thread->priority != mutex->orignal_prio) {
        __os_thread_prio_set(mutex->own_thread, mutex->orignal_prio);
    }
    DEBUG_LOG(("--mutex:%d release\r\n", mutex->id));

    // Check if any threads are waiting
    list = mutex->suspend_queue;   
    if (list != NULL){
        TCB_t *thread = __CAST_LIST_TO_TCB(list);
        // detach the thread form suspend queue
        __os_list_detach( &(mutex->suspend_queue), &(thread->list));
        
        __enable_irq();     
        // wakeup this thread
        os_thread_resume(thread);
    } 

    __enable_irq();
    return os_ok;
}


status_t os_mutex_delete(uint32_t id)
{
    os_mutex_t *mutex;

    mutex = __os_mutex_find_id(id);
    // check if the semaphore of this ID is inexistent
    os_assert(mutex);
    // mutex is working now
    if (mutex->value == 1) return OS_error;

    __disable_irq();

    // detach all threads from suspend queue
    while (mutex->suspend_queue != NULL) {
        TCB_t *thread = __CAST_LIST_TO_TCB(mutex->suspend_queue);
        __os_list_detach_first(&(mutex->suspend_queue));   

        __enable_irq();
        os_thread_resume(thread); 
    }

    // detach the mutex from mutex queue
    __os_list_detach(&mutex_queue, &mutex->list);
    // os_free up memory space 
    os_free(mutex);

    DEBUG_LOG(("--mutex:%d del\r\n", mutex->id));
    __enable_irq();

    return os_ok;
}   

#endif // mutex


#ifdef OS_SEMAPHORE
///////////////////////////////////////////////////////////////////////////////////////////////
//  semaphore 

list_t *sem_queue=NULL;

static os_sem_t *__os_sem_find_id(uint32_t id)
{
    list_t *list = sem_queue;
    if (list == NULL) return NULL;
    
    // tarverse the mutex queue
    do {        
        if (__CAST_LIST_TO_SEM(list)->id == id)
            return __CAST_LIST_TO_SEM(list);
                    
        list = list->next;
    // already traverse the queue once
    } while(list != sem_queue);

    return NULL;
}

int8_t __os_sem_get_value(uint32_t id)
{
    list_t *list = sem_queue;
    if (list == NULL) return -1;
    
    // tarverse the mutex queue
    do {        
        if (__CAST_LIST_TO_SEM(list)->id == id)
            return __CAST_LIST_TO_SEM(list)->value;
                    
        list = list->next;
    // already traverse the queue once
    } while(list != sem_queue);

    return -1;
}

status_t os_sem_create(uint32_t id, int value)
{
    os_sem_t *sem;

    // check if the mutex of this ID is existing
    os_assert(!__os_mutex_find_id(id));    
    // check if the delay is illegal
    os_assert(value>=0);
    
    __disable_irq();

    sem = (os_sem_t *)os_malloc(sizeof(os_sem_t));
    // initialize parameter
    sem->id = id; 
    sem->value = value;
    sem->suspend_queue = NULL;    
    __os_list_init(&sem->list);
    // add to semaphore queue
    __os_list_add_end(&sem_queue, &sem->list);
    
    __enable_irq();

    DEBUG_LOG(("--sem:%d create :%d\r\n", sem->id, sem->value));
    
    return os_ok;
}


status_t os_sem_acquire(uint32_t id, int delay)
{
    os_sem_t *sem;
    TCB_t *thread;

    // find the semaphore of ID
    sem = __os_sem_find_id(id);
    // check if the semaphore of this ID is inexistent
    os_assert(sem);
    // check if the delay is legal
    os_assert(delay>=NOWAIT);
    
    // thread who want to acquire semaphore
    thread = os_get_current_thread();

    // check if semaphore is empty
    if (sem->value == 0) {
        // do not wait 
        if (delay == NOWAIT) return OS_error;
        
        // sleep and wait
        os_thread_sleep_ipc(thread, delay, &sem->suspend_queue);
        /* When the thread resume, the code will 
         * go through here. See details in mutex.
         */
        
        // if the thread is awakened because of semaphore deleting;
        if (__os_sem_find_id(id) == NULL) {
            return OS_error;
        }
        // if the thread is awakened because of timeout, return timeout
        if (thread->timer.status == Timer_Timeout) {
            DEBUG_LOG(("--sem:%d %s timeout\r\n", sem->id, thread->name));
            return os_timeout;
        }
    }

    // It shows that the semaphore has value if the code comes here
    // take a semaphore
    sem->value--;
    
    DEBUG_LOG(("--sem:%d acq by %s :%d\r\n", sem->id, thread->name, sem->value));

    return os_ok;  
}


status_t os_sem_release(uint32_t id)
{
    os_sem_t *sem; 
    list_t *list;

    sem = __os_sem_find_id(id);
    // check if the semaphore of this ID is inexistent
    os_assert(sem);

    __disable_irq();

    sem->value += 1;    
    DEBUG_LOG(("--sem%d rel :%d\r\n", sem->id, sem->value));

    // Check if any threads are waiting
    list = sem->suspend_queue;    
    if (list != NULL){
        TCB_t *thread = __CAST_LIST_TO_TCB(list);
        // detach the thread form suspend queue
        __os_list_detach(&(sem->suspend_queue), &(thread->list));
        
        __enable_irq();

        DEBUG_LOG(("--sem:%d to %s :%d\r\n", sem->id, thread->name, sem->value));
        // wakeup this thread
        os_thread_resume(thread);
    }

    __enable_irq();
    return os_ok;
}


status_t os_sem_delete(uint32_t id)
{
    os_sem_t *sem;

    // check if the semaphore of this ID is inexistent
    sem = __os_sem_find_id(id);
    os_assert(sem);

    __disable_irq();

    // detach all threads from suspend queue
    while (sem->suspend_queue != NULL) {
        TCB_t *thread = __CAST_LIST_TO_TCB(sem->suspend_queue);
        __os_list_detach_first(&(sem->suspend_queue));    
        
        __enable_irq();
        os_thread_resume(thread);
    }

    // detach the mutex from mutex queue
    __os_list_detach(&sem_queue, &sem->list);
    // os_free up memory space 
    os_free(sem);

    __enable_irq();
    DEBUG_LOG(("--sem:%d del\r\n", sem->id));

    return os_ok;
}

#endif // semaphore


#ifdef OS_MAILBOX
///////////////////////////////////////////////////////////////////////////////////////////////
//  mbox 

list_t *mbox_queue;

static os_mbox_t *__os_mbox_find_id(uint32_t id)
{
    list_t *list = mbox_queue;
    if (list == NULL) return NULL;
    
    // tarverse the mutex queue
    do {        
        if (__CAST_LIST_TO_MAILBOX(list)->id == id)
            return __CAST_LIST_TO_MAILBOX(list);
                    
        list = list->next;
    // already traverse the queue once
    } while(list != mbox_queue);

    return NULL;
}

int8_t __os_mbox_get_num(uint32_t id)
{
    list_t *list = mbox_queue;
    if (list == NULL) return -1;
    
    // tarverse the mutex queue
    do {        
        if (__CAST_LIST_TO_MAILBOX(list)->id == id)
            return __CAST_LIST_TO_MAILBOX(list)->num;
                    
        list = list->next;
    // already traverse the queue once
    } while(list != mbox_queue);

    return -1;
}

inline uint8_t os_mbox_id_check(uint32_t id)
{
    if (__os_mbox_find_id(id) == NULL) {
        return 0;
    }
    else return 1;
}

status_t os_mbox_create(uint32_t id, uint32_t max_num)
{
    // check if the mutex of this ID is existing
    os_assert(!__os_mbox_find_id(id));
    // check if the max_num is legal
    os_assert(max_num>0);
        
    __disable_irq();
    
    os_mbox_t *mbox = (os_mbox_t *)os_malloc(sizeof(os_mbox_t));
    // initialize parameter
    mbox->id = id; 
    mbox->num = 0;
    mbox->max_num = max_num;
    mbox->buf_list = NULL;
    mbox->send_queue = NULL;
    mbox->recv_queue = NULL;    
    __os_list_init(&mbox->list);
    // add to mailbox queue
    __os_list_add_end(&mbox_queue, &mbox->list);
    
    __enable_irq();

    DEBUG_LOG(("--mbox%d create\r\n", mbox->id));
    
    return os_ok;
}

status_t os_mbox_send(uint32_t id, uint32_t data, int delay)
{
    os_mbox_t   *mbox; 
    TCB_t     *thread;
    os_mbuf_t *mbuf;

    mbox = __os_mbox_find_id(id);
    // check if the mailbox of this id is inexistent
    os_assert(mbox);
    // check if the delay is legal
    os_assert(delay>=NOWAIT);

    // thread who want to send mail
    thread = os_get_current_thread();

    // check if mail box is full
    if (mbox->num >= mbox->max_num) {
        // do not wait
        if (delay == NOWAIT) return OS_error;
        
        // sleep and wait for vacancy
        os_thread_sleep_ipc(thread, delay, &mbox->send_queue);
        /* When the thread resume, the code will 
         * go through here. See details in mutex.
         */                
        // if the thread is awakened because of mailbox deleting;
        if (__os_mbox_find_id(id) == NULL) {
            return OS_error;
        }
        
        // if the thread is awakened because of timeout, return timeout
        if (thread->timer.status == Timer_Timeout) {
            DEBUG_LOG(("--mbox:%d %s timeout\r\n", mbox->id, thread->name));
            return os_timeout;
        }
    } 
    
    // It shows that the mbox can hold mail if the code comes here

    __disable_irq();
    
    mbuf = (os_mbuf_t *)os_malloc(sizeof(os_mbuf_t));
    __os_list_init(&mbuf->list);
    // copy data
    mbuf->data = data;
    // add to buf queue
    __os_list_add_end(&mbox->buf_list, &mbuf->list);
    // num of mails adds one
    mbox->num++; 

    // Check if any threads is suspended for receiving
    if (mbox->recv_queue){
        thread = __CAST_LIST_TO_TCB(mbox->recv_queue);
        __os_list_detach(&(mbox->recv_queue), &(thread->list));
        
        __enable_irq();
        // wakeup this thread
        os_thread_resume(thread);
    }

    __enable_irq();
    DEBUG_LOG(("--%s send to mbox%d\r\n", thread->name, mbox->id));

    return os_ok;
}


status_t os_mbox_recv(uint32_t id, uint32_t *data, int delay)
{
    os_mbox_t   *mbox;
    TCB_t     *thread; 
    os_mbuf_t *mbuf;

    mbox = __os_mbox_find_id(id);
    // if this id is nonexistent, return error
    if (mbox == NULL) return OS_error; 

    // thread who want to receive mail
    thread = os_get_current_thread();

    // if no mail in mailbox
    if (mbox->num == 0) {
        // do not wait 
        if (delay == NOWAIT) return OS_error;
        
        // sleep and wait
        os_thread_sleep_ipc(thread, delay, &mbox->recv_queue);
        /* When the thread resume, the code will 
         * go through here. See details in mutex.
         */        
        // if the thread is awakened because of mailbox deleting;
        if (__os_mbox_find_id(id) == NULL) {
            return OS_error;
        }

        // if the thread is awakened because of timeout, return timeout
        if (thread->timer.status == Timer_Timeout) {
            DEBUG_LOG(("--mbox:%d %s timeout\r\n", mbox->id, thread->name));
            return os_timeout;
        }
    }    
    
    // It shows that the mailbox isn't empty if the code comes here
    
    __disable_irq();   

    // take out the first mail
    mbuf = (os_mbuf_t *)mbox->buf_list;
    *data = mbuf->data;
    // detach the mail from mbuf list
    __os_list_detach_first(&mbox->buf_list);
    os_free(mbuf);
    mbox->num--;    

    // Check if any threads is suspended for sending
    if (mbox->send_queue) {
        thread = __CAST_LIST_TO_TCB(mbox->send_queue);
        __os_list_detach_first(&mbox->send_queue);
        
        __enable_irq();
        os_thread_resume(thread);
    }      

    __enable_irq();
    DEBUG_LOG(("--%s recv from mbox%d\r\n", thread->name, mbox->id));

    return os_ok;  
}

status_t os_mbox_delete(uint32_t id)
{
    os_mbox_t  *mbox;
    TCB_t      *thread;

    __disable_irq();

    mbox = __os_mbox_find_id(id);
    // check if the mailbox of this ID is inexistent
    os_assert(mbox);

    // detach all threads from suspend queue for sending
    while(mbox->send_queue != NULL) {
        thread = __CAST_LIST_TO_TCB(mbox->send_queue);
        __os_list_detach_first(&(mbox->send_queue));
        os_thread_wakeup(thread);   
    } 
    
    // detach all threads from suspend queue for receiving
    while(mbox->recv_queue != NULL) {
        thread = __CAST_LIST_TO_TCB(mbox->recv_queue);
        __os_list_detach_first(&(mbox->recv_queue));
        os_thread_wakeup(thread);   
    } 
    
    // clear buf list
    while(mbox->buf_list != NULL) {
        __os_list_detach_first(&(mbox->buf_list));  
    } 

    // detach the mailbox from mailbox queue
    __os_list_detach(&mbox_queue, &mbox->list);
    os_free(mbox);

    __enable_irq();
    DEBUG_LOG(("--mbox%d del\r\n", mbox->id));

    return os_ok;
}

#endif // mail box


#ifdef OS_MSG_QUEUE
///////////////////////////////////////////////////////////////////////////////////////////////
//  message queue 

list_t *mqueue_queue;

static os_mqueue_t *__os_mqueue_find_id(uint32_t id)
{
    list_t *list = mqueue_queue;
    if (list == NULL) return NULL;
    
    // tarverse the mutex queue
    do {        
        if (__CAST_LIST_TO_MSG_QUEUE(list)->id == id)
            return __CAST_LIST_TO_MSG_QUEUE(list);
                    
        list = list->next;
    // already traverse the queue once
    } while(list != mqueue_queue);

    return NULL;
}

int8_t __os_mqueue_get_used(uint32_t id)
{
    list_t *list = mqueue_queue;
    if (list == NULL) return -1;
    
    // tarverse the mutex queue
    do {        
        if (__CAST_LIST_TO_MSG_QUEUE(list)->id == id)
            return __CAST_LIST_TO_MSG_QUEUE(list)->size;
                    
        list = list->next;
    // already traverse the queue once
    } while(list != mqueue_queue);

    return -1;
}


static inline status_t __os_msg_send_size_add(os_mqueue_t* mq, uint32_t size)
{
    os_send_size_t *s = (os_send_size_t *)os_malloc(sizeof(os_send_size_t));
    __os_list_init(&s->list);
    s->size = size;
    // add size
    __os_list_add_end(&(mq->size_queue), &(s->list));

    return os_ok;
}

static inline uint32_t __os_msg_send_size_first(os_mqueue_t* mq)
{
    return ((os_send_size_t *)mq->size_queue)->size;
}

status_t os_mqueue_create(uint32_t id, uint32_t max_size)
{
    // check if the mutex of this ID is existing
    os_assert(!__os_mqueue_find_id(id));
    // check if the max_num is legal
    os_assert(max_size>0);  
    
    __disable_irq();
        
    os_mqueue_t *mqueue = (os_mqueue_t *)os_malloc(sizeof(os_mqueue_t));
    // initialize parameter
    mqueue->id = id; 
    mqueue->size = 0;
    mqueue->max_size = max_size;
    mqueue->buf_list = NULL;
    mqueue->send_queue = NULL;
    mqueue->size_queue = NULL;
    mqueue->recv_queue = NULL;    
    __os_list_init(&mqueue->list);
    // add to queue of message queue
    __os_list_add_end(&mqueue_queue, &mqueue->list);
    
    __enable_irq();

    //DEBUG_LOG(("--mqueue%d create\r\n", mqueue->id));
    
    return os_ok;
}


status_t os_mqueue_send(uint32_t id, void *buf, uint32_t size, int delay)
{
    os_mqueue_t *mqueue; 
    TCB_t       *thread;
    os_msg_t    *msg;

    mqueue = __os_mqueue_find_id(id);
    // check if the message queue of this id is inexistent
    os_assert(mqueue);
    // check if the delay is legal
    os_assert(delay>=NOWAIT);


    // thread who want to send message
    thread = os_get_current_thread();

    // if mail box is full, delay or don't wait
    if (mqueue->size+size > mqueue->max_size) {
        // do not wait
        if (delay == NOWAIT) return OS_error;

        __disable_irq();
        __os_msg_send_size_add(mqueue, size);
        __enable_irq();
        // sleep and wait for vacancy
        os_thread_sleep_ipc(thread, delay, &mqueue->send_queue); 
        /* When the thread resume, the code will 
         * go through here. See details in mutex.
         */
        // if the thread is awakened because of mssage queueu deleting;
        if (__os_mqueue_find_id(id) == NULL) {
            return OS_error;
        }        
        // if the thread is awakened because of timeout, return timeout
        if (thread->timer.status == Timer_Timeout) {
            DEBUG_LOG(("--mbox:%d %s timeout\r\n", mqueue->id, thread->name));
            return os_timeout;
        }
    } 
    
    // It shows that the message queue isn't empty if the code comes here
    
    __disable_irq();

    // add to the end of buf list
    msg = (os_msg_t *)os_malloc(sizeof(os_msg_t));
    msg->buf = os_malloc(size);
    msg->size = size;
    __os_list_init(&msg->list);
    memcpy(msg->buf, buf, size);
    __os_list_add_end(&mqueue->buf_list, &msg->list);
    mqueue->size += size; 

    // Check for reveiving suspended
    if (mqueue->recv_queue){
        thread = __CAST_LIST_TO_TCB(mqueue->recv_queue);
        __os_list_detach(&(mqueue->recv_queue), &(thread->list));
        
        __enable_irq();
        // wakeup this thread
        os_thread_resume(thread);
    }

    __enable_irq();
    //DEBUG_LOG(("--%s send to mqueue%d\r\n", thread->name, mqueue->id));

    return os_ok;
}

status_t os_mqueue_recv(uint32_t id, void *buf, uint32_t* len, int delay)
{
    os_mqueue_t *mqueue;
    TCB_t       *thread; 
    os_msg_t    *msg;
    uint32_t    size;

    *len = 0;

    mqueue = __os_mqueue_find_id(id);
    // if this id is nonexistent, return error
    if (mqueue == NULL) return OS_error;

    // thread who want to receive message
    thread = os_get_current_thread();

    // if no message in queue
    if (mqueue->size == 0) {
        // do not wait 
        if (delay == NOWAIT) return OS_error;

        // sleep and wait
        os_thread_sleep_ipc(thread, delay, &mqueue->recv_queue);
        /* When the thread resume, the code will 
         * go through here. See details in mutex.
         */
        // if the thread is awakened because of mssage queueu deleting;
        if (__os_mqueue_find_id(id) == NULL) {
            return OS_error;
        }        
        // if the thread is awakened because of timeout, return timeout
        if (thread->timer.status == Timer_Timeout) {
            DEBUG_LOG(("--mbox:%d %s timeout\r\n", mqueue->id, thread->name));
            return os_timeout;
        }
    }    

    __disable_irq();
    
    if (mqueue->size) {
        // take out the first message
        msg = (os_msg_t *)mqueue->buf_list;
        memcpy(buf, msg->buf, msg->size);
        __os_list_detach(&mqueue->buf_list, &msg->list);
        size = msg->size;
        os_free(msg->buf);
        os_free(msg); 
        mqueue->size -= size;   
    }

    // Check for sending suspended
    if (mqueue->send_queue) {
        // inform send suspended thread
        thread = __CAST_LIST_TO_TCB(mqueue->send_queue);
        // check if bufer area has enough space
        if (mqueue->size+__os_msg_send_size_first(mqueue) <= mqueue->max_size) {
            __os_list_detach_first(&(mqueue->size_queue));
            __os_list_detach(&(mqueue->send_queue), &(thread->list));
            __enable_irq();
            os_thread_resume(thread);        
        }
    }      

    __enable_irq();
    //DEBUG_LOG(("--%s recv from mqueue%d\r\n", thread->name, mqueue->id));

    *len = size;
    return os_ok;  
}

status_t os_mqueue_delete(uint32_t id)
{
    os_mqueue_t *mqueue;
    TCB_t *thread;

    __disable_irq();

    mqueue = __os_mqueue_find_id(id);

    if (mqueue == NULL) return OS_error;
    
    // detach all threads from suspend queue for sending
    while(mqueue->send_queue != NULL) {
        thread = __CAST_LIST_TO_TCB(mqueue->send_queue);
        __os_list_detach_first(&(mqueue->send_queue));
        os_thread_wakeup(thread);   
    } 
    
    // detach all threads from suspend queue for receiving
    while(mqueue->recv_queue != NULL) {
        thread = __CAST_LIST_TO_TCB(mqueue->recv_queue);
        __os_list_detach_first(&(mqueue->recv_queue));
        os_thread_wakeup(thread);   
    } 

    // clear buf list
    do {
        __os_list_detach_first(&(mqueue->buf_list));  
    } while(mqueue->buf_list != NULL);

    __os_list_detach(&mqueue_queue, &mqueue->list);

    os_free(mqueue);

    __enable_irq();
    DEBUG_LOG(("--mqueue%d del\r\n", mqueue->id));

    return os_ok;
}

#endif // message queue
