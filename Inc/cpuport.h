/*
 * @Author: your name
 * @Date: 2021-02-22 06:23:23
 * @LastEditTime: 2021-03-26 13:53:50
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \MDK-ARMf:\project\myRTOS\nucleo-64\hello\RTOS\Inc\cpuport.h
 */
#ifndef _CPU_PORT_H_
#define _CPU_PORT_H_

void os_thread_start_first_thread(unsigned int *thread_sp);

void os_thread_switch(unsigned int *current, unsigned int *next);

unsigned int os_SysTick_Config(unsigned int ticks);

#endif
