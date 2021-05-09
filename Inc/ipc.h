/*
 * @Author: your name
 * @Date: 2021-03-16 20:58:03
 * @LastEditTime: 2021-05-09 01:04:44
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \MDK-ARMf:\project\myRTOS\nucleo-64\hello\RTOS\Inc\ipc.h
 */
#ifndef __IPC_H__
#define __IPC_H__

#include "osdef.h"

#ifdef OS_MUTEX 
///////////////////////////////////////////////////////////////////////////////////////////////
//  mutex

#define __CAST_LIST_TO_MUTEX(addr)      ((os_mutex_t *)((size_t)(addr)))

typedef struct {
    list_t      list;

    uint8_t     id;
    uint8_t     value;
    TCB_t       *own_thread;
    uint8_t     orignal_prio;
    list_t      *suspend_queue;
} os_mutex_t;

// static os_mutex_t *__os_mutex_find_id(uint32_t id);
uint8_t os_mutex_id_check(uint32_t id);
status_t os_mutex_create(uint32_t id);
status_t os_mutex_acquire(uint32_t id, int delay);
status_t os_mutex_release(uint32_t id);
status_t os_mutex_delete(uint32_t id);

#endif // OS_MUTEX 


#ifdef OS_SEMAPHORE
///////////////////////////////////////////////////////////////////////////////////////////////
//  semaphore

#define __CAST_LIST_TO_SEM(addr)        ((os_sem_t *)((size_t)(addr)))

typedef struct {
    list_t      list;
    uint8_t     id;
    uint8_t     value;
        
    list_t      *suspend_queue;
} os_sem_t;

// static os_sem_t *__os_sem_find_id(uint32_t id);
int8_t   __os_sem_get_value(uint32_t id);
status_t os_sem_create(uint32_t id, int value);
status_t os_sem_acquire(uint32_t id, int delay);
status_t os_sem_release(uint32_t id);
status_t os_sem_delete(uint32_t id);

#endif // OS_SEMAPHORE


#ifdef OS_MAILBOX
///////////////////////////////////////////////////////////////////////////////////////////////
//  mbox

#define __CAST_LIST_TO_MAILBOX(addr)    ((os_mbox_t *)((size_t)(addr)))

typedef struct 
{
    list_t      list;
    uint32_t    data;
} os_mbuf_t;

typedef struct {
    list_t      list;
    uint8_t     id;

    list_t      *buf_list;
    uint32_t    max_num;
    uint8_t     num;

    list_t      *send_queue;
    list_t      *recv_queue;
} os_mbox_t;

uint8_t os_mbox_id_check(uint32_t id);
status_t os_mbox_create(uint32_t id, uint32_t buf_size);
status_t os_mbox_send(uint32_t id, uint32_t data, int delay);
status_t os_mbox_recv(uint32_t id, uint32_t *data, int delay);
status_t os_mbox_delete(uint32_t id);

#endif // OS_MAILBOX

#ifdef OS_MSG_QUEUE
///////////////////////////////////////////////////////////////////////////////////////////////
//  message_queue

#define __CAST_LIST_TO_MSG_QUEUE(addr)  ((os_mqueue_t *)((size_t)(addr)))

typedef struct 
{
    list_t      list;
    void        *buf;
    uint16_t    size;
} os_msg_t;

typedef struct 
{
    list_t      list;
    uint16_t    size;
} os_send_size_t;

typedef struct {
    list_t      list;
    uint8_t     id;

    list_t      *buf_list;
    uint32_t    max_size;
    uint16_t     size;

    list_t      *send_queue;
    list_t      *size_queue;
    list_t      *recv_queue;
} os_mqueue_t;

status_t os_mqueue_create(uint32_t id, uint32_t buf_size);
status_t os_mqueue_send(uint32_t id, void *buf, uint32_t size, int delay);
status_t os_mqueue_recv(uint32_t id, void *buf, uint32_t* len, int delay);
status_t os_mqueue_delete(uint32_t id);

#endif // OS_MSG_QUEUE


#endif
