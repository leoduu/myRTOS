/*
 * @Author: your name
 * @Date: 2021-02-22 07:33:33
 * @LastEditTime: 2021-05-03 16:50:07
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \MDK-ARMf:\project\myRTOS\nucleo-64\hello\RTOS\Inc\osdef.h
 */
#ifndef __OS_DEF_H__
#define __OS_DEF_H__

#include "stdlib.h"
#include "string.h"
#include "stdio.h"
#include "stm32l4xx_hal.h"

#include "osconfig.h"
#include "cpuport.h"

#include "mem.h"

///////////////////////////////////////////////////////////////////////////////////////////
// variable type define

// Thread List
typedef struct list_node
{
    struct list_node *prev;
    struct list_node *next;
} list_t;

typedef struct 
{    
    uint32_t R4;
    uint32_t R5;
    uint32_t R6;
    uint32_t R7;
    uint32_t R8;
    uint32_t R9;
    uint32_t R10;
    uint32_t R11;
    
    // exception
    uint32_t R0;
    uint32_t R1;
    uint32_t R2;
    uint32_t R3;
    uint32_t R12;
    uint32_t LR;
    uint32_t PC;
    uint32_t PSR;
    
} stack_frame_t;


#define FOREVER  0
#define NOWAIT  -1

typedef enum {
    Timer_Leisure,
    Timer_Running,
    Timer_Stop,
    Timer_Timeout,
} timer_status_t;

typedef struct
{
    list_t  list;

    uint32_t stop_tick;
    uint32_t timeout_tick;

    timer_status_t status;  

} timer_t;

// enum of thread status
typedef enum {
    Thread_Init = 0,
    Thread_Ready,
    Thread_Suspended,
    Thread_Terminal
} thread_status_t;

// struct of TCB
struct thread_control_block {
    char        name[THREAD_NAME_LEN];

    list_t      list;

    uint8_t     priority;       //the value is lower ,and the priority is higher
    
    thread_status_t     status;         //ready suspend died

    uint32_t    sp;
    uint32_t    stack_size;
    void        *entry;
    void        *param;

    timer_t     timer;
};
typedef struct thread_control_block TCB_t;

typedef enum os_status
{
    os_ok = 0,
    OS_error,
    os_timeout
} status_t;


///////////////////////////////////////////////////////////////////////////////////////////
// define
#define __OFFSETOF(type, member)        ((size_t) &((type *)0)->member)
#define __CAST_LIST_TO_TCB(addr)        ((TCB_t *)((size_t)(addr) - __OFFSETOF(TCB_t,list))) 
#define __CAST_TIMER_TO_TCB(addr)       ((TCB_t *)((size_t)(addr) - __OFFSETOF(TCB_t,timer))) 
#define __CAST_LIST_TO_TIMER(addr)      ((timer_t *)((size_t)(addr)))


///////////////////////////////////////////////////////////////////////////////////////////
// scheduler.c

status_t os_scheudler_init(void);
status_t os_scheudler_start(void);

TCB_t *os_get_current_thread(void);
uint32_t os_get_current_tick(void);
uint8_t os_get_highest_priority(void);

void os_tick_increase(void);
void os_scheudle(void);

void os_scheudle_check_suspended_thread(void);


///////////////////////////////////////////////////////////////////////////////////////////
// thread.c

TCB_t *os_thread_create(const char      *name, 
                        void            (*entry)(void), 
                        void            *param,
                        uint16_t        priority, 
                        uint32_t        stack_size);

status_t os_thread_init(TCB_t          	*tcb,
                        const char	    *name, 
                        void            (*entry)(void), 
                        void            *param,
                        uint16_t        priority, 
                        uint32_t        *stack_start,
                        uint32_t        stack_size);

status_t __os_thread_list_init(list_t *list);

status_t os_thread_start(TCB_t *thread);
status_t os_thread_ready(TCB_t *thread);
status_t os_thread_resume(TCB_t *thread);
status_t os_thread_wakeup(TCB_t *thread);
status_t os_thread_sleep(TCB_t *thread, uint32_t delay);
status_t os_thread_sleep_ipc(TCB_t *thread, uint32_t delay, list_t **list);
status_t os_thread_terminate(TCB_t *thread);
status_t __os_thread_prio_set(TCB_t *thread, uint8_t prio);
static inline status_t os_delay(uint32_t delay)
{
    return os_thread_sleep(os_get_current_thread(), delay);
}

#define idle_size 128

void idle_entry(void);

extern list_t *ready_queue[THREAD_PRIORITY_MAX];
extern list_t *suspend_queue;
extern list_t *terminate_queue;
extern TCB_t idle;
extern uint32_t idle_stack[idle_size];
extern uint16_t priority_mark;


///////////////////////////////////////////////////////////////////////////////////////////
//timer.c
void os_timer_queue_init(void);
status_t os_timer_init(timer_t *timer);
status_t os_timer_stop(timer_t *timer);
status_t os_timer_resume(timer_t *timer);
status_t os_timer_start(timer_t *timer, uint32_t delay);
status_t os_timer_finish(timer_t *timer);
status_t os_timer_check(void);
status_t os_timer_add_to_list(timer_t *timer);


///////////////////////////////////////////////////////////////////////////////////////////
//server.c
#ifdef  OS_ASSERT

#define os_assert(expr) \
    (expr) ? ((void)0U): os_assert_failed((uint8_t *)__FILE__, __LINE__)
    
void os_assert_failed(uint8_t *file, uint32_t line);
    
#else
  #define os_assert(expr) ((void)0U)
#endif

static inline void __os_list_init(list_t *l) {l->prev = l; l->next = l;}
status_t __os_list_add_end(list_t **l, list_t *n);
status_t __os_list_add_first(list_t **l, list_t *n);
status_t __os_list_add_after(list_t *l, list_t *n);
status_t __os_list_add_before(list_t *l, list_t *n);
status_t __os_list_detch_after(list_t *l, list_t *n);
status_t __os_list_detach(list_t **l, list_t *n);
status_t __os_list_detach_first(list_t **l);


#endif
