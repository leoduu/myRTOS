/*
 * @Author: your name
 * @Date: 2021-02-22 05:08:35
 * @LastEditTime: 2021-05-03 15:42:48
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \MDK-ARMf:\project\myRTOS\nucleo-64\hello\RTOS\Src\scheudler.c
 */
#include "osdef.h"

TCB_t *current_thread;
TCB_t *next_thread;

volatile uint32_t os_tick;

extern const uint8_t __priority_array[256];

/**
 * @description: Initialize scheudler
 * @param void
 * @return status_t
 */
status_t os_scheudler_init(void)
{    
    __disable_irq();
	
    // initialze memory management
    os_mem_init();
	
    // initialize all queue
    for (uint8_t i=0; i<THREAD_PRIORITY_MAX; ++i) {
        ready_queue[i] = NULL;
	}           
    terminate_queue = NULL;

    // nothing in all priority
    priority_mark = 0;
    // initialize os_tick to 
    os_tick = 0;

    // initialize priority_mark
    priority_mark = 0;    

		DEBUG_LOG(("--os init\r\n"));
	
    // set pendsv priority to lowest
    NVIC_SetPriority(PendSV_IRQn, 0xFF);
    
    // initialize systick, generate 1ms interrupt
    os_SysTick_Config(SystemCoreClock / OS_TICK_PER_SECOND);	
    SysTick->CTRL &= ~(SysTick_CTRL_ENABLE_Msk);
    
    return os_ok;
}

status_t os_scheudler_start(void)
{

    // create the idle thread
    os_thread_init(&idle, "idle", idle_entry, NULL, 15, idle_stack, sizeof(idle_stack));
		// put idle to ready queue
    os_thread_ready(&idle);
    current_thread = &idle;	
    
	    DEBUG_LOG(("--os start!!!\r\n"));

    __enable_irq();
	
    os_thread_start_first_thread(&idle.sp);
    
    return os_ok;
}

/**
 * @description: get current thread
 * @param void
 * @return Thread Control Block
 */
inline TCB_t *os_get_current_thread(void)
{
    return current_thread;
}

inline uint32_t os_get_current_tick(void)
{
    return os_tick;
}

/**
 * @description: 
 * @param {*}
 * @return {*}
 */
static inline uint8_t os_get_highest_priority(void)
{
    if (priority_mark & 0xFF) 
        return __priority_array[priority_mark & 0xFF];

    return __priority_array[(priority_mark & 0xFF00) >> 8 ] + 8; 
}

/**
 * @description: 
 * @param {*}
 * @return {*}
 */
void os_tick_increase(void)
{
    __disable_irq();

	// os time increase
    os_tick++;
	// check and resume wakeup thread
    os_timer_check();

    os_scheudle();

}

/**
 * @description: 
 * @param {*}
 * @return {*}
 */
void os_scheudle(void)
{
    TCB_t *thread = os_get_current_thread();
    uint8_t current_prio = os_get_highest_priority();

    // get next thread
    if (thread->status == Thread_Ready && current_prio == thread->priority) {

        next_thread = __CAST_LIST_TO_TCB(thread->list.next);
        // if next thread is same, do nothing
        if (thread == next_thread) {
            __enable_irq();
            return;
        } else{}
    } else 
        next_thread = __CAST_LIST_TO_TCB(ready_queue[current_prio]);


    // put current thread to tail of the queue
    if (current_thread->status == Thread_Ready)
        ready_queue[current_thread->priority] = current_thread->list.next;
        
    // update current thread
    current_thread = next_thread;
    
        __enable_irq();

    // switch to next thread
    os_thread_switch(&thread->sp, &next_thread->sp);

}


const uint8_t __priority_array[] = 
{
    /* 00 */ 0, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    /* 10 */ 4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    /* 20 */ 5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    /* 30 */ 4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    /* 40 */ 6, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    /* 50 */ 4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    /* 60 */ 5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    /* 70 */ 4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    /* 80 */ 7, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    /* 90 */ 4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    /* A0 */ 5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    /* B0 */ 4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    /* C0 */ 6, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    /* D0 */ 4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    /* E0 */ 5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
    /* F0 */ 4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0
};
