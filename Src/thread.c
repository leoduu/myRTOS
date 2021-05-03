/*
 * @Author: your name
 * @Date: 2021-02-22 07:04:56
 * @LastEditTime: 2021-05-03 17:38:05
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \MDK-ARMf:\project\myRTOS\nucleo-64\hello\RTOS\Src\thread.c
 */
#include "osdef.h"

TCB_t idle;
uint32_t idle_stack[idle_size];

list_t *ready_queue[THREAD_PRIORITY_MAX];
list_t *terminate_queue;
uint16_t priority_mark;

/**
 * @description: 
 * @param {*}
 * @return {status_t}
 */
TCB_t *os_thread_create(const char	    *name, 
                        void          	(*entry)(void), 
                        void            *param,
                        uint16_t      	priority, 
                        const uint32_t  stack_size)
{
    uint32_t *stack_start;

    stack_start = (uint32_t *)malloc(sizeof(uint32_t)*stack_size);

    TCB_t *thread;
    thread = (TCB_t *)malloc(sizeof(TCB_t));

    os_thread_init(thread, name, entry, param, priority, stack_start, stack_size);

    return thread;
}

/**
 * @description: 
 * @param {*}
 * @return {status_t}
 */
status_t os_thread_init(TCB_t         	*thread,
                        const char  	*name, 
                        void           	(*entry)(void), 
                        void            *param,
                        uint16_t       	priority, 
                        uint32_t        *stack_start,
                        uint32_t       	stack_size)
{ 
    // initialize priority
    thread->priority = priority;

    // initialize status
    thread->status = Thread_Init;

    // initialize timer
    os_timer_init(&thread->timer);

    // initialize name
    memcpy(thread->name, name, THREAD_NAME_LEN);
    thread->name[THREAD_NAME_LEN - 1] = '\0';

    // initialize function entry and param
    thread->entry = entry;
    thread->param = param;

    // initialize stack, reserve 16*4 Bytes for stack_frame
    memset((void*)stack_start, '#', stack_size);
    thread->sp = (uint32_t)stack_start + stack_size - sizeof(stack_frame_t);
    thread->stack_size = stack_size;

    // initialize entry and xPSR
    stack_frame_t *stack_frame = (stack_frame_t *)thread->sp;
    stack_frame->R0 = (uint32_t) param;
    stack_frame->PC = (uint32_t) entry;
    // set Thumb flag bit
    stack_frame->PSR = (uint32_t) 0x01000000UL;
    
    // initialize list node
    __os_list_init(&(thread->list));
    
    DEBUG_LOG(("--%s create\r\n", thread->name));

    return os_ok;
}

/**
 * @description: 
 * @param {TCB_t} *thread
 * @return {status_t}
 */
status_t os_thread_start(TCB_t *thread)
{
    
    // if thread is ready or terminal, do nothing
    if (thread->status != Thread_Init)      return os_ok;        
    if (thread->status == Thread_Terminal)  return OS_error;
    
    // add thread to the end of ready_queue 
    __os_list_add_end( &(ready_queue[thread->priority]), &(thread->list));	
    
    // add this priority to priority_mark
    priority_mark |= 1 << thread->priority;
    
    // update statue to ready
    thread->status = Thread_Ready;

    DEBUG_LOG(("--%s start\r\n", thread->name));

    os_scheudle();
    
	return os_ok;
}

/**
 * @description: 
 * @param {TCB_t} *thread
 * @return {status_t}
 */
status_t os_thread_ready(TCB_t *thread)
{
    // if thread is ready or terminal, do nothing
    if (thread->status == Thread_Ready)     return os_ok;        
    if (thread->status == Thread_Terminal)  return OS_error;

    // add thread to the end of ready_queue 
    __os_list_add_end( &(ready_queue[thread->priority]), &(thread->list));
    	
    // add this priority to priority_mark
    priority_mark |= 1 << thread->priority;
    
    // update statue to ready
    thread->status = Thread_Ready;

    DEBUG_LOG(("--%s ready\r\n", thread->name));
    
	return os_ok;
}

/**
 * @description: 
 * @param {TCB_t} *thread
 * @return {status_t}
 */
status_t os_thread_resume(TCB_t *thread)
{
    // if thread is ready or terminal, do nothing
    if (thread->status == Thread_Ready)     return os_ok;    
    if (thread->status == Thread_Terminal)  return OS_error;    

    __disable_irq();

    // deatch thread from suspended_queue and add to the end of ready_queue 
    __os_list_add_end( &(ready_queue[thread->priority]), &(thread->list));	

    // add this priority to priority_mark
    priority_mark |= 1 << thread->priority;
    
    // update statue to ready
    thread->status = Thread_Ready;

    __enable_irq();

    DEBUG_LOG(("--%s resume\r\n", thread->name));

    os_scheudle();
    
	return os_ok;
}

/**
 * @description: thread wakeup but don't join to scheudle immediately
 * @param {TCB_t} *thread
 * @return {*}
 */
status_t os_thread_wakeup(TCB_t *thread)
{
    // if thread is ready or terminal, do nothing
    if (thread->status == Thread_Ready)     return os_ok;    
    if (thread->status == Thread_Terminal)  return OS_error;   
    
    __disable_irq(); 

    // deatch thread from suspended_queue and add to the end of ready_queue 
    __os_list_add_end( &(ready_queue[thread->priority]), &(thread->list));	

    // add this priority to priority_mark
    priority_mark |= 1 << thread->priority;
    
    // update statue to ready
    thread->status = Thread_Ready;

    __enable_irq();

    DEBUG_LOG(("--%s wakeup\r\n", thread->name));
    
	return os_ok;
}

/**
 * @description: 
 * @param {TCB_t} *thread
 * @return {status_t}
 */
status_t os_thread_sleep(TCB_t *thread, uint32_t delay)
{
    // if thread is suspended or terminal, do nothing
    if (thread->status == Thread_Suspended) return os_ok;    
    if (thread->status == Thread_Terminal)  return OS_error;
    //if (delay == 0) return os_ok;

    __disable_irq();    

    // deatch thread from  ready_queue and add to the end of suspended_queue 
    uint8_t prio = thread->priority;
    __os_list_detach( &(ready_queue[prio]), &(thread->list));

    // clear this priority bit if this thread's priority is nothing
    if (ready_queue[prio] == NULL){
        priority_mark &= ~(1 << prio);
    } 
    
    // check if thread need a timer
    if (delay != FOREVER)
        os_timer_start(&thread->timer, delay);

    // update status to suspended
    thread->status = Thread_Suspended;

    __enable_irq();
    
    DEBUG_LOG(("--%s sleep\r\n", thread->name));

    // do a scheudle
    os_scheudle();

    return os_ok;
}

/**
 * @description: 
 * @param {TCB_t} *thread
 * @return {status_t}
 */
status_t os_thread_sleep_ipc(TCB_t *thread, uint32_t delay, list_t **list)
{
    // if thread is suspended or terminal, do nothing
    if (thread->status == Thread_Suspended) return os_ok;    
    if (thread->status == Thread_Terminal)  return OS_error;

    __disable_irq();

    // deatch thread from  ready_queue and add to the end of suspended_queue 
    uint8_t prio = thread->priority;
    __os_list_detach( &(ready_queue[prio]), &(thread->list));
    // add thread to suspend_queue of this mutex
    __os_list_add_end(list, &thread->list);

    // clear this priority bit if this thread's priority is nothing
    if (ready_queue[prio] == NULL){
        priority_mark &= ~(1 << prio);
    } 
    
    if (delay != FOREVER)
        os_timer_start(&thread->timer, delay);

    // update status to suspended
    thread->status = Thread_Suspended;

    __enable_irq();
    
    DEBUG_LOG(("--%s sleep for ipc\r\n", thread->name));

    // do a scheudle
    os_scheudle();

    return os_ok;
}

/**
 * @description: 
 * @param {TCB_t} *thread
 * @return {status_t}
 */
status_t os_thread_terminate(TCB_t *thread)
{
    // if thread already died, do nothing
    if (thread->status == Thread_Terminal)
        return OS_error;

    __disable_irq();

    uint8_t prio = thread->priority;

    // if thread is ready, detach it from ready queue and update priority_mark
    if (thread->status == Thread_Ready) {
        __os_list_detach( &(ready_queue[prio]), &(thread->list));        
        if (ready_queue[prio] == NULL){
            priority_mark &= ~(1 << prio);
        } 
    }
    
    // add thread to terminate queue and do handle in idle
    __os_list_add_end(&terminate_queue, &(thread->list));

    // update status to terminal
    thread->status = Thread_Terminal;

    __enable_irq();

    DEBUG_LOG(("--%s terminate\r\n", thread->name));

    if (thread == os_get_current_thread()) 
        os_scheudle();

    return os_ok;
}

status_t __os_thread_prio_set(TCB_t *thread, uint8_t prio)
{    
    // deatch thread from  ready_queue and add to the end of suspended_queue 
    __os_list_detach( &(ready_queue[thread->priority]), &(thread->list));

    // clear this priority bit if this thread's priority is nothing
    if (ready_queue[thread->priority] == NULL){
        priority_mark &= ~(1 << thread->priority);
    } 

    // change to new priority
    thread->priority = prio;
    __os_list_add_end( &(ready_queue[prio]), &(thread->list));	
    
    // add this priority to priority_mark
    priority_mark |= 1 << thread->priority;
    
    return os_ok;
}

/**
 * @description: 
 * @param {status_t}
 * @return {status_t}
 */
void idle_entry(void)
{
    while(1) {
        // if (led_state)
        //     HAL_GPIO_WritePin(GPIOA, GPIO_PIN_5, GPIO_PIN_SET);
        // else 
            // HAL_GPIO_WritePin(GPIOA, GPIO_PIN_5, GPIO_PIN_RESET);
    }
} 
