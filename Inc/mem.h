/*
 * @Author: your name
 * @Date: 2021-04-21 10:18:11
 * @LastEditTime: 2021-05-03 15:43:31
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \MDK-ARMd:\project\myRTOS\nucleo-64\hello\RTOS\Inc\mem.h
 */
#ifndef _MEM_H_
#define _MEM_H_

#include "stdint.h"
#include "stdio.h"
#include "string.h"
#include "osdef.h"

#define MEM_FREE   0xabc0
#define MEM_USED   0xabc1
#define MEM_END    0xabc2

// magic(3Bytes for verify and 1Bytes for useing flag) 
// list(8Byte)
typedef struct mem_node
{
    uint32_t magic;    // 0xabc + 1Byte flag
    struct mem_node *prev;
    struct mem_node *next;

} os_mem_t;


void os_align(uint32_t *addr, uint8_t flag);

void os_mem_init(void);

void *os_malloc(const uint32_t size);
void os_free(void *p);
void os_mem_show(void);


#endif
