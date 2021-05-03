/*
 * @Author: your name
 * @Date: 2021-03-04 13:43:31
 * @LastEditTime: 2021-04-30 11:38:33
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \MDK-ARMf:\project\myRTOS\nucleo-64\hello\Core\Src\rtos.c
 */
#include "rtos.h"

TCB_t thread1;
TCB_t thread2;

uint32_t thread1_stack[256];
uint32_t thread2_stack[256];

const uint8_t thread1_prio = 3; 
const uint8_t thread2_prio = 3; 

extern UART_HandleTypeDef huart3;

#define MBOX_ID     1
#define MBOX_NUM    5

void thread1_func(void)
{
    while (1) {     
        
        if (!os_mbox_id_check(MBOX_ID)) {
            // send 5 mails per 1 second
            for (int i=0; i<5; i++) {
                os_mbox_send(MBOX_ID, i, NOWAIT); 
                DEBUG_USER(("t1 send %d\n", i)); 
                os_delay(1000);
            }
            // and then delete the mailbox
            os_mbox_delete(MBOX_ID);
            DEBUG_USER(("t1 mbox delete\n"));  
        } 
        else {
            os_delay(2000);
            DEBUG_USER(("t1 no mbox\n"));  
        }        
    }    
}

void thread2_func(void)
{
    uint32_t temp;

    while (1) {

        switch (os_mbox_recv(MBOX_ID, &temp, FOREVER)) {
        
        case os_ok:
            DEBUG_USER(("t2 recv %d\n", temp));
            break;
        case os_timeout:
            DEBUG_USER(("t2 recv timeout\n"));
            break;
        default:
            break;
        }

        if (!os_mbox_id_check(MBOX_ID)) {
            DEBUG_USER(("t2 no mbox\n"));
            os_delay(2000);
        }
			
		//os_delay(2000);
    }    
}


void test_thread_init(void)
{
    os_thread_init(&thread1, "Thread1", thread1_func, NULL, 
		            thread1_prio, thread1_stack, 1024);	

    os_thread_init(&thread2, "Thread2", thread2_func, NULL,
		            thread2_prio, thread2_stack, 1024);     
      
    os_thread_ready(&thread1);
    os_thread_ready(&thread2);

    os_mbox_create(MBOX_ID, MBOX_NUM);
}
