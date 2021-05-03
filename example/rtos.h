/*
 * @Author: your name
 * @Date: 2021-03-04 13:43:44
 * @LastEditTime: 2021-03-17 21:15:49
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \MDK-ARMf:\project\myRTOS\nucleo-64\hello\Core\Inc\rtos.h
 */
#ifndef __RTOS_H__
#define __RTOS_H__

#include "osdef.h"
#include "ipc.h"


void thread1_func(void);
void thread2_func(void);
void thread3_func(void);
void test_thread_init(void);

#endif
