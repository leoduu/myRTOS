/*
 * @Author: your name
 * @Date: 2021-03-03 22:04:10
 * @LastEditTime: 2021-04-30 12:36:43
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \MDK-ARMf:\project\myRTOS\nucleo-64\hello\RTOS\Src\service.c
 */
#include "osdef.h"

extern UART_HandleTypeDef huart3;
uint8_t ch;

int fputc(int c, FILE * f)
{
    ch = c;
    HAL_UART_Transmit(&huart3, &ch, 1, 0xFF);  //发送串口
    return c;
}

#ifdef  OS_ASSERT

void os_assert_failed(uint8_t *file, uint32_t line) 
{
    DEBUG_ASSERT(("--assert: file:%s line:%d\n", file, line));
    __disable_irq();
    while (1);
    //__enable_irq();      
}

#endif 


/**
 * @description: add node to the end of list
 * @param {list_t} **l  list   
 * @param {list_t} *n   node
 * @return {status_t}
 */
status_t __os_list_add_end(list_t **l, list_t *n)
{
    if (NULL == n) 
        return OS_error;

    if (NULL == *l){
        *l = n;
    } else {
        n->prev = (*l)->prev;
        n->next = (*l); 
        (*l)->prev->next = n;
        (*l)->prev = n;      
    }

    return os_ok;
}

status_t __os_list_add_first(list_t **l, list_t *n)
{
    if (NULL == n) 
        return OS_error;
        
    if (NULL == *l){
        *l = n;
    } else {
        n->prev = (*l)->prev;
        n->next = (*l);
        (*l)->prev->next = n;
        (*l)->prev = n;
        (*l) = n;
    }

    return os_ok;
}

status_t __os_list_add_after(list_t *l, list_t *n)
{
    if (NULL == l || NULL == n) 
        return OS_error;

    n->prev = l;
    n->next = l->next;
    
    if (l->next != NULL) 
        l->next->prev = n;
    l->next = n;
    

    return os_ok;
}

status_t __os_list_add_before(list_t *l, list_t *n)
{
    if (NULL == l || NULL == n) 
        return OS_error;
        
    n->prev = l->prev;
    n->next = l;
    if (l->prev != NULL)
        l->prev->next = n;
    l->prev = n;        

    return os_ok;
}

status_t __os_list_detch_after(list_t *l, list_t *n)
{
    if (NULL == l || NULL == n) 
        return OS_error;
        
    if (n->next != NULL)
        n->next->prev = l;
    l->next = n->next;

    n->next = n;
    n->prev = n;  

    return os_ok;
}

/**
 * @description: detach node from list, 
 *               and to judge the list is empty or not
 * @param {list_t} **l  list
 * @param {list_t} *n   node
 * @return {status_t}
 */
status_t __os_list_detach(list_t **l, list_t *n)
{
    if ( NULL == *l ) 
        return OS_error;
		
    if (n->next == n) {
        *l = NULL;
    } else {
        if ( *l == n ) *l = n->next;

        n->prev->next = n->next;
        n->next->prev = n->prev;
    }

    n->next = n;
    n->prev = n;
    
    return os_ok;
}

status_t __os_list_detach_first(list_t **l)
{
    if ( NULL == *l ) 
        return OS_error;
		
    if ((*l)->next == *l) {
        *l = NULL;
    } else {
        *l = (*l)->next;
    }
    
    return os_ok;
}
