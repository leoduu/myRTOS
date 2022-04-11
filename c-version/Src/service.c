/*
 * @Author: your name
 * @Date: 2021-03-03 22:04:10
 * @LastEditTime: 2021-05-09 01:08:15
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \MDK-ARMf:\project\myRTOS\nucleo-64\hello\RTOS\Src\service.c
 */
#include "osdef.h"
#include <stdarg.h>
#include "ipc.h"

extern UART_HandleTypeDef huart3;
uint8_t ch;

// int fputc(int c, FILE * f)
// {
//     ch = c;
//     HAL_UART_Transmit(&huart3, &ch, 1, 0xFF);  //发送串口
//     return c;
// }

int os_putc(char c)
{
    ch = c;
    HAL_UART_Transmit(&huart3, &ch, 1, 0x01);  //发送串口
    return c;
}

static uint32_t os_vsprintf(char *buf, const char *fmt, va_list va_ptr)
{
    int value, i;
    uint32_t len = 0;
    char *str = NULL;
    char num[20];    
	const char *digits="0123456789ABCDEF";

    while (*fmt != '\0') {

        if (*fmt == '%') {
            switch (*(++fmt)) {
            case 'd':
                value = va_arg(va_ptr, int);
                i = 0;
                do {
                    num[i++] = value % 10;
                    value /= 10;
                } while (value);
                while(i) buf[len++] = digits[num[--i]];
                break;   
            
            case 'c':
                buf[len++] = va_arg(va_ptr, int);
                break;

            case 's':
                str = va_arg(va_ptr, char*);
                while (*str) buf[len++] = *str++;
                break;
            
            case 'x':
                value = va_arg(va_ptr, int);
                i = 0;
                while (value) {
                    num[i++] = value & 0xF;
                    value >>= 4;
                }
                while(i) buf[len++] = digits[num[--i]]; 

            default:
                break;
            }
        }
        else {
            buf[len++] = *fmt;
        }
        ++fmt;
    }
    return len;
}

void os_printf(const char *fmt, ...)
{
    va_list va_ptr; 
    char data[80] = {'\0'};
    uint32_t len;

    va_start(va_ptr, fmt);

    len = os_vsprintf(data, fmt, va_ptr);

    for (uint8_t i=0; i<len; i++) {
        os_putc(data[i]);
    }

    va_end(va_ptr);    
}

void os_printf_delay(const char *fmt, ...)
{
    va_list va_ptr; 
    char data[80] = {'\0'};
    uint32_t len;

    va_start(va_ptr, fmt);

    len = os_vsprintf(data, fmt, va_ptr);

    os_mqueue_send(DEBUG_DELAY, data, len, NOWAIT);

    va_end(va_ptr);    
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
