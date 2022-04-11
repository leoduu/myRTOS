/*
 * @Author: your name
 * @Date: 2021-03-12 00:46:04
 * @LastEditTime: 2021-05-03 16:50:19
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \MDK-ARMd:\project\myRTOS\nucleo-64\hello\RTOS\Src\timer.c
 */
#include "osdef.h"

list_t *timer_queue;

inline void os_timer_queue_init(void)
{
    timer_queue = NULL;
    timer_queue->prev = NULL;
    timer_queue->next = NULL;
}

status_t os_timer_init(timer_t *timer)
{
    __disable_irq();

    timer->timeout_tick = 0;
    timer->status = Timer_Leisure;
    __os_list_init(&timer->list);

    __enable_irq();

    return os_ok;
}

status_t os_timer_stop(timer_t *timer)
{
    if (timer->status == Timer_Running) {
        __disable_irq();

        timer->stop_tick = os_get_current_tick();
        timer->status = Timer_Stop;
        __os_list_detach(&timer_queue, &timer->list);

        __enable_irq();
    }
    else 
        return OS_error;

    return os_ok;
}

status_t os_timer_resume(timer_t *timer)
{
    if (timer->status == Timer_Stop) {
        __disable_irq();

        timer->timeout_tick += os_get_current_tick() - timer->stop_tick; 
        timer->status = Timer_Running;
        os_timer_add_to_list(timer);

        __enable_irq();
    } 
    else 
        return OS_error;

    return os_ok;
}

status_t os_timer_start(timer_t *timer, uint32_t delay)
{
    __disable_irq();

    timer->status = Timer_Running;
    timer->timeout_tick = os_get_current_tick() + delay;    

    os_timer_add_to_list(timer);

    __enable_irq();

    return os_ok;
}

status_t os_timer_finish(timer_t *timer)
{
    __disable_irq();

    timer->status = Timer_Leisure;
    timer->timeout_tick = 0;

    if (timer->status == Timer_Running) 
        __os_list_detach(&timer_queue, &timer->list);
    
    __enable_irq();

    return os_ok;
}


status_t os_timer_check(void)
{
    if (timer_queue == NULL)
        return OS_error;

    list_t *list = timer_queue;
    timer_t *timer;

    // check if the previous timer has timed out
    do {
        timer = __CAST_LIST_TO_TIMER(list);

        // this timer already overtime
        if (timer->timeout_tick <= os_get_current_tick()) {
            // save next timer
            list = list->next;    

            timer->status = Timer_Timeout;
            __os_list_detach(&timer_queue, &timer->list);            
            os_thread_wakeup(__CAST_TIMER_TO_TCB(timer));
                
        } else {
            // other timers isn't overtime
            break;
        }
    } while (timer_queue != NULL && list != timer_queue);

    return os_ok;
}

status_t os_timer_add_to_list(timer_t *timer)
{
    __disable_irq();

    // if timer_queue is NULL or timeout_tick of timer to jion is smaller than first timer
    if (timer_queue == NULL || __CAST_LIST_TO_TIMER(timer_queue)->timeout_tick > timer->timeout_tick) {
        __os_list_add_first(&timer_queue, &timer->list);

        __enable_irq();
        return os_ok;
    }
    
    list_t *list = timer_queue->next;
    // traverse and compare all the timer
    while (list != timer_queue)
    {
        if (__CAST_LIST_TO_TIMER(list)->timeout_tick > timer->timeout_tick){
            __os_list_add_before(list, &timer->list);

            __enable_irq();
            return os_ok;
        }
        list = list->next;
    }
    // if none smaller than this, add to the end of queue
    __os_list_add_end(&timer_queue, &timer->list);

    __enable_irq();
    return os_ok;
}
