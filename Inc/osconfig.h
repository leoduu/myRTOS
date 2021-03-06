/*
 * @Author: your name
 * @Date: 2021-02-22 07:08:25
 * @LastEditTime: 2021-05-09 14:11:09
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \MDK-ARMf:\project\myRTOS\nucleo-64\hello\RTOS\Inc\osconfig.h
 */
#ifndef __OS_CONFIG_H__
#define __OS_CONFIG_H__

#include "stdint.h"

#define THREAD_NAME_LEN         10
#define THREAD_PRIORITY_MAX     16

#define DEBUG_FLAG          1
#define DEBUG_USER_FLAG     0   
#define ASSERT_FLAG         1

#define MUTEX_FLAG          1
#define SEMAPHORE_FLAG      1
#define MAILBOX_FLAG        1
#define MSG_QUEUE_FLAG      1


////////////////////////////////////
// ipc
#if MUTEX_FLAG == 1
    #define OS_MUTEX
#endif

#if SEMAPHORE_FLAG == 1
    #define OS_SEMAPHORE
#endif

#if MAILBOX_FLAG == 1
    #define OS_MAILBOX
#endif

#if MSG_QUEUE_FLAG == 1
    #define OS_MSG_QUEUE
#endif


////////////////////////////////////
// debug
#define DEBUG_DELAY  1


#if DEBUG_FLAG == 1
    #define DEBUG_LOG(msg) \
        os_printf_delay("[%d] ", os_tick); \
        os_printf_delay("[%ld] ", SysTick->VAL); \
        os_printf_delay msg
#else 
    #define DEBUG_LOG(msg)
#endif

#if DEBUG_USER_FLAG == 1
    #define DEBUG_USER(msg) os_printf msg
#else 
    #define DEBUG_USER(msg)
#endif

#if ASSERT_FLAG == 1
    #define OS_ASSERT
    #define DEBUG_ASSERT(msg) os_printf msg
#else
    
    #define DEBUG_ASSERT(msg)
#endif
//////////////////////////////////
// systick frequence
#define OS_TICK_PER_SECOND 1000

#endif
